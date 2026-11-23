//! The Rust code a caller writes to record a run of a procedure template
//! through a [`GuidedProcedure`](crate::GuidedProcedure).

use core::fmt::Write;

use aps_procedure_templates::{
    ProcedureTemplateSlot, procedure_template_slots, procedure_template_steps, procedure_templates,
};
use diesel_builders::GetColumnExt;

use crate::{ProcedureTemplateGraph, StepNode, TemplateNode, Visitor, walk_template};

/// Writes the chain of [`GuidedProcedure`](crate::GuidedProcedure) calls
/// recording a run of a template graph: one `and_then` per procedure in the
/// order the walk asks for them, each saying which slots its closure binds.
#[derive(Debug, Clone, Copy)]
pub struct GuidedProcedurePseudocode<'graph> {
    /// The graph the run follows.
    graph: &'graph ProcedureTemplateGraph,
}

impl<'graph> GuidedProcedurePseudocode<'graph> {
    /// The pseudocode of a run of `graph`.
    #[must_use]
    pub const fn new(graph: &'graph ProcedureTemplateGraph) -> Self {
        Self { graph }
    }

    /// The calls recording a run, with `E` as the error type of each closure.
    ///
    /// # Example
    ///
    /// ```rust
    /// use core::convert::Infallible;
    ///
    /// use aps_test_utils::{Replica, Tenant};
    /// use procedure_template_visitor::{GuidedProcedurePseudocode, ProcedureTemplateGraph};
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
    /// let pseudocode = GuidedProcedurePseudocode::new(&graph).pseudocode::<Infallible>();
    /// assert!(pseudocode.starts_with("let make_a_pizza = GuidedProcedure::new("));
    /// ```
    #[must_use]
    pub fn pseudocode<E>(&self) -> String {
        let error_type = core::any::type_name::<E>();
        let error_type = if error_type == "!" { "core::convert::Infallible" } else { error_type };
        let root = self
            .graph
            .root()
            .template()
            .get_column_ref::<procedure_templates::name>()
            .to_lowercase()
            .replace(' ', "_");
        let mut writer = Writer { error_type, code: String::new() };
        writer.line(format_args!(
            "let {root} = GuidedProcedure::new(&graph, organization, project, author, conn)"
        ));
        let Ok(()) = self.graph.accept(&mut writer);
        writer.line(format_args!("    .finish()?;"));
        writer.code
    }
}

/// The visitor writing one `and_then` per procedure of the run.
struct Writer<'error> {
    /// Name of the error type of each closure.
    error_type: &'error str,
    /// The code written so far.
    code: String,
}

impl Writer<'_> {
    /// Appends `text` and a line break.
    fn line(&mut self, text: core::fmt::Arguments<'_>) {
        self.code.write_fmt(text).expect("writing to a string cannot fail");
        self.code.push('\n');
    }
}

impl<'graph> Visitor<'graph> for Writer<'_> {
    type Error = core::convert::Infallible;

    fn visit_template(
        &mut self,
        graph: &'graph ProcedureTemplateGraph,
        path: &[&'graph StepNode],
        template: &'graph TemplateNode,
    ) -> Result<(), Self::Error> {
        let name = template.template().get_column_ref::<procedure_templates::name>();
        let procedure_table = procedure_table(
            template.template().get_column_ref::<procedure_templates::table_name_id>(),
        );
        let own_slots: Vec<&ProcedureTemplateSlot> = template
            .slots()
            .iter()
            .filter(|slot| graph.slot_owner(path, slot).depth() == path.len())
            .collect();
        match path.last() {
            None => self.line(format_args!("    // {name}")),
            Some(step) => {
                let step_name = step.step().get_column_ref::<procedure_template_steps::name>();
                self.line(format_args!("    // {name}, step \"{step_name}\""));
                if step.step().get_column::<procedure_template_steps::optional>() {
                    self.line(format_args!("    // optional, `.skip(ended_at, note)` passes it"));
                }
                if let Some(repeat_slot) =
                    step.step().get_column::<procedure_template_steps::repeat_slot_id>()
                {
                    let repeat_slot = graph
                        .parent_of(step)
                        .slot(repeat_slot)
                        .expect("a step repeats over a slot of its parent template");
                    self.line(format_args!(
                        "    // once per asset bound to `{}`, one call each",
                        repeat_slot.get_column_ref::<procedure_template_slots::name>()
                    ));
                }
            }
        }
        let error_type = self.error_type;
        self.line(format_args!(
            "    .and_then::<{procedure_table}, {error_type}>(|builder, slots, conn| {{"
        ));
        self.line(format_args!(
            "        // Set the times, the performer and the measurements on `builder`."
        ));
        if !own_slots.is_empty() {
            let names: Vec<String> = own_slots
                .iter()
                .map(|slot| {
                    format!("`{}`", slot.get_column_ref::<procedure_template_slots::name>())
                })
                .collect();
            self.line(format_args!("        // Bind {} through `slots`.", names.join(", ")));
        }
        self.line(format_args!("        Ok(builder)"));
        self.line(format_args!("    }})?"));
        walk_template(self, graph, path, template)
    }
}

/// The procedure table running a template of `template_table`, which the
/// schema names by replacing the template suffix.
fn procedure_table(template_table: &str) -> String {
    let stem = template_table
        .strip_suffix("_templates")
        .expect("a procedure template table name ends in `_templates`");
    format!("{stem}s")
}
