//! The slots a procedure binds itself, those no step binding fills.

use aps_assets::assets;
use aps_resources::{
    ProcedureTemplateSlot, asset_models, procedure_template_slots, procedure_templates,
};
use diesel_builders::{GetColumn, GetColumnExt};
use rosetta_uuid::Uuid;

use crate::{StepNode, TemplateNode, guided_procedure::GuidedProcedureError};

/// One row a procedure binds itself.
#[derive(Debug, Clone, Copy)]
pub(super) struct OwnRow<'graph> {
    /// The slot the row fills.
    pub(super) slot: &'graph ProcedureTemplateSlot,
    /// Model of the asset.
    pub(super) asset_model: Uuid,
    /// The asset, or `None` for an asset known only by its model.
    pub(super) asset: Option<Uuid>,
}

/// The slots of the procedure being recorded that its step does not bind,
/// which the closure of
/// [`GuidedProcedure::and_then`](crate::GuidedProcedure::and_then) fills with
/// assets or models.
#[derive(Debug)]
pub struct Slots<'graph> {
    /// Template of the procedure.
    template: &'graph TemplateNode,
    /// The step the procedure runs, `None` for the root.
    step: Option<&'graph StepNode>,
    /// The rows bound so far.
    rows: Vec<OwnRow<'graph>>,
}

impl<'graph> Slots<'graph> {
    /// No row yet for the procedure of `template` running `step`.
    pub(super) const fn new(
        template: &'graph TemplateNode,
        step: Option<&'graph StepNode>,
    ) -> Self {
        Self { template, step, rows: Vec::new() }
    }

    /// The rows bound.
    pub(super) fn into_rows(self) -> Vec<OwnRow<'graph>> {
        self.rows
    }

    /// The slots to bind here, those the step does not fill with the
    /// parent's assets.
    pub fn open(&self) -> impl Iterator<Item = &'graph ProcedureTemplateSlot> {
        let step = self.step;
        self.template.slots().iter().filter(move |slot| {
            step.is_none_or(|step| {
                step.binding_of(GetColumnExt::get_column::<procedure_template_slots::id>(*slot))
                    .is_none()
            })
        })
    }

    /// Binds `asset` to the open slot named `slot`.
    ///
    /// # Errors
    ///
    /// Returns an error when the template declares no slot named `slot` or
    /// its step fills it.
    pub fn bind_asset(
        &mut self,
        slot: &str,
        asset: &(impl GetColumn<assets::id> + GetColumn<assets::model_id>),
    ) -> Result<&mut Self, GuidedProcedureError> {
        let slot = self.open_slot(slot)?;
        self.rows.push(OwnRow {
            slot,
            asset_model: GetColumn::<assets::model_id>::get_column(asset),
            asset: Some(GetColumn::<assets::id>::get_column(asset)),
        });
        Ok(self)
    }

    /// Binds an asset known only by its `model` to the open slot named
    /// `slot`.
    ///
    /// # Errors
    ///
    /// Returns an error when the template declares no slot named `slot` or
    /// its step fills it.
    pub fn bind_model(
        &mut self,
        slot: &str,
        model: &impl GetColumn<asset_models::id>,
    ) -> Result<&mut Self, GuidedProcedureError> {
        let slot = self.open_slot(slot)?;
        self.rows.push(OwnRow { slot, asset_model: model.get_column(), asset: None });
        Ok(self)
    }

    /// The open slot named `name`.
    fn open_slot(&self, name: &str) -> Result<&'graph ProcedureTemplateSlot, GuidedProcedureError> {
        let template =
            || GetColumnExt::get_column::<procedure_templates::name>(self.template.template());
        let slot = self
            .template
            .slots()
            .iter()
            .find(|slot| {
                GetColumnExt::get_column_ref::<procedure_template_slots::name>(*slot) == name
            })
            .ok_or_else(|| {
                GuidedProcedureError::UnknownSlot { template: template(), slot: name.to_owned() }
            })?;
        if self.step.is_some_and(|step| {
            step.binding_of(GetColumnExt::get_column::<procedure_template_slots::id>(slot))
                .is_some()
        }) {
            return Err(GuidedProcedureError::InheritedSlot {
                template: template(),
                slot: name.to_owned(),
            });
        }
        Ok(slot)
    }
}
