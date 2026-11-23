//! The order of the steps of a procedure template and the slots they bind.

use aps_asset_model_ancestors::*;
use aps_procedure_templates::*;
use diesel_builders::{BuildableTable, BuilderResult, Insert, TableBuilder, prelude::*};
use validation_errors::ValidationError;

use crate::validation::{IntoValidationError, flatten};

/// A step of a procedure template.
pub trait ProcedureTemplateStepNode {
    /// Inserts the edge making `successor` start only after this step
    /// completes. Both steps belong to one template, and this one sits at a
    /// strictly lower position.
    ///
    /// # Errors
    ///
    /// Returns the error of the connection, which refuses steps of different
    /// templates or a successor at a position not above this step's.
    fn precedes<C>(
        &self,
        successor: &ProcedureTemplateStep,
        conn: &mut C,
    ) -> BuilderResult<ProcedureTemplateStepEdge, ValidationError>
    where
        TableBuilder<procedure_template_step_edges::table>:
            Insert<C, ValidationError: IntoValidationError>;

    /// Inserts the binding filling `child_slot`, a slot of the template this
    /// step runs, with the asset the parent procedure binds to `parent_slot`, a
    /// slot of the template this step belongs to.
    ///
    /// # Errors
    ///
    /// Returns the error of the connection, which refuses slots of other
    /// templates, a parent slot whose model is not the child slot's or below
    /// it, and a pair of roles `slot_role_pairs` does not admit.
    fn bind<C>(
        &self,
        child_slot: &ProcedureTemplateSlot,
        parent_slot: &ProcedureTemplateSlot,
        conn: &mut C,
    ) -> BuilderResult<ProcedureTemplateSlotBinding, ValidationError>
    where
        TableBuilder<procedure_template_slot_bindings::table>:
            Insert<C, ValidationError: IntoValidationError>;
}

impl ProcedureTemplateStepNode for ProcedureTemplateStep {
    fn precedes<C>(
        &self,
        successor: &ProcedureTemplateStep,
        conn: &mut C,
    ) -> BuilderResult<ProcedureTemplateStepEdge, ValidationError>
    where
        TableBuilder<procedure_template_step_edges::table>:
            Insert<C, ValidationError: IntoValidationError>,
    {
        flatten(
            procedure_template_step_edges::table::builder()
                .predecessor_step_id(*self.id())
                .successor_step_id(*successor.id())
                .parent_template_id(*self.parent_template_id())
                .organization_id(*self.organization_id())
                .predecessor_position(*self.position())
                .successor_position(*successor.position())
                .insert(conn),
        )
    }

    fn bind<C>(
        &self,
        child_slot: &ProcedureTemplateSlot,
        parent_slot: &ProcedureTemplateSlot,
        conn: &mut C,
    ) -> BuilderResult<ProcedureTemplateSlotBinding, ValidationError>
    where
        TableBuilder<procedure_template_slot_bindings::table>:
            Insert<C, ValidationError: IntoValidationError>,
    {
        flatten(
            procedure_template_slot_bindings::table::builder()
                .step_id(*self.id())
                .child_slot_id(*child_slot.id())
                .parent_template_id(*self.parent_template_id())
                .child_template_id(*self.child_template_id())
                .parent_slot_id(*parent_slot.id())
                .child_asset_model_id(*child_slot.asset_model_id())
                .parent_asset_model_id(*parent_slot.asset_model_id())
                .child_role_id(child_slot.role_id().clone())
                .parent_role_id(parent_slot.role_id().clone())
                .organization_id(*self.organization_id())
                .insert(conn),
        )
    }
}
