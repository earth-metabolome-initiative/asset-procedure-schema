//! Auto-generated crate for the `asset_models` table.
#[derive(
    Clone,
    Debug,
    Hash,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    serde :: Serialize,
    serde :: Deserialize,
    diesel :: Queryable,
    diesel :: Selectable,
    diesel :: Identifiable,
    diesel_builders :: prelude :: TableModel,
)]
/// Struct representing a row in the `asset_models` table.
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (table_name = asset_models)]
pub struct AssetModel {
    /// Field representing the `id` column in table `asset_models`.
    # [table_model (default = :: rosetta_uuid :: Uuid :: new_v4 ())]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `asset_model_table_id` column in table
    /// `asset_models`.
    #[table_model(default = "asset_models")]
    #[infallible]
    asset_model_table_id: String,
    /// Field representing the `name` column in table `asset_models`.
    name: String,
    /// Field representing the `description` column in table `asset_models`.
    description: String,
    /// Field representing the `parent_model_id` column in table `asset_models`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    parent_model_id: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `creator_id` column in table `asset_models`.
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    creator_id: ::rosetta_uuid::Uuid,
    /// Field representing the `created_at` column in table `asset_models`.
    # [table_model (default = :: rosetta_timestamp :: TimestampUTC :: default ())]
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    created_at: ::rosetta_timestamp::TimestampUTC,
    /// Field representing the `editor_id` column in table `asset_models`.
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    editor_id: ::rosetta_uuid::Uuid,
    /// Field representing the `edited_at` column in table `asset_models`.
    # [table_model (default = :: rosetta_timestamp :: TimestampUTC :: default ())]
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    edited_at: ::rosetta_timestamp::TimestampUTC,
}
::diesel_builders::prelude::unique_index!(asset_models::name);
::diesel_builders::prelude::unique_index!(asset_models::id, asset_models::parent_model_id);
:: diesel_builders :: prelude :: fk ! ((asset_models :: asset_model_table_id) -> (:: aps_asset_model_tables :: asset_model_tables :: id));
:: diesel_builders :: prelude :: fk ! ((asset_models :: parent_model_id) -> (asset_models :: id));
:: diesel_builders :: prelude :: fk ! ((asset_models :: creator_id) -> (:: aps_users :: users :: id));
:: diesel_builders :: prelude :: fk ! ((asset_models :: editor_id) -> (:: aps_users :: users :: id));
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
            && parent_model_id.as_ref().is_some_and(|parent_model_id| id == parent_model_id)
        {
            return Err(::validation_errors::ValidationError::equal(
                "asset_models",
                crate::asset_models::id::NAME,
                crate::asset_models::parent_model_id::NAME,
            ));
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
                "asset_models",
                crate::asset_models::name::NAME,
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
            && name == description
        {
            return Err(::validation_errors::ValidationError::equal(
                "asset_models",
                crate::asset_models::name::NAME,
                crate::asset_models::description::NAME,
            ));
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
                "asset_models",
                crate::asset_models::description::NAME,
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
            && name == description
        {
            return Err(::validation_errors::ValidationError::equal(
                "asset_models",
                crate::asset_models::name::NAME,
                crate::asset_models::description::NAME,
            ));
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
            && id == parent_model_id
        {
            return Err(::validation_errors::ValidationError::equal(
                "asset_models",
                crate::asset_models::id::NAME,
                crate::asset_models::parent_model_id::NAME,
            ));
        }
        Ok(())
    }
}
impl ::diesel_builders::ValidateColumn<asset_models::created_at>
    for <asset_models::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column_in_context(
        &self,
        created_at: &::rosetta_timestamp::TimestampUTC,
    ) -> Result<(), Self::Error> {
        use diesel::Column;
        if let Some(edited_at) =
            <Self as diesel_builders::MayGetColumn<asset_models::edited_at>>::may_get_column_ref(
                self,
            )
            && created_at > edited_at
        {
            return Err(::validation_errors::ValidationError::smaller_than(
                "asset_models",
                crate::asset_models::created_at::NAME,
                crate::asset_models::edited_at::NAME,
            ));
        }
        Ok(())
    }
}
impl ::diesel_builders::ValidateColumn<asset_models::edited_at>
    for <asset_models::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column_in_context(
        &self,
        edited_at: &::rosetta_timestamp::TimestampUTC,
    ) -> Result<(), Self::Error> {
        use diesel::Column;
        if let Some(created_at) = <Self as diesel_builders::MayGetColumn<
            asset_models::created_at,
        >>::may_get_column_ref(self)
            && created_at > edited_at
        {
            return Err(::validation_errors::ValidationError::smaller_than(
                "asset_models",
                crate::asset_models::created_at::NAME,
                crate::asset_models::edited_at::NAME,
            ));
        }
        Ok(())
    }
}
