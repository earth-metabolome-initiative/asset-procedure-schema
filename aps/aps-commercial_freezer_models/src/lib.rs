//! Auto-generated crate for the `commercial_freezer_models` table.
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
/// Struct representing a row in the `commercial_freezer_models` table.
#[table_model(ancestors(
    aps_entities::entities,
    aps_ownables::ownables,
    aps_namespaced_ownables::namespaced_ownables,
    aps_asset_models::asset_models,
    aps_commercial_products::commercial_products,
    aps_physical_asset_models::physical_asset_models,
    aps_freezer_models::freezer_models
))]
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (belongs_to (aps_commercial_products :: CommercialProduct , foreign_key = id))]
# [table_model (foreign_key ((freezer_model_id ,) , (:: aps_freezer_models :: freezer_models :: id)))]
# [table_model (foreign_key ((id ,) , (:: aps_freezer_models :: freezer_models :: id)))]
# [table_model (foreign_key ((id ,) , (:: aps_commercial_products :: commercial_products :: id)))]
#[table_model(default(aps_entities::entities::table_name_id, "commercial_freezer_models"))]
# [diesel (table_name = commercial_freezer_models)]
pub struct CommercialFreezerModel {
    /// Field representing the `id` column in table `commercial_freezer_models`.
    #[same_as(aps_asset_models::asset_models::id)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `freezer_model_id` column in table
    /// `commercial_freezer_models`.
    #[same_as(aps_asset_models::asset_models::parent_model_id)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    freezer_model_id: ::rosetta_uuid::Uuid,
}
impl ::diesel_builders::GetColumn<aps_asset_models::asset_models::id> for CommercialFreezerModel {
    fn get_column_ref(
        &self,
    ) -> &<commercial_freezer_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_commercial_products::commercial_products::id>
    for CommercialFreezerModel
{
    fn get_column_ref(
        &self,
    ) -> &<commercial_freezer_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_entities::entities::id> for CommercialFreezerModel {
    fn get_column_ref(
        &self,
    ) -> &<commercial_freezer_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_freezer_models::freezer_models::id>
    for CommercialFreezerModel
{
    fn get_column_ref(
        &self,
    ) -> &<commercial_freezer_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_namespaced_ownables::namespaced_ownables::id>
    for CommercialFreezerModel
{
    fn get_column_ref(
        &self,
    ) -> &<commercial_freezer_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_ownables::ownables::id> for CommercialFreezerModel {
    fn get_column_ref(
        &self,
    ) -> &<commercial_freezer_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_physical_asset_models::physical_asset_models::id>
    for CommercialFreezerModel
{
    fn get_column_ref(
        &self,
    ) -> &<commercial_freezer_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
