//! Auto-generated crate for the `digital_assets` table.
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
/// Struct representing a row in the `digital_assets` table.
#[table_model(ancestors(aps_assets::assets))]
#[table_model(default(aps_assets::assets::asset_table_id, "digital_assets"))]
# [diesel (table_name = digital_assets)]
pub struct DigitalAsset {
    /// Field representing the `id` column in table `digital_assets`.
    #[same_as(aps_assets::assets::id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `digital_asset_model_id` column in table
    /// `digital_assets`.
    #[same_as(aps_assets::assets::model_id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    digital_asset_model_id: ::rosetta_uuid::Uuid,
}
:: diesel_builders :: prelude :: fk ! ((digital_assets :: id) -> (:: aps_assets :: assets :: id));
:: diesel_builders :: prelude :: fk ! ((digital_assets :: digital_asset_model_id) -> (:: aps_digital_asset_models :: digital_asset_models :: id));
impl diesel_builders::GetColumn<aps_assets::assets::id> for DigitalAsset {
    fn get_column_ref(&self) -> &<digital_assets::id as diesel_builders::Typed>::ColumnType {
        &self.id
    }
}
