//! Auto-generated crate for the `parent_procedure_templates` table.
#[derive(
    Copy,
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
/// Struct representing a row in the `parent_procedure_templates` table.
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (belongs_to (aps_users :: User , foreign_key = creator_id))]
#[diesel(primary_key(parent_id, child_id))]
# [table_model (foreign_key ((parent_id ,) , (:: aps_procedure_templates :: procedure_templates :: id)))]
# [table_model (foreign_key ((child_id ,) , (:: aps_procedure_templates :: procedure_templates :: id)))]
# [table_model (foreign_key ((creator_id ,) , (:: aps_users :: users :: id)))]
# [diesel (table_name = parent_procedure_templates)]
pub struct ParentProcedureTemplate {
    /// The parent_id procedure_id template
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    parent_id: ::rosetta_uuid::Uuid,
    /// The child_id procedure_id template
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    child_id: ::rosetta_uuid::Uuid,
    /// The user who created this relationship
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    creator_id: ::rosetta_uuid::Uuid,
    /// The timestamp when this relationship was created
    # [table_model (default = :: rosetta_timestamp :: TimestampUTC :: default ())]
    #[infallible]
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    created_at: ::rosetta_timestamp::TimestampUTC,
}
impl ::diesel_builders::ValidateColumn<parent_procedure_templates::parent_id>
    for <parent_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
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
                return Err (:: validation_errors :: ValidationError :: equal (< crate :: parent_procedure_templates :: table as :: diesel_builders :: TableExt > :: TABLE_NAME , crate :: parent_procedure_templates :: parent_id :: NAME , crate :: parent_procedure_templates :: child_id :: NAME)) ;
            }
        }
        Ok(())
    }
}
impl ::diesel_builders::ValidateColumn<parent_procedure_templates::child_id>
    for <parent_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
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
                return Err (:: validation_errors :: ValidationError :: equal (< crate :: parent_procedure_templates :: table as :: diesel_builders :: TableExt > :: TABLE_NAME , crate :: parent_procedure_templates :: parent_id :: NAME , crate :: parent_procedure_templates :: child_id :: NAME)) ;
            }
        }
        Ok(())
    }
}
