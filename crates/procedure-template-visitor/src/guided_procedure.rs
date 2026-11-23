//! Records a finished run of a procedure template, one procedure at a time in
//! the order its steps run.

mod error;
mod slots;

use aps_asset_model_ancestors::{
    ProcedureAsset, procedure_assets, procedure_template_slot_bindings,
};
use aps_procedure_templates::{
    ProcedureTemplateSlot, procedure_template_slots, procedure_template_steps, procedure_templates,
};
use aps_procedures::procedures;
use aps_resources::{ownables, resources};
use aps_traits::validation::{IntoValidationError, flatten, validated};
use diesel::{
    Connection, ExpressionMethods, QueryDsl, RunQueryDsl,
    dsl::{Eq, Find, Update},
    query_dsl::methods::ExecuteDsl,
};
use diesel_builders::{
    BuildableTable, GetColumnExt, Insert, MayGetColumnExt, SetColumn, SetColumnExt, TableBuilder,
    TableExt, TrySetColumn, TrySetColumnExt, ValidateColumn,
};
pub use error::GuidedProcedureError;
use procedure_like::ProcedureTableLike;
use rosetta_utc::TimestampUTC;
use rosetta_uuid::Uuid;
use slots::OwnRow;
pub use slots::Slots;

use crate::{ProcedureTemplateGraph, StepNode, TemplateNode};

/// The statement completing a procedure.
pub type Complete = Update<
    Find<procedures::table, Uuid>,
    (Eq<procedures::status_id, &'static str>, Eq<procedures::ended_at, TimestampUTC>),
>;

/// The statement skipping a step's procedure.
pub type Skip = Update<
    Find<procedures::table, Uuid>,
    (
        Eq<procedures::status_id, &'static str>,
        Eq<procedures::ended_at, TimestampUTC>,
        Eq<procedures::note, Option<String>>,
    ),
>;

/// A builder of a procedure table, which the walk fills with the columns
/// placing the procedure in its run and then inserts.
pub trait ProcedureBuilder<C>:
    SetColumn<resources::id>
    + SetColumn<resources::organization_id>
    + SetColumn<procedures::organization_id>
    + SetColumn<procedures::project_id>
    + SetColumn<ownables::organization_id>
    + SetColumn<ownables::project_id>
    + SetColumn<ownables::created_by_id>
    + SetColumn<procedures::table_name_id>
    + SetColumn<procedures::procedure_template_id>
    + SetColumn<procedures::status_id>
    + SetColumn<procedures::parent_procedure_id>
    + SetColumn<procedures::parent_template_id>
    + SetColumn<procedures::step_id>
    + SetColumn<procedures::root_procedure_id>
    + SetColumn<procedures::repeat_procedure_asset_id>
    + SetColumn<procedures::performed_by_id>
    + TrySetColumn<procedures::ended_at>
    + ValidateColumn<procedures::ended_at, Error: IntoValidationError>
    + diesel_builders::MayGetColumn<procedures::ended_at>
    + Insert<C, ValidationError: IntoValidationError>
    + Sized
{
}

impl<C, B> ProcedureBuilder<C> for B where
    B: SetColumn<resources::id>
        + SetColumn<resources::organization_id>
        + SetColumn<procedures::organization_id>
        + SetColumn<procedures::project_id>
        + SetColumn<ownables::organization_id>
        + SetColumn<ownables::project_id>
        + SetColumn<ownables::created_by_id>
        + SetColumn<procedures::table_name_id>
        + SetColumn<procedures::procedure_template_id>
        + SetColumn<procedures::status_id>
        + SetColumn<procedures::parent_procedure_id>
        + SetColumn<procedures::parent_template_id>
        + SetColumn<procedures::step_id>
        + SetColumn<procedures::root_procedure_id>
        + SetColumn<procedures::repeat_procedure_asset_id>
        + SetColumn<procedures::performed_by_id>
        + TrySetColumn<procedures::ended_at>
        + ValidateColumn<procedures::ended_at, Error: IntoValidationError>
        + diesel_builders::MayGetColumn<procedures::ended_at>
        + Insert<C, ValidationError: IntoValidationError>
{
}

/// A procedure recorded and not yet completed, whose steps the walk is in.
#[derive(Debug)]
struct Frame<'graph> {
    /// Template of the procedure.
    template: &'graph TemplateNode,
    /// Identifier of the procedure.
    procedure: Uuid,
    /// Time the procedure ended, written when the walk leaves it.
    ended_at: TimestampUTC,
    /// The rows binding the procedure's assets.
    rows: Vec<ProcedureAsset>,
    /// Index of the step the walk records next.
    next_step: usize,
    /// Runs of the next step recorded, for a step repeated per asset.
    runs: usize,
}

/// The procedure the walk records next.
#[derive(Debug, Clone, Copy)]
enum Next<'graph> {
    /// The root of the run.
    Root,
    /// A run of `step` of the procedure on top of the walk, for the parent
    /// row `repeat` when the step repeats.
    Step {
        /// The step.
        step: &'graph StepNode,
        /// The parent row of the repeat slot the run is for.
        repeat: Option<Uuid>,
    },
    /// Nothing, the run is recorded.
    Done,
}

/// Records a finished run of a procedure template graph. Each call records
/// the next procedure in the order the steps run, the root first, then the
/// steps of each procedure in `position` order, each repeated step once per
/// asset its parent bound to its repeat slot. A procedure is inserted
/// `in_progress`, binds the assets its step hands down and those its closure
/// gives, and is completed with the `ended_at` its closure set once the walk
/// has recorded its steps.
#[derive(Debug)]
pub struct GuidedProcedure<'graph, C> {
    /// The template graph the run follows.
    graph: &'graph ProcedureTemplateGraph,
    /// The connection the rows are written through.
    connection: &'graph mut C,
    /// Organization holding the run.
    organization: Uuid,
    /// Project holding the run.
    project: Uuid,
    /// User recording the run, and by default performing it.
    author: Uuid,
    /// Identifier of the root procedure, once recorded.
    root: Option<Uuid>,
    /// The procedures recorded and not yet completed, the root first.
    frames: Vec<Frame<'graph>>,
}

impl<'graph, C> GuidedProcedure<'graph, C>
where
    C: Connection,
    Complete: ExecuteDsl<C>,
    TableBuilder<resources::table>: Insert<C, ValidationError: IntoValidationError>,
    TableBuilder<procedure_assets::table>: Insert<C, ValidationError: IntoValidationError>,
{
    /// A run of `graph` in `project` of `organization`, recorded by `author`
    /// through `connection`.
    #[must_use]
    pub fn new(
        graph: &'graph ProcedureTemplateGraph,
        organization: Uuid,
        project: Uuid,
        author: Uuid,
        connection: &'graph mut C,
    ) -> Self {
        Self { graph, connection, organization, project, author, root: None, frames: Vec::new() }
    }

    /// Records the next procedure as a row of `T`. The walk sets the columns
    /// placing it in the run, `performed_by_id` to the author and `status_id`
    /// to `in_progress`. `complete` sets `started_at`, `ended_at`, any other
    /// performer and the measurements, and binds through its [`Slots`] the
    /// slots the step does not bind.
    ///
    /// # Errors
    ///
    /// Returns the error of `complete`, or a [`GuidedProcedureError`] when
    /// the run is recorded, `T` does not run the next template, `complete`
    /// left `ended_at` unset or the connection refuses a row.
    pub fn and_then<T, E>(
        mut self,
        complete: impl FnOnce(TableBuilder<T>, &mut Slots<'graph>, &mut C) -> Result<TableBuilder<T>, E>,
    ) -> Result<Self, E>
    where
        T: ProcedureTableLike + BuildableTable<DefaultError: IntoValidationError>,
        TableBuilder<T>: ProcedureBuilder<C>,
        E: From<GuidedProcedureError>,
    {
        let (template, step, repeat) = self.next_procedure::<T>()?;
        let procedure = self.procedure_builder::<T>(template, step, repeat, "in_progress")?;
        let mut slots = Slots::new(template, step);
        let builder = complete(procedure.builder, &mut slots, self.connection)?;
        let ended_at =
            builder.may_get_column_ref::<procedures::ended_at>().cloned().flatten().ok_or_else(
                || {
                    GuidedProcedureError::MissingEndedAt {
                        template: template.template().get_column::<procedure_templates::name>(),
                    }
                },
            )?;
        let builder =
            validated(builder.try_set_column::<procedures::ended_at>(None::<TimestampUTC>))
                .map_err(GuidedProcedureError::from)?;
        flatten(builder.insert(self.connection)).map_err(GuidedProcedureError::from)?;
        let mut rows = self.inherited_rows(procedure.id, template, step, repeat)?;
        for own in slots.into_rows() {
            rows.push(self.bind(procedure.id, template, own, None)?);
        }
        self.advance(step);
        self.frames.push(Frame {
            template,
            procedure: procedure.id,
            ended_at,
            rows,
            next_step: 0,
            runs: 0,
        });
        Ok(self)
    }

    /// Records the next procedure, a run of a step, as a row of `T` skipped
    /// at `ended_at`, with `note` giving the reason a step the template does
    /// not mark `optional` needs.
    ///
    /// # Errors
    ///
    /// Returns an error when the run is recorded, the next procedure is the
    /// root, `T` does not run the next template or the connection refuses a
    /// row.
    pub fn skip<T>(
        mut self,
        ended_at: TimestampUTC,
        note: Option<String>,
    ) -> Result<Self, GuidedProcedureError>
    where
        T: ProcedureTableLike + BuildableTable<DefaultError: IntoValidationError>,
        TableBuilder<T>: ProcedureBuilder<C>,
        Skip: ExecuteDsl<C>,
    {
        let (template, step, repeat) = self.next_procedure::<T>()?;
        if step.is_none() {
            return Err(GuidedProcedureError::RootSkipped {
                template: template.template().get_column::<procedure_templates::name>(),
            });
        }
        let procedure = self.procedure_builder::<T>(template, step, repeat, "planned")?;
        let builder = procedure.builder.set_column::<procedures::performed_by_id>(None::<Uuid>);
        flatten(builder.insert(self.connection))?;
        diesel::update(procedures::table.find(procedure.id))
            .set((
                procedures::status_id.eq("skipped"),
                procedures::ended_at.eq(ended_at),
                procedures::note.eq(note),
            ))
            .execute(self.connection)?;
        self.advance(step);
        Ok(self)
    }

    /// Completes the procedures the walk is still in, and checks that every
    /// procedure of the run is recorded.
    ///
    /// # Errors
    ///
    /// Returns an error when a procedure is still to record or the connection
    /// refuses completing one.
    pub fn finish(mut self) -> Result<(), GuidedProcedureError> {
        match self.next()? {
            Next::Done => Ok(()),
            Next::Root => {
                Err(GuidedProcedureError::Unrecorded {
                    template: self
                        .graph
                        .root()
                        .template()
                        .get_column::<procedure_templates::name>(),
                })
            }
            Next::Step { step, .. } => {
                Err(GuidedProcedureError::Unrecorded {
                    template: self
                        .graph
                        .child_of(step)
                        .template()
                        .get_column::<procedure_templates::name>(),
                })
            }
        }
    }

    /// The procedure the walk records next, completing on the way every
    /// procedure whose steps are all recorded.
    fn next(&mut self) -> Result<Next<'graph>, GuidedProcedureError> {
        if self.root.is_none() {
            return Ok(Next::Root);
        }
        while let Some(frame) = self.frames.last_mut() {
            let template = frame.template;
            if let Some(step) = template.task_graph().steps().get(frame.next_step) {
                let Some(repeat_slot) =
                    step.step().get_column::<procedure_template_steps::repeat_slot_id>()
                else {
                    return Ok(Next::Step { step, repeat: None });
                };
                if let Some(row) = frame
                    .rows
                    .iter()
                    .filter(|row| row.get_column::<procedure_assets::slot_id>() == repeat_slot)
                    .nth(frame.runs)
                {
                    return Ok(Next::Step {
                        step,
                        repeat: Some(row.get_column::<procedure_assets::id>()),
                    });
                }
                frame.next_step += 1;
                frame.runs = 0;
                continue;
            }
            let Frame { procedure, ended_at, .. } =
                self.frames.pop().expect("the loop holds the last frame");
            diesel::update(procedures::table.find(procedure))
                .set((procedures::status_id.eq("completed"), procedures::ended_at.eq(ended_at)))
                .execute(self.connection)?;
        }
        Ok(Next::Done)
    }

    /// The template the next procedure runs, the step it runs and the repeat
    /// row it is for, checking that `T` runs that template.
    fn next_procedure<T: ProcedureTableLike>(
        &mut self,
    ) -> Result<(&'graph TemplateNode, Option<&'graph StepNode>, Option<Uuid>), GuidedProcedureError>
    {
        let (template, step, repeat) = match self.next()? {
            Next::Done => return Err(GuidedProcedureError::NoMoreProcedures),
            Next::Root => (self.graph.root(), None, None),
            Next::Step { step, repeat } => (self.graph.child_of(step), Some(step), repeat),
        };
        let expected = <T::ProcedureTemplateTable as TableExt>::TABLE_NAME;
        let found = template.template().get_column_ref::<procedure_templates::table_name_id>();
        if found != expected {
            return Err(GuidedProcedureError::UnexpectedTable {
                expected,
                found: found.clone(),
                template: template.template().get_column::<procedure_templates::name>(),
            });
        }
        Ok((template, step, repeat))
    }

    /// The builder of a new procedure, with the columns placing it in the run.
    fn procedure_builder<T>(
        &mut self,
        template: &'graph TemplateNode,
        step: Option<&'graph StepNode>,
        repeat: Option<Uuid>,
        status: &'static str,
    ) -> Result<NewProcedure<T>, GuidedProcedureError>
    where
        T: ProcedureTableLike + BuildableTable<DefaultError: IntoValidationError>,
        TableBuilder<T>: ProcedureBuilder<C>,
    {
        let id = Uuid::new_v4();
        let root = *self.root.get_or_insert(id);
        let parent = self.frames.last();
        let builder = validated(T::try_builder())?
            .set_column::<resources::id>(id)
            .set_column::<resources::organization_id>(self.organization)
            .set_column::<procedures::organization_id>(self.organization)
            .set_column::<procedures::project_id>(self.project)
            .set_column::<ownables::organization_id>(self.organization)
            .set_column::<ownables::project_id>(self.project)
            .set_column::<ownables::created_by_id>(self.author)
            .set_column::<procedures::table_name_id>(<T as TableExt>::TABLE_NAME.to_owned())
            .set_column::<procedures::procedure_template_id>(
                template.template().get_column::<procedure_templates::id>(),
            )
            .set_column::<procedures::status_id>(status.to_owned())
            .set_column::<procedures::parent_procedure_id>(parent.map(|frame| frame.procedure))
            .set_column::<procedures::parent_template_id>(
                parent
                    .map(|frame| frame.template.template().get_column::<procedure_templates::id>()),
            )
            .set_column::<procedures::step_id>(
                step.map(|step| step.step().get_column::<procedure_template_steps::id>()),
            )
            .set_column::<procedures::root_procedure_id>(root)
            .set_column::<procedures::repeat_procedure_asset_id>(repeat)
            .set_column::<procedures::performed_by_id>(Some(self.author));
        Ok(NewProcedure { id, builder })
    }

    /// Inserts the rows of the procedure `procedure` that `step` hands down
    /// from the parent's rows, only the repeat row for a slot bound to the
    /// repeat slot of a repeated run.
    fn inherited_rows(
        &mut self,
        procedure: Uuid,
        template: &'graph TemplateNode,
        step: Option<&'graph StepNode>,
        repeat: Option<Uuid>,
    ) -> Result<Vec<ProcedureAsset>, GuidedProcedureError> {
        let (Some(step), Some(parent)) = (step, self.frames.last()) else {
            return Ok(Vec::new());
        };
        let parent_procedure = parent.procedure;
        let mut handed: Vec<(&'graph ProcedureTemplateSlot, ProcedureAsset)> = Vec::new();
        for binding in step.bindings() {
            let slot = template
                .slot(binding.get_column::<procedure_template_slot_bindings::child_slot_id>())
                .expect("a binding names a slot of the step's template");
            let parent_slot =
                binding.get_column::<procedure_template_slot_bindings::parent_slot_id>();
            let repeated = step.step().get_column::<procedure_template_steps::repeat_slot_id>()
                == Some(parent_slot);
            handed.extend(
                parent
                    .rows
                    .iter()
                    .filter(|row| row.get_column::<procedure_assets::slot_id>() == parent_slot)
                    .filter(|row| {
                        !repeated || Some(row.get_column::<procedure_assets::id>()) == repeat
                    })
                    .map(|row| (slot, row.clone())),
            );
        }
        handed
            .into_iter()
            .map(|(slot, row)| {
                let own = OwnRow {
                    slot,
                    asset_model: row.get_column::<procedure_assets::asset_model_id>(),
                    asset: row.get_column::<procedure_assets::asset_id>(),
                };
                let parent_row = row.get_column::<procedure_assets::id>();
                self.bind(procedure, template, own, Some((parent_row, parent_procedure)))
            })
            .collect()
    }

    /// Inserts the row binding `own` to `procedure`, inherited from the
    /// parent row and procedure `parent` when given.
    fn bind(
        &mut self,
        procedure: Uuid,
        template: &'graph TemplateNode,
        own: OwnRow<'graph>,
        parent: Option<(Uuid, Uuid)>,
    ) -> Result<ProcedureAsset, GuidedProcedureError> {
        Ok(flatten(
            procedure_assets::table::builder()
                .set_column::<procedure_assets::id>(Uuid::new_v4())
                .set_column::<procedure_assets::organization_id>(self.organization)
                .set_column::<procedure_assets::procedure_id>(procedure)
                .set_column::<procedure_assets::procedure_template_id>(
                    template.template().get_column::<procedure_templates::id>(),
                )
                .set_column::<procedure_assets::status_id>("in_progress".to_owned())
                .set_column::<procedure_assets::slot_id>(
                    own.slot.get_column::<procedure_template_slots::id>(),
                )
                .set_column::<procedure_assets::slot_asset_model_id>(
                    own.slot.get_column::<procedure_template_slots::asset_model_id>(),
                )
                .set_column::<procedure_assets::slot_role_id>(
                    own.slot.get_column::<procedure_template_slots::role_id>(),
                )
                .set_column::<procedure_assets::asset_model_id>(own.asset_model)
                .set_column::<procedure_assets::asset_id>(own.asset)
                .set_column::<procedure_assets::parent_procedure_asset_id>(
                    parent.map(|(row, _)| row),
                )
                .set_column::<procedure_assets::parent_procedure_id>(
                    parent.map(|(_, procedure)| procedure),
                )
                .insert(self.connection),
        )?)
    }

    /// Moves the walk past the procedure just recorded for `step`.
    fn advance(&mut self, step: Option<&'graph StepNode>) {
        let (Some(step), Some(frame)) = (step, self.frames.last_mut()) else {
            return;
        };
        if step.step().get_column::<procedure_template_steps::repeat_slot_id>().is_some() {
            frame.runs += 1;
        } else {
            frame.next_step += 1;
        }
    }
}

/// A procedure about to be inserted.
struct NewProcedure<T: BuildableTable> {
    /// Its identifier.
    id: Uuid,
    /// Its builder, holding the columns placing it in the run.
    builder: TableBuilder<T>,
}
