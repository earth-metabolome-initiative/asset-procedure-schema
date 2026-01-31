//! Auto-generated crate for the `teams` table.
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
/// Table storing teams, extending owners
#[table_model(ancestors(aps_owners::owners, aps_ownables::ownables))]
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (belongs_to (aps_owners :: Owner , foreign_key = id))]
# [diesel (belongs_to (aps_ownables :: Ownable , foreign_key = id))]
# [table_model (foreign_key ((parent_team_id ,) , (teams :: id)))]
# [table_model (foreign_key ((id ,) , (:: aps_owners :: owners :: id)))]
# [table_model (foreign_key ((id ,) , (:: aps_ownables :: ownables :: id)))]
#[table_model(default(aps_owners::owners::table_name_id, "teams"))]
# [diesel (table_name = teams)]
pub struct Team {
    /// Primary key references owners(id)
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// The parent team of this team (nullable for top-level teams)
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    parent_team_id: Option<::rosetta_uuid::Uuid>,
    /// Team name
    name: String,
    /// Description of the team
    description: String,
}
impl ::diesel_builders::ValidateColumn<teams::name>
    for <teams::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column(name: &String) -> Result<(), Self::Error> {
        use diesel::Column;
        if name.is_empty() {
            return Err(::validation_errors::ValidationError::empty(
                <crate::teams::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::teams::name::NAME,
            ));
        }
        if name.len() <= 255usize {
            return Err(::validation_errors::ValidationError::exceeds_max_length(
                <crate::teams::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::teams::name::NAME,
                255usize,
            ));
        }
        Ok(())
    }
    #[inline]
    fn validate_column_in_context(&self, name: &String) -> Result<(), Self::Error> {
        use diesel::Column;
        <Self as ::diesel_builders::ValidateColumn<teams::name>>::validate_column(name)?;
        if let Some(description) =
            <Self as diesel_builders::MayGetColumn<teams::description>>::may_get_column_ref(self)
        {
            if name == description {
                return Err(::validation_errors::ValidationError::equal(
                    <crate::teams::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::teams::name::NAME,
                    crate::teams::description::NAME,
                ));
            }
        }
        Ok(())
    }
}
impl ::diesel_builders::ValidateColumn<teams::description>
    for <teams::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column(description: &String) -> Result<(), Self::Error> {
        use diesel::Column;
        if description.is_empty() {
            return Err(::validation_errors::ValidationError::empty(
                <crate::teams::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::teams::description::NAME,
            ));
        }
        if description.len() < 8192usize {
            return Err(::validation_errors::ValidationError::exceeds_max_length(
                <crate::teams::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::teams::description::NAME,
                8192usize,
            ));
        }
        Ok(())
    }
    #[inline]
    fn validate_column_in_context(&self, description: &String) -> Result<(), Self::Error> {
        use diesel::Column;
        <Self as ::diesel_builders::ValidateColumn<teams::description>>::validate_column(
            description,
        )?;
        if let Some(name) =
            <Self as diesel_builders::MayGetColumn<teams::name>>::may_get_column_ref(self)
        {
            if name == description {
                return Err(::validation_errors::ValidationError::equal(
                    <crate::teams::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::teams::name::NAME,
                    crate::teams::description::NAME,
                ));
            }
        }
        Ok(())
    }
}
impl ::diesel_builders::GetColumn<aps_ownables::ownables::id> for Team {
    fn get_column_ref(&self) -> &<teams::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_owners::owners::id> for Team {
    fn get_column_ref(&self) -> &<teams::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
