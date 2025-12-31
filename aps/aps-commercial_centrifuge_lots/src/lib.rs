//! Auto-generated crate for the `commercial_centrifuge_lots` table.
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
    aps_centrifuge_models::centrifuge_models,
    aps_commercial_product_lots::commercial_product_lots
))]
# [diesel (table_name = commercial_centrifuge_lots)]
pub struct CommercialCentrifugeLot {
    /// Undocumented column
    #[same_as(aps_commercial_product_lots::commercial_product_lots::id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Undocumented column
    #[same_as(aps_commercial_product_lots::commercial_product_lots::product_model_id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    commercial_centrifuge_model_id: ::rosetta_uuid::Uuid,
}
:: diesel_builders :: prelude :: fk ! ((commercial_centrifuge_lots :: commercial_centrifuge_model_id) -> (:: aps_commercial_centrifuge_models :: commercial_centrifuge_models :: id));
:: diesel_builders :: prelude :: fk ! ((commercial_centrifuge_lots :: id) -> (:: aps_commercial_product_lots :: commercial_product_lots :: id));
:: diesel_builders :: prelude :: fk ! ((commercial_centrifuge_lots :: id) -> (:: aps_centrifuge_models :: centrifuge_models :: id));
