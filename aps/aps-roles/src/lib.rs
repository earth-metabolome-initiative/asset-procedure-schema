//! Auto-generated crate for the `roles` table.
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
    :: diesel_builders :: prelude :: TableModel,
)]
/// Roles
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (table_name = roles)]
pub struct Role {
    /// Field representing the `id` column in table `roles`.
    #[infallible]
    id: i16,
    /// Field representing the `name` column in table `roles`.
    name: String,
}
::diesel_builders::prelude::unique_index!(roles::name);
impl ::diesel_builders::ValidateColumn<roles::name>
    for <roles::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column(name: &String) -> Result<(), Self::Error> {
        use diesel::Column;
        if name.is_empty() {
            return Err(::validation_errors::ValidationError::empty(
                <crate::roles::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::roles::name::NAME,
            ));
        }
        if name.len() < 255usize {
            return Err(::validation_errors::ValidationError::exceeds_max_length(
                <crate::roles::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::roles::name::NAME,
                255usize,
            ));
        }
        Ok(())
    }
}
