//! Auto-generated crate for the `volumetric_container_models` table.
#[derive(
    Copy,
    Clone,
    Debug,
    PartialOrd,
    PartialEq,
    :: serde :: Serialize,
    :: serde :: Deserialize,
    :: diesel :: Queryable,
    :: diesel :: Selectable,
    :: diesel :: Identifiable,
    :: diesel :: Associations,
    :: diesel_builders :: prelude :: TableModel,
)]
/// Struct representing a row in the `volumetric_container_models` table.
#[table_model(ancestors(
    aps_entities::entities,
    aps_ownables::ownables,
    aps_namespaced_ownables::namespaced_ownables,
    aps_asset_models::asset_models,
    aps_physical_asset_models::physical_asset_models,
    aps_container_models::container_models
))]
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (belongs_to (aps_container_models :: ContainerModel , foreign_key = id))]
# [table_model (foreign_key ((id ,) , (:: aps_container_models :: container_models :: id)))]
#[table_model(default(aps_entities::entities::table_name_id, "volumetric_container_models"))]
# [diesel (table_name = volumetric_container_models)]
pub struct VolumetricContainerModel {
    /// Field representing the `id` column in table
    /// `volumetric_container_models`.
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Volume in liters. The maximum volume of the container.
    volume: f32,
}
impl ::diesel_builders::ValidateColumn<volumetric_container_models::volume>
    for <volumetric_container_models::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column(volume: &f32) -> Result<(), Self::Error> {
        use diesel::Column;
        if volume <= &0f32 {
            return Err (:: validation_errors :: ValidationError :: strictly_greater_than_value (< crate :: volumetric_container_models :: table as :: diesel_builders :: TableExt > :: TABLE_NAME , crate :: volumetric_container_models :: volume :: NAME , 0f64)) ;
        }
        Ok(())
    }
}
impl ::diesel_builders::GetColumn<aps_asset_models::asset_models::id> for VolumetricContainerModel {
    fn get_column_ref(
        &self,
    ) -> &<volumetric_container_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_container_models::container_models::id>
    for VolumetricContainerModel
{
    fn get_column_ref(
        &self,
    ) -> &<volumetric_container_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_entities::entities::id> for VolumetricContainerModel {
    fn get_column_ref(
        &self,
    ) -> &<volumetric_container_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_namespaced_ownables::namespaced_ownables::id>
    for VolumetricContainerModel
{
    fn get_column_ref(
        &self,
    ) -> &<volumetric_container_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_ownables::ownables::id> for VolumetricContainerModel {
    fn get_column_ref(
        &self,
    ) -> &<volumetric_container_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_physical_asset_models::physical_asset_models::id>
    for VolumetricContainerModel
{
    fn get_column_ref(
        &self,
    ) -> &<volumetric_container_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
