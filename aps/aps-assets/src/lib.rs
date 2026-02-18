//! Auto-generated crate for the `assets` table.
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
/// Struct representing a row in the `assets` table.
#[table_model(ancestors(
    aps_entities::entities,
    aps_ownables::ownables,
    aps_namespaced_ownables::namespaced_ownables
))]
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (belongs_to (aps_namespaced_ownables :: NamespacedOwnable , foreign_key = id))]
# [diesel (belongs_to (aps_asset_models :: AssetModel , foreign_key = model_id))]
# [table_model (foreign_key ((id ,) , (:: aps_namespaced_ownables :: namespaced_ownables :: id)))]
# [table_model (foreign_key ((model_id ,) , (:: aps_asset_models :: asset_models :: id)))]
#[table_model(default(aps_entities::entities::table_name_id, "assets"))]
# [diesel (table_name = assets)]
pub struct Asset {
    /// Field representing the `id` column in table `assets`.
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `model_id` column in table `assets`.
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    model_id: ::rosetta_uuid::Uuid,
}
::diesel_builders::prelude::unique_index!(assets::id, assets::model_id);
impl ::diesel_builders::GetColumn<aps_entities::entities::id> for Asset {
    fn get_column_ref(&self) -> &<assets::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_namespaced_ownables::namespaced_ownables::id> for Asset {
    fn get_column_ref(&self) -> &<assets::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_ownables::ownables::id> for Asset {
    fn get_column_ref(&self) -> &<assets::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
