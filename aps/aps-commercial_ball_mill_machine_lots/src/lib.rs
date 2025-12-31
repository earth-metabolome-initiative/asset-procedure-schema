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
    serde :: Serialize,
    serde :: Deserialize,
    diesel :: Queryable,
    diesel :: Selectable,
    diesel :: Identifiable,
    diesel_builders :: prelude :: TableModel,
)]
/// Undocumented table
#[table_model(ancestors(
    aps_asset_models::asset_models,
    aps_physical_asset_models::physical_asset_models,
    aps_ball_mill_machine_models::ball_mill_machine_models,
    aps_commercial_product_lots::commercial_product_lots
))]
# [diesel (table_name = commercial_ball_mill_machine_lots)]
pub struct CommercialBallMillMachineLot {
    /// Undocumented column
    #[same_as(aps_commercial_product_lots::commercial_product_lots::id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Undocumented column
    #[same_as(aps_commercial_product_lots::commercial_product_lots::product_model_id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    commercial_ball_mill_machine_model_id: ::rosetta_uuid::Uuid,
}
:: diesel_builders :: prelude :: fk ! ((commercial_ball_mill_machine_lots :: commercial_ball_mill_machine_model_id) -> (:: aps_commercial_ball_mill_machine_models :: commercial_ball_mill_machine_models :: id));
:: diesel_builders :: prelude :: fk ! ((commercial_ball_mill_machine_lots :: id) -> (:: aps_commercial_product_lots :: commercial_product_lots :: id));
:: diesel_builders :: prelude :: fk ! ((commercial_ball_mill_machine_lots :: id) -> (:: aps_ball_mill_machine_models :: ball_mill_machine_models :: id));
