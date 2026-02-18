//! Auto-generated crate for the `physical_asset_model_lifecycle_classes` table.
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
/// Catalog of allowed lifecycle classes for physical asset models.
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (table_name = physical_asset_model_lifecycle_classes)]
pub struct PhysicalAssetModelLifecycleClass {
    /// Stable lifecycle class identifier.
    id: String,
}
impl ::diesel_builders::ValidateColumn<physical_asset_model_lifecycle_classes::id>
    for <physical_asset_model_lifecycle_classes::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column(id: &String) -> Result<(), Self::Error> {
        use diesel::Column;
        if id.is_empty() {
            return Err (:: validation_errors :: ValidationError :: empty (< crate :: physical_asset_model_lifecycle_classes :: table as :: diesel_builders :: TableExt > :: TABLE_NAME , crate :: physical_asset_model_lifecycle_classes :: id :: NAME)) ;
        }
        if id.len() > 255usize {
            return Err (:: validation_errors :: ValidationError :: exceeds_max_length (< crate :: physical_asset_model_lifecycle_classes :: table as :: diesel_builders :: TableExt > :: TABLE_NAME , crate :: physical_asset_model_lifecycle_classes :: id :: NAME , 255usize)) ;
        }
        Ok(())
    }
}
