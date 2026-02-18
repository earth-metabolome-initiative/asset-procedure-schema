//! Auto-generated crate for the `volume_measuring_devices` table.
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
/// Physical volume-measuring devices tracked in APS inventory.
#[table_model(ancestors(
    aps_entities::entities,
    aps_ownables::ownables,
    aps_namespaced_ownables::namespaced_ownables,
    aps_assets::assets,
    aps_physical_assets::physical_assets
))]
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (belongs_to (aps_physical_assets :: PhysicalAsset , foreign_key = id))]
# [diesel (belongs_to (aps_volume_measuring_device_models :: VolumeMeasuringDeviceModel , foreign_key = volume_measuring_device_model_id))]
# [table_model (foreign_key ((id ,) , (:: aps_physical_assets :: physical_assets :: id)))]
# [table_model (foreign_key ((volume_measuring_device_model_id ,) , (:: aps_volume_measuring_device_models :: volume_measuring_device_models :: id)))]
#[table_model(default(aps_entities::entities::table_name_id, "volume_measuring_devices"))]
# [diesel (table_name = volume_measuring_devices)]
pub struct VolumeMeasuringDevice {
    /// Stable asset identifier inherited from `physical_assets`.
    #[same_as(aps_physical_assets::physical_assets::id)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Volume-measuring model instantiated by this physical asset.
    #[same_as(aps_physical_assets::physical_assets::physical_asset_model_id)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    volume_measuring_device_model_id: ::rosetta_uuid::Uuid,
}
impl ::diesel_builders::GetColumn<aps_assets::assets::id> for VolumeMeasuringDevice {
    fn get_column_ref(
        &self,
    ) -> &<volume_measuring_devices::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_entities::entities::id> for VolumeMeasuringDevice {
    fn get_column_ref(
        &self,
    ) -> &<volume_measuring_devices::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_namespaced_ownables::namespaced_ownables::id>
    for VolumeMeasuringDevice
{
    fn get_column_ref(
        &self,
    ) -> &<volume_measuring_devices::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_ownables::ownables::id> for VolumeMeasuringDevice {
    fn get_column_ref(
        &self,
    ) -> &<volume_measuring_devices::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_physical_assets::physical_assets::id>
    for VolumeMeasuringDevice
{
    fn get_column_ref(
        &self,
    ) -> &<volume_measuring_devices::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
