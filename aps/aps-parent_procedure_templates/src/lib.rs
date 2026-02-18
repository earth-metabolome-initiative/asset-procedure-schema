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
#[table_model(ancestors(aps_entities::entities, aps_ownables::ownables))]
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (belongs_to (aps_ownables :: Ownable , foreign_key = id))]
# [table_model (foreign_key ((id ,) , (:: aps_ownables :: ownables :: id)))]
# [table_model (foreign_key ((parent_id ,) , (:: aps_procedure_templates :: procedure_templates :: id)))]
# [table_model (foreign_key ((child_id ,) , (:: aps_procedure_templates :: procedure_templates :: id)))]
#[table_model(default(aps_entities::entities::table_name_id, "parent_procedure_templates"))]
# [diesel (table_name = parent_procedure_templates)]
pub struct ParentProcedureTemplate {
    /// Field representing the `id` column in table
    /// `parent_procedure_templates`.
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// The parent_id procedure_id template
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    parent_id: ::rosetta_uuid::Uuid,
    /// The child_id procedure_id template
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    child_id: ::rosetta_uuid::Uuid,
}
::diesel_builders::prelude::unique_index!(
    parent_procedure_templates::parent_id,
    parent_procedure_templates::child_id
);
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
impl ::diesel_builders::GetColumn<aps_entities::entities::id> for ParentProcedureTemplate {
    fn get_column_ref(
        &self,
    ) -> &<parent_procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_ownables::ownables::id> for ParentProcedureTemplate {
    fn get_column_ref(
        &self,
    ) -> &<parent_procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
