//! Auto-generated crate for the `namespaced_ownables` table.
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
/// Struct representing a row in the `namespaced_ownables` table.
#[table_model(ancestors(aps_entities::entities, aps_ownables::ownables))]
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (belongs_to (aps_ownables :: Ownable , foreign_key = id))]
# [diesel (belongs_to (aps_namespaces :: Namespace , foreign_key = namespace_id))]
# [table_model (foreign_key ((id ,) , (:: aps_ownables :: ownables :: id)))]
# [table_model (foreign_key ((namespace_id ,) , (:: aps_namespaces :: namespaces :: id)))]
#[table_model(default(aps_entities::entities::table_name_id, "namespaced_ownables"))]
# [diesel (table_name = namespaced_ownables)]
pub struct NamespacedOwnable {
    /// Field representing the `id` column in table `namespaced_ownables`.
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `namespace_id` column in table
    /// `namespaced_ownables`.
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    namespace_id: ::rosetta_uuid::Uuid,
    /// Field representing the `name` column in table `namespaced_ownables`.
    name: String,
    /// Field representing the `description` column in table
    /// `namespaced_ownables`.
    description: String,
}
::diesel_builders::prelude::unique_index!(
    namespaced_ownables::namespace_id,
    namespaced_ownables::name
);
impl ::diesel_builders::ValidateColumn<namespaced_ownables::name>
    for <namespaced_ownables::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column(name: &String) -> Result<(), Self::Error> {
        use diesel::Column;
        if name.is_empty() {
            return Err(::validation_errors::ValidationError::empty(
                <crate::namespaced_ownables::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::namespaced_ownables::name::NAME,
            ));
        }
        if name.len() >= 255usize {
            return Err(::validation_errors::ValidationError::exceeds_max_length(
                <crate::namespaced_ownables::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::namespaced_ownables::name::NAME,
                255usize,
            ));
        }
        Ok(())
    }
    #[inline]
    fn validate_column_in_context(&self, name: &String) -> Result<(), Self::Error> {
        use diesel::Column;
        <Self as ::diesel_builders::ValidateColumn<namespaced_ownables::name>>::validate_column(
            name,
        )?;
        if let Some(description) = <Self as diesel_builders::MayGetColumn<
            namespaced_ownables::description,
        >>::may_get_column_ref(self)
            && name == description
        {
            return Err(::validation_errors::ValidationError::equal(
                <crate::namespaced_ownables::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::namespaced_ownables::name::NAME,
                crate::namespaced_ownables::description::NAME,
            ));
        }
        Ok(())
    }
}
impl ::diesel_builders::ValidateColumn<namespaced_ownables::description>
    for <namespaced_ownables::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column(description: &String) -> Result<(), Self::Error> {
        use diesel::Column;
        if description.is_empty() {
            return Err(::validation_errors::ValidationError::empty(
                <crate::namespaced_ownables::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::namespaced_ownables::description::NAME,
            ));
        }
        if description.len() > 8192usize {
            return Err(::validation_errors::ValidationError::exceeds_max_length(
                <crate::namespaced_ownables::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::namespaced_ownables::description::NAME,
                8192usize,
            ));
        }
        Ok(())
    }
    #[inline]
    fn validate_column_in_context(&self, description: &String) -> Result<(), Self::Error> {
        use diesel::Column;
        < Self as :: diesel_builders :: ValidateColumn < namespaced_ownables :: description >> :: validate_column (description ,) ? ;
        if let Some(name) =
            <Self as diesel_builders::MayGetColumn<namespaced_ownables::name>>::may_get_column_ref(
                self,
            )
            && name == description
        {
            return Err(::validation_errors::ValidationError::equal(
                <crate::namespaced_ownables::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::namespaced_ownables::name::NAME,
                crate::namespaced_ownables::description::NAME,
            ));
        }
        Ok(())
    }
}
impl ::diesel_builders::GetColumn<aps_entities::entities::id> for NamespacedOwnable {
    fn get_column_ref(
        &self,
    ) -> &<namespaced_ownables::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_ownables::ownables::id> for NamespacedOwnable {
    fn get_column_ref(
        &self,
    ) -> &<namespaced_ownables::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
