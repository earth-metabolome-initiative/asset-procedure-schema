//! Auto-generated crate for the `freezers` table.
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
/// Struct representing a row in the `freezers` table.
#[table_model(ancestors(aps_assets::assets, aps_physical_assets::physical_assets))]
#[table_model(default(aps_assets::assets::asset_table_id, "freezers"))]
# [diesel (table_name = freezers)]
pub struct Freezer {
    /// Field representing the `id` column in table `freezers`.
    #[same_as(aps_physical_assets::physical_assets::id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `commercial_freezer_lot_id` column in table
    /// `freezers`.
    #[same_as(aps_physical_assets::physical_assets::physical_asset_model_id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    commercial_freezer_lot_id: ::rosetta_uuid::Uuid,
}
:: diesel_builders :: prelude :: fk ! ((freezers :: id) -> (:: aps_physical_assets :: physical_assets :: id));
:: diesel_builders :: prelude :: fk ! ((freezers :: commercial_freezer_lot_id) -> (:: aps_commercial_freezer_lots :: commercial_freezer_lots :: id));
impl diesel_builders::GetColumn<aps_assets::assets::id> for Freezer {
    fn get_column_ref(&self) -> &<freezers::id as diesel_builders::Typed>::ColumnType {
        &self.id
    }
}
impl diesel_builders::GetColumn<aps_physical_assets::physical_assets::id> for Freezer {
    fn get_column_ref(&self) -> &<freezers::id as diesel_builders::Typed>::ColumnType {
        &self.id
    }
}
