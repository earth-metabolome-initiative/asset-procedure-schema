//! Auto-generated crate for the `commercial_freezer_lots` table.
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
/// Struct representing a row in the `commercial_freezer_lots` table.
#[table_model(ancestors(
    aps_asset_models::asset_models,
    aps_physical_asset_models::physical_asset_models,
    aps_freezer_models::freezer_models,
    aps_commercial_product_lots::commercial_product_lots
))]
#[table_model(default(
    aps_asset_models::asset_models::asset_model_table_id,
    "commercial_freezer_lots"
))]
# [diesel (table_name = commercial_freezer_lots)]
pub struct CommercialFreezerLot {
    /// Field representing the `id` column in table `commercial_freezer_lots`.
    #[same_as(aps_commercial_product_lots::commercial_product_lots::id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `commercial_freezer_model_id` column in table
    /// `commercial_freezer_lots`.
    #[same_as(aps_commercial_product_lots::commercial_product_lots::product_model_id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    commercial_freezer_model_id: ::rosetta_uuid::Uuid,
}
:: diesel_builders :: prelude :: fk ! ((commercial_freezer_lots :: commercial_freezer_model_id) -> (:: aps_commercial_freezer_models :: commercial_freezer_models :: id));
:: diesel_builders :: prelude :: fk ! ((commercial_freezer_lots :: id) -> (:: aps_commercial_product_lots :: commercial_product_lots :: id));
:: diesel_builders :: prelude :: fk ! ((commercial_freezer_lots :: id) -> (:: aps_freezer_models :: freezer_models :: id));
