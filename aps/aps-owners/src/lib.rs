//! Auto-generated crate for the `owners` table.
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
/// Table storing owners (base entity for users, teams, projects)
#[table_model(ancestors(aps_entities::entities))]
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (belongs_to (aps_entities :: Entity , foreign_key = id))]
# [diesel (belongs_to (aps_table_names :: TableName , foreign_key = table_name_id))]
# [table_model (foreign_key ((id ,) , (:: aps_entities :: entities :: id)))]
# [table_model (foreign_key ((table_name_id ,) , (:: aps_table_names :: table_names :: id)))]
#[table_model(default(aps_entities::entities::table_name_id, "owners"))]
# [diesel (table_name = owners)]
pub struct Owner {
    /// Surrogate primary key for the owner entity
    # [table_model (default = :: rosetta_uuid :: Uuid :: utc_v7 ())]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// The type of owner (e.g., 'user', 'team', etc.)
    table_name_id: String,
    /// Time of creation
    # [table_model (default = :: rosetta_timestamp :: TimestampUTC :: default ())]
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    created_at: ::rosetta_timestamp::TimestampUTC,
    /// Time of last update
    # [table_model (default = :: rosetta_timestamp :: TimestampUTC :: default ())]
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    edited_at: ::rosetta_timestamp::TimestampUTC,
}
impl ::diesel_builders::ValidateColumn<owners::table_name_id>
    for <owners::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column(table_name_id: &String) -> Result<(), Self::Error> {
        use diesel::Column;
        if table_name_id.is_empty() {
            return Err(::validation_errors::ValidationError::empty(
                <crate::owners::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::owners::table_name_id::NAME,
            ));
        }
        if table_name_id.len() < 255usize {
            return Err(::validation_errors::ValidationError::exceeds_max_length(
                <crate::owners::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::owners::table_name_id::NAME,
                255usize,
            ));
        }
        Ok(())
    }
}
impl ::diesel_builders::ValidateColumn<owners::created_at>
    for <owners::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column_in_context(
        &self,
        created_at: &::rosetta_timestamp::TimestampUTC,
    ) -> Result<(), Self::Error> {
        use diesel::Column;
        if let Some(edited_at) =
            <Self as diesel_builders::MayGetColumn<owners::edited_at>>::may_get_column_ref(self)
        {
            if created_at > edited_at {
                return Err(::validation_errors::ValidationError::smaller_than(
                    <crate::owners::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::owners::created_at::NAME,
                    crate::owners::edited_at::NAME,
                ));
            }
        }
        Ok(())
    }
}
impl ::diesel_builders::ValidateColumn<owners::edited_at>
    for <owners::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column_in_context(
        &self,
        edited_at: &::rosetta_timestamp::TimestampUTC,
    ) -> Result<(), Self::Error> {
        use diesel::Column;
        if let Some(created_at) =
            <Self as diesel_builders::MayGetColumn<owners::created_at>>::may_get_column_ref(self)
        {
            if created_at > edited_at {
                return Err(::validation_errors::ValidationError::smaller_than(
                    <crate::owners::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::owners::created_at::NAME,
                    crate::owners::edited_at::NAME,
                ));
            }
        }
        Ok(())
    }
}
impl ::diesel_builders::GetColumn<aps_entities::entities::id> for Owner {
    fn get_column_ref(&self) -> &<owners::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
