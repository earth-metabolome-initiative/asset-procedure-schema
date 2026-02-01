//! Auto-generated crate for the `ownables` table.
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
/// Table storing ownables (base entity for ownable assets, procedures, etc.)
#[table_model(ancestors(aps_entities::entities))]
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (belongs_to (aps_entities :: Entity , foreign_key = id))]
# [diesel (belongs_to (aps_ownable_tables :: OwnableTable , foreign_key = ownable_table_id))]
# [diesel (belongs_to (aps_owners :: Owner , foreign_key = owner_id))]
# [table_model (foreign_key ((id ,) , (:: aps_entities :: entities :: id)))]
# [table_model (foreign_key ((ownable_table_id ,) , (:: aps_ownable_tables :: ownable_tables :: id)))]
# [table_model (foreign_key ((owner_id ,) , (:: aps_owners :: owners :: id)))]
# [table_model (foreign_key ((creator_id ,) , (:: aps_users :: users :: id)))]
# [table_model (foreign_key ((editor_id ,) , (:: aps_users :: users :: id)))]
#[table_model(default(aps_entities::entities::table_name_id, "ownables"))]
# [diesel (table_name = ownables)]
pub struct Ownable {
    /// Surrogate primary key for the ownable entity
    # [table_model (default = :: rosetta_uuid :: Uuid :: utc_v7 ())]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// The type of ownable (e.g., 'asset', 'procedure', etc.)
    ownable_table_id: String,
    /// Owner of the ownable entity
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    owner_id: ::rosetta_uuid::Uuid,
    /// Creator of the ownable entity
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    creator_id: ::rosetta_uuid::Uuid,
    /// Editor of the ownable entity
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    editor_id: ::rosetta_uuid::Uuid,
    /// Time of creation
    # [table_model (default = :: rosetta_timestamp :: TimestampUTC :: default ())]
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    created_at: ::rosetta_timestamp::TimestampUTC,
    /// Time of last update
    # [table_model (default = :: rosetta_timestamp :: TimestampUTC :: default ())]
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    edited_at: ::rosetta_timestamp::TimestampUTC,
}
impl ::diesel_builders::ValidateColumn<ownables::ownable_table_id>
    for <ownables::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column(ownable_table_id: &String) -> Result<(), Self::Error> {
        use diesel::Column;
        if ownable_table_id.is_empty() {
            return Err(::validation_errors::ValidationError::empty(
                <crate::ownables::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::ownables::ownable_table_id::NAME,
            ));
        }
        if ownable_table_id.len() < 255usize {
            return Err(::validation_errors::ValidationError::exceeds_max_length(
                <crate::ownables::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::ownables::ownable_table_id::NAME,
                255usize,
            ));
        }
        Ok(())
    }
}
impl ::diesel_builders::ValidateColumn<ownables::created_at>
    for <ownables::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column_in_context(
        &self,
        created_at: &::rosetta_timestamp::TimestampUTC,
    ) -> Result<(), Self::Error> {
        use diesel::Column;
        if let Some(edited_at) =
            <Self as diesel_builders::MayGetColumn<ownables::edited_at>>::may_get_column_ref(self)
        {
            if created_at > edited_at {
                return Err(::validation_errors::ValidationError::smaller_than(
                    <crate::ownables::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::ownables::created_at::NAME,
                    crate::ownables::edited_at::NAME,
                ));
            }
        }
        Ok(())
    }
}
impl ::diesel_builders::ValidateColumn<ownables::edited_at>
    for <ownables::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column_in_context(
        &self,
        edited_at: &::rosetta_timestamp::TimestampUTC,
    ) -> Result<(), Self::Error> {
        use diesel::Column;
        if let Some(created_at) =
            <Self as diesel_builders::MayGetColumn<ownables::created_at>>::may_get_column_ref(self)
        {
            if created_at > edited_at {
                return Err(::validation_errors::ValidationError::smaller_than(
                    <crate::ownables::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::ownables::created_at::NAME,
                    crate::ownables::edited_at::NAME,
                ));
            }
        }
        Ok(())
    }
}
impl ::diesel_builders::GetColumn<aps_entities::entities::id> for Ownable {
    fn get_column_ref(&self) -> &<ownables::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
