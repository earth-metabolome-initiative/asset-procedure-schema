//! Auto-generated crate for the `users` table.
#[derive(
    Copy,
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
# [diesel (table_name = users)]
pub struct User {
    /// Undocumented column
    # [table_model (default = :: rosetta_uuid :: Uuid :: new_v4 ())]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Undocumented column
    # [table_model (default = :: rosetta_timestamp :: TimestampUTC :: default ())]
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    created_at: ::rosetta_timestamp::TimestampUTC,
    /// Undocumented column
    # [table_model (default = :: rosetta_timestamp :: TimestampUTC :: default ())]
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    updated_at: ::rosetta_timestamp::TimestampUTC,
}
impl ::diesel_builders::ValidateColumn<users::created_at>
    for <users::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError<&'static str>;
    #[inline]
    fn validate_column_in_context(
        &self,
        created_at: &::rosetta_timestamp::TimestampUTC,
    ) -> Result<(), Self::Error> {
        use diesel::Column;
        if let Some(updated_at) =
            <Self as diesel_builders::MayGetColumn<users::updated_at>>::may_get_column_ref(self)
        {
            if created_at > updated_at {
                return Err(validation_errors::prelude::ValidationError::smaller_than(
                    crate::users::created_at::NAME,
                    crate::users::updated_at::NAME,
                ));
            }
        }
        Ok(())
    }
}
impl ::diesel_builders::ValidateColumn<users::updated_at>
    for <users::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError<&'static str>;
    #[inline]
    fn validate_column_in_context(
        &self,
        updated_at: &::rosetta_timestamp::TimestampUTC,
    ) -> Result<(), Self::Error> {
        use diesel::Column;
        if let Some(created_at) =
            <Self as diesel_builders::MayGetColumn<users::created_at>>::may_get_column_ref(self)
        {
            if created_at > updated_at {
                return Err(validation_errors::prelude::ValidationError::smaller_than(
                    crate::users::created_at::NAME,
                    crate::users::updated_at::NAME,
                ));
            }
        }
        Ok(())
    }
}
