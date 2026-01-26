//! Auto-generated crate for the `ball_mill_machine_models` table.
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
/// Struct representing a row in the `ball_mill_machine_models` table.
#[table_model(ancestors(
    aps_asset_models::asset_models,
    aps_physical_asset_models::physical_asset_models
))]
# [diesel (belongs_to (aps_physical_asset_models :: PhysicalAssetModel , foreign_key = id))]
# [table_model (foreign_key ((id ,) , (:: aps_physical_asset_models :: physical_asset_models :: id)))]
#[table_model(default(
    aps_asset_models::asset_models::asset_model_table_id,
    "ball_mill_machine_models"
))]
# [diesel (table_name = ball_mill_machine_models)]
pub struct BallMillMachineModel {
    /// Field representing the `id` column in table `ball_mill_machine_models`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
}
impl ::diesel_builders::GetColumn<aps_asset_models::asset_models::id> for BallMillMachineModel {
    fn get_column_ref(
        &self,
    ) -> &<ball_mill_machine_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_physical_asset_models::physical_asset_models::id>
    for BallMillMachineModel
{
    fn get_column_ref(
        &self,
    ) -> &<ball_mill_machine_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
::diesel::allow_tables_to_appear_in_same_query!(
    ball_mill_machine_models,
    ::aps_commercial_product_lots::commercial_product_lots
);
::diesel::allow_tables_to_appear_in_same_query!(
    ball_mill_machine_models,
    ::aps_commercial_products::commercial_products
);
