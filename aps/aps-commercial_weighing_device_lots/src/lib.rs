//! Auto-generated crate for the `commercial_weighing_device_lots` table.
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
/// Struct representing a row in the `commercial_weighing_device_lots` table.
#[table_model(ancestors(
    aps_entities::entities,
    aps_ownables::ownables,
    aps_namespaced_ownables::namespaced_ownables,
    aps_asset_models::asset_models,
    aps_physical_asset_models::physical_asset_models,
    aps_commercial_product_lots::commercial_product_lots,
    aps_weighing_device_models::weighing_device_models
))]
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (belongs_to (aps_commercial_weighing_device_models :: CommercialWeighingDeviceModel , foreign_key = commercial_weighing_device_model_id))]
# [diesel (belongs_to (aps_commercial_product_lots :: CommercialProductLot , foreign_key = id))]
# [diesel (belongs_to (aps_weighing_device_models :: WeighingDeviceModel , foreign_key = id))]
# [table_model (foreign_key ((commercial_weighing_device_model_id ,) , (:: aps_commercial_weighing_device_models :: commercial_weighing_device_models :: id)))]
# [table_model (foreign_key ((id ,) , (:: aps_commercial_product_lots :: commercial_product_lots :: id)))]
# [table_model (foreign_key ((id ,) , (:: aps_weighing_device_models :: weighing_device_models :: id)))]
#[table_model(default(aps_entities::entities::table_name_id, "commercial_weighing_device_lots"))]
# [diesel (table_name = commercial_weighing_device_lots)]
pub struct CommercialWeighingDeviceLot {
    /// Field representing the `id` column in table
    /// `commercial_weighing_device_lots`.
    #[same_as(aps_commercial_product_lots::commercial_product_lots::id)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `commercial_weighing_device_model_id` column in
    /// table `commercial_weighing_device_lots`.
    #[same_as(aps_commercial_product_lots::commercial_product_lots::product_model_id)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    commercial_weighing_device_model_id: ::rosetta_uuid::Uuid,
}
impl ::diesel_builders::GetColumn<aps_asset_models::asset_models::id>
    for CommercialWeighingDeviceLot
{
    fn get_column_ref(
        &self,
    ) -> &<commercial_weighing_device_lots::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_commercial_product_lots::commercial_product_lots::id>
    for CommercialWeighingDeviceLot
{
    fn get_column_ref(
        &self,
    ) -> &<commercial_weighing_device_lots::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_entities::entities::id> for CommercialWeighingDeviceLot {
    fn get_column_ref(
        &self,
    ) -> &<commercial_weighing_device_lots::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_namespaced_ownables::namespaced_ownables::id>
    for CommercialWeighingDeviceLot
{
    fn get_column_ref(
        &self,
    ) -> &<commercial_weighing_device_lots::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_ownables::ownables::id> for CommercialWeighingDeviceLot {
    fn get_column_ref(
        &self,
    ) -> &<commercial_weighing_device_lots::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_physical_asset_models::physical_asset_models::id>
    for CommercialWeighingDeviceLot
{
    fn get_column_ref(
        &self,
    ) -> &<commercial_weighing_device_lots::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_weighing_device_models::weighing_device_models::id>
    for CommercialWeighingDeviceLot
{
    fn get_column_ref(
        &self,
    ) -> &<commercial_weighing_device_lots::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
