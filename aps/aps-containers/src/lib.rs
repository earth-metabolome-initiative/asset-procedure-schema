//! Auto-generated crate for the `containers` table.
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
/// Struct representing a row in the `containers` table.
#[table_model(ancestors(aps_assets::assets, aps_physical_assets::physical_assets))]
#[table_model(default(aps_assets::assets::asset_table_id, "containers"))]
# [diesel (table_name = containers)]
pub struct Container {
    /// Field representing the `id` column in table `containers`.
    #[same_as(aps_physical_assets::physical_assets::id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `container_model_id` column in table
    /// `containers`.
    #[same_as(aps_physical_assets::physical_assets::physical_asset_model_id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    container_model_id: ::rosetta_uuid::Uuid,
}
:: diesel_builders :: prelude :: fk ! ((containers :: id) -> (:: aps_physical_assets :: physical_assets :: id));
:: diesel_builders :: prelude :: fk ! ((containers :: container_model_id) -> (:: aps_container_models :: container_models :: id));
impl diesel_builders::GetColumn<aps_assets::assets::id> for Container {
    fn get_column_ref(&self) -> &<containers::id as diesel_builders::Typed>::ColumnType {
        &self.id
    }
}
impl diesel_builders::GetColumn<aps_physical_assets::physical_assets::id> for Container {
    fn get_column_ref(&self) -> &<containers::id as diesel_builders::Typed>::ColumnType {
        &self.id
    }
}
