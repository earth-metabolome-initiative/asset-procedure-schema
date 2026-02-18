//! Auto-generated crate for the `volumetric_containers` table.
#[derive(
    Copy,
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
/// Struct representing a row in the `volumetric_containers` table.
#[table_model(ancestors(
    aps_entities::entities,
    aps_ownables::ownables,
    aps_namespaced_ownables::namespaced_ownables,
    aps_assets::assets,
    aps_physical_assets::physical_assets,
    aps_containers::containers
))]
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (belongs_to (aps_containers :: Container , foreign_key = id))]
# [diesel (belongs_to (aps_volumetric_container_models :: VolumetricContainerModel , foreign_key = volumetric_container_model_id))]
# [table_model (foreign_key ((id ,) , (:: aps_containers :: containers :: id)))]
# [table_model (foreign_key ((volumetric_container_model_id ,) , (:: aps_volumetric_container_models :: volumetric_container_models :: id)))]
#[table_model(default(aps_entities::entities::table_name_id, "volumetric_containers"))]
# [diesel (table_name = volumetric_containers)]
pub struct VolumetricContainer {
    /// Field representing the `id` column in table `volumetric_containers`.
    #[same_as(aps_physical_assets::physical_assets::id)]
    #[same_as(aps_containers::containers::id)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `volumetric_container_model_id` column in table
    /// `volumetric_containers`.
    #[same_as(aps_containers::containers::container_model_id)]
    #[same_as(aps_physical_assets::physical_assets::physical_asset_model_id)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    volumetric_container_model_id: ::rosetta_uuid::Uuid,
}
impl ::diesel_builders::GetColumn<aps_assets::assets::id> for VolumetricContainer {
    fn get_column_ref(
        &self,
    ) -> &<volumetric_containers::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_containers::containers::id> for VolumetricContainer {
    fn get_column_ref(
        &self,
    ) -> &<volumetric_containers::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_entities::entities::id> for VolumetricContainer {
    fn get_column_ref(
        &self,
    ) -> &<volumetric_containers::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_namespaced_ownables::namespaced_ownables::id>
    for VolumetricContainer
{
    fn get_column_ref(
        &self,
    ) -> &<volumetric_containers::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_ownables::ownables::id> for VolumetricContainer {
    fn get_column_ref(
        &self,
    ) -> &<volumetric_containers::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_physical_assets::physical_assets::id>
    for VolumetricContainer
{
    fn get_column_ref(
        &self,
    ) -> &<volumetric_containers::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
