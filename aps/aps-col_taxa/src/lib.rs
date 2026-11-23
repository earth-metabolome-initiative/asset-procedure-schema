//! Auto-generated crate for the `col_taxa` table.
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
/// Source-specific Catalogue of Life identifiers mapped to canonical organism
/// taxa.
#[table_model(ancestors(aps_entities::entities, aps_organism_taxa::organism_taxa))]
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (belongs_to (aps_organism_taxa :: OrganismTaxon , foreign_key = id))]
# [table_model (foreign_key ((id ,) , (:: aps_organism_taxa :: organism_taxa :: id)))]
#[table_model(default(aps_entities::entities::table_name_id, "col_taxa"))]
# [diesel (table_name = col_taxa)]
pub struct ColTaxon {
    /// Stable taxon identifier inherited from canonical `organism_taxa`.
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Catalogue of Life taxon identifier from the source taxonomy dataset.
    col_id: String,
}
::diesel_builders::prelude::unique_index!(col_taxa::col_id);
impl ::diesel_builders::ValidateColumn<col_taxa::col_id>
    for <col_taxa::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column(col_id: &String) -> Result<(), Self::Error> {
        use diesel::Column;
        if col_id.is_empty() {
            return Err(::validation_errors::ValidationError::empty(
                <crate::col_taxa::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::col_taxa::col_id::NAME,
            ));
        }
        if col_id.len() >= 255usize {
            return Err(::validation_errors::ValidationError::exceeds_max_length(
                <crate::col_taxa::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::col_taxa::col_id::NAME,
                255usize,
            ));
        }
        Ok(())
    }
}
impl ::diesel_builders::GetColumn<aps_entities::entities::id> for ColTaxon {
    fn get_column_ref(&self) -> &<col_taxa::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_organism_taxa::organism_taxa::id> for ColTaxon {
    fn get_column_ref(&self) -> &<col_taxa::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
