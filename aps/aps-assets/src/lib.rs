//! Auto-generated crate for the `assets` table.
#[derive(
    Clone,
    Eq,
    PartialEq,
    serde :: Serialize,
    serde :: Deserialize,
    diesel :: Queryable,
    diesel :: Selectable,
    diesel :: Identifiable,
    diesel_builders :: prelude :: TableModel,
)]
/// Undocumented table
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (table_name = assets)]
pub struct Asset {
    /// Undocumented column
    # [table_model (default = :: rosetta_uuid :: Uuid :: new_v4 ())]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Undocumented column
    #[infallible]
    most_concrete_table: String,
    /// Undocumented column
    name: Option<String>,
    /// Undocumented column
    description: Option<String>,
    /// Undocumented column
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    model_id: ::rosetta_uuid::Uuid,
    /// Undocumented column
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    creator_id: ::rosetta_uuid::Uuid,
    /// Undocumented column
    # [table_model (default = :: rosetta_timestamp :: TimestampUTC :: default ())]
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    created_at: ::rosetta_timestamp::TimestampUTC,
    /// Undocumented column
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    editor_id: ::rosetta_uuid::Uuid,
    /// Undocumented column
    # [table_model (default = :: rosetta_timestamp :: TimestampUTC :: default ())]
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    edited_at: ::rosetta_timestamp::TimestampUTC,
}
::diesel_builders::prelude::unique_index!(assets::id, assets::model_id);
::diesel_builders::prelude::unique_index!(assets::name, assets::model_id);
:: diesel_builders :: prelude :: fk ! ((assets :: model_id) -> (:: aps_asset_models :: asset_models :: id));
:: diesel_builders :: prelude :: fk ! ((assets :: creator_id) -> (:: aps_users :: users :: id));
:: diesel_builders :: prelude :: fk ! ((assets :: editor_id) -> (:: aps_users :: users :: id));
impl ::diesel_builders::ValidateColumn<assets::name>
    for <assets::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError<&'static str>;
    #[inline]
    fn validate_column(name: &String) -> Result<(), Self::Error> {
        use diesel::Column;
        if name.is_empty() {
            return Err(validation_errors::prelude::ValidationError::empty(
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
            return Err(validation_errors::prelude::ValidationError::equal(
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
    type Error = ::validation_errors::ValidationError<&'static str>;
    #[inline]
    fn validate_column(description: &String) -> Result<(), Self::Error> {
        use diesel::Column;
        if description.is_empty() {
            return Err(validation_errors::prelude::ValidationError::empty(
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
            return Err(validation_errors::prelude::ValidationError::equal(
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
    type Error = ::validation_errors::ValidationError<&'static str>;
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
            return Err(validation_errors::prelude::ValidationError::smaller_than(
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
    type Error = ::validation_errors::ValidationError<&'static str>;
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
            return Err(validation_errors::prelude::ValidationError::smaller_than(
                crate::assets::created_at::NAME,
                crate::assets::edited_at::NAME,
            ));
        }
        Ok(())
    }
}
