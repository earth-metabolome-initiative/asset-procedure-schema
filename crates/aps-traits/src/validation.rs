//! The validation errors of the builders of the `aps/` crates, as one type.

use core::convert::Infallible;

use diesel_builders::{BuilderError, BuilderResult, EitherValidationError};
use validation_errors::ValidationError;

/// A validation error of a builder of the `aps/` crates. A table extending
/// others joins its error with theirs, and the root tables validate nothing.
pub trait IntoValidationError {
    /// The validation error this one carries.
    fn into_validation_error(self) -> ValidationError;
}

impl IntoValidationError for ValidationError {
    fn into_validation_error(self) -> ValidationError {
        self
    }
}

impl IntoValidationError for Infallible {
    fn into_validation_error(self) -> ValidationError {
        match self {}
    }
}

impl<L: IntoValidationError, R: IntoValidationError> IntoValidationError
    for EitherValidationError<L, R>
{
    fn into_validation_error(self) -> ValidationError {
        match self {
            Self::Left(error) => error.into_validation_error(),
            Self::Right(error) => error.into_validation_error(),
        }
    }
}

/// `outcome` with its validation error as a [`ValidationError`].
///
/// # Errors
///
/// Returns the error of `outcome`.
pub fn flatten<T, E: IntoValidationError>(
    outcome: BuilderResult<T, E>,
) -> BuilderResult<T, ValidationError> {
    outcome.map_err(|error| {
        match error {
            BuilderError::Diesel(error) => BuilderError::Diesel(error),
            BuilderError::Incomplete(error) => BuilderError::Incomplete(error),
            BuilderError::Validation(error) => {
                BuilderError::Validation(error.into_validation_error())
            }
        }
    })
}

/// `outcome` of a validated default or setter, its error as a
/// [`BuilderError`].
///
/// # Errors
///
/// Returns the error of `outcome`.
pub fn validated<T, E: IntoValidationError>(
    outcome: Result<T, E>,
) -> BuilderResult<T, ValidationError> {
    outcome.map_err(|error| BuilderError::Validation(error.into_validation_error()))
}
