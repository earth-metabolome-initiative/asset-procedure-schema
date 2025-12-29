//! Auto-generated crate for the `digital_asset_models` table.
#[derive(
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
#[table_model(ancestors(aps_asset_models::asset_models))]
# [diesel (table_name = digital_asset_models)]
pub struct DigitalAssetModel {
    /// Undocumented column
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Undocumented column
    mime_type: String,
}
:: diesel_builders :: prelude :: fk ! ((digital_asset_models :: id) -> (:: aps_asset_models :: asset_models :: id));
