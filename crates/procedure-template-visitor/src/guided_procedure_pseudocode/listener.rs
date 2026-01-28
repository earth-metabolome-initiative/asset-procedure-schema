//! Submodule defining the listener used for the `GuidedProcedureBuilder`.

use std::{convert::Infallible, fmt::Debug};

use aps_procedure_template_asset_models::ProcedureTemplateAssetModel;
use aps_procedure_templates::*;
use diesel_builders::TableExt;

use crate::PTGListener;
#[derive(Debug, Clone, Copy)]
pub(super) struct GPPListener {
    /// Skip procedures which are not extensions of the `procedure_templates`
    /// table, and therefore can always be automatically inserted without user
    /// guidance.
    skip_base_procedures: bool,
}

impl GPPListener {
    /// Creates a new `GPPListener` instance.
    pub(super) fn new(skip_base_procedures: bool) -> Self {
        Self { skip_base_procedures }
    }
}

pub enum GPPListenerOutput<'graph> {
    NoOp,
    Template(&'graph ProcedureTemplate),
}

impl<'graph> PTGListener<'graph> for GPPListener {
    type Output = GPPListenerOutput<'graph>;
    type FilteredSuccessors<I>
        = I
    where
        I: Iterator<Item = &'graph ProcedureTemplate>;
    type Error = Infallible;

    fn enter_foreign_procedure_template(
        &mut self,
        _foreign_procedure_template: &ProcedureTemplate,
    ) -> Result<Self::Output, Self::Error> {
        Ok(GPPListenerOutput::NoOp)
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
        _parents: &[&'graph ProcedureTemplate],
        child: &'graph ProcedureTemplate,
    ) -> Result<Self::Output, Self::Error> {
        if child.procedure_template_table_id()
            == <procedure_templates::table as TableExt>::TABLE_NAME
            && self.skip_base_procedures
        {
            Ok(GPPListenerOutput::NoOp)
        } else {
            Ok(GPPListenerOutput::Template(child))
        }
    }

    fn leave_procedure_template(
        &mut self,
        _parents: &[&ProcedureTemplate],
        _child: &ProcedureTemplate,
    ) -> Result<Self::Output, Self::Error> {
        Ok(GPPListenerOutput::NoOp)
    }

    fn enter_leaf_ptam(
        &mut self,
        _parents: &[&ProcedureTemplate],
        _leaf: &ProcedureTemplate,
        _procedure_template_asset_model: &ProcedureTemplateAssetModel,
    ) -> Result<Self::Output, Self::Error> {
        Ok(GPPListenerOutput::NoOp)
    }

    fn filter_successors<I>(
        &mut self,
        successors: I,
    ) -> Result<Self::FilteredSuccessors<I>, Self::Error>
    where
        I: Iterator<Item = &'graph ProcedureTemplate>,
    {
        Ok(successors)
    }
}
