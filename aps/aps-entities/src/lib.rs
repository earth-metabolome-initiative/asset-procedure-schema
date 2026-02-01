//! Auto-generated crate for the `entities` table.
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
/// Base Table for all entities (Owners, Ownables, etc.)
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (belongs_to (aps_table_names :: TableName , foreign_key = table_name_id))]
# [diesel (belongs_to (aps_roles :: Role , foreign_key = minimum_role_id))]
# [table_model (foreign_key ((table_name_id ,) , (:: aps_table_names :: table_names :: id)))]
# [table_model (foreign_key ((minimum_role_id ,) , (:: aps_roles :: roles :: id)))]
# [diesel (table_name = entities)]
pub struct Entity {
    /// Universal UUIDv7 ID
    # [table_model (default = :: rosetta_uuid :: Uuid :: utc_v7 ())]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// The specific table this entity belongs to (e.g., 'users', 'assets')
    table_name_id: String,
    /// Defines the minimum role required to view the entity.
    #[table_model(default = 1i16)]
    #[infallible]
    minimum_role_id: i16,
    /// Time of creation, using UTC timezone
    # [table_model (default = :: rosetta_utc :: TimestampUTC :: default ())]
    # [diesel (sql_type = :: rosetta_utc :: diesel_impls :: TimestampUTC)]
    created_at: ::rosetta_utc::TimestampUTC,
    /// Time of last edit, using UTC timezone
    # [table_model (default = :: rosetta_utc :: TimestampUTC :: default ())]
    # [diesel (sql_type = :: rosetta_utc :: diesel_impls :: TimestampUTC)]
    edited_at: ::rosetta_utc::TimestampUTC,
}
impl ::diesel_builders::ValidateColumn<entities::table_name_id>
    for <entities::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column(table_name_id: &String) -> Result<(), Self::Error> {
        use diesel::Column;
        if table_name_id.is_empty() {
            return Err(::validation_errors::ValidationError::empty(
                <crate::entities::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::entities::table_name_id::NAME,
            ));
        }
        if table_name_id.len() > 255usize {
            return Err(::validation_errors::ValidationError::exceeds_max_length(
                <crate::entities::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::entities::table_name_id::NAME,
                255usize,
            ));
        }
        Ok(())
    }
}
impl ::diesel_builders::ValidateColumn<entities::created_at>
    for <entities::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column(created_at: &::rosetta_utc::TimestampUTC) -> Result<(), Self::Error> {
        use diesel::Column;
        if *created_at >= ::rosetta_utc::TimestampUTC::now() {
            return Err(::validation_errors::ValidationError::in_the_future(
                <crate::entities::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::entities::created_at::NAME,
            ));
        }
        Ok(())
    }
    #[inline]
    fn validate_column_in_context(
        &self,
        created_at: &::rosetta_utc::TimestampUTC,
    ) -> Result<(), Self::Error> {
        use diesel::Column;
        <Self as ::diesel_builders::ValidateColumn<entities::created_at>>::validate_column(
            created_at,
        )?;
        if let Some(edited_at) =
            <Self as diesel_builders::MayGetColumn<entities::edited_at>>::may_get_column_ref(self)
            && created_at > edited_at
        {
            return Err(::validation_errors::ValidationError::smaller_than(
                <crate::entities::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::entities::created_at::NAME,
                crate::entities::edited_at::NAME,
            ));
        }
        Ok(())
    }
}
impl ::diesel_builders::ValidateColumn<entities::edited_at>
    for <entities::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column(edited_at: &::rosetta_utc::TimestampUTC) -> Result<(), Self::Error> {
        use diesel::Column;
        if *edited_at >= ::rosetta_utc::TimestampUTC::now() {
            return Err(::validation_errors::ValidationError::in_the_future(
                <crate::entities::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::entities::edited_at::NAME,
            ));
        }
        Ok(())
    }
    #[inline]
    fn validate_column_in_context(
        &self,
        edited_at: &::rosetta_utc::TimestampUTC,
    ) -> Result<(), Self::Error> {
        use diesel::Column;
        <Self as ::diesel_builders::ValidateColumn<entities::edited_at>>::validate_column(
            edited_at,
        )?;
        if let Some(created_at) =
            <Self as diesel_builders::MayGetColumn<entities::created_at>>::may_get_column_ref(self)
            && created_at > edited_at
        {
            return Err(::validation_errors::ValidationError::smaller_than(
                <crate::entities::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::entities::created_at::NAME,
                crate::entities::edited_at::NAME,
            ));
        }
        Ok(())
    }
}
