//! Auto-generated crate for the `table_names` table.
#[derive(
    Clone,
    Default,
    Debug,
    Hash,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    :: serde :: Serialize,
    :: serde :: Deserialize,
    :: diesel :: Queryable,
    :: diesel :: Selectable,
    :: diesel :: Identifiable,
    :: diesel_builders :: prelude :: TableModel,
)]
/// Table storing all table names for type identification
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (table_name = table_names)]
pub struct TableName {
    /// Field representing the `id` column in table `table_names`.
    id: String,
}
impl ::diesel_builders::ValidateColumn<table_names::id>
    for <table_names::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column(id: &String) -> Result<(), Self::Error> {
        use diesel::Column;
        if id.is_empty() {
            return Err(::validation_errors::ValidationError::empty(
                <crate::table_names::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::table_names::id::NAME,
            ));
        }
        if id.len() < 255usize {
            return Err(::validation_errors::ValidationError::exceeds_max_length(
                <crate::table_names::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::table_names::id::NAME,
                255usize,
            ));
        }
        Ok(())
    }
}
