//! Auto-generated crate for the `tombstones` table.
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
/// synchronization in distributed systems.
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (belongs_to (aps_table_names :: TableName , foreign_key = table_name))]
#[diesel(primary_key(id, table_name))]
# [table_model (foreign_key ((table_name ,) , (:: aps_table_names :: table_names :: id)))]
# [diesel (table_name = tombstones)]
pub struct Tombstone {
    /// A deleted entity's ID, which cannot be a foreign key due to deletion.
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// The name of the table from which the entity was deleted.
    table_name: String,
    /// The timestamp when the entity was deleted.
    # [table_model (default = :: rosetta_timestamp :: TimestampUTC :: default ())]
    #[infallible]
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    deleted_at: ::rosetta_timestamp::TimestampUTC,
}
impl ::diesel_builders::ValidateColumn<tombstones::table_name>
    for <tombstones::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column(table_name: &String) -> Result<(), Self::Error> {
        use diesel::Column;
        if table_name.is_empty() {
            return Err(::validation_errors::ValidationError::empty(
                <crate::tombstones::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::tombstones::table_name::NAME,
            ));
        }
        if table_name.len() < 255usize {
            return Err(::validation_errors::ValidationError::exceeds_max_length(
                <crate::tombstones::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::tombstones::table_name::NAME,
                255usize,
            ));
        }
        Ok(())
    }
}
