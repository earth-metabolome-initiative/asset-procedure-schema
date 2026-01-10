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
    serde :: Serialize,
    serde :: Deserialize,
    diesel :: Queryable,
    diesel :: Selectable,
    diesel :: Identifiable,
    diesel_builders :: prelude :: TableModel,
)]
/// Struct representing a row in the `weighing_devices` table.
#[table_model(ancestors(aps_assets::assets, aps_physical_assets::physical_assets))]
#[table_model(default(aps_assets::assets::asset_table_id, "weighing_devices"))]
# [diesel (table_name = weighing_devices)]
pub struct WeighingDevice {
    /// Field representing the `id` column in table `weighing_devices`.
    #[same_as(aps_physical_assets::physical_assets::id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `weighing_device_model_id` column in table
    /// `weighing_devices`.
    #[same_as(aps_physical_assets::physical_assets::physical_asset_model_id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    weighing_device_model_id: ::rosetta_uuid::Uuid,
}
:: diesel_builders :: prelude :: fk ! ((weighing_devices :: id) -> (:: aps_physical_assets :: physical_assets :: id));
:: diesel_builders :: prelude :: fk ! ((weighing_devices :: weighing_device_model_id) -> (:: aps_weighing_device_models :: weighing_device_models :: id));
impl diesel_builders::GetColumn<aps_assets::assets::id> for WeighingDevice {
    fn get_column_ref(&self) -> &<weighing_devices::id as diesel_builders::Typed>::ColumnType {
        &self.id
    }
}
impl diesel_builders::GetColumn<aps_physical_assets::physical_assets::id> for WeighingDevice {
    fn get_column_ref(&self) -> &<weighing_devices::id as diesel_builders::Typed>::ColumnType {
        &self.id
    }
}
