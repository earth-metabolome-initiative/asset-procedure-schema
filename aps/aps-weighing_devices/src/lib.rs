//! Auto-generated crate for the `weighing_devices` table.
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
/// Struct representing a row in the `weighing_devices` table.
#[table_model(ancestors(
    aps_ownables::ownables,
    aps_assets::assets,
    aps_physical_assets::physical_assets
))]
# [diesel (belongs_to (aps_physical_assets :: PhysicalAsset , foreign_key = id))]
# [diesel (belongs_to (aps_weighing_device_models :: WeighingDeviceModel , foreign_key = weighing_device_model_id))]
# [table_model (foreign_key ((id ,) , (:: aps_physical_assets :: physical_assets :: id)))]
# [table_model (foreign_key ((weighing_device_model_id ,) , (:: aps_weighing_device_models :: weighing_device_models :: id)))]
#[table_model(default(aps_ownables::ownables::ownable_table_id, "weighing_devices"))]
# [diesel (table_name = weighing_devices)]
pub struct WeighingDevice {
    /// Field representing the `id` column in table `weighing_devices`.
    #[same_as(aps_physical_assets::physical_assets::id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// The model of the weighing device.
    #[same_as(aps_physical_assets::physical_assets::physical_asset_model_id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    weighing_device_model_id: ::rosetta_uuid::Uuid,
}
impl ::diesel_builders::GetColumn<aps_assets::assets::id> for WeighingDevice {
    fn get_column_ref(
        &self,
    ) -> &<weighing_devices::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_ownables::ownables::id> for WeighingDevice {
    fn get_column_ref(
        &self,
    ) -> &<weighing_devices::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_physical_assets::physical_assets::id> for WeighingDevice {
    fn get_column_ref(
        &self,
    ) -> &<weighing_devices::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
