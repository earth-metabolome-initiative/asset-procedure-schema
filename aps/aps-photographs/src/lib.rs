//! Auto-generated crate for the `photographs` table.
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
/// Struct representing a row in the `photographs` table.
#[table_model(ancestors(aps_assets::assets, aps_digital_assets::digital_assets))]
#[table_model(default(aps_assets::assets::asset_table_id, "photographs"))]
# [diesel (table_name = photographs)]
pub struct Photograph {
    /// Field representing the `id` column in table `photographs`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
}
:: diesel_builders :: prelude :: fk ! ((photographs :: id) -> (:: aps_digital_assets :: digital_assets :: id));
