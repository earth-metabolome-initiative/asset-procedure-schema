//! Submodule defining the `GuidedProcedureBuilder` structure and its associated
//! methods.

mod error;
mod iterator;
mod listener;
use aps_users::User;
use listener::GPBListener;

use crate::{procedure_template_graph::ProcedureTemplateGraph, ptg_visitor::PTGVisitor};
#[derive(Debug)]
/// Struct providing functionalities to help the user concretely build a
/// procedure.
pub struct GuidedProcedure<'graph, C> {
    visitor: PTGVisitor<'graph, ProcedureTemplateGraph, GPBListener<'graph, C>>,
}

impl<'graph, C> GuidedProcedure<'graph, C> {
    /// Creates a new `GuidedProcedureBuilder` instance.
    #[must_use]
    pub fn new(
        graph: &'graph ProcedureTemplateGraph,
        author: &'graph User,
        connection: &'graph mut C,
    ) -> GuidedProcedure<'graph, C> {
        GuidedProcedure {
            visitor: PTGVisitor::new(graph, GPBListener::new(graph, author, connection)),
        }
    }

    /// Returns a reference to the underlying author.
    pub(super) fn author(&self) -> &User {
        self.visitor.listener().author()
    }
}
