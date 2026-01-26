//! Submodule defining the listener used for the `GuidedProcedureBuilder`.

use std::{collections::HashMap, fmt::Debug};

use aps_procedure_asset_models::*;
use aps_procedure_template_asset_models::*;
use aps_procedure_templates::*;
use aps_procedures::*;
use aps_users::User;
use diesel::Identifiable;
use diesel_builders::{
    BuildableTable, BuilderError, ColumnTyped, DescendantOf, DescendantWithSelf, GetColumn, Insert,
    NestedTables, TableBuilder, ValueTyped,
};
use rosetta_uuid::Uuid;
use validation_errors::ValidationError;

use crate::{
    PTGListener, ProcedureTemplateGraph, guided_procedure::error::InternalGuidedProcedureError,
    structs::OwnershipLike,
};
#[derive(Debug)]
pub(super) struct GPBListener<'listener, C> {
    graph: &'listener ProcedureTemplateGraph,
    connection: &'listener mut C,
    author: &'listener User,
    designated_successor: Option<&'listener ProcedureTemplate>,
    /// Stack of parent procedures corresponding to the currently visited
    /// procedure templates.
    parent_procedures: Vec<Procedure>,
    /// The procedure that was most recently inserted, to be used as the
    /// predecessor for the next procedure to be inserted.
    predecessor_procedure: Option<Procedure>,
    /// Map from a procedure template asset model ID to the corresponding
    /// procedure asset model ID.
    procedure_asset_models: HashMap<Uuid, Uuid>,
}

impl<C> GPBListener<'_, C> {
    pub(super) fn procedure_asset(
        &self,
        parents: &[&ProcedureTemplate],
        ptam_id: Uuid,
    ) -> Option<Uuid> {
        let ptam: &ProcedureTemplateAssetModel =
            self.graph.ptam_by_primary_key(ptam_id).expect("PTAM not found in graph");
        let reference_ptam: &ProcedureTemplateAssetModel =
            self.graph.reference_based_on_alias(parents, ptam).expect("Alias not found in graph");
        self.procedure_asset_models.get(&reference_ptam.id()).copied()
    }
}

impl<'listener, C> GPBListener<'listener, C> {
    pub(super) fn new(
        graph: &'listener ProcedureTemplateGraph,
        author: &'listener User,
        connection: &'listener mut C,
    ) -> Self {
        Self {
            graph,
            connection,
            author,
            designated_successor: None,
            parent_procedures: Vec::new(),
            predecessor_procedure: None,
            procedure_asset_models: HashMap::new(),
        }
    }

    pub(super) fn connection(&mut self) -> &mut C {
        self.connection
    }

    pub(super) fn author(&self) -> &User {
        self.author
    }

    pub(super) fn last_parent_procedure(&self) -> Option<&Procedure> {
        self.parent_procedures.last()
    }

    pub(super) fn predecessor_procedure(&self) -> Option<&Procedure> {
        self.predecessor_procedure.as_ref()
    }

    pub(super) fn register_ptam_pam_pair(&mut self, ptam_id: Uuid, pam_id: Uuid) {
        self.procedure_asset_models.insert(ptam_id, pam_id);
    }

    pub(super) fn reference_based_on_alias(
        &self,
        parents: &[&ProcedureTemplate],
        ptam: &'listener ProcedureTemplateAssetModel,
    ) -> Option<&'listener ProcedureTemplateAssetModel> {
        self.graph.reference_based_on_alias(parents, ptam)
    }

    /// Retrieves a procedure template asset model by its primary key.
    pub(super) fn ptam_by_primary_key(
        &self,
        ptam_id: Uuid,
    ) -> Option<&'listener ProcedureTemplateAssetModel> {
        self.graph.ptam_by_primary_key(ptam_id)
    }

    /// Pushes a provided processed procedure template onto the parents
    /// stack.
    pub(super) fn push_parent(&mut self, procedure: Procedure) {
        self.parent_procedures.push(procedure);
    }
}

impl<'graph, C> PTGListener<'graph> for GPBListener<'graph, C> {
    type Output = Option<(Vec<&'graph ProcedureTemplate>, &'graph ProcedureTemplate)>;
    type FilteredSuccessors<I>
        = Option<&'graph ProcedureTemplate>
    where
        I: Iterator<Item = &'graph ProcedureTemplate>;
    type Error = InternalGuidedProcedureError<'graph>;

    fn enter_foreign_procedure_template(
        &mut self,
        _foreign_procedure_template: &ProcedureTemplate,
    ) -> Result<Self::Output, Self::Error> {
        Ok(None)
    }

    fn continue_task(
        &mut self,
        _parents: &[&ProcedureTemplate],
        _predecessors: &[&ProcedureTemplate],
        _child: &ProcedureTemplate,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn enter_procedure_template(
        &mut self,
        parents: &[&'graph ProcedureTemplate],
        child: &'graph ProcedureTemplate,
    ) -> Result<Self::Output, Self::Error> {
        Ok(Some((parents.to_vec(), child)))
    }

    fn leave_procedure_template(
        &mut self,
        _parents: &[&ProcedureTemplate],
        _child: &ProcedureTemplate,
    ) -> Result<Self::Output, Self::Error> {
        self.predecessor_procedure = self.parent_procedures.pop();
        Ok(None)
    }

    fn enter_leaf_ptam(
        &mut self,
        _parents: &[&ProcedureTemplate],
        _leaf: &ProcedureTemplate,
        _procedure_template_asset_model: &ProcedureTemplateAssetModel,
    ) -> Result<Self::Output, Self::Error> {
        Ok(None)
    }

    fn filter_successors<I>(
        &mut self,
        successors: I,
    ) -> Result<Self::FilteredSuccessors<I>, Self::Error>
    where
        I: Iterator<Item = &'graph ProcedureTemplate>,
    {
        let successors: Vec<&ProcedureTemplate> = successors.collect();

        Ok(match successors.as_slice() {
            [] => None,
            [single_successor] => Some(single_successor),
            _ => {
                let Some(designated_successor) = self.designated_successor else {
                    return Err(InternalGuidedProcedureError::UnclearSuccessor {
                        viable_successors: successors,
                    });
                };
                if successors.contains(&designated_successor) {
                    Some(designated_successor)
                } else {
                    return Err(InternalGuidedProcedureError::DesignatedSuccessorNotFound {
                        designated_successor,
                        viable_successors: successors,
                    });
                }
            }
        })
    }
}
