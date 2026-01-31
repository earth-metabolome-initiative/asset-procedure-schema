//! Auto-generated crate for the `geopositioning_device_models` table.
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
/// Struct representing a row in the `geopositioning_device_models` table.
#[table_model(ancestors(
    aps_ownables::ownables,
    aps_asset_models::asset_models,
    aps_physical_asset_models::physical_asset_models
))]
# [diesel (belongs_to (aps_physical_asset_models :: PhysicalAssetModel , foreign_key = id))]
# [table_model (foreign_key ((id ,) , (:: aps_physical_asset_models :: physical_asset_models :: id)))]
#[table_model(default(aps_ownables::ownables::ownable_table_id, "geopositioning_device_models"))]
# [diesel (table_name = geopositioning_device_models)]
pub struct GeopositioningDeviceModel {
    /// Field representing the `id` column in table
    /// `geopositioning_device_models`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
}
impl ::diesel_builders::GetColumn<aps_asset_models::asset_models::id>
    for GeopositioningDeviceModel
{
    fn get_column_ref(
        &self,
    ) -> &<geopositioning_device_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_ownables::ownables::id> for GeopositioningDeviceModel {
    fn get_column_ref(
        &self,
    ) -> &<geopositioning_device_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_physical_asset_models::physical_asset_models::id>
    for GeopositioningDeviceModel
{
    fn get_column_ref(
        &self,
    ) -> &<geopositioning_device_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
