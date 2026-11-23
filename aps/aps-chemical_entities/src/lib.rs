//! Auto-generated crate for the `chemical_entities` table.
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
/// Canonical registry of chemical entities keyed by InChI and InChIKey.
#[table_model(ancestors(aps_entities::entities))]
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (belongs_to (aps_entities :: Entity , foreign_key = id))]
# [table_model (foreign_key ((id ,) , (:: aps_entities :: entities :: id)))]
#[table_model(default(aps_entities::entities::table_name_id, "chemical_entities"))]
# [diesel (table_name = chemical_entities)]
pub struct ChemicalEntity {
    /// Stable entity identifier inherited from the global `entities` root.
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Full International Chemical Identifier encoding molecular structure.
    inchi: String,
    /// Condensed 27-character hash identifier for fast indexing and lookup.
    inchi_key: String,
}
::diesel_builders::prelude::unique_index!(chemical_entities::inchi);
::diesel_builders::prelude::unique_index!(chemical_entities::inchi_key);
impl ::diesel_builders::ValidateColumn<chemical_entities::inchi>
    for <chemical_entities::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column(inchi: &String) -> Result<(), Self::Error> {
        use diesel::Column;
        if inchi.is_empty() {
            return Err(::validation_errors::ValidationError::empty(
                <crate::chemical_entities::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::chemical_entities::inchi::NAME,
            ));
        }
        if inchi.len() > 8192usize {
            return Err(::validation_errors::ValidationError::exceeds_max_length(
                <crate::chemical_entities::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::chemical_entities::inchi::NAME,
                8192usize,
            ));
        }
        Ok(())
    }
}
impl ::diesel_builders::ValidateColumn<chemical_entities::inchi_key>
    for <chemical_entities::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column(inchi_key: &String) -> Result<(), Self::Error> {
        use diesel::Column;
        if inchi_key.is_empty() {
            return Err(::validation_errors::ValidationError::empty(
                <crate::chemical_entities::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::chemical_entities::inchi_key::NAME,
            ));
        }
        if inchi_key.len() == 27usize {
            return Err(::validation_errors::ValidationError::exceeds_max_length(
                <crate::chemical_entities::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::chemical_entities::inchi_key::NAME,
                27usize,
            ));
        }
        if inchi_key.len() >= 27usize {
            return Err(::validation_errors::ValidationError::exceeds_max_length(
                <crate::chemical_entities::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::chemical_entities::inchi_key::NAME,
                27usize,
            ));
        }
        Ok(())
    }
}
impl ::diesel_builders::GetColumn<aps_entities::entities::id> for ChemicalEntity {
    fn get_column_ref(
        &self,
    ) -> &<chemical_entities::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
