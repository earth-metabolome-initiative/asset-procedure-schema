//! The errors of a [`GuidedProcedure`](crate::GuidedProcedure).

use diesel_builders::BuilderError;
use validation_errors::ValidationError;

/// Why a [`GuidedProcedure`](crate::GuidedProcedure) refused a call.
#[derive(Debug, thiserror::Error)]
pub enum GuidedProcedureError {
    /// Every procedure of the run is already recorded.
    #[error("every procedure of the run is already recorded")]
    NoMoreProcedures,
    /// The run ended with procedures still to record.
    #[error("the procedure of template \"{template}\" is still to record")]
    Unrecorded {
        /// Name of the template of the next procedure.
        template: String,
    },
    /// The call names a procedure table that does not run the next template.
    #[error(
        "template \"{template}\" of table `{found}` runs next, the call names a procedure of `{expected}`"
    )]
    UnexpectedTable {
        /// Template table the procedure table of the call runs.
        expected: &'static str,
        /// Table of the template that runs next.
        found: String,
        /// Name of the template that runs next.
        template: String,
    },
    /// The root of a run cannot be skipped, since only a step can.
    #[error("the root \"{template}\" of the run cannot be skipped")]
    RootSkipped {
        /// Name of the root template.
        template: String,
    },
    /// The closure left `ended_at` unset, which completing the procedure needs.
    #[error("the procedure of template \"{template}\" has no `ended_at` to complete with")]
    MissingEndedAt {
        /// Name of the template.
        template: String,
    },
    /// The closure bound a slot the template does not declare.
    #[error("template \"{template}\" declares no slot \"{slot}\"")]
    UnknownSlot {
        /// Name of the template.
        template: String,
        /// Name the closure gave.
        slot: String,
    },
    /// The closure bound a slot its step fills with the parent's assets.
    #[error("slot \"{slot}\" of template \"{template}\" takes the assets its step binds to it")]
    InheritedSlot {
        /// Name of the template.
        template: String,
        /// Name of the slot.
        slot: String,
    },
    /// A builder refused a row.
    #[error(transparent)]
    Builder(#[from] BuilderError<ValidationError>),
    /// The connection refused a statement.
    #[error(transparent)]
    Diesel(#[from] diesel::result::Error),
}
