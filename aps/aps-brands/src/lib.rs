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
    :: diesel :: Associations,
    :: diesel_builders :: prelude :: TableModel,
)]
/// Struct representing a row in the `brands` table.
#[table_model(ancestors(aps_entities::entities, aps_ownables::ownables))]
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (belongs_to (aps_ownables :: Ownable , foreign_key = id))]
# [table_model (foreign_key ((id ,) , (:: aps_ownables :: ownables :: id)))]
#[table_model(default(aps_entities::entities::table_name_id, "brands"))]
# [diesel (table_name = brands)]
pub struct Brand {
    /// Field representing the `id` column in table `brands`.
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `name` column in table `brands`.
    name: String,
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
                <crate::brands::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::brands::name::NAME,
            ));
        }
        if name.len() >= 255usize {
            return Err(::validation_errors::ValidationError::exceeds_max_length(
                <crate::brands::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::brands::name::NAME,
                255usize,
            ));
        }
        Ok(())
    }
}
impl ::diesel_builders::GetColumn<aps_entities::entities::id> for Brand {
    fn get_column_ref(&self) -> &<brands::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_ownables::ownables::id> for Brand {
    fn get_column_ref(&self) -> &<brands::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
