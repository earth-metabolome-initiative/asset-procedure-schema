//! Auto-generated crate for the `asset_model_tables` table.
#[derive(
    Clone,
    Default,
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
/// Struct representing a row in the `asset_model_tables` table.
# [diesel (table_name = asset_model_tables)]
pub struct AssetModelTable {
    /// Field representing the `id` column in table `asset_model_tables`.
    id: String,
}
