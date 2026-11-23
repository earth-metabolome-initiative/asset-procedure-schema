//! Traits writing the containment rules of container models and the slots,
//! steps, step edges and slot bindings of procedure templates, on any Diesel
//! connection the regenerated `aps/` crates support.

mod can_contain;
mod procedure_template_node;
mod procedure_template_step_node;
mod slot_role;
pub mod validation;

pub use can_contain::CanContain;
pub use procedure_template_node::{ProcedureTemplateNode, Publish};
pub use procedure_template_step_node::ProcedureTemplateStepNode;
pub use slot_role::SlotRole;
pub use validation::IntoValidationError;
