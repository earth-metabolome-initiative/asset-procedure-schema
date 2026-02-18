//! Auto-generated crate for the `commercial_phone_device_models` table.
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
/// geopositioning models, so each row must satisfy both commercial ancestries.
#[table_model(ancestors(
    aps_entities::entities,
    aps_ownables::ownables,
    aps_namespaced_ownables::namespaced_ownables,
    aps_asset_models::asset_models,
    aps_commercial_products::commercial_products,
    aps_physical_asset_models::physical_asset_models,
    aps_camera_models::camera_models,
    aps_geopositioning_device_models::geopositioning_device_models,
    aps_commercial_camera_models::commercial_camera_models,
    aps_commercial_geopositioning_device_models::commercial_geopositioning_device_models
))]
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (belongs_to (aps_commercial_camera_models :: CommercialCameraModel , foreign_key = id))]
# [diesel (belongs_to (aps_commercial_geopositioning_device_models :: CommercialGeopositioningDeviceModel , foreign_key = id))]
# [table_model (foreign_key ((id ,) , (:: aps_commercial_camera_models :: commercial_camera_models :: id)))]
# [table_model (foreign_key ((id ,) , (:: aps_commercial_geopositioning_device_models :: commercial_geopositioning_device_models :: id)))]
#[table_model(default(aps_entities::entities::table_name_id, "commercial_phone_device_models"))]
# [diesel (table_name = commercial_phone_device_models)]
pub struct CommercialPhoneDeviceModel {
    /// Stable commercial model identifier shared with both commercial parent
    /// models.
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
}
impl ::diesel_builders::GetColumn<aps_asset_models::asset_models::id>
    for CommercialPhoneDeviceModel
{
    fn get_column_ref(
        &self,
    ) -> &<commercial_phone_device_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_camera_models::camera_models::id>
    for CommercialPhoneDeviceModel
{
    fn get_column_ref(
        &self,
    ) -> &<commercial_phone_device_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_commercial_camera_models::commercial_camera_models::id>
    for CommercialPhoneDeviceModel
{
    fn get_column_ref(
        &self,
    ) -> &<commercial_phone_device_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl
    ::diesel_builders::GetColumn<
        aps_commercial_geopositioning_device_models::commercial_geopositioning_device_models::id,
    > for CommercialPhoneDeviceModel
{
    fn get_column_ref(
        &self,
    ) -> &<commercial_phone_device_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_commercial_products::commercial_products::id>
    for CommercialPhoneDeviceModel
{
    fn get_column_ref(
        &self,
    ) -> &<commercial_phone_device_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_entities::entities::id> for CommercialPhoneDeviceModel {
    fn get_column_ref(
        &self,
    ) -> &<commercial_phone_device_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl
    ::diesel_builders::GetColumn<aps_geopositioning_device_models::geopositioning_device_models::id>
    for CommercialPhoneDeviceModel
{
    fn get_column_ref(
        &self,
    ) -> &<commercial_phone_device_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_namespaced_ownables::namespaced_ownables::id>
    for CommercialPhoneDeviceModel
{
    fn get_column_ref(
        &self,
    ) -> &<commercial_phone_device_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_ownables::ownables::id> for CommercialPhoneDeviceModel {
    fn get_column_ref(
        &self,
    ) -> &<commercial_phone_device_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_physical_asset_models::physical_asset_models::id>
    for CommercialPhoneDeviceModel
{
    fn get_column_ref(
        &self,
    ) -> &<commercial_phone_device_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
