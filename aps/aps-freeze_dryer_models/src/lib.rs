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
/// Catalog of freeze dryer model definitions.
#[table_model(ancestors(
    aps_entities::entities,
    aps_ownables::ownables,
    aps_namespaced_ownables::namespaced_ownables,
    aps_asset_models::asset_models,
    aps_physical_asset_models::physical_asset_models
))]
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (belongs_to (aps_physical_asset_models :: PhysicalAssetModel , foreign_key = id))]
# [table_model (foreign_key ((id ,) , (:: aps_physical_asset_models :: physical_asset_models :: id)))]
#[table_model(default(aps_entities::entities::table_name_id, "freeze_dryer_models"))]
# [diesel (table_name = freeze_dryer_models)]
pub struct FreezeDryerModel {
    /// Stable model identifier inherited from physical_asset_models.
    #[infallible]
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
impl ::diesel_builders::GetColumn<aps_entities::entities::id> for FreezeDryerModel {
    fn get_column_ref(
        &self,
    ) -> &<freeze_dryer_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_namespaced_ownables::namespaced_ownables::id>
    for FreezeDryerModel
{
    fn get_column_ref(
        &self,
    ) -> &<freeze_dryer_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_ownables::ownables::id> for FreezeDryerModel {
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
