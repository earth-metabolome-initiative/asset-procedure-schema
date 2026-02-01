//! Submodule defining a function to recursively load the sub-procedure
//! templates of a given procedure template.

use std::rc::Rc;

use aps_next_procedure_templates::*;
use aps_parent_procedure_templates::*;
use aps_procedure_templates::*;
use diesel_builders::{GetColumnExt, LoadMany, NestedModel, TableModel, prelude::LoadNestedFirst};
use geometric_traits::{
    impls::GenericBiMatrix2D,
    prelude::{GenericEdgesBuilder, MonopartiteGraph, SortedVec},
    traits::EdgesBuilder,
};

use crate::structs::{Hierarchy, TaskGraph};

impl Hierarchy {
    /// Returns the task graph for the given procedure template, if it has
    /// sub-procedure templates; otherwise, returns `None`.
    ///
    /// # Arguments
    ///
    /// * `procedure_template` - The procedure template for which to load the
    ///   task graph.
    /// * `conn` - The database connection to use for loading the task graph.
    ///
    /// # Errors
    ///
    /// * Returns a `diesel::result::Error` if there is an error loading the
    ///   task graph from the database.
    fn task_graph<C>(
        &self,
        procedure_template: &NestedModel<procedure_templates::table>,
        conn: &mut C,
    ) -> Result<Option<TaskGraph>, diesel::result::Error>
    where
        (parent_procedure_templates::parent_id,): LoadMany<C>,
        (next_procedure_templates::parent_id,): LoadMany<C>,
        (procedure_templates::id,): LoadNestedFirst<procedure_templates::table, C>,
        ParentProcedureTemplate: FKParentProcedureTemplatesChildId<C>,
        NextProcedureTemplate:
            FKNextProcedureTemplatesPredecessorId<C> + FKNextProcedureTemplatesSuccessorId<C>,
    {
        let parent_child_relations = <(parent_procedure_templates::parent_id,)>::load_many(
            (procedure_template.get_column::<procedure_templates::id>(),),
            conn,
        )?;

        if parent_child_relations.is_empty() {
            return Ok(None);
        }

        let mut nodes = parent_child_relations
            .iter()
            .map(|relation| {
                let child = relation.child(conn)?.nested(conn)?;

                // We find the curresponding Rc<ProcedureTemplate> in the hierarchy's
                // nodes vocabulary so to avoid duplicating memory allocation in the
                // finalized data structure.
                let rc_child = self
                    .hierarchy
                    .nodes_vocabulary()
                    .binary_search_by(|pt| pt.as_ref().cmp(&child))
                    .map(|index| self.hierarchy.nodes_vocabulary()[index].clone())
                    .expect("Child procedure template not found in hierarchy's vocabulary");

                Ok(rc_child)
            })
            .collect::<Result<Vec<Rc<NestedModel<procedure_templates::table>>>, diesel::result::Error>>()?;

        nodes.sort_unstable();

        let sorted_nodes = SortedVec::try_from(nodes).unwrap();

        let next_relations = <(next_procedure_templates::parent_id,)>::load_many(
            (procedure_template.get_column::<procedure_templates::id>(),),
            conn,
        )?;

        let mut edges: Vec<(usize, usize)> = Vec::new();
        for next_relation in next_relations {
            let predecessor = next_relation.predecessor(conn)?.nested(conn)?;
            let successor = next_relation.successor(conn)?.nested(conn)?;

            let source_index = sorted_nodes
                .binary_search_by(|pt| pt.as_ref().cmp(&predecessor))
                .expect("Predecessor not found in nodes vocabulary");
            let destination_index = sorted_nodes
                .binary_search_by(|pt| pt.as_ref().cmp(&successor))
                .expect("Successor not found in nodes vocabulary");
            edges.push((source_index, destination_index));
        }

        edges.sort_unstable();

        let number_of_nodes = sorted_nodes.len();
        let edges = GenericEdgesBuilder::default()
            .expected_number_of_edges(edges.len())
            .expected_shape(number_of_nodes)
            .edges(edges)
            .build()
            .expect("Failed to build task graph");

        let biedges = GenericBiMatrix2D::new(edges);

        Ok(Some(TaskGraph::new(sorted_nodes, biedges)))
    }

    /// Returns the task graphs for all procedure templates in the hierarchy.
    /// Each task graph corresponds to a procedure template in the hierarchy,
    /// in the same order as the procedure templates in the hierarchy's nodes
    /// vocabulary.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection to use for loading the task graphs.
    ///
    /// # Errors
    ///
    /// * Returns a `diesel::result::Error` if there is an error loading the
    ///   task graphs from the database.
    pub(crate) fn task_graphs<C>(
        &self,
        conn: &mut C,
    ) -> Result<Vec<Option<TaskGraph>>, diesel::result::Error>
    where
        (parent_procedure_templates::parent_id,): LoadMany<C>,
        (next_procedure_templates::parent_id,): LoadMany<C>,
        (procedure_templates::id,): LoadNestedFirst<procedure_templates::table, C>,
        ParentProcedureTemplate: FKParentProcedureTemplatesChildId<C>,
        NextProcedureTemplate:
            FKNextProcedureTemplatesPredecessorId<C> + FKNextProcedureTemplatesSuccessorId<C>,
    {
        let mut task_graphs = Vec::with_capacity(self.hierarchy.number_of_nodes());
        for procedure_template in self.hierarchy.nodes_vocabulary().iter() {
            task_graphs.push(self.task_graph(procedure_template, conn)?);
        }
        Ok(task_graphs)
    }
}
