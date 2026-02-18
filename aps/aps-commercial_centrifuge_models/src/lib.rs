//! Auto-generated crate for the `commercial_centrifuge_models` table.
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
/// Catalog of commercial centrifuge models.
#[table_model(ancestors(
    aps_entities::entities,
    aps_ownables::ownables,
    aps_namespaced_ownables::namespaced_ownables,
    aps_asset_models::asset_models,
    aps_commercial_products::commercial_products,
    aps_physical_asset_models::physical_asset_models,
    aps_centrifuge_models::centrifuge_models
))]
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (belongs_to (aps_commercial_products :: CommercialProduct , foreign_key = id))]
# [table_model (foreign_key ((centrifuge_model_id ,) , (:: aps_centrifuge_models :: centrifuge_models :: id)))]
# [table_model (foreign_key ((id ,) , (:: aps_centrifuge_models :: centrifuge_models :: id)))]
# [table_model (foreign_key ((id ,) , (:: aps_commercial_products :: commercial_products :: id)))]
#[table_model(default(aps_entities::entities::table_name_id, "commercial_centrifuge_models"))]
# [diesel (table_name = commercial_centrifuge_models)]
pub struct CommercialCentrifugeModel {
    /// Stable commercial model identifier shared with parent model tables.
    #[same_as(aps_asset_models::asset_models::id)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Base centrifuge model represented by this commercial model.
    #[same_as(aps_asset_models::asset_models::parent_model_id)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    centrifuge_model_id: ::rosetta_uuid::Uuid,
}
impl ::diesel_builders::GetColumn<aps_asset_models::asset_models::id>
    for CommercialCentrifugeModel
{
    fn get_column_ref(
        &self,
    ) -> &<commercial_centrifuge_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_centrifuge_models::centrifuge_models::id>
    for CommercialCentrifugeModel
{
    fn get_column_ref(
        &self,
    ) -> &<commercial_centrifuge_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_commercial_products::commercial_products::id>
    for CommercialCentrifugeModel
{
    fn get_column_ref(
        &self,
    ) -> &<commercial_centrifuge_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_entities::entities::id> for CommercialCentrifugeModel {
    fn get_column_ref(
        &self,
    ) -> &<commercial_centrifuge_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_namespaced_ownables::namespaced_ownables::id>
    for CommercialCentrifugeModel
{
    fn get_column_ref(
        &self,
    ) -> &<commercial_centrifuge_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_ownables::ownables::id> for CommercialCentrifugeModel {
    fn get_column_ref(
        &self,
    ) -> &<commercial_centrifuge_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_physical_asset_models::physical_asset_models::id>
    for CommercialCentrifugeModel
{
    fn get_column_ref(
        &self,
    ) -> &<commercial_centrifuge_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
