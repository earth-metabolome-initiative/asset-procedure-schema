//! Auto-generated crate for the `procedure_templates` table.
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
/// Struct representing a row in the `procedure_templates` table.
#[table_model(ancestors(aps_entities::entities, aps_ownables::ownables))]
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (belongs_to (aps_ownables :: Ownable , foreign_key = id))]
# [table_model (foreign_key ((id ,) , (:: aps_ownables :: ownables :: id)))]
#[table_model(default(aps_entities::entities::table_name_id, "procedure_templates"))]
# [diesel (table_name = procedure_templates)]
pub struct ProcedureTemplate {
    /// Identifier of the procedure_id template
    # [table_model (default = :: rosetta_uuid :: Uuid :: utc_v7 ())]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Version of the procedure_id template.
    #[table_model(default = 1i32)]
    #[infallible]
    version: i32,
    /// Human-readable name of the procedure_id template
    name: String,
    /// Human-readable description of the procedure_id template
    description: String,
    /// Whether this procedure_id template is deprecated and should not be used
    /// for new procedures
    #[table_model(default = false)]
    #[infallible]
    deprecated: bool,
}
::diesel_builders::prelude::unique_index!(procedure_templates::name);
impl ::diesel_builders::ValidateColumn<procedure_templates::name>
    for <procedure_templates::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column(name: &String) -> Result<(), Self::Error> {
        use diesel::Column;
        if name.is_empty() {
            return Err(::validation_errors::ValidationError::empty(
                <crate::procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::procedure_templates::name::NAME,
            ));
        }
        if name.len() > 255usize {
            return Err(::validation_errors::ValidationError::exceeds_max_length(
                <crate::procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::procedure_templates::name::NAME,
                255usize,
            ));
        }
        Ok(())
    }
    #[inline]
    fn validate_column_in_context(&self, name: &String) -> Result<(), Self::Error> {
        use diesel::Column;
        <Self as ::diesel_builders::ValidateColumn<procedure_templates::name>>::validate_column(
            name,
        )?;
        if let Some(description) = <Self as diesel_builders::MayGetColumn<
            procedure_templates::description,
        >>::may_get_column_ref(self)
        {
            if name == description {
                return Err(::validation_errors::ValidationError::equal(
                    <crate::procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::procedure_templates::name::NAME,
                    crate::procedure_templates::description::NAME,
                ));
            }
        }
        Ok(())
    }
}
impl ::diesel_builders::ValidateColumn<procedure_templates::description>
    for <procedure_templates::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column(description: &String) -> Result<(), Self::Error> {
        use diesel::Column;
        if description.is_empty() {
            return Err(::validation_errors::ValidationError::empty(
                <crate::procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::procedure_templates::description::NAME,
            ));
        }
        if description.len() > 8192usize {
            return Err(::validation_errors::ValidationError::exceeds_max_length(
                <crate::procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::procedure_templates::description::NAME,
                8192usize,
            ));
        }
        Ok(())
    }
    #[inline]
    fn validate_column_in_context(&self, description: &String) -> Result<(), Self::Error> {
        use diesel::Column;
        < Self as :: diesel_builders :: ValidateColumn < procedure_templates :: description >> :: validate_column (description ,) ? ;
        if let Some(name) =
            <Self as diesel_builders::MayGetColumn<procedure_templates::name>>::may_get_column_ref(
                self,
            )
        {
            if name == description {
                return Err(::validation_errors::ValidationError::equal(
                    <crate::procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::procedure_templates::name::NAME,
                    crate::procedure_templates::description::NAME,
                ));
            }
        }
        Ok(())
    }
}
impl ::diesel_builders::GetColumn<aps_entities::entities::id> for ProcedureTemplate {
    fn get_column_ref(
        &self,
    ) -> &<procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_ownables::ownables::id> for ProcedureTemplate {
    fn get_column_ref(
        &self,
    ) -> &<procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
