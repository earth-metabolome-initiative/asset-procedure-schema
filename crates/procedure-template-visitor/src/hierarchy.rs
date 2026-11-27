//! The procedure templates a root template runs through its steps, each with
//! its slots, its parameters and the task graph of its steps.

use std::collections::BTreeMap;

use aps_procedures::{
    ProcedureTemplateParameter, procedure_template_parameters, procedure_template_step_parameters,
};
use aps_resources::{
    ProcedureTemplate, ProcedureTemplateSlot, procedure_template_slot_bindings,
    procedure_template_slots, procedure_template_step_edges, procedure_template_steps,
    procedure_templates,
};
use diesel_builders::{GetColumnExt, LoadFirst, LoadMany};
use rosetta_uuid::Uuid;

use crate::{StepNode, TaskGraph};

/// A procedure template with its slots, its parameters and the task graph of
/// its steps.
#[derive(Debug, Clone, PartialEq)]
pub struct TemplateNode {
    /// The template.
    template: ProcedureTemplate,
    /// The slots of the template, sorted by name and then by identifier.
    slots: Vec<ProcedureTemplateSlot>,
    /// The parameters of the template, sorted by name.
    parameters: Vec<ProcedureTemplateParameter>,
    /// The steps the template runs.
    task_graph: TaskGraph,
}

impl TemplateNode {
    /// The template.
    #[must_use]
    pub const fn template(&self) -> &ProcedureTemplate {
        &self.template
    }

    /// The slots of the template, sorted by name and then by identifier.
    #[must_use]
    pub fn slots(&self) -> &[ProcedureTemplateSlot] {
        &self.slots
    }

    /// The slot `id` of the template, if it declares one.
    #[must_use]
    pub fn slot(&self, id: Uuid) -> Option<&ProcedureTemplateSlot> {
        self.slots.iter().find(|slot| slot.get_column::<procedure_template_slots::id>() == id)
    }

    /// The parameters of the template, sorted by name.
    #[must_use]
    pub fn parameters(&self) -> &[ProcedureTemplateParameter] {
        &self.parameters
    }

    /// The steps the template runs.
    #[must_use]
    pub const fn task_graph(&self) -> &TaskGraph {
        &self.task_graph
    }

    /// Whether the template runs no step.
    #[must_use]
    pub fn is_leaf(&self) -> bool {
        self.task_graph.is_empty()
    }
}

/// A root procedure template and every template its steps run, directly or
/// through the steps of those templates. A template run by several steps is
/// loaded once.
#[derive(Debug, Clone, PartialEq)]
pub struct ProcedureTemplateGraph {
    /// Identifier of the root template.
    root: Uuid,
    /// The templates, by identifier.
    templates: BTreeMap<Uuid, TemplateNode>,
}

impl ProcedureTemplateGraph {
    /// Loads the graph of `root` through `conn`.
    ///
    /// # Errors
    ///
    /// Returns the error of the connection.
    ///
    /// # Example
    ///
    /// ```rust
    /// use aps_test_utils::{Replica, Tenant};
    /// use procedure_template_visitor::ProcedureTemplateGraph;
    ///
    /// let mut replica = Replica::new();
    /// let tenant = replica.deliver(|conn| Ok(Tenant::insert(conn))).unwrap();
    /// replica.act_as(Some(tenant.user()));
    /// let graph = replica
    ///     .write(|conn| {
    ///         let pizza = tenant.pizza(&["Add toppings"], conn);
    ///         ProcedureTemplateGraph::new(pizza.template(), conn)
    ///     })
    ///     .unwrap();
    /// assert_eq!(graph.number_of_procedure_templates(), 4);
    /// assert!(graph.is_simple_path());
    /// ```
    pub fn new<C>(
        root: &impl diesel_builders::GetColumn<procedure_templates::id>,
        conn: &mut C,
    ) -> diesel::QueryResult<Self>
    where
        (procedure_templates::id,): LoadFirst<C>,
        (procedure_template_slots::procedure_template_id,): LoadMany<C>,
        (procedure_template_steps::parent_template_id,): LoadMany<C>,
        (procedure_template_step_edges::parent_template_id,): LoadMany<C>,
        (procedure_template_slot_bindings::parent_template_id,): LoadMany<C>,
        (procedure_template_parameters::procedure_template_id,): LoadMany<C>,
        (procedure_template_step_parameters::parent_template_id,): LoadMany<C>,
    {
        let root = root.get_column();
        let mut templates = BTreeMap::new();
        let mut pending = vec![root];
        while let Some(id) = pending.pop() {
            if templates.contains_key(&id) {
                continue;
            }
            let template = <(procedure_templates::id,)>::load_first((id,), conn)?;
            let mut slots =
                <(procedure_template_slots::procedure_template_id,)>::load_many((id,), conn)?;
            slots.sort_unstable_by(|left, right| {
                left.get_column_ref::<procedure_template_slots::name>()
                    .cmp(right.get_column_ref::<procedure_template_slots::name>())
                    .then_with(|| {
                        left.get_column_ref::<procedure_template_slots::id>()
                            .cmp(right.get_column_ref::<procedure_template_slots::id>())
                    })
            });
            let mut parameters =
                <(procedure_template_parameters::procedure_template_id,)>::load_many((id,), conn)?;
            parameters.sort_unstable_by(|left, right| {
                left.get_column_ref::<procedure_template_parameters::name>()
                    .cmp(right.get_column_ref::<procedure_template_parameters::name>())
            });
            let steps = <(procedure_template_steps::parent_template_id,)>::load_many((id,), conn)?;
            pending.extend(
                steps
                    .iter()
                    .map(|step| step.get_column::<procedure_template_steps::child_template_id>()),
            );
            let edges =
                <(procedure_template_step_edges::parent_template_id,)>::load_many((id,), conn)?;
            let bindings =
                <(procedure_template_slot_bindings::parent_template_id,)>::load_many((id,), conn)?;
            let fixed_parameters =
                <(procedure_template_step_parameters::parent_template_id,)>::load_many(
                    (id,),
                    conn,
                )?;
            templates.insert(
                id,
                TemplateNode {
                    template,
                    slots,
                    parameters,
                    task_graph: TaskGraph::new(steps, edges, bindings, fixed_parameters),
                },
            );
        }
        Ok(Self { root, templates })
    }

    /// The root template.
    #[must_use]
    pub fn root(&self) -> &TemplateNode {
        self.template(self.root)
    }

    /// The template `id`.
    ///
    /// # Panics
    ///
    /// Panics when `id` is not a template of the graph.
    #[must_use]
    pub fn template(&self, id: Uuid) -> &TemplateNode {
        self.templates.get(&id).unwrap_or_else(|| panic!("template {id} is not in the graph"))
    }

    /// The template `step` runs.
    #[must_use]
    pub fn child_of(&self, step: &StepNode) -> &TemplateNode {
        self.template(step.step().get_column::<procedure_template_steps::child_template_id>())
    }

    /// The template `step` belongs to.
    #[must_use]
    pub fn parent_of(&self, step: &StepNode) -> &TemplateNode {
        self.template(step.step().get_column::<procedure_template_steps::parent_template_id>())
    }

    /// The templates of the graph, sorted by identifier.
    pub fn templates(&self) -> impl Iterator<Item = &TemplateNode> {
        self.templates.values()
    }

    /// The number of distinct templates in the graph, the root included.
    #[must_use]
    pub fn number_of_procedure_templates(&self) -> usize {
        self.templates.len()
    }

    /// Whether the steps of every template form one chain.
    #[must_use]
    pub fn is_simple_path(&self) -> bool {
        self.templates.values().all(|node| node.task_graph.is_simple_path())
    }
}
