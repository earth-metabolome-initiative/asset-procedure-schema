//! The errors drawing a procedure template graph.

use mermaid_builder::{ConfigError, EdgeError, NodeError, StyleClassError};

/// An error drawing a procedure template graph.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The flowchart builder refused a node, an edge, a style class or the
    /// configuration.
    #[error("Mermaid flowchart error: {0}")]
    Mermaid(#[from] mermaid_builder::Error),
    /// A slot names a role outside the five of `slot_roles`.
    #[error("slot `{slot}` names the unknown role `{role}`")]
    UnknownSlotRole {
        /// Name of the slot.
        slot: String,
        /// The role it names.
        role: String,
    },
}

impl From<ConfigError> for Error {
    fn from(error: ConfigError) -> Self {
        Self::Mermaid(error.into())
    }
}

impl From<EdgeError> for Error {
    fn from(error: EdgeError) -> Self {
        Self::Mermaid(error.into())
    }
}

impl From<NodeError> for Error {
    fn from(error: NodeError) -> Self {
        Self::Mermaid(error.into())
    }
}

impl From<StyleClassError> for Error {
    fn from(error: StyleClassError) -> Self {
        Self::Mermaid(error.into())
    }
}
