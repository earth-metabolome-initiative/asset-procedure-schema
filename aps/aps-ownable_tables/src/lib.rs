//! Auto-generated crate for the `ownable_tables` table.
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
/// Table storing ownable tables
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (table_name = ownable_tables)]
pub struct OwnableTable {
    /// Name of an ownable table
    id: String,
}
impl ::diesel_builders::ValidateColumn<ownable_tables::id>
    for <ownable_tables::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column(id: &String) -> Result<(), Self::Error> {
        use diesel::Column;
        if id.is_empty() {
            return Err(::validation_errors::ValidationError::empty(
                <crate::ownable_tables::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::ownable_tables::id::NAME,
            ));
        }
        if id.len() < 255usize {
            return Err(::validation_errors::ValidationError::exceeds_max_length(
                <crate::ownable_tables::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::ownable_tables::id::NAME,
                255usize,
            ));
        }
        Ok(())
    }
}
