//! Auto-generated crate for the `organism_taxonomies` table.
#[derive(
    Clone,
    Default,
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
/// Struct representing a row in the `organism_taxonomies` table.
#[table_model(ancestors(aps_table_names::table_names))]
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (belongs_to (aps_table_names :: TableName , foreign_key = id))]
# [table_model (foreign_key ((id ,) , (:: aps_table_names :: table_names :: id)))]
# [diesel (table_name = organism_taxonomies)]
pub struct OrganismTaxonomy {
    /// Field representing the `id` column in table `organism_taxonomies`.
    id: String,
}
impl ::diesel_builders::ValidateColumn<organism_taxonomies::id>
    for <organism_taxonomies::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column(id: &String) -> Result<(), Self::Error> {
        use diesel::Column;
        if id.is_empty() {
            return Err(::validation_errors::ValidationError::empty(
                <crate::organism_taxonomies::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::organism_taxonomies::id::NAME,
            ));
        }
        if id.len() > 255usize {
            return Err(::validation_errors::ValidationError::exceeds_max_length(
                <crate::organism_taxonomies::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::organism_taxonomies::id::NAME,
                255usize,
            ));
        }
        Ok(())
    }
}
impl ::diesel_builders::GetColumn<aps_table_names::table_names::id> for OrganismTaxonomy {
    fn get_column_ref(
        &self,
    ) -> &<organism_taxonomies::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
