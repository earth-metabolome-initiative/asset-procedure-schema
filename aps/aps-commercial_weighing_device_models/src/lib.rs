//! Auto-generated crate for the `commercial_weighing_device_models` table.
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
/// Catalog of commercial weighing-device models.
#[table_model(ancestors(
    aps_entities::entities,
    aps_ownables::ownables,
    aps_namespaced_ownables::namespaced_ownables,
    aps_asset_models::asset_models,
    aps_commercial_products::commercial_products,
    aps_physical_asset_models::physical_asset_models,
    aps_weighing_device_models::weighing_device_models
))]
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (belongs_to (aps_commercial_products :: CommercialProduct , foreign_key = id))]
# [table_model (foreign_key ((weighing_device_model_id ,) , (:: aps_weighing_device_models :: weighing_device_models :: id)))]
# [table_model (foreign_key ((id ,) , (:: aps_weighing_device_models :: weighing_device_models :: id)))]
# [table_model (foreign_key ((id ,) , (:: aps_commercial_products :: commercial_products :: id)))]
#[table_model(default(aps_entities::entities::table_name_id, "commercial_weighing_device_models"))]
# [diesel (table_name = commercial_weighing_device_models)]
pub struct CommercialWeighingDeviceModel {
    /// Stable commercial model identifier shared with parent model tables.
    #[same_as(aps_asset_models::asset_models::id)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Base weighing-device model represented by this commercial model.
    #[same_as(aps_asset_models::asset_models::parent_model_id)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    weighing_device_model_id: ::rosetta_uuid::Uuid,
}
impl ::diesel_builders::GetColumn<aps_asset_models::asset_models::id>
    for CommercialWeighingDeviceModel
{
    fn get_column_ref(
        &self,
    ) -> &<commercial_weighing_device_models::id as ::diesel_builders::ColumnTyped>::ColumnType
    {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_commercial_products::commercial_products::id>
    for CommercialWeighingDeviceModel
{
    fn get_column_ref(
        &self,
    ) -> &<commercial_weighing_device_models::id as ::diesel_builders::ColumnTyped>::ColumnType
    {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_entities::entities::id> for CommercialWeighingDeviceModel {
    fn get_column_ref(
        &self,
    ) -> &<commercial_weighing_device_models::id as ::diesel_builders::ColumnTyped>::ColumnType
    {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_namespaced_ownables::namespaced_ownables::id>
    for CommercialWeighingDeviceModel
{
    fn get_column_ref(
        &self,
    ) -> &<commercial_weighing_device_models::id as ::diesel_builders::ColumnTyped>::ColumnType
    {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_ownables::ownables::id> for CommercialWeighingDeviceModel {
    fn get_column_ref(
        &self,
    ) -> &<commercial_weighing_device_models::id as ::diesel_builders::ColumnTyped>::ColumnType
    {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_physical_asset_models::physical_asset_models::id>
    for CommercialWeighingDeviceModel
{
    fn get_column_ref(
        &self,
    ) -> &<commercial_weighing_device_models::id as ::diesel_builders::ColumnTyped>::ColumnType
    {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_weighing_device_models::weighing_device_models::id>
    for CommercialWeighingDeviceModel
{
    fn get_column_ref(
        &self,
    ) -> &<commercial_weighing_device_models::id as ::diesel_builders::ColumnTyped>::ColumnType
    {
        &self.id
    }
}
