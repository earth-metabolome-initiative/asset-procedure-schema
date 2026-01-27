//! Submodule providing errors which may occur in the core structures
//! visualization module.

use mermaid_builder::{ConfigError, EdgeError, NodeError};

#[derive(Debug, thiserror::Error)]
/// Error type for the core structures visualization module.
pub enum Error {
    #[error("Mermaid Entity Relationship Diagram error: {0}")]
    /// Error related to Mermaid Entity Relationship Diagrams.
    Mermaid(#[from] mermaid_builder::Error),
    #[error("Mermaid Flowchart error: {0}")]
    /// Error related to Diesel database operations.
    Diesel(#[from] diesel::result::Error),
}

impl From<ConfigError> for Error {
    fn from(err: ConfigError) -> Self {
        Error::Mermaid(err.into())
    }
}

impl From<EdgeError> for Error {
    fn from(err: EdgeError) -> Self {
        Error::Mermaid(err.into())
    }
}

impl From<NodeError> for Error {
    fn from(err: NodeError) -> Self {
        Error::Mermaid(err.into())
    }
}
