//! Submodule defining the errors which might occur when using the
//! `GuidedProcedurePseudocode`.

#[derive(Debug, thiserror::Error, Copy, Clone, PartialEq, Eq)]
/// Enum representing the possible errors which might occur when using the
/// `GuidedProcedurePseudocode`.
pub enum GuidedProcedurePseudocodeError {
    #[error(
        "The provided graph is not a simple path (a chain of procedure templates without branches)."
    )]
    /// The provided graph is not a simple path.
    NotASimplePath,
    #[error(
        "There are no procedures for which user guidance is required after applying the skip logic."
    )]
    /// There are no procedures for which user guidance is required after
    /// applying the skip logic.
    NoUnskippedProcedures,
}
