//! Auto-generated crate for the `container_sealer_models` table.
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
/// Any physical asset whose primary function is to seal, cover, or close a
/// container opening.
#[table_model(ancestors(
    aps_asset_models::asset_models,
    aps_physical_asset_models::physical_asset_models
))]
# [diesel (belongs_to (aps_physical_asset_models :: PhysicalAssetModel , foreign_key = id))]
# [table_model (foreign_key ((id ,) , (:: aps_physical_asset_models :: physical_asset_models :: id)))]
#[table_model(default(
    aps_asset_models::asset_models::asset_model_table_id,
    "container_sealer_models"
))]
# [diesel (table_name = container_sealer_models)]
pub struct ContainerSealerModel {
    /// Field representing the `id` column in table `container_sealer_models`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
}
impl ::diesel_builders::GetColumn<aps_asset_models::asset_models::id> for ContainerSealerModel {
    fn get_column_ref(
        &self,
    ) -> &<container_sealer_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_physical_asset_models::physical_asset_models::id>
    for ContainerSealerModel
{
    fn get_column_ref(
        &self,
    ) -> &<container_sealer_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
