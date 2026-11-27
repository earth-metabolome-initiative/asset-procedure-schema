//! A visitor over the templates, slots and steps of a procedure template
//! graph, each template visited once per step path reaching it.

use aps_resources::ProcedureTemplateSlot;

use crate::{ProcedureTemplateGraph, StepNode, TemplateNode};

/// An operation over a [`ProcedureTemplateGraph`]. Each method receives the
/// steps from the root down to the template at hand, so a template several
/// steps run is visited once per step. The default methods walk the whole
/// graph, and an operation overrides the ones it acts on, calling the matching
/// `walk_*` function to keep descending.
pub trait Visitor<'graph> {
    /// The error that stops the visit.
    type Error;

    /// Visits `template`, reached through `path`, by default its slots and
    /// then its steps in `position` order.
    ///
    /// # Errors
    ///
    /// Returns the first error of the visit.
    fn visit_template(
        &mut self,
        graph: &'graph ProcedureTemplateGraph,
        path: &[&'graph StepNode],
        template: &'graph TemplateNode,
    ) -> Result<(), Self::Error> {
        walk_template(self, graph, path, template)
    }

    /// Visits `slot` of `template`, reached through `path`.
    ///
    /// # Errors
    ///
    /// Returns the error the operation raises on the slot.
    fn visit_slot(
        &mut self,
        graph: &'graph ProcedureTemplateGraph,
        path: &[&'graph StepNode],
        template: &'graph TemplateNode,
        slot: &'graph ProcedureTemplateSlot,
    ) -> Result<(), Self::Error> {
        let _ = (graph, path, template, slot);
        Ok(())
    }

    /// Visits `step` of the template `path` reaches, by default the template
    /// the step runs.
    ///
    /// # Errors
    ///
    /// Returns the first error of the visit.
    fn visit_step(
        &mut self,
        graph: &'graph ProcedureTemplateGraph,
        path: &[&'graph StepNode],
        step: &'graph StepNode,
    ) -> Result<(), Self::Error> {
        walk_step(self, graph, path, step)
    }
}

/// Visits the slots of `template` and then its steps in `position` order.
///
/// # Errors
///
/// Returns the first error of the visit.
pub fn walk_template<'graph, V: Visitor<'graph> + ?Sized>(
    visitor: &mut V,
    graph: &'graph ProcedureTemplateGraph,
    path: &[&'graph StepNode],
    template: &'graph TemplateNode,
) -> Result<(), V::Error> {
    for slot in template.slots() {
        visitor.visit_slot(graph, path, template, slot)?;
    }
    for step in template.task_graph().steps() {
        visitor.visit_step(graph, path, step)?;
    }
    Ok(())
}

/// Visits the template `step` runs, reached through `path` and `step`.
///
/// # Errors
///
/// Returns the first error of the visit.
pub fn walk_step<'graph, V: Visitor<'graph> + ?Sized>(
    visitor: &mut V,
    graph: &'graph ProcedureTemplateGraph,
    path: &[&'graph StepNode],
    step: &'graph StepNode,
) -> Result<(), V::Error> {
    let mut path = path.to_vec();
    path.push(step);
    visitor.visit_template(graph, &path, graph.child_of(step))
}

impl ProcedureTemplateGraph {
    /// Visits the graph from its root with `visitor`.
    ///
    /// # Errors
    ///
    /// Returns the first error of the visit.
    pub fn accept<'graph, V: Visitor<'graph> + ?Sized>(
        &'graph self,
        visitor: &mut V,
    ) -> Result<(), V::Error> {
        visitor.visit_template(self, &[], self.root())
    }
}
