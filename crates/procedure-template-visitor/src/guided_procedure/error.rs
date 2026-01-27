//! Submodule defining the errors which might occur when using the
//! `GuidedProcedure`.

use aps_procedure_templates::*;

#[derive(Debug, thiserror::Error)]
/// Enum representing the possible errors which might occur when using the
/// `GuidedProcedure`.
pub enum InternalGuidedProcedureError<'graph> {
    #[error("The designated successor '{}' was not found in the viable successors: [{}]", .designated_successor.name(), .viable_successors.iter().map(|pt| pt.name().as_str()).collect::<Vec<_>>().join(", "))]
    /// When the provided designated successor was not found in the viable
    /// successors.
    DesignatedSuccessorNotFound {
        designated_successor: &'graph ProcedureTemplate,
        viable_successors: Vec<&'graph ProcedureTemplate>,
    },
    #[error("Multiple viable successors were found: [{}]. Please specify a designated successor.", .viable_successors.iter().map(|pt| pt.name().as_str()).collect::<Vec<_>>().join(", "))]
    /// When no designated successor was provided but multiple viable successors
    /// were found.
    UnclearSuccessor { viable_successors: Vec<&'graph ProcedureTemplate> },
    #[error("A builder not yet processed for the procedure template \"{}\" from table \"{}\". You most likely need to add another `.and_then(|builder, conn| {{...}})` to your guided procedure.", .0.name(), .0.procedure_template_table_id())]
    /// A builder was not yet processed.
    UnprocessedBuilder(&'graph ProcedureTemplate),
    #[error("A database error occurred: {0}")]
    /// An error occurred while interacting with the database.
    Diesel(diesel::result::Error),
}

#[derive(Debug, thiserror::Error)]
/// Enum representing the possible errors which might occur when using the
/// `GuidedProcedure`.
pub enum GuidedProcedureError {
    #[error("No more builders are available.")]
    /// No more builders are available.
    NoMoreBuilders,
    #[error("There are unprocessed builders for the procedure template \"{}\" from table \"{}\". You most likely need to add another `.and_then(|builder, conn| {{...}})` to your guided procedure.", .0.name(), .0.procedure_template_table_id())]
    /// Incomplete processing of builders.
    UnprocessedBuilder(Box<ProcedureTemplate>),
    #[error("A database error occurred: {0}")]
    /// An error occurred while interacting with the database.
    Diesel(diesel::result::Error),
    #[error("Expected builder of type `{expected}`, but a builder of type `{found}` is required to build the procedure template \"{}\" from table \"{}\".", .template.name(), .template.procedure_template_table_id())]
    /// Unexpected builder type encountered.
    UnexpectedBuilder { expected: &'static str, found: String, template: Box<ProcedureTemplate> },
}

impl<'graph> From<InternalGuidedProcedureError<'graph>> for GuidedProcedureError {
    fn from(value: InternalGuidedProcedureError<'graph>) -> Self {
        match value {
            InternalGuidedProcedureError::DesignatedSuccessorNotFound { .. } => {
                todo!("Define error handling for DesignatedSuccessorNotFound")
            }
            InternalGuidedProcedureError::UnclearSuccessor { viable_successors: _ } => {
                todo!("Define error handling for UnclearSuccessor")
            }
            InternalGuidedProcedureError::UnprocessedBuilder(template) => {
                GuidedProcedureError::UnprocessedBuilder(Box::new((*template).clone()))
            }
            InternalGuidedProcedureError::Diesel(e) => GuidedProcedureError::Diesel(e),
        }
    }
}
