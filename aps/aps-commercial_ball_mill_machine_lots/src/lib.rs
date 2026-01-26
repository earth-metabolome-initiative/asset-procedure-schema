//! Auto-generated crate for the `commercial_ball_mill_machine_lots` table.
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
/// Struct representing a row in the `commercial_ball_mill_machine_lots` table.
#[table_model(ancestors(
    aps_asset_models::asset_models,
    aps_physical_asset_models::physical_asset_models,
    aps_ball_mill_machine_models::ball_mill_machine_models,
    aps_commercial_product_lots::commercial_product_lots
))]
# [diesel (belongs_to (aps_commercial_ball_mill_machine_models :: CommercialBallMillMachineModel , foreign_key = commercial_ball_mill_machine_model_id))]
# [diesel (belongs_to (aps_commercial_product_lots :: CommercialProductLot , foreign_key = id))]
# [diesel (belongs_to (aps_ball_mill_machine_models :: BallMillMachineModel , foreign_key = id))]
# [table_model (foreign_key ((commercial_ball_mill_machine_model_id ,) , (:: aps_commercial_ball_mill_machine_models :: commercial_ball_mill_machine_models :: id)))]
# [table_model (foreign_key ((id ,) , (:: aps_commercial_product_lots :: commercial_product_lots :: id)))]
# [table_model (foreign_key ((id ,) , (:: aps_ball_mill_machine_models :: ball_mill_machine_models :: id)))]
#[table_model(default(
    aps_asset_models::asset_models::asset_model_table_id,
    "commercial_ball_mill_machine_lots"
))]
# [diesel (table_name = commercial_ball_mill_machine_lots)]
pub struct CommercialBallMillMachineLot {
    /// Field representing the `id` column in table
    /// `commercial_ball_mill_machine_lots`.
    #[same_as(aps_commercial_product_lots::commercial_product_lots::id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `commercial_ball_mill_machine_model_id` column in
    /// table `commercial_ball_mill_machine_lots`.
    #[same_as(aps_commercial_product_lots::commercial_product_lots::product_model_id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    commercial_ball_mill_machine_model_id: ::rosetta_uuid::Uuid,
}
impl ::diesel_builders::GetColumn<aps_asset_models::asset_models::id>
    for CommercialBallMillMachineLot
{
    fn get_column_ref(
        &self,
    ) -> &<commercial_ball_mill_machine_lots::id as ::diesel_builders::ColumnTyped>::ColumnType
    {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_ball_mill_machine_models::ball_mill_machine_models::id>
    for CommercialBallMillMachineLot
{
    fn get_column_ref(
        &self,
    ) -> &<commercial_ball_mill_machine_lots::id as ::diesel_builders::ColumnTyped>::ColumnType
    {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_commercial_product_lots::commercial_product_lots::id>
    for CommercialBallMillMachineLot
{
    fn get_column_ref(
        &self,
    ) -> &<commercial_ball_mill_machine_lots::id as ::diesel_builders::ColumnTyped>::ColumnType
    {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_physical_asset_models::physical_asset_models::id>
    for CommercialBallMillMachineLot
{
    fn get_column_ref(
        &self,
    ) -> &<commercial_ball_mill_machine_lots::id as ::diesel_builders::ColumnTyped>::ColumnType
    {
        &self.id
    }
}
