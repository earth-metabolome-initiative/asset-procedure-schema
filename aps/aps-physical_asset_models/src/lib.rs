//! Auto-generated crate for the `physical_asset_models` table.
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
/// Struct representing a row in the `physical_asset_models` table.
#[table_model(ancestors(aps_asset_models::asset_models))]
# [diesel (belongs_to (aps_asset_models :: AssetModel , foreign_key = id))]
# [table_model (foreign_key ((id ,) , (:: aps_asset_models :: asset_models :: id)))]
#[table_model(default(
    aps_asset_models::asset_models::asset_model_table_id,
    "physical_asset_models"
))]
# [diesel (table_name = physical_asset_models)]
pub struct PhysicalAssetModel {
    /// Field representing the `id` column in table `physical_asset_models`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
}
impl ::diesel_builders::GetColumn<aps_asset_models::asset_models::id> for PhysicalAssetModel {
    fn get_column_ref(
        &self,
    ) -> &<physical_asset_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
