//! Auto-generated crate for the `asset_models` table.
#[derive(
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
#[table_model(ancestors(aps_entities::entities, aps_ownables::ownables))]
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (belongs_to (aps_ownables :: Ownable , foreign_key = id))]
# [diesel (belongs_to (aps_namespaces :: Namespace , foreign_key = namespace_id))]
# [table_model (foreign_key ((id ,) , (:: aps_ownables :: ownables :: id)))]
# [table_model (foreign_key ((namespace_id ,) , (:: aps_namespaces :: namespaces :: id)))]
# [table_model (foreign_key ((parent_model_id ,) , (asset_models :: id)))]
#[table_model(default(aps_entities::entities::table_name_id, "asset_models"))]
# [diesel (table_name = asset_models)]
pub struct AssetModel {
    /// Field representing the `id` column in table `asset_models`.
    # [table_model (default = :: rosetta_uuid :: Uuid :: utc_v7 ())]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `name` column in table `asset_models`.
    name: String,
    /// Field representing the `description` column in table `asset_models`.
    description: String,
    /// Field representing the `namespace_id` column in table `asset_models`.
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    namespace_id: ::rosetta_uuid::Uuid,
    /// Field representing the `parent_model_id` column in table `asset_models`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    parent_model_id: Option<::rosetta_uuid::Uuid>,
}
::diesel_builders::prelude::unique_index!(asset_models::id, asset_models::parent_model_id);
::diesel_builders::prelude::unique_index!(asset_models::namespace_id, asset_models::name);
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
impl ::diesel_builders::ValidateColumn<asset_models::name>
    for <asset_models::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column(name: &String) -> Result<(), Self::Error> {
        use diesel::Column;
        if name.is_empty() {
            return Err(::validation_errors::ValidationError::empty(
                <crate::asset_models::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::asset_models::name::NAME,
            ));
        }
        if name.len() >= 255usize {
            return Err(::validation_errors::ValidationError::exceeds_max_length(
                <crate::asset_models::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::asset_models::name::NAME,
                255usize,
            ));
        }
        Ok(())
    }
    #[inline]
    fn validate_column_in_context(&self, name: &String) -> Result<(), Self::Error> {
        use diesel::Column;
        <Self as ::diesel_builders::ValidateColumn<asset_models::name>>::validate_column(name)?;
        if let Some(description) = <Self as diesel_builders::MayGetColumn<
            asset_models::description,
        >>::may_get_column_ref(self)
        {
            if name == description {
                return Err(::validation_errors::ValidationError::equal(
                    <crate::asset_models::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::asset_models::name::NAME,
                    crate::asset_models::description::NAME,
                ));
            }
        }
        Ok(())
    }
}
impl ::diesel_builders::ValidateColumn<asset_models::description>
    for <asset_models::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column(description: &String) -> Result<(), Self::Error> {
        use diesel::Column;
        if description.is_empty() {
            return Err(::validation_errors::ValidationError::empty(
                <crate::asset_models::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::asset_models::description::NAME,
            ));
        }
        if description.len() > 8192usize {
            return Err(::validation_errors::ValidationError::exceeds_max_length(
                <crate::asset_models::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::asset_models::description::NAME,
                8192usize,
            ));
        }
        Ok(())
    }
    #[inline]
    fn validate_column_in_context(&self, description: &String) -> Result<(), Self::Error> {
        use diesel::Column;
        <Self as ::diesel_builders::ValidateColumn<asset_models::description>>::validate_column(
            description,
        )?;
        if let Some(name) =
            <Self as diesel_builders::MayGetColumn<asset_models::name>>::may_get_column_ref(self)
        {
            if name == description {
                return Err(::validation_errors::ValidationError::equal(
                    <crate::asset_models::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::asset_models::name::NAME,
                    crate::asset_models::description::NAME,
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
impl ::diesel_builders::GetColumn<aps_ownables::ownables::id> for AssetModel {
    fn get_column_ref(&self) -> &<asset_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
