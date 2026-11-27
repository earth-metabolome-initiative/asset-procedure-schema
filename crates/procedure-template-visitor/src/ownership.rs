//! The slot whose procedure rows a step's slot inherits, found by following
//! the slot bindings of the steps upwards.

use aps_resources::{
    ProcedureTemplateSlot, procedure_template_slot_bindings, procedure_template_slots,
};
use diesel_builders::GetColumnExt;

use crate::{ProcedureTemplateGraph, StepNode};

/// The slot a slot's rows come from, and how many steps below the root the
/// template declaring it runs.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SlotOwner<'graph> {
    /// Number of steps from the root to the template declaring `slot`.
    depth: usize,
    /// The slot no step binding fills, whose procedure binds its own rows.
    slot: &'graph ProcedureTemplateSlot,
}

impl<'graph> SlotOwner<'graph> {
    /// Number of steps from the root to the template declaring the slot, zero
    /// for a slot of the root.
    #[must_use]
    pub const fn depth(&self) -> usize {
        self.depth
    }

    /// The slot no step binding fills, whose procedure binds its own rows.
    #[must_use]
    pub const fn slot(&self) -> &'graph ProcedureTemplateSlot {
        self.slot
    }
}

impl ProcedureTemplateGraph {
    /// The slot whose rows the procedure reached through `path`, the steps
    /// from the root down, inherits for `slot`, a slot of the template the
    /// last step of `path` runs or of the root when `path` is empty. Each step
    /// binding `slot` hands it the rows of its parent slot, so the owner is
    /// the first slot upwards no binding fills.
    ///
    /// # Panics
    ///
    /// Panics when a binding names a parent slot its template does not
    /// declare, which the composite keys of the schema rule out.
    #[must_use]
    pub fn slot_owner<'graph>(
        &'graph self,
        path: &[&'graph StepNode],
        slot: &'graph ProcedureTemplateSlot,
    ) -> SlotOwner<'graph> {
        let mut owner = SlotOwner { depth: path.len(), slot };
        while let Some(step) = owner.depth.checked_sub(1).map(|depth| path[depth]) {
            let Some(binding) =
                step.binding_of(owner.slot.get_column::<procedure_template_slots::id>())
            else {
                break;
            };
            owner.slot = self
                .parent_of(step)
                .slot(binding.get_column::<procedure_template_slot_bindings::parent_slot_id>())
                .expect("a binding names a slot of the parent template");
            owner.depth -= 1;
        }
        owner
    }
}
