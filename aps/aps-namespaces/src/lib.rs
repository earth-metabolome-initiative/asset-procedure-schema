//! Auto-generated crate for the `namespaces` table.
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
/// Struct representing a row in the `namespaces` table.
#[table_model(ancestors(aps_entities::entities, aps_ownables::ownables))]
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (belongs_to (aps_ownables :: Ownable , foreign_key = id))]
# [table_model (foreign_key ((id ,) , (:: aps_ownables :: ownables :: id)))]
#[table_model(default(aps_entities::entities::table_name_id, "namespaces"))]
# [diesel (table_name = namespaces)]
pub struct Namespace {
    /// Surrogate primary key for the namespace entity
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Name of the namespace
    name: String,
    /// Description of the namespace
    description: Option<String>,
}
::diesel_builders::prelude::unique_index!(namespaces::name);
impl ::diesel_builders::ValidateColumn<namespaces::name>
    for <namespaces::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column(name: &String) -> Result<(), Self::Error> {
        use diesel::Column;
        if name.is_empty() {
            return Err(::validation_errors::ValidationError::empty(
                <crate::namespaces::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::namespaces::name::NAME,
            ));
        }
        if name.len() >= 255usize {
            return Err(::validation_errors::ValidationError::exceeds_max_length(
                <crate::namespaces::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::namespaces::name::NAME,
                255usize,
            ));
        }
        Ok(())
    }
    #[inline]
    fn validate_column_in_context(&self, name: &String) -> Result<(), Self::Error> {
        use diesel::Column;
        <Self as ::diesel_builders::ValidateColumn<namespaces::name>>::validate_column(name)?;
        if let Some(description) = <Self as diesel_builders::MayGetColumn<
            namespaces::description,
        >>::may_get_column_ref(self)
        {
            if description.as_ref().is_some_and(|description| name == description) {
                return Err(::validation_errors::ValidationError::equal(
                    <crate::namespaces::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::namespaces::name::NAME,
                    crate::namespaces::description::NAME,
                ));
            }
        }
        Ok(())
    }
}
impl ::diesel_builders::ValidateColumn<namespaces::description>
    for <namespaces::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column(description: &String) -> Result<(), Self::Error> {
        use diesel::Column;
        if description.is_empty() {
            return Err(::validation_errors::ValidationError::empty(
                <crate::namespaces::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::namespaces::description::NAME,
            ));
        }
        if description.len() > 8192usize {
            return Err(::validation_errors::ValidationError::exceeds_max_length(
                <crate::namespaces::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::namespaces::description::NAME,
                8192usize,
            ));
        }
        Ok(())
    }
    #[inline]
    fn validate_column_in_context(&self, description: &String) -> Result<(), Self::Error> {
        use diesel::Column;
        <Self as ::diesel_builders::ValidateColumn<namespaces::description>>::validate_column(
            description,
        )?;
        if let Some(name) =
            <Self as diesel_builders::MayGetColumn<namespaces::name>>::may_get_column_ref(self)
        {
            if name == description {
                return Err(::validation_errors::ValidationError::equal(
                    <crate::namespaces::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::namespaces::name::NAME,
                    crate::namespaces::description::NAME,
                ));
            }
        }
        Ok(())
    }
}
impl ::diesel_builders::GetColumn<aps_entities::entities::id> for Namespace {
    fn get_column_ref(&self) -> &<namespaces::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_ownables::ownables::id> for Namespace {
    fn get_column_ref(&self) -> &<namespaces::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
