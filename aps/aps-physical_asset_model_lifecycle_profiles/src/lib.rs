//! Auto-generated crate for the `physical_asset_model_lifecycle_profiles`
//! table.
#[derive(
    Clone,
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
    :: diesel :: Associations,
    :: diesel_builders :: prelude :: TableModel,
)]
/// Lifecycle metadata for physical asset models.
#[table_model(ancestors(
    aps_entities::entities,
    aps_ownables::ownables,
    aps_namespaced_ownables::namespaced_ownables,
    aps_asset_models::asset_models,
    aps_physical_asset_models::physical_asset_models
))]
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (belongs_to (aps_physical_asset_models :: PhysicalAssetModel , foreign_key = id))]
# [diesel (belongs_to (aps_physical_asset_model_lifecycle_classes :: PhysicalAssetModelLifecycleClass , foreign_key = lifecycle_class_id))]
# [table_model (foreign_key ((id ,) , (:: aps_physical_asset_models :: physical_asset_models :: id)))]
# [table_model (foreign_key ((lifecycle_class_id ,) , (:: aps_physical_asset_model_lifecycle_classes :: physical_asset_model_lifecycle_classes :: id)))]
#[table_model(default(
    aps_entities::entities::table_name_id,
    "physical_asset_model_lifecycle_profiles"
))]
# [diesel (table_name = physical_asset_model_lifecycle_profiles)]
pub struct PhysicalAssetModelLifecycleProfile {
    /// Physical asset model that this lifecycle profile describes.
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Lifecycle class identifier independent of taxonomy folders.
    lifecycle_class_id: String,
    /// Suggested maximum number of uses for reusable models.
    recommended_max_use: Option<i16>,
}
impl ::diesel_builders::ValidateColumn<physical_asset_model_lifecycle_profiles::lifecycle_class_id>
    for <physical_asset_model_lifecycle_profiles::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column(lifecycle_class_id: &String) -> Result<(), Self::Error> {
        use diesel::Column;
        if lifecycle_class_id.is_empty() {
            return Err (:: validation_errors :: ValidationError :: empty (< crate :: physical_asset_model_lifecycle_profiles :: table as :: diesel_builders :: TableExt > :: TABLE_NAME , crate :: physical_asset_model_lifecycle_profiles :: lifecycle_class_id :: NAME)) ;
        }
        if lifecycle_class_id.len() > 255usize {
            return Err (:: validation_errors :: ValidationError :: exceeds_max_length (< crate :: physical_asset_model_lifecycle_profiles :: table as :: diesel_builders :: TableExt > :: TABLE_NAME , crate :: physical_asset_model_lifecycle_profiles :: lifecycle_class_id :: NAME , 255usize)) ;
        }
        Ok(())
    }
}
impl ::diesel_builders::ValidateColumn<physical_asset_model_lifecycle_profiles::recommended_max_use>
    for <physical_asset_model_lifecycle_profiles::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column(recommended_max_use: &i16) -> Result<(), Self::Error> {
        use diesel::Column;
        if recommended_max_use <= &0i16 {
            return Err (:: validation_errors :: ValidationError :: strictly_greater_than_value (< crate :: physical_asset_model_lifecycle_profiles :: table as :: diesel_builders :: TableExt > :: TABLE_NAME , crate :: physical_asset_model_lifecycle_profiles :: recommended_max_use :: NAME , 0f64)) ;
        }
        Ok(())
    }
}
impl ::diesel_builders::GetColumn<aps_asset_models::asset_models::id>
    for PhysicalAssetModelLifecycleProfile
{
    fn get_column_ref(
        &self,
    ) -> &<physical_asset_model_lifecycle_profiles::id as ::diesel_builders::ColumnTyped>::ColumnType
    {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_entities::entities::id>
    for PhysicalAssetModelLifecycleProfile
{
    fn get_column_ref(
        &self,
    ) -> &<physical_asset_model_lifecycle_profiles::id as ::diesel_builders::ColumnTyped>::ColumnType
    {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_namespaced_ownables::namespaced_ownables::id>
    for PhysicalAssetModelLifecycleProfile
{
    fn get_column_ref(
        &self,
    ) -> &<physical_asset_model_lifecycle_profiles::id as ::diesel_builders::ColumnTyped>::ColumnType
    {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_ownables::ownables::id>
    for PhysicalAssetModelLifecycleProfile
{
    fn get_column_ref(
        &self,
    ) -> &<physical_asset_model_lifecycle_profiles::id as ::diesel_builders::ColumnTyped>::ColumnType
    {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_physical_asset_models::physical_asset_models::id>
    for PhysicalAssetModelLifecycleProfile
{
    fn get_column_ref(
        &self,
    ) -> &<physical_asset_model_lifecycle_profiles::id as ::diesel_builders::ColumnTyped>::ColumnType
    {
        &self.id
    }
}
