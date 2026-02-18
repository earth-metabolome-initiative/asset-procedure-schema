//! Traits and utilities for assets and procedures.
pub mod compatible_with;
mod procedure_template_node;
mod reusability;
pub use compatible_with::{CanContain, CompatibleWith};
pub use procedure_template_node::ProcedureTemplateNode;
pub use reusability::ReusabilityBuilderExt;
