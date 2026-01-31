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
    :: serde :: Serialize,
    :: serde :: Deserialize,
    :: diesel :: Queryable,
    :: diesel :: Selectable,
    :: diesel :: Identifiable,
    :: diesel :: Associations,
    :: diesel_builders :: prelude :: TableModel,
)]
/// Struct representing a row in the `freezers` table.
#[table_model(ancestors(
    aps_ownables::ownables,
    aps_assets::assets,
    aps_physical_assets::physical_assets
))]
# [diesel (belongs_to (aps_physical_assets :: PhysicalAsset , foreign_key = id))]
# [diesel (belongs_to (aps_commercial_freezer_lots :: CommercialFreezerLot , foreign_key = commercial_freezer_lot_id))]
# [table_model (foreign_key ((id ,) , (:: aps_physical_assets :: physical_assets :: id)))]
# [table_model (foreign_key ((commercial_freezer_lot_id ,) , (:: aps_commercial_freezer_lots :: commercial_freezer_lots :: id)))]
#[table_model(default(aps_ownables::ownables::ownable_table_id, "freezers"))]
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
impl ::diesel_builders::GetColumn<aps_assets::assets::id> for Freezer {
    fn get_column_ref(&self) -> &<freezers::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_ownables::ownables::id> for Freezer {
    fn get_column_ref(&self) -> &<freezers::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_physical_assets::physical_assets::id> for Freezer {
    fn get_column_ref(&self) -> &<freezers::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
