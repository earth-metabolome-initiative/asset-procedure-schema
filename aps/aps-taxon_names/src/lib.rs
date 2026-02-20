//! Auto-generated crate for the `taxon_names` table.
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
/// Struct representing a row in the `taxon_names` table.
#[table_model(ancestors(aps_entities::entities))]
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (belongs_to (aps_entities :: Entity , foreign_key = id))]
# [diesel (belongs_to (aps_organism_taxa :: OrganismTaxon , foreign_key = organism_taxon_id))]
# [diesel (belongs_to (aps_organism_taxonomies :: OrganismTaxonomy , foreign_key = organism_taxonomy_id))]
# [table_model (foreign_key ((id ,) , (:: aps_entities :: entities :: id)))]
# [table_model (foreign_key ((organism_taxon_id ,) , (:: aps_organism_taxa :: organism_taxa :: id)))]
# [table_model (foreign_key ((class_id ,) , (:: aps_name_classes :: name_classes :: id)))]
# [table_model (foreign_key ((organism_taxonomy_id ,) , (:: aps_organism_taxonomies :: organism_taxonomies :: id)))]
#[table_model(default(aps_entities::entities::table_name_id, "taxon_names"))]
# [diesel (table_name = taxon_names)]
pub struct TaxonName {
    /// Field representing the `id` column in table `taxon_names`.
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `organism_taxon_id` column in table
    /// `taxon_names`.
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    organism_taxon_id: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `name` column in table `taxon_names`.
    name: String,
    /// Field representing the `class_id` column in table `taxon_names`.
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    class_id: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `organism_taxonomy_id` column in table
    /// `taxon_names`.
    organism_taxonomy_id: Option<String>,
}
::diesel_builders::prelude::unique_index!(taxon_names::organism_taxon_id, taxon_names::name);
impl ::diesel_builders::ValidateColumn<taxon_names::name>
    for <taxon_names::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column(name: &String) -> Result<(), Self::Error> {
        use diesel::Column;
        if name.is_empty() {
            return Err(::validation_errors::ValidationError::empty(
                <crate::taxon_names::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::taxon_names::name::NAME,
            ));
        }
        if name.len() > 255usize {
            return Err(::validation_errors::ValidationError::exceeds_max_length(
                <crate::taxon_names::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::taxon_names::name::NAME,
                255usize,
            ));
        }
        Ok(())
    }
}
impl ::diesel_builders::ValidateColumn<taxon_names::organism_taxonomy_id>
    for <taxon_names::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column(organism_taxonomy_id: &String) -> Result<(), Self::Error> {
        use diesel::Column;
        if organism_taxonomy_id.is_empty() {
            return Err(::validation_errors::ValidationError::empty(
                <crate::taxon_names::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::taxon_names::organism_taxonomy_id::NAME,
            ));
        }
        if organism_taxonomy_id.len() > 255usize {
            return Err(::validation_errors::ValidationError::exceeds_max_length(
                <crate::taxon_names::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::taxon_names::organism_taxonomy_id::NAME,
                255usize,
            ));
        }
        Ok(())
    }
}
impl ::diesel_builders::GetColumn<aps_entities::entities::id> for TaxonName {
    fn get_column_ref(&self) -> &<taxon_names::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
