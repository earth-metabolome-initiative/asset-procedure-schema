//! Auto-generated crate for the `next_procedure_templates` table.
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
#[diesel(primary_key(parent_id, predecessor_id, successor_id))]
# [diesel (table_name = next_procedure_templates)]
pub struct NextProcedureTemplate {
    /// Undocumented column
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    parent_id: ::rosetta_uuid::Uuid,
    /// Undocumented column
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    predecessor_id: ::rosetta_uuid::Uuid,
    /// Undocumented column
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    successor_id: ::rosetta_uuid::Uuid,
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
:: diesel_builders :: prelude :: fk ! ((next_procedure_templates :: parent_id) -> (:: aps_procedure_templates :: procedure_templates :: id));
:: diesel_builders :: prelude :: fk ! ((next_procedure_templates :: predecessor_id) -> (:: aps_procedure_templates :: procedure_templates :: id));
:: diesel_builders :: prelude :: fk ! ((next_procedure_templates :: successor_id) -> (:: aps_procedure_templates :: procedure_templates :: id));
:: diesel_builders :: prelude :: fk ! ((next_procedure_templates :: creator_id) -> (:: aps_users :: users :: id));
:: diesel_builders :: prelude :: fk ! ((next_procedure_templates :: parent_id , next_procedure_templates :: predecessor_id) -> (:: aps_parent_procedure_templates :: parent_procedure_templates :: parent_id , :: aps_parent_procedure_templates :: parent_procedure_templates :: child_id));
:: diesel_builders :: prelude :: fk ! ((next_procedure_templates :: parent_id , next_procedure_templates :: successor_id) -> (:: aps_parent_procedure_templates :: parent_procedure_templates :: parent_id , :: aps_parent_procedure_templates :: parent_procedure_templates :: child_id));
impl ::diesel_builders::ValidateColumn<next_procedure_templates::predecessor_id>
    for <next_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError<&'static str>;
    #[inline]
    fn validate_column_in_context(
        &self,
        predecessor_id: &::rosetta_uuid::Uuid,
    ) -> Result<(), Self::Error> {
        use diesel::Column;
        if let Some(successor_id) = <Self as diesel_builders::MayGetColumn<
            next_procedure_templates::successor_id,
        >>::may_get_column_ref(self)
            && predecessor_id == successor_id
        {
            return Err(validation_errors::prelude::ValidationError::equal(
                crate::next_procedure_templates::predecessor_id::NAME,
                crate::next_procedure_templates::successor_id::NAME,
            ));
        }
        Ok(())
    }
}
impl ::diesel_builders::ValidateColumn<next_procedure_templates::successor_id>
    for <next_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError<&'static str>;
    #[inline]
    fn validate_column_in_context(
        &self,
        successor_id: &::rosetta_uuid::Uuid,
    ) -> Result<(), Self::Error> {
        use diesel::Column;
        if let Some(predecessor_id) = <Self as diesel_builders::MayGetColumn<
            next_procedure_templates::predecessor_id,
        >>::may_get_column_ref(self)
            && predecessor_id == successor_id
        {
            return Err(validation_errors::prelude::ValidationError::equal(
                crate::next_procedure_templates::predecessor_id::NAME,
                crate::next_procedure_templates::successor_id::NAME,
            ));
        }
        Ok(())
    }
}
