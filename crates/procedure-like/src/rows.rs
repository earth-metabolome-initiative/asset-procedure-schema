//! Values and columns the generated `TypedRows` trait of the `aps` crate
//! takes, so a runner writes and reads the rows of any typed procedure,
//! template or asset table it knows only by `table_name_id`.

use std::convert::Infallible;

use diesel_builders::{BuilderError, EitherValidationError, IncompleteBuilderError};
use rosetta_utc::TimestampUTC;
use rosetta_uuid::Uuid;
use sqlitegis::diesel::GeographyValue;
use validation_errors::ValidationError;

use crate::shape::SqlType;

/// A value of a shaped column, one variant for each [`SqlType`].
#[derive(Debug, Clone, PartialEq)]
pub enum ColumnValue {
    /// A `DOUBLE PRECISION` value.
    DoublePrecision(f64),
    /// A `SMALLINT` value.
    SmallInt(i16),
    /// A `GEOGRAPHY(POINT, 4326)` value.
    GeographyPoint(GeographyValue),
    /// A `UUID` value.
    Uuid(Uuid),
}

impl ColumnValue {
    /// The SQL type of the column this value fits.
    #[must_use]
    pub const fn sql_type(&self) -> SqlType {
        match self {
            Self::DoublePrecision(_) => SqlType::DoublePrecision,
            Self::SmallInt(_) => SqlType::SmallInt,
            Self::GeographyPoint(_) => SqlType::GeographyPoint,
            Self::Uuid(_) => SqlType::Uuid,
        }
    }
}

/// A Rust type the columns of one [`SqlType`] hold.
pub trait ColumnValueType: Into<ColumnValue> + Sized {
    /// SQL type of the columns holding this type.
    const SQL_TYPE: SqlType;

    /// The value of this type `value` holds, or `value` back when it holds
    /// another type.
    ///
    /// # Errors
    ///
    /// Returns `value` when it holds another type.
    fn from_column_value(value: ColumnValue) -> Result<Self, ColumnValue>;
}

/// Implements [`ColumnValueType`] and `From<$rust> for ColumnValue` for each
/// variant.
macro_rules! column_value_types {
    ($($variant:ident($rust:ty)),* $(,)?) => {$(
        impl From<$rust> for ColumnValue {
            fn from(value: $rust) -> Self {
                Self::$variant(value)
            }
        }

        impl ColumnValueType for $rust {
            const SQL_TYPE: SqlType = SqlType::$variant;

            fn from_column_value(value: ColumnValue) -> Result<Self, ColumnValue> {
                match value {
                    ColumnValue::$variant(value) => Ok(value),
                    other => Err(other),
                }
            }
        }
    )*};
}

column_value_types!(
    DoublePrecision(f64),
    SmallInt(i16),
    GeographyPoint(GeographyValue),
    Uuid(Uuid),
);

/// `value` as the Rust type `T` of `table.column`.
///
/// # Errors
///
/// Returns [`TypedRowError::ColumnType`] when `value` holds another type.
pub fn column_value<T: ColumnValueType>(
    table: &'static str,
    column: &'static str,
    value: ColumnValue,
) -> Result<T, TypedRowError> {
    T::from_column_value(value).map_err(|value| TypedRowError::ColumnType {
        table,
        column,
        expected: T::SQL_TYPE,
        given: value.sql_type(),
    })
}

/// The columns of `procedures` a typed procedure row is inserted with.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcedureColumns {
    /// Identifier of the procedure.
    pub id: Uuid,
    /// Organization holding the procedure.
    pub organization_id: Uuid,
    /// Project holding the procedure.
    pub project_id: Uuid,
    /// User creating the procedure.
    pub created_by_id: Uuid,
    /// Template the procedure executes.
    pub procedure_template_id: Uuid,
    /// Status of the procedure.
    pub status_id: String,
    /// Procedure running the step this procedure executes.
    pub parent_procedure_id: Option<Uuid>,
    /// Template of the parent procedure.
    pub parent_template_id: Option<Uuid>,
    /// Step of the parent template this procedure executes.
    pub step_id: Option<Uuid>,
    /// Root of the procedure tree.
    pub root_procedure_id: Uuid,
    /// Procedure asset this procedure repeats the step for.
    pub repeat_procedure_asset_id: Option<Uuid>,
    /// When the procedure is planned for.
    pub planned_for: Option<TimestampUTC>,
    /// When the procedure started.
    pub started_at: Option<TimestampUTC>,
    /// When the procedure ended.
    pub ended_at: Option<TimestampUTC>,
    /// User performing the procedure.
    pub performed_by_id: Option<Uuid>,
    /// Note on the procedure.
    pub note: Option<String>,
}

/// The columns of `assets` a produced asset row is inserted with.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssetColumns {
    /// Identifier of the asset.
    pub id: Uuid,
    /// Organization holding the asset.
    pub organization_id: Uuid,
    /// Project holding the asset.
    pub project_id: Uuid,
    /// User creating the asset.
    pub created_by_id: Uuid,
    /// Model the asset instantiates.
    pub model_id: Uuid,
    /// Name of the asset, unique in its project.
    pub name: String,
    /// Description of the asset.
    pub description: Option<String>,
}

/// Why a typed row could not be written or read.
#[derive(Debug, thiserror::Error)]
pub enum TypedRowError {
    /// No typed table of the requested kind has this name.
    #[error("no typed table of this kind is named `{0}`")]
    UnknownTable(String),
    /// The table has no shaped column of this name.
    #[error("`{table}` has no shaped column `{column}`")]
    UnknownColumn {
        /// The table.
        table: &'static str,
        /// The requested column.
        column: String,
    },
    /// The value does not fit the column's SQL type.
    #[error("`{table}.{column}` holds {expected:?} values, not {given:?}")]
    ColumnType {
        /// The table.
        table: &'static str,
        /// The column.
        column: &'static str,
        /// SQL type of the column.
        expected: SqlType,
        /// SQL type of the given value.
        given: SqlType,
    },
    /// A column an insert must fill has no value.
    #[error("`{table}.{column}` needs a value")]
    MissingValue {
        /// The table.
        table: &'static str,
        /// The column.
        column: &'static str,
    },
    /// The builder refused a value.
    #[error(transparent)]
    Validation(#[from] ValidationError),
    /// The builder lacks a mandatory related row.
    #[error(transparent)]
    Incomplete(#[from] IncompleteBuilderError),
    /// The database refused the statement.
    #[error(transparent)]
    Database(#[from] diesel::result::Error),
}

impl From<Infallible> for TypedRowError {
    fn from(never: Infallible) -> Self {
        match never {}
    }
}

impl<E: Into<Self>> From<BuilderError<E>> for TypedRowError {
    fn from(error: BuilderError<E>) -> Self {
        match error {
            BuilderError::Diesel(error) => Self::Database(error),
            BuilderError::Incomplete(error) => Self::Incomplete(error),
            BuilderError::Validation(error) => error.into(),
        }
    }
}

impl<L: Into<Self>, R: Into<Self>> From<EitherValidationError<L, R>> for TypedRowError {
    fn from(error: EitherValidationError<L, R>) -> Self {
        match error {
            EitherValidationError::Left(error) => error.into(),
            EitherValidationError::Right(error) => error.into(),
        }
    }
}
