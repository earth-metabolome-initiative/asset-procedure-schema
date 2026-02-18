//! Auto-generated crate for the `phone_devices` table.
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
/// camera and a geopositioning device.
#[table_model(ancestors(
    aps_entities::entities,
    aps_ownables::ownables,
    aps_namespaced_ownables::namespaced_ownables,
    aps_assets::assets,
    aps_physical_assets::physical_assets,
    aps_cameras::cameras,
    aps_geopositioning_devices::geopositioning_devices
))]
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (belongs_to (aps_cameras :: Camera , foreign_key = id))]
# [diesel (belongs_to (aps_geopositioning_devices :: GeopositioningDevice , foreign_key = id))]
# [table_model (foreign_key ((id ,) , (:: aps_cameras :: cameras :: id)))]
# [table_model (foreign_key ((id ,) , (:: aps_geopositioning_devices :: geopositioning_devices :: id)))]
#[table_model(default(aps_entities::entities::table_name_id, "phone_devices"))]
# [diesel (table_name = phone_devices)]
pub struct PhoneDevice {
    /// Stable device identifier shared with parent camera/geopositioning
    /// devices.
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
}
impl ::diesel_builders::GetColumn<aps_assets::assets::id> for PhoneDevice {
    fn get_column_ref(&self) -> &<phone_devices::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_cameras::cameras::id> for PhoneDevice {
    fn get_column_ref(&self) -> &<phone_devices::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_entities::entities::id> for PhoneDevice {
    fn get_column_ref(&self) -> &<phone_devices::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_geopositioning_devices::geopositioning_devices::id>
    for PhoneDevice
{
    fn get_column_ref(&self) -> &<phone_devices::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_namespaced_ownables::namespaced_ownables::id>
    for PhoneDevice
{
    fn get_column_ref(&self) -> &<phone_devices::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_ownables::ownables::id> for PhoneDevice {
    fn get_column_ref(&self) -> &<phone_devices::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_physical_assets::physical_assets::id> for PhoneDevice {
    fn get_column_ref(&self) -> &<phone_devices::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
