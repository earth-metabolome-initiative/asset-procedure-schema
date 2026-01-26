//! Auto-generated crate for the `freeze_dryer_models` table.
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
/// Struct representing a row in the `freeze_dryer_models` table.
#[table_model(ancestors(
    aps_asset_models::asset_models,
    aps_physical_asset_models::physical_asset_models
))]
# [diesel (belongs_to (aps_physical_asset_models :: PhysicalAssetModel , foreign_key = id))]
# [table_model (foreign_key ((id ,) , (:: aps_physical_asset_models :: physical_asset_models :: id)))]
#[table_model(default(
    aps_asset_models::asset_models::asset_model_table_id,
    "freeze_dryer_models"
))]
# [diesel (table_name = freeze_dryer_models)]
pub struct FreezeDryerModel {
    /// Field representing the `id` column in table `freeze_dryer_models`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
}
impl ::diesel_builders::GetColumn<aps_asset_models::asset_models::id> for FreezeDryerModel {
    fn get_column_ref(
        &self,
    ) -> &<freeze_dryer_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_physical_asset_models::physical_asset_models::id>
    for FreezeDryerModel
{
    fn get_column_ref(
        &self,
    ) -> &<freeze_dryer_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
