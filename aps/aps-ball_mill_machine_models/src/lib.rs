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
    serde :: Serialize,
    serde :: Deserialize,
    diesel :: Queryable,
    diesel :: Selectable,
    diesel :: Identifiable,
    diesel_builders :: prelude :: TableModel,
)]
/// Struct representing a row in the `ball_mill_machine_models` table.
#[table_model(ancestors(
    aps_asset_models::asset_models,
    aps_physical_asset_models::physical_asset_models
))]
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
:: diesel_builders :: prelude :: fk ! ((ball_mill_machine_models :: id) -> (:: aps_physical_asset_models :: physical_asset_models :: id));
