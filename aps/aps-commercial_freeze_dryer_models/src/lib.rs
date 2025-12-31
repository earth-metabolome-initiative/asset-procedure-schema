//! Auto-generated crate for the `commercial_freeze_dryer_models` table.
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
    aps_commercial_products::commercial_products,
    aps_freeze_dryer_models::freeze_dryer_models
))]
# [diesel (table_name = commercial_freeze_dryer_models)]
pub struct CommercialFreezeDryerModel {
    /// Undocumented column
    #[same_as(aps_asset_models::asset_models::id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Undocumented column
    #[same_as(aps_asset_models::asset_models::parent_model_id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    freeze_dryer_model_id: ::rosetta_uuid::Uuid,
}
:: diesel_builders :: prelude :: fk ! ((commercial_freeze_dryer_models :: freeze_dryer_model_id) -> (:: aps_freeze_dryer_models :: freeze_dryer_models :: id));
:: diesel_builders :: prelude :: fk ! ((commercial_freeze_dryer_models :: id) -> (:: aps_freeze_dryer_models :: freeze_dryer_models :: id));
:: diesel_builders :: prelude :: fk ! ((commercial_freeze_dryer_models :: id) -> (:: aps_commercial_products :: commercial_products :: id));
