//! Auto-generated crate for the `commercial_geopositioning_device_lots` table.
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
/// Struct representing a row in the `commercial_geopositioning_device_lots`
/// table.
#[table_model(ancestors(
    aps_entities::entities,
    aps_ownables::ownables,
    aps_asset_models::asset_models,
    aps_physical_asset_models::physical_asset_models,
    aps_commercial_product_lots::commercial_product_lots,
    aps_geopositioning_device_models::geopositioning_device_models
))]
# [diesel (belongs_to (aps_commercial_geopositioning_device_models :: CommercialGeopositioningDeviceModel , foreign_key = commercial_geopositioning_device_model_id))]
# [diesel (belongs_to (aps_commercial_product_lots :: CommercialProductLot , foreign_key = id))]
# [diesel (belongs_to (aps_geopositioning_device_models :: GeopositioningDeviceModel , foreign_key = id))]
# [table_model (foreign_key ((commercial_geopositioning_device_model_id ,) , (:: aps_commercial_geopositioning_device_models :: commercial_geopositioning_device_models :: id)))]
# [table_model (foreign_key ((id ,) , (:: aps_commercial_product_lots :: commercial_product_lots :: id)))]
# [table_model (foreign_key ((id ,) , (:: aps_geopositioning_device_models :: geopositioning_device_models :: id)))]
#[table_model(default(
    aps_entities::entities::table_name_id,
    "commercial_geopositioning_device_lots"
))]
# [diesel (table_name = commercial_geopositioning_device_lots)]
pub struct CommercialGeopositioningDeviceLot {
    /// Field representing the `id` column in table
    /// `commercial_geopositioning_device_lots`.
    #[same_as(aps_commercial_product_lots::commercial_product_lots::id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `commercial_geopositioning_device_model_id`
    /// column in table `commercial_geopositioning_device_lots`.
    #[same_as(aps_commercial_product_lots::commercial_product_lots::product_model_id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    commercial_geopositioning_device_model_id: ::rosetta_uuid::Uuid,
}
impl ::diesel_builders::GetColumn<aps_asset_models::asset_models::id>
    for CommercialGeopositioningDeviceLot
{
    fn get_column_ref(
        &self,
    ) -> &<commercial_geopositioning_device_lots::id as ::diesel_builders::ColumnTyped>::ColumnType
    {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_commercial_product_lots::commercial_product_lots::id>
    for CommercialGeopositioningDeviceLot
{
    fn get_column_ref(
        &self,
    ) -> &<commercial_geopositioning_device_lots::id as ::diesel_builders::ColumnTyped>::ColumnType
    {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_entities::entities::id>
    for CommercialGeopositioningDeviceLot
{
    fn get_column_ref(
        &self,
    ) -> &<commercial_geopositioning_device_lots::id as ::diesel_builders::ColumnTyped>::ColumnType
    {
        &self.id
    }
}
impl
    ::diesel_builders::GetColumn<aps_geopositioning_device_models::geopositioning_device_models::id>
    for CommercialGeopositioningDeviceLot
{
    fn get_column_ref(
        &self,
    ) -> &<commercial_geopositioning_device_lots::id as ::diesel_builders::ColumnTyped>::ColumnType
    {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_ownables::ownables::id>
    for CommercialGeopositioningDeviceLot
{
    fn get_column_ref(
        &self,
    ) -> &<commercial_geopositioning_device_lots::id as ::diesel_builders::ColumnTyped>::ColumnType
    {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_physical_asset_models::physical_asset_models::id>
    for CommercialGeopositioningDeviceLot
{
    fn get_column_ref(
        &self,
    ) -> &<commercial_geopositioning_device_lots::id as ::diesel_builders::ColumnTyped>::ColumnType
    {
        &self.id
    }
}
