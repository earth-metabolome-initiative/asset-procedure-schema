//! Auto-generated crate for the `brands` table.
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
    :: diesel_builders :: prelude :: TableModel,
)]
/// Struct representing a row in the `brands` table.
# [table_model (error = :: validation_errors :: ValidationError)]
# [table_model (foreign_key ((creator_id ,) , (:: aps_users :: users :: id)))]
# [table_model (foreign_key ((editor_id ,) , (:: aps_users :: users :: id)))]
# [diesel (table_name = brands)]
pub struct Brand {
    /// Field representing the `id` column in table `brands`.
    # [table_model (default = :: rosetta_uuid :: Uuid :: new_v4 ())]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `name` column in table `brands`.
    name: String,
    /// Field representing the `creator_id` column in table `brands`.
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    creator_id: ::rosetta_uuid::Uuid,
    /// Field representing the `created_at` column in table `brands`.
    # [table_model (default = :: rosetta_timestamp :: TimestampUTC :: default ())]
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    created_at: ::rosetta_timestamp::TimestampUTC,
    /// Field representing the `editor_id` column in table `brands`.
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    editor_id: ::rosetta_uuid::Uuid,
    /// Field representing the `edited_at` column in table `brands`.
    # [table_model (default = :: rosetta_timestamp :: TimestampUTC :: default ())]
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    edited_at: ::rosetta_timestamp::TimestampUTC,
}
impl ::diesel_builders::ValidateColumn<brands::name>
    for <brands::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column(name: &String) -> Result<(), Self::Error> {
        use diesel::Column;
        if name.is_empty() {
            return Err(::validation_errors::ValidationError::empty(
                "brands",
                crate::brands::name::NAME,
            ));
        }
        Ok(())
    }
}
impl ::diesel_builders::ValidateColumn<brands::created_at>
    for <brands::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column_in_context(
        &self,
        created_at: &::rosetta_timestamp::TimestampUTC,
    ) -> Result<(), Self::Error> {
        use diesel::Column;
        if let Some(edited_at) =
            <Self as diesel_builders::MayGetColumn<brands::edited_at>>::may_get_column_ref(self)
            && created_at > edited_at
        {
            return Err(::validation_errors::ValidationError::smaller_than(
                "brands",
                crate::brands::created_at::NAME,
                crate::brands::edited_at::NAME,
            ));
        }
        Ok(())
    }
}
impl ::diesel_builders::ValidateColumn<brands::edited_at>
    for <brands::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column_in_context(
        &self,
        edited_at: &::rosetta_timestamp::TimestampUTC,
    ) -> Result<(), Self::Error> {
        use diesel::Column;
        if let Some(created_at) =
            <Self as diesel_builders::MayGetColumn<brands::created_at>>::may_get_column_ref(self)
            && created_at > edited_at
        {
            return Err(::validation_errors::ValidationError::smaller_than(
                "brands",
                crate::brands::created_at::NAME,
                crate::brands::edited_at::NAME,
            ));
        }
        Ok(())
    }
}
