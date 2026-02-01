//! Submodule defining a function to recursively load the sub-procedure
//! templates of a given procedure template.

use std::rc::Rc;

use aps_asset_models::asset_models;
use aps_procedure_template_asset_models::*;
use aps_procedure_templates::*;
use aps_reused_procedure_template_asset_models::*;
use diesel_builders::{GetColumnExt, LoadMany, NestedModel, TableModel, prelude::LoadNestedFirst};
use geometric_traits::{
    impls::CSR2D,
    prelude::{GenericBiGraph, GenericEdgesBuilder, MonopartiteGraph, SortedVec},
    traits::EdgesBuilder,
};

use crate::structs::{Hierarchy, Ownership};

impl Hierarchy {
    /// Returns the ownership bipartite graph of the procedure templates in the
    /// hierarchy with the procedure template asset models they reference.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a database connection.
    ///
    /// # Errors
    ///
    /// * Returns a `diesel::result::Error` if there is an issue querying the
    ///   database.
    pub(crate) fn ownership<C>(&self, conn: &mut C) -> Result<Ownership, diesel::result::Error>
    where
        (reused_procedure_template_asset_models::procedure_template_id,): LoadMany<C>,
        (procedure_template_asset_models::procedure_template_id,): LoadMany<C>,
        (procedure_templates::id,): LoadNestedFirst<procedure_templates::table, C>,
        (asset_models::id,): LoadNestedFirst<asset_models::table, C>,
        ProcedureTemplateAssetModel: FKProcedureTemplateAssetModelsProcedureTemplateId<C>
            + FKProcedureTemplateAssetModelsAssetModelId<C>,
        ReusedProcedureTemplateAssetModel:
            FKReusedProcedureTemplateAssetModelsProcedureTemplateAssetModelId<C>,
    {
        let mut foreign_procedure_templates: Vec<NestedModel<procedure_templates::table>> =
            Vec::new();
        let mut procedure_template_asset_models = Vec::new();
        let mut edges = Vec::new();

        for (i, procedure_template) in self.hierarchy.nodes_vocabulary().iter().enumerate() {
            let reused_ptams =
                <(reused_procedure_template_asset_models::procedure_template_id,)>::load_many(
                    (procedure_template.get_column::<procedure_templates::id>(),),
                    conn,
                )?;
            let mut used_ptams =
                <(procedure_template_asset_models::procedure_template_id,)>::load_many(
                    (procedure_template.get_column::<procedure_templates::id>(),),
                    conn,
                )
                .unwrap();

            for reused_ptam in reused_ptams {
                let ptam = reused_ptam.procedure_template_asset_model(conn)?;
                used_ptams.push(ptam);
            }

            for ptam in used_ptams {
                // If the owner of the procedure template asset model is not in
                // the hierarchy, add it to the foreign procedure templates.
                if self
                    .hierarchy
                    .nodes_vocabulary()
                    .binary_search_by(|pt| {
                        pt.get_column::<procedure_templates::id>().cmp(ptam.procedure_template_id())
                    })
                    .is_err()
                {
                    let ptam_owner: NestedModel<procedure_templates::table> =
                        ptam.procedure_template(conn)?.nested(conn)?;
                    foreign_procedure_templates.push(ptam_owner);
                }

                edges.push((i, ptam.clone()));
                procedure_template_asset_models.push(ptam);
            }
        }

        foreign_procedure_templates.sort_unstable();
        foreign_procedure_templates.dedup();
        let foreign_procedure_templates = SortedVec::try_from(foreign_procedure_templates).unwrap();
        procedure_template_asset_models.sort_unstable();
        procedure_template_asset_models.dedup();
        let procedure_template_asset_models =
            SortedVec::try_from(procedure_template_asset_models).unwrap();

        let mut edges = edges
            .into_iter()
            .map(|(i, ptam)| (i, procedure_template_asset_models.binary_search(&ptam).unwrap()))
            .collect::<Vec<(usize, usize)>>();

        edges.sort_unstable();
        edges.dedup();

        let edges: CSR2D<usize, usize, usize> = GenericEdgesBuilder::default()
            .expected_number_of_edges(edges.len())
            .expected_shape((
                self.hierarchy.nodes_vocabulary().len(),
                procedure_template_asset_models.len(),
            ))
            .edges(edges)
            .build()
            .expect("Failed to build ownership edges");

        let graph = GenericBiGraph::try_from((
            self.hierarchy.nodes_vocabulary().clone(),
            Rc::new(procedure_template_asset_models),
            edges,
        ))
        .expect("Failed to build ownership graph");

        Ownership::new(graph, foreign_procedure_templates, conn)
    }
}
