//! Auto-generated crate for the `asset_tables` table.
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
/// and facilitate DAG traversal.
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (table_name = asset_tables)]
pub struct AssetTable {
    /// and facilitate DAG traversal.
    id: String,
}
impl ::diesel_builders::ValidateColumn<asset_tables::id>
    for <asset_tables::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column(id: &String) -> Result<(), Self::Error> {
        use diesel::Column;
        if id.is_empty() {
            return Err(::validation_errors::ValidationError::empty(
                "asset_tables",
                crate::asset_tables::id::NAME,
            ));
        }
        Ok(())
    }
}
