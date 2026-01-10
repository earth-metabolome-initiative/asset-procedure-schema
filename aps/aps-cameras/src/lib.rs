//! Auto-generated crate for the `cameras` table.
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
/// Struct representing a row in the `cameras` table.
#[table_model(ancestors(aps_assets::assets, aps_physical_assets::physical_assets))]
#[table_model(default(aps_assets::assets::asset_table_id, "cameras"))]
# [diesel (table_name = cameras)]
pub struct Camera {
    /// Field representing the `id` column in table `cameras`.
    #[same_as(aps_physical_assets::physical_assets::id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `commercial_camera_lot_id` column in table
    /// `cameras`.
    #[same_as(aps_physical_assets::physical_assets::physical_asset_model_id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    commercial_camera_lot_id: ::rosetta_uuid::Uuid,
}
:: diesel_builders :: prelude :: fk ! ((cameras :: id) -> (:: aps_physical_assets :: physical_assets :: id));
:: diesel_builders :: prelude :: fk ! ((cameras :: commercial_camera_lot_id) -> (:: aps_commercial_camera_lots :: commercial_camera_lots :: id));
impl diesel_builders::GetColumn<aps_assets::assets::id> for Camera {
    fn get_column_ref(&self) -> &<cameras::id as diesel_builders::Typed>::ColumnType {
        &self.id
    }
}
impl diesel_builders::GetColumn<aps_physical_assets::physical_assets::id> for Camera {
    fn get_column_ref(&self) -> &<cameras::id as diesel_builders::Typed>::ColumnType {
        &self.id
    }
}
