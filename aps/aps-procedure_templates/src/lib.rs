//! Auto-generated crate for the `procedure_templates` table.
#[derive(
    Clone,
    Eq,
    PartialEq,
    serde :: Serialize,
    serde :: Deserialize,
    diesel :: Queryable,
    diesel :: Selectable,
    diesel :: Identifiable,
    diesel_builders :: prelude :: TableModel,
)]
/// Undocumented table
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (table_name = procedure_templates)]
pub struct ProcedureTemplate {
    /// Undocumented column
    # [table_model (default = :: rosetta_uuid :: Uuid :: new_v4 ())]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Undocumented column
    #[infallible]
    most_concrete_table: String,
    /// Undocumented column
    #[table_model(default = 1i32)]
    #[infallible]
    version: i32,
    /// Undocumented column
    name: String,
    /// Undocumented column
    description: String,
    /// Undocumented column
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    creator_id: ::rosetta_uuid::Uuid,
    /// Undocumented column
    # [table_model (default = :: rosetta_timestamp :: TimestampUTC :: default ())]
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    created_at: ::rosetta_timestamp::TimestampUTC,
    /// Undocumented column
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    editor_id: ::rosetta_uuid::Uuid,
    /// Undocumented column
    # [table_model (default = :: rosetta_timestamp :: TimestampUTC :: default ())]
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    edited_at: ::rosetta_timestamp::TimestampUTC,
    /// Undocumented column
    #[table_model(default = false)]
    #[infallible]
    deprecated: bool,
}
::diesel_builders::prelude::unique_index!(procedure_templates::name);
:: diesel_builders :: prelude :: fk ! ((procedure_templates :: creator_id) -> (:: aps_users :: users :: id));
:: diesel_builders :: prelude :: fk ! ((procedure_templates :: editor_id) -> (:: aps_users :: users :: id));
impl ::diesel_builders::ValidateColumn<procedure_templates::name>
    for <procedure_templates::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError<&'static str>;
    #[inline]
    fn validate_column(name: &String) -> Result<(), Self::Error> {
        use diesel::Column;
        if name.is_empty() {
            return Err(validation_errors::prelude::ValidationError::empty(
                crate::procedure_templates::name::NAME,
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
            && name == description
        {
            return Err(validation_errors::prelude::ValidationError::equal(
                crate::procedure_templates::name::NAME,
                crate::procedure_templates::description::NAME,
            ));
        }
        Ok(())
    }
}
impl ::diesel_builders::ValidateColumn<procedure_templates::description>
    for <procedure_templates::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError<&'static str>;
    #[inline]
    fn validate_column(description: &String) -> Result<(), Self::Error> {
        use diesel::Column;
        if description.is_empty() {
            return Err(validation_errors::prelude::ValidationError::empty(
                crate::procedure_templates::description::NAME,
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
            && name == description
        {
            return Err(validation_errors::prelude::ValidationError::equal(
                crate::procedure_templates::name::NAME,
                crate::procedure_templates::description::NAME,
            ));
        }
        Ok(())
    }
}
impl ::diesel_builders::ValidateColumn<procedure_templates::created_at>
    for <procedure_templates::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError<&'static str>;
    #[inline]
    fn validate_column_in_context(
        &self,
        created_at: &::rosetta_timestamp::TimestampUTC,
    ) -> Result<(), Self::Error> {
        use diesel::Column;
        if let Some(edited_at) = <Self as diesel_builders::MayGetColumn<
            procedure_templates::edited_at,
        >>::may_get_column_ref(self)
            && created_at > edited_at
        {
            return Err(validation_errors::prelude::ValidationError::smaller_than(
                crate::procedure_templates::created_at::NAME,
                crate::procedure_templates::edited_at::NAME,
            ));
        }
        Ok(())
    }
}
impl ::diesel_builders::ValidateColumn<procedure_templates::edited_at>
    for <procedure_templates::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError<&'static str>;
    #[inline]
    fn validate_column_in_context(
        &self,
        edited_at: &::rosetta_timestamp::TimestampUTC,
    ) -> Result<(), Self::Error> {
        use diesel::Column;
        if let Some(created_at) = <Self as diesel_builders::MayGetColumn<
            procedure_templates::created_at,
        >>::may_get_column_ref(self)
            && created_at > edited_at
        {
            return Err(validation_errors::prelude::ValidationError::smaller_than(
                crate::procedure_templates::created_at::NAME,
                crate::procedure_templates::edited_at::NAME,
            ));
        }
        Ok(())
    }
}
