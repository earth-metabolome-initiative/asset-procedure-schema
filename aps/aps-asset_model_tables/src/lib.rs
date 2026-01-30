//! Auto-generated crate for the `ownables` table.
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
# [diesel (table_name = ownables)]
pub struct AssetModelTable {
    /// and facilitate DAG traversal.
    id: String,
}
impl ::diesel_builders::ValidateColumn<ownables::id>
    for <ownables::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column(id: &String) -> Result<(), Self::Error> {
        use diesel::Column;
        if id.is_empty() {
            return Err(::validation_errors::ValidationError::empty(
                "ownables",
                crate::ownables::id::NAME,
            ));
        }
        Ok(())
    }
}
