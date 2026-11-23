//! Auto-generated crate for the `organism_taxa` table.
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
/// Canonical registry of organism taxa with parent-child taxonomy links.
#[table_model(ancestors(aps_entities::entities))]
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (belongs_to (aps_entities :: Entity , foreign_key = id))]
# [diesel (belongs_to (aps_organism_taxonomies :: OrganismTaxonomy , foreign_key = organism_taxonomy_id))]
# [table_model (foreign_key ((id ,) , (:: aps_entities :: entities :: id)))]
# [table_model (foreign_key ((rank_id ,) , (:: aps_taxon_ranks :: taxon_ranks :: id)))]
# [table_model (foreign_key ((parent_organism_taxon_id ,) , (organism_taxa :: id)))]
# [table_model (foreign_key ((organism_taxonomy_id ,) , (:: aps_organism_taxonomies :: organism_taxonomies :: id)))]
#[table_model(default(aps_entities::entities::table_name_id, "organism_taxa"))]
# [diesel (table_name = organism_taxa)]
pub struct OrganismTaxon {
    /// Stable entity identifier inherited from the global `entities` root.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `rank_id` column in table `organism_taxa`.
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    rank_id: Option<::rosetta_uuid::Uuid>,
    /// Optional immediate parent taxon in the classification hierarchy.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    parent_organism_taxon_id: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `organism_taxonomy_id` column in table
    /// `organism_taxa`.
    organism_taxonomy_id: Option<String>,
}
impl ::diesel_builders::ValidateColumn<organism_taxa::id>
    for <organism_taxa::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column_in_context(&self, id: &::rosetta_uuid::Uuid) -> Result<(), Self::Error> {
        use diesel::Column;
        if let Some(parent_organism_taxon_id) = <Self as diesel_builders::MayGetColumn<
            organism_taxa::parent_organism_taxon_id,
        >>::may_get_column_ref(self)
            && parent_organism_taxon_id
                .as_ref()
                .is_some_and(|parent_organism_taxon_id| id == parent_organism_taxon_id)
        {
            return Err(::validation_errors::ValidationError::equal(
                <crate::organism_taxa::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::organism_taxa::id::NAME,
                crate::organism_taxa::parent_organism_taxon_id::NAME,
            ));
        }
        Ok(())
    }
}
impl ::diesel_builders::ValidateColumn<organism_taxa::parent_organism_taxon_id>
    for <organism_taxa::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column_in_context(
        &self,
        parent_organism_taxon_id: &::rosetta_uuid::Uuid,
    ) -> Result<(), Self::Error> {
        use diesel::Column;
        if let Some(id) =
            <Self as diesel_builders::MayGetColumn<organism_taxa::id>>::may_get_column_ref(self)
            && id == parent_organism_taxon_id
        {
            return Err(::validation_errors::ValidationError::equal(
                <crate::organism_taxa::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::organism_taxa::id::NAME,
                crate::organism_taxa::parent_organism_taxon_id::NAME,
            ));
        }
        Ok(())
    }
}
impl ::diesel_builders::ValidateColumn<organism_taxa::organism_taxonomy_id>
    for <organism_taxa::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column(organism_taxonomy_id: &String) -> Result<(), Self::Error> {
        use diesel::Column;
        if organism_taxonomy_id.is_empty() {
            return Err(::validation_errors::ValidationError::empty(
                <crate::organism_taxa::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::organism_taxa::organism_taxonomy_id::NAME,
            ));
        }
        if organism_taxonomy_id.len() > 255usize {
            return Err(::validation_errors::ValidationError::exceeds_max_length(
                <crate::organism_taxa::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::organism_taxa::organism_taxonomy_id::NAME,
                255usize,
            ));
        }
        Ok(())
    }
}
impl ::diesel_builders::GetColumn<aps_entities::entities::id> for OrganismTaxon {
    fn get_column_ref(&self) -> &<organism_taxa::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
