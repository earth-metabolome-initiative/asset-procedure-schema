//! Auto-generated crate for the `asset_models` table.
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
/// Struct representing a row in the `asset_models` table.
#[table_model(ancestors(
    aps_entities::entities,
    aps_ownables::ownables,
    aps_namespaced_ownables::namespaced_ownables
))]
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (belongs_to (aps_namespaced_ownables :: NamespacedOwnable , foreign_key = id))]
# [table_model (foreign_key ((id ,) , (:: aps_namespaced_ownables :: namespaced_ownables :: id)))]
# [table_model (foreign_key ((parent_model_id ,) , (asset_models :: id)))]
#[table_model(default(aps_entities::entities::table_name_id, "asset_models"))]
# [diesel (table_name = asset_models)]
pub struct AssetModel {
    /// Field representing the `id` column in table `asset_models`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `parent_model_id` column in table `asset_models`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    parent_model_id: Option<::rosetta_uuid::Uuid>,
}
::diesel_builders::prelude::unique_index!(asset_models::id, asset_models::parent_model_id);
impl ::diesel_builders::ValidateColumn<asset_models::id>
    for <asset_models::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column_in_context(&self, id: &::rosetta_uuid::Uuid) -> Result<(), Self::Error> {
        use diesel::Column;
        if let Some(parent_model_id) = <Self as diesel_builders::MayGetColumn<
            asset_models::parent_model_id,
        >>::may_get_column_ref(self)
        {
            if parent_model_id.as_ref().is_some_and(|parent_model_id| id == parent_model_id) {
                return Err(::validation_errors::ValidationError::equal(
                    <crate::asset_models::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::asset_models::id::NAME,
                    crate::asset_models::parent_model_id::NAME,
                ));
            }
        }
        Ok(())
    }
}
impl ::diesel_builders::ValidateColumn<asset_models::parent_model_id>
    for <asset_models::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column_in_context(
        &self,
        parent_model_id: &::rosetta_uuid::Uuid,
    ) -> Result<(), Self::Error> {
        use diesel::Column;
        if let Some(id) =
            <Self as diesel_builders::MayGetColumn<asset_models::id>>::may_get_column_ref(self)
        {
            if id == parent_model_id {
                return Err(::validation_errors::ValidationError::equal(
                    <crate::asset_models::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::asset_models::id::NAME,
                    crate::asset_models::parent_model_id::NAME,
                ));
            }
        }
        Ok(())
    }
}
impl ::diesel_builders::GetColumn<aps_entities::entities::id> for AssetModel {
    fn get_column_ref(&self) -> &<asset_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_namespaced_ownables::namespaced_ownables::id> for AssetModel {
    fn get_column_ref(&self) -> &<asset_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_ownables::ownables::id> for AssetModel {
    fn get_column_ref(&self) -> &<asset_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
