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
    serde :: Serialize,
    serde :: Deserialize,
    diesel :: Queryable,
    diesel :: Selectable,
    diesel :: Identifiable,
    diesel_builders :: prelude :: TableModel,
)]
/// Struct representing a row in the `commercial_freezer_models` table.
#[table_model(ancestors(
    aps_asset_models::asset_models,
    aps_physical_asset_models::physical_asset_models,
    aps_commercial_products::commercial_products,
    aps_freezer_models::freezer_models
))]
#[table_model(default(
    aps_asset_models::asset_models::asset_model_table_id,
    "commercial_freezer_models"
))]
# [diesel (table_name = commercial_freezer_models)]
pub struct CommercialFreezerModel {
    /// Field representing the `id` column in table `commercial_freezer_models`.
    #[same_as(aps_asset_models::asset_models::id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `freezer_model_id` column in table
    /// `commercial_freezer_models`.
    #[same_as(aps_asset_models::asset_models::parent_model_id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    freezer_model_id: ::rosetta_uuid::Uuid,
}
:: diesel_builders :: prelude :: fk ! ((commercial_freezer_models :: freezer_model_id) -> (:: aps_freezer_models :: freezer_models :: id));
:: diesel_builders :: prelude :: fk ! ((commercial_freezer_models :: id) -> (:: aps_freezer_models :: freezer_models :: id));
:: diesel_builders :: prelude :: fk ! ((commercial_freezer_models :: id) -> (:: aps_commercial_products :: commercial_products :: id));
impl diesel_builders::GetColumn<aps_asset_models::asset_models::id> for CommercialFreezerModel {
    fn get_column_ref(
        &self,
    ) -> &<commercial_freezer_models::id as diesel_builders::Typed>::ColumnType {
        &self.id
    }
}
impl diesel_builders::GetColumn<aps_commercial_products::commercial_products::id>
    for CommercialFreezerModel
{
    fn get_column_ref(
        &self,
    ) -> &<commercial_freezer_models::id as diesel_builders::Typed>::ColumnType {
        &self.id
    }
}
impl diesel_builders::GetColumn<aps_freezer_models::freezer_models::id> for CommercialFreezerModel {
    fn get_column_ref(
        &self,
    ) -> &<commercial_freezer_models::id as diesel_builders::Typed>::ColumnType {
        &self.id
    }
}
impl diesel_builders::GetColumn<aps_physical_asset_models::physical_asset_models::id>
    for CommercialFreezerModel
{
    fn get_column_ref(
        &self,
    ) -> &<commercial_freezer_models::id as diesel_builders::Typed>::ColumnType {
        &self.id
    }
}
