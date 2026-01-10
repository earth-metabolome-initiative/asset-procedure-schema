//! Auto-generated crate for the `freezer_models` table.
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
/// Struct representing a row in the `freezer_models` table.
#[table_model(ancestors(
    aps_asset_models::asset_models,
    aps_physical_asset_models::physical_asset_models
))]
#[table_model(default(aps_asset_models::asset_models::asset_model_table_id, "freezer_models"))]
# [diesel (table_name = freezer_models)]
pub struct FreezerModel {
    /// Field representing the `id` column in table `freezer_models`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
}
:: diesel_builders :: prelude :: fk ! ((freezer_models :: id) -> (:: aps_physical_asset_models :: physical_asset_models :: id));
impl diesel_builders::GetColumn<aps_asset_models::asset_models::id> for FreezerModel {
    fn get_column_ref(&self) -> &<freezer_models::id as diesel_builders::Typed>::ColumnType {
        &self.id
    }
}
impl diesel_builders::GetColumn<aps_physical_asset_models::physical_asset_models::id>
    for FreezerModel
{
    fn get_column_ref(&self) -> &<freezer_models::id as diesel_builders::Typed>::ColumnType {
        &self.id
    }
}
