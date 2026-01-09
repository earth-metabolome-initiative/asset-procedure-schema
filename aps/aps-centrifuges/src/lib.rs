//! Auto-generated crate for the `centrifuges` table.
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
/// Struct representing a row in the `centrifuges` table.
#[table_model(ancestors(aps_assets::assets, aps_physical_assets::physical_assets))]
#[table_model(default(aps_assets::assets::asset_table_id, "centrifuges"))]
# [diesel (table_name = centrifuges)]
pub struct Centrifuge {
    /// Field representing the `id` column in table `centrifuges`.
    #[same_as(aps_physical_assets::physical_assets::id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `commercial_centrifuge_lot_id` column in table
    /// `centrifuges`.
    #[same_as(aps_physical_assets::physical_assets::physical_asset_model_id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    commercial_centrifuge_lot_id: ::rosetta_uuid::Uuid,
}
:: diesel_builders :: prelude :: fk ! ((centrifuges :: id) -> (:: aps_physical_assets :: physical_assets :: id));
:: diesel_builders :: prelude :: fk ! ((centrifuges :: commercial_centrifuge_lot_id) -> (:: aps_commercial_centrifuge_lots :: commercial_centrifuge_lots :: id));
