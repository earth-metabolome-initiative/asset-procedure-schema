//! Auto-generated crate for the `geopositioning_devices` table.
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
/// Struct representing a row in the `geopositioning_devices` table.
#[table_model(ancestors(aps_assets::assets, aps_physical_assets::physical_assets))]
#[table_model(default(aps_assets::assets::asset_table_id, "geopositioning_devices"))]
# [diesel (table_name = geopositioning_devices)]
pub struct GeopositioningDevice {
    /// Field representing the `id` column in table `geopositioning_devices`.
    #[same_as(aps_physical_assets::physical_assets::id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `commercial_geopositioning_device_lot_id` column
    /// in table `geopositioning_devices`.
    #[same_as(aps_physical_assets::physical_assets::physical_asset_model_id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    commercial_geopositioning_device_lot_id: ::rosetta_uuid::Uuid,
}
:: diesel_builders :: prelude :: fk ! ((geopositioning_devices :: id) -> (:: aps_physical_assets :: physical_assets :: id));
:: diesel_builders :: prelude :: fk ! ((geopositioning_devices :: commercial_geopositioning_device_lot_id) -> (:: aps_commercial_geopositioning_device_lots :: commercial_geopositioning_device_lots :: id));
impl diesel_builders::GetColumn<aps_assets::assets::id> for GeopositioningDevice {
    fn get_column_ref(
        &self,
    ) -> &<geopositioning_devices::id as diesel_builders::Typed>::ColumnType {
        &self.id
    }
}
impl diesel_builders::GetColumn<aps_physical_assets::physical_assets::id> for GeopositioningDevice {
    fn get_column_ref(
        &self,
    ) -> &<geopositioning_devices::id as diesel_builders::Typed>::ColumnType {
        &self.id
    }
}
