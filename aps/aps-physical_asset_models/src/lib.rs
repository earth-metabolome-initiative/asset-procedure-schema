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
#[table_model(ancestors(
    aps_entities::entities,
    aps_ownables::ownables,
    aps_namespaced_ownables::namespaced_ownables,
    aps_asset_models::asset_models
))]
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (belongs_to (aps_asset_models :: AssetModel , foreign_key = id))]
# [table_model (foreign_key ((id ,) , (:: aps_asset_models :: asset_models :: id)))]
#[table_model(default(aps_entities::entities::table_name_id, "physical_asset_models"))]
# [diesel (table_name = physical_asset_models)]
pub struct PhysicalAssetModel {
    /// Field representing the `id` column in table `physical_asset_models`.
    #[infallible]
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
impl ::diesel_builders::GetColumn<aps_entities::entities::id> for PhysicalAssetModel {
    fn get_column_ref(
        &self,
    ) -> &<physical_asset_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_namespaced_ownables::namespaced_ownables::id>
    for PhysicalAssetModel
{
    fn get_column_ref(
        &self,
    ) -> &<physical_asset_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_ownables::ownables::id> for PhysicalAssetModel {
    fn get_column_ref(
        &self,
    ) -> &<physical_asset_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
