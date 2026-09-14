//! Auto-generated crate for the `taxon_ranks` table.
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
/// Struct representing a row in the `taxon_ranks` table.
#[table_model(ancestors(aps_entities::entities))]
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (belongs_to (aps_entities :: Entity , foreign_key = id))]
# [diesel (belongs_to (aps_organism_taxonomies :: OrganismTaxonomy , foreign_key = organism_taxonomy_id))]
# [table_model (foreign_key ((id ,) , (:: aps_entities :: entities :: id)))]
# [table_model (foreign_key ((organism_taxonomy_id ,) , (:: aps_organism_taxonomies :: organism_taxonomies :: id)))]
#[table_model(default(aps_entities::entities::table_name_id, "taxon_ranks"))]
# [diesel (table_name = taxon_ranks)]
pub struct TaxonRank {
    /// Field representing the `id` column in table `taxon_ranks`.
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `name` column in table `taxon_ranks`.
    name: String,
    /// Field representing the `organism_taxonomy_id` column in table
    /// `taxon_ranks`.
    organism_taxonomy_id: Option<String>,
}
::diesel_builders::prelude::unique_index!(taxon_ranks::id, taxon_ranks::organism_taxonomy_id);
impl ::diesel_builders::ValidateColumn<taxon_ranks::name>
    for <taxon_ranks::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column(name: &String) -> Result<(), Self::Error> {
        use diesel::Column;
        if name.is_empty() {
            return Err(::validation_errors::ValidationError::empty(
                <crate::taxon_ranks::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::taxon_ranks::name::NAME,
            ));
        }
        if name.len() > 255usize {
            return Err(::validation_errors::ValidationError::exceeds_max_length(
                <crate::taxon_ranks::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::taxon_ranks::name::NAME,
                255usize,
            ));
        }
        Ok(())
    }
}
impl ::diesel_builders::ValidateColumn<taxon_ranks::organism_taxonomy_id>
    for <taxon_ranks::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column(organism_taxonomy_id: &String) -> Result<(), Self::Error> {
        use diesel::Column;
        if organism_taxonomy_id.is_empty() {
            return Err(::validation_errors::ValidationError::empty(
                <crate::taxon_ranks::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::taxon_ranks::organism_taxonomy_id::NAME,
            ));
        }
        if organism_taxonomy_id.len() > 255usize {
            return Err(::validation_errors::ValidationError::exceeds_max_length(
                <crate::taxon_ranks::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::taxon_ranks::organism_taxonomy_id::NAME,
                255usize,
            ));
        }
        Ok(())
    }
}
impl ::diesel_builders::GetColumn<aps_entities::entities::id> for TaxonRank {
    fn get_column_ref(&self) -> &<taxon_ranks::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
