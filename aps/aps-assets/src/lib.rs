//! Auto-generated crate for the `assets` table.
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
/// Struct representing a row in the `assets` table.
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (belongs_to (aps_asset_tables :: AssetTable , foreign_key = asset_table_id))]
# [diesel (belongs_to (aps_asset_models :: AssetModel , foreign_key = model_id))]
# [table_model (foreign_key ((asset_table_id ,) , (:: aps_asset_tables :: asset_tables :: id)))]
# [table_model (foreign_key ((model_id ,) , (:: aps_asset_models :: asset_models :: id)))]
# [table_model (foreign_key ((creator_id ,) , (:: aps_users :: users :: id)))]
# [table_model (foreign_key ((editor_id ,) , (:: aps_users :: users :: id)))]
# [diesel (table_name = assets)]
pub struct Asset {
    /// Field representing the `id` column in table `assets`.
    # [table_model (default = :: rosetta_uuid :: Uuid :: new_v4 ())]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `asset_table_id` column in table `assets`.
    #[table_model(default = "assets")]
    #[infallible]
    asset_table_id: String,
    /// Field representing the `name` column in table `assets`.
    name: Option<String>,
    /// Field representing the `description` column in table `assets`.
    description: Option<String>,
    /// Field representing the `model_id` column in table `assets`.
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `creator_id` column in table `assets`.
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    creator_id: ::rosetta_uuid::Uuid,
    /// Field representing the `created_at` column in table `assets`.
    # [table_model (default = :: rosetta_timestamp :: TimestampUTC :: default ())]
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    created_at: ::rosetta_timestamp::TimestampUTC,
    /// Field representing the `editor_id` column in table `assets`.
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    editor_id: ::rosetta_uuid::Uuid,
    /// Field representing the `edited_at` column in table `assets`.
    # [table_model (default = :: rosetta_timestamp :: TimestampUTC :: default ())]
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    edited_at: ::rosetta_timestamp::TimestampUTC,
}
::diesel_builders::prelude::unique_index!(assets::id, assets::model_id);
::diesel_builders::prelude::unique_index!(assets::name, assets::model_id);
impl ::diesel_builders::ValidateColumn<assets::name>
    for <assets::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column(name: &String) -> Result<(), Self::Error> {
        use diesel::Column;
        if name.is_empty() {
            return Err(::validation_errors::ValidationError::empty(
                "assets",
                crate::assets::name::NAME,
            ));
        }
        Ok(())
    }
    #[inline]
    fn validate_column_in_context(&self, name: &String) -> Result<(), Self::Error> {
        use diesel::Column;
        <Self as ::diesel_builders::ValidateColumn<assets::name>>::validate_column(name)?;
        if let Some(description) =
            <Self as diesel_builders::MayGetColumn<assets::description>>::may_get_column_ref(self)
            && description.as_ref().is_some_and(|description| name == description)
        {
            return Err(::validation_errors::ValidationError::equal(
                "assets",
                crate::assets::name::NAME,
                crate::assets::description::NAME,
            ));
        }
        Ok(())
    }
}
impl ::diesel_builders::ValidateColumn<assets::description>
    for <assets::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column(description: &String) -> Result<(), Self::Error> {
        use diesel::Column;
        if description.is_empty() {
            return Err(::validation_errors::ValidationError::empty(
                "assets",
                crate::assets::description::NAME,
            ));
        }
        Ok(())
    }
    #[inline]
    fn validate_column_in_context(&self, description: &String) -> Result<(), Self::Error> {
        use diesel::Column;
        <Self as ::diesel_builders::ValidateColumn<assets::description>>::validate_column(
            description,
        )?;
        if let Some(name) =
            <Self as diesel_builders::MayGetColumn<assets::name>>::may_get_column_ref(self)
            && name.as_ref().is_some_and(|name| name == description)
        {
            return Err(::validation_errors::ValidationError::equal(
                "assets",
                crate::assets::name::NAME,
                crate::assets::description::NAME,
            ));
        }
        Ok(())
    }
}
impl ::diesel_builders::ValidateColumn<assets::created_at>
    for <assets::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column_in_context(
        &self,
        created_at: &::rosetta_timestamp::TimestampUTC,
    ) -> Result<(), Self::Error> {
        use diesel::Column;
        if let Some(edited_at) =
            <Self as diesel_builders::MayGetColumn<assets::edited_at>>::may_get_column_ref(self)
            && created_at > edited_at
        {
            return Err(::validation_errors::ValidationError::smaller_than(
                "assets",
                crate::assets::created_at::NAME,
                crate::assets::edited_at::NAME,
            ));
        }
        Ok(())
    }
}
impl ::diesel_builders::ValidateColumn<assets::edited_at>
    for <assets::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column_in_context(
        &self,
        edited_at: &::rosetta_timestamp::TimestampUTC,
    ) -> Result<(), Self::Error> {
        use diesel::Column;
        if let Some(created_at) =
            <Self as diesel_builders::MayGetColumn<assets::created_at>>::may_get_column_ref(self)
            && created_at > edited_at
        {
            return Err(::validation_errors::ValidationError::smaller_than(
                "assets",
                crate::assets::created_at::NAME,
                crate::assets::edited_at::NAME,
            ));
        }
        Ok(())
    }
}
