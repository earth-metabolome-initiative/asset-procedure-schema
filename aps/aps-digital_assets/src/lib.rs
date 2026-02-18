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
    :: serde :: Serialize,
    :: serde :: Deserialize,
    :: diesel :: Queryable,
    :: diesel :: Selectable,
    :: diesel :: Identifiable,
    :: diesel :: Associations,
    :: diesel_builders :: prelude :: TableModel,
)]
/// Struct representing a row in the `digital_assets` table.
#[table_model(ancestors(
    aps_entities::entities,
    aps_ownables::ownables,
    aps_namespaced_ownables::namespaced_ownables,
    aps_assets::assets
))]
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (belongs_to (aps_digital_asset_models :: DigitalAssetModel , foreign_key = digital_asset_model_id))]
# [table_model (foreign_key ((id ,) , (:: aps_assets :: assets :: id)))]
# [table_model (foreign_key ((digital_asset_model_id ,) , (:: aps_digital_asset_models :: digital_asset_models :: id)))]
#[table_model(default(aps_entities::entities::table_name_id, "digital_assets"))]
# [diesel (table_name = digital_assets)]
pub struct DigitalAsset {
    /// Field representing the `id` column in table `digital_assets`.
    #[same_as(aps_assets::assets::id)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `digital_asset_model_id` column in table
    /// `digital_assets`.
    #[same_as(aps_assets::assets::model_id)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    digital_asset_model_id: ::rosetta_uuid::Uuid,
}
impl ::diesel_builders::GetColumn<aps_assets::assets::id> for DigitalAsset {
    fn get_column_ref(
        &self,
    ) -> &<digital_assets::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_entities::entities::id> for DigitalAsset {
    fn get_column_ref(
        &self,
    ) -> &<digital_assets::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_namespaced_ownables::namespaced_ownables::id>
    for DigitalAsset
{
    fn get_column_ref(
        &self,
    ) -> &<digital_assets::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_ownables::ownables::id> for DigitalAsset {
    fn get_column_ref(
        &self,
    ) -> &<digital_assets::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
