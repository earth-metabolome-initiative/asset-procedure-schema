//! Auto-generated crate for the `digital_assets` table.
#[derive(
    Copy,
    Clone,
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
#[table_model(ancestors(aps_assets::assets))]
# [diesel (table_name = digital_assets)]
pub struct DigitalAsset {
    /// Undocumented column
    #[same_as(aps_assets::assets::id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Undocumented column
    #[same_as(aps_assets::assets::model_id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    digital_asset_model_id: ::rosetta_uuid::Uuid,
}
:: diesel_builders :: prelude :: fk ! ((digital_assets :: id) -> (:: aps_assets :: assets :: id));
:: diesel_builders :: prelude :: fk ! ((digital_assets :: digital_asset_model_id) -> (:: aps_digital_asset_models :: digital_asset_models :: id));
