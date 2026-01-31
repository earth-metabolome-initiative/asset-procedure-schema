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
    :: serde :: Serialize,
    :: serde :: Deserialize,
    :: diesel :: Queryable,
    :: diesel :: Selectable,
    :: diesel :: Identifiable,
    :: diesel :: Associations,
    :: diesel_builders :: prelude :: TableModel,
)]
/// Struct representing a row in the `geopositioning_devices` table.
#[table_model(ancestors(
    aps_ownables::ownables,
    aps_assets::assets,
    aps_physical_assets::physical_assets
))]
# [diesel (belongs_to (aps_physical_assets :: PhysicalAsset , foreign_key = id))]
# [diesel (belongs_to (aps_commercial_geopositioning_device_lots :: CommercialGeopositioningDeviceLot , foreign_key = commercial_geopositioning_device_lot_id))]
# [table_model (foreign_key ((id ,) , (:: aps_physical_assets :: physical_assets :: id)))]
# [table_model (foreign_key ((commercial_geopositioning_device_lot_id ,) , (:: aps_commercial_geopositioning_device_lots :: commercial_geopositioning_device_lots :: id)))]
#[table_model(default(aps_ownables::ownables::ownable_table_id, "geopositioning_devices"))]
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
impl ::diesel_builders::GetColumn<aps_assets::assets::id> for GeopositioningDevice {
    fn get_column_ref(
        &self,
    ) -> &<geopositioning_devices::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_ownables::ownables::id> for GeopositioningDevice {
    fn get_column_ref(
        &self,
    ) -> &<geopositioning_devices::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_physical_assets::physical_assets::id>
    for GeopositioningDevice
{
    fn get_column_ref(
        &self,
    ) -> &<geopositioning_devices::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
