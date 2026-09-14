//! Auto-generated crate for the `ncbi_taxa` table.
#[derive(
    Copy,
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
/// Source-specific NCBI taxonomy identifiers mapped to canonical organism taxa.
#[table_model(ancestors(aps_entities::entities, aps_organism_taxa::organism_taxa))]
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (belongs_to (aps_organism_taxa :: OrganismTaxon , foreign_key = id))]
# [table_model (foreign_key ((id ,) , (:: aps_organism_taxa :: organism_taxa :: id)))]
#[table_model(default(aps_entities::entities::table_name_id, "ncbi_taxa"))]
# [diesel (table_name = ncbi_taxa)]
pub struct NcbiTaxon {
    /// Stable taxon identifier inherited from canonical `organism_taxa`.
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Positive NCBI taxonomy identifier assigned by the NCBI taxonomy dataset.
    ncbi_id: i32,
}
::diesel_builders::prelude::unique_index!(ncbi_taxa::ncbi_id);
impl ::diesel_builders::ValidateColumn<ncbi_taxa::ncbi_id>
    for <ncbi_taxa::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column(ncbi_id: &i32) -> Result<(), Self::Error> {
        use diesel::Column;
        if ncbi_id <= &0i32 {
            return Err(::validation_errors::ValidationError::strictly_greater_than_value(
                <crate::ncbi_taxa::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::ncbi_taxa::ncbi_id::NAME,
                0f64,
            ));
        }
        Ok(())
    }
}
impl ::diesel_builders::GetColumn<aps_entities::entities::id> for NcbiTaxon {
    fn get_column_ref(&self) -> &<ncbi_taxa::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_organism_taxa::organism_taxa::id> for NcbiTaxon {
    fn get_column_ref(&self) -> &<ncbi_taxa::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
