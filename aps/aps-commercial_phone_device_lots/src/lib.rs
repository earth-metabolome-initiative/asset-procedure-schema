//! Auto-generated crate for the `commercial_phone_device_lots` table.
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
/// camera lot and a commercial geopositioning-device lot.
#[table_model(ancestors(
    aps_entities::entities,
    aps_ownables::ownables,
    aps_namespaced_ownables::namespaced_ownables,
    aps_asset_models::asset_models,
    aps_physical_asset_models::physical_asset_models,
    aps_camera_models::camera_models,
    aps_commercial_product_lots::commercial_product_lots,
    aps_geopositioning_device_models::geopositioning_device_models,
    aps_commercial_camera_lots::commercial_camera_lots,
    aps_commercial_geopositioning_device_lots::commercial_geopositioning_device_lots
))]
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (belongs_to (aps_commercial_camera_lots :: CommercialCameraLot , foreign_key = id))]
# [diesel (belongs_to (aps_commercial_geopositioning_device_lots :: CommercialGeopositioningDeviceLot , foreign_key = id))]
# [table_model (foreign_key ((id ,) , (:: aps_commercial_camera_lots :: commercial_camera_lots :: id)))]
# [table_model (foreign_key ((id ,) , (:: aps_commercial_geopositioning_device_lots :: commercial_geopositioning_device_lots :: id)))]
#[table_model(default(aps_entities::entities::table_name_id, "commercial_phone_device_lots"))]
# [diesel (table_name = commercial_phone_device_lots)]
pub struct CommercialPhoneDeviceLot {
    /// Stable lot identifier shared with both commercial parent lot tables.
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
}
impl ::diesel_builders::GetColumn<aps_asset_models::asset_models::id> for CommercialPhoneDeviceLot {
    fn get_column_ref(
        &self,
    ) -> &<commercial_phone_device_lots::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_camera_models::camera_models::id>
    for CommercialPhoneDeviceLot
{
    fn get_column_ref(
        &self,
    ) -> &<commercial_phone_device_lots::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_commercial_camera_lots::commercial_camera_lots::id>
    for CommercialPhoneDeviceLot
{
    fn get_column_ref(
        &self,
    ) -> &<commercial_phone_device_lots::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl
    ::diesel_builders::GetColumn<
        aps_commercial_geopositioning_device_lots::commercial_geopositioning_device_lots::id,
    > for CommercialPhoneDeviceLot
{
    fn get_column_ref(
        &self,
    ) -> &<commercial_phone_device_lots::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_commercial_product_lots::commercial_product_lots::id>
    for CommercialPhoneDeviceLot
{
    fn get_column_ref(
        &self,
    ) -> &<commercial_phone_device_lots::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_entities::entities::id> for CommercialPhoneDeviceLot {
    fn get_column_ref(
        &self,
    ) -> &<commercial_phone_device_lots::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl
    ::diesel_builders::GetColumn<aps_geopositioning_device_models::geopositioning_device_models::id>
    for CommercialPhoneDeviceLot
{
    fn get_column_ref(
        &self,
    ) -> &<commercial_phone_device_lots::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_namespaced_ownables::namespaced_ownables::id>
    for CommercialPhoneDeviceLot
{
    fn get_column_ref(
        &self,
    ) -> &<commercial_phone_device_lots::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_ownables::ownables::id> for CommercialPhoneDeviceLot {
    fn get_column_ref(
        &self,
    ) -> &<commercial_phone_device_lots::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_physical_asset_models::physical_asset_models::id>
    for CommercialPhoneDeviceLot
{
    fn get_column_ref(
        &self,
    ) -> &<commercial_phone_device_lots::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
