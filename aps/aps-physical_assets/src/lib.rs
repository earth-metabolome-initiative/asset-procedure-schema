//! Auto-generated crate for the `physical_assets` table.
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
/// Struct representing a row in the `physical_assets` table.
#[table_model(ancestors(aps_entities::entities, aps_ownables::ownables, aps_assets::assets))]
# [diesel (belongs_to (aps_physical_asset_models :: PhysicalAssetModel , foreign_key = physical_asset_model_id))]
# [table_model (foreign_key ((id ,) , (:: aps_assets :: assets :: id)))]
# [table_model (foreign_key ((physical_asset_model_id ,) , (:: aps_physical_asset_models :: physical_asset_models :: id)))]
#[table_model(default(aps_entities::entities::table_name_id, "physical_assets"))]
# [diesel (table_name = physical_assets)]
pub struct PhysicalAsset {
    /// Field representing the `id` column in table `physical_assets`.
    #[same_as(aps_assets::assets::id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `physical_asset_model_id` column in table
    /// `physical_assets`.
    #[same_as(aps_assets::assets::model_id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    physical_asset_model_id: ::rosetta_uuid::Uuid,
}
impl ::diesel_builders::GetColumn<aps_assets::assets::id> for PhysicalAsset {
    fn get_column_ref(
        &self,
    ) -> &<physical_assets::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_entities::entities::id> for PhysicalAsset {
    fn get_column_ref(
        &self,
    ) -> &<physical_assets::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_ownables::ownables::id> for PhysicalAsset {
    fn get_column_ref(
        &self,
    ) -> &<physical_assets::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
