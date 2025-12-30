//! Auto-generated crate for the `parent_procedure_templates` table.
#[derive(
    Copy,
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
#[diesel(primary_key(parent_id, child_id))]
# [diesel (table_name = parent_procedure_templates)]
pub struct ParentProcedureTemplate {
    /// Undocumented column
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    parent_id: ::rosetta_uuid::Uuid,
    /// Undocumented column
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    child_id: ::rosetta_uuid::Uuid,
    /// Undocumented column
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    creator_id: ::rosetta_uuid::Uuid,
    /// Undocumented column
    # [table_model (default = :: rosetta_timestamp :: TimestampUTC :: default ())]
    #[infallible]
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    created_at: ::rosetta_timestamp::TimestampUTC,
}
:: diesel_builders :: prelude :: fk ! ((parent_procedure_templates :: parent_id) -> (:: aps_procedure_templates :: procedure_templates :: id));
:: diesel_builders :: prelude :: fk ! ((parent_procedure_templates :: child_id) -> (:: aps_procedure_templates :: procedure_templates :: id));
:: diesel_builders :: prelude :: fk ! ((parent_procedure_templates :: creator_id) -> (:: aps_users :: users :: id));
impl ::diesel_builders::ValidateColumn<parent_procedure_templates::parent_id>
    for <parent_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError<&'static str>;
    #[inline]
    fn validate_column_in_context(
        &self,
        parent_id: &::rosetta_uuid::Uuid,
    ) -> Result<(), Self::Error> {
        use diesel::Column;
        if let Some(child_id) = <Self as diesel_builders::MayGetColumn<
            parent_procedure_templates::child_id,
        >>::may_get_column_ref(self)
        {
            if parent_id == child_id {
                return Err(validation_errors::prelude::ValidationError::equal(
                    crate::parent_procedure_templates::parent_id::NAME,
                    crate::parent_procedure_templates::child_id::NAME,
                ));
            }
        }
        Ok(())
    }
}
impl ::diesel_builders::ValidateColumn<parent_procedure_templates::child_id>
    for <parent_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError<&'static str>;
    #[inline]
    fn validate_column_in_context(
        &self,
        child_id: &::rosetta_uuid::Uuid,
    ) -> Result<(), Self::Error> {
        use diesel::Column;
        if let Some(parent_id) = <Self as diesel_builders::MayGetColumn<
            parent_procedure_templates::parent_id,
        >>::may_get_column_ref(self)
        {
            if parent_id == child_id {
                return Err(validation_errors::prelude::ValidationError::equal(
                    crate::parent_procedure_templates::parent_id::NAME,
                    crate::parent_procedure_templates::child_id::NAME,
                ));
            }
        }
        Ok(())
    }
}
