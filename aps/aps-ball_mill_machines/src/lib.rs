//! Auto-generated crate for the `ball_mill_machines` table.
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
/// Struct representing a row in the `ball_mill_machines` table.
#[table_model(ancestors(aps_assets::assets, aps_physical_assets::physical_assets))]
#[table_model(default(aps_assets::assets::asset_table_id, "ball_mill_machines"))]
# [diesel (table_name = ball_mill_machines)]
pub struct BallMillMachine {
    /// Field representing the `id` column in table `ball_mill_machines`.
    #[same_as(aps_physical_assets::physical_assets::id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `commercial_ball_mill_machine_lot_id` column in
    /// table `ball_mill_machines`.
    #[same_as(aps_physical_assets::physical_assets::physical_asset_model_id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    commercial_ball_mill_machine_lot_id: ::rosetta_uuid::Uuid,
}
:: diesel_builders :: prelude :: fk ! ((ball_mill_machines :: id) -> (:: aps_physical_assets :: physical_assets :: id));
:: diesel_builders :: prelude :: fk ! ((ball_mill_machines :: commercial_ball_mill_machine_lot_id) -> (:: aps_commercial_ball_mill_machine_lots :: commercial_ball_mill_machine_lots :: id));
impl diesel_builders::GetColumn<aps_assets::assets::id> for BallMillMachine {
    fn get_column_ref(&self) -> &<ball_mill_machines::id as diesel_builders::Typed>::ColumnType {
        &self.id
    }
}
impl diesel_builders::GetColumn<aps_physical_assets::physical_assets::id> for BallMillMachine {
    fn get_column_ref(&self) -> &<ball_mill_machines::id as diesel_builders::Typed>::ColumnType {
        &self.id
    }
}
