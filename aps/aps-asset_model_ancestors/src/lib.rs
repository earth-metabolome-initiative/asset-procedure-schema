//! Auto-generated crate for the `asset_model_ancestors` table.
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
#[diesel(primary_key(descendant_model_id, ancestor_model_id))]
# [diesel (table_name = asset_model_ancestors)]
pub struct AssetModelAncestor {
    /// Undocumented column
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    descendant_model_id: ::rosetta_uuid::Uuid,
    /// Undocumented column
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    ancestor_model_id: ::rosetta_uuid::Uuid,
}
:: diesel_builders :: prelude :: fk ! ((asset_model_ancestors :: descendant_model_id) -> (:: aps_asset_models :: asset_models :: id));
:: diesel_builders :: prelude :: fk ! ((asset_model_ancestors :: ancestor_model_id) -> (:: aps_asset_models :: asset_models :: id));
