//! Auto-generated crate for the `next_procedure_templates` table.
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
/// Struct representing a row in the `next_procedure_templates` table.
#[table_model(ancestors(aps_entities::entities, aps_ownables::ownables))]
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (belongs_to (aps_ownables :: Ownable , foreign_key = id))]
# [table_model (foreign_key ((id ,) , (:: aps_ownables :: ownables :: id)))]
# [table_model (foreign_key ((parent_id ,) , (:: aps_procedure_templates :: procedure_templates :: id)))]
# [table_model (foreign_key ((predecessor_id ,) , (:: aps_procedure_templates :: procedure_templates :: id)))]
# [table_model (foreign_key ((successor_id ,) , (:: aps_procedure_templates :: procedure_templates :: id)))]
# [table_model (foreign_key ((parent_id , predecessor_id ,) , (:: aps_parent_procedure_templates :: parent_procedure_templates :: parent_id , :: aps_parent_procedure_templates :: parent_procedure_templates :: child_id)))]
# [table_model (foreign_key ((parent_id , successor_id ,) , (:: aps_parent_procedure_templates :: parent_procedure_templates :: parent_id , :: aps_parent_procedure_templates :: parent_procedure_templates :: child_id)))]
#[table_model(default(aps_entities::entities::table_name_id, "next_procedure_templates"))]
# [diesel (table_name = next_procedure_templates)]
pub struct NextProcedureTemplate {
    /// Field representing the `id` column in table `next_procedure_templates`.
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// The parent_id procedure_id template
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    parent_id: ::rosetta_uuid::Uuid,
    /// The predecessor_id procedure_id template
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    predecessor_id: ::rosetta_uuid::Uuid,
    /// The successor_id procedure_id template
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    successor_id: ::rosetta_uuid::Uuid,
}
::diesel_builders::prelude::unique_index!(
    next_procedure_templates::parent_id,
    next_procedure_templates::predecessor_id,
    next_procedure_templates::successor_id
);
impl ::diesel_builders::ValidateColumn<next_procedure_templates::parent_id>
    for <next_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column_in_context(
        &self,
        parent_id: &::rosetta_uuid::Uuid,
    ) -> Result<(), Self::Error> {
        use diesel::Column;
        if let Some(predecessor_id) = <Self as diesel_builders::MayGetColumn<
            next_procedure_templates::predecessor_id,
        >>::may_get_column_ref(self)
            && parent_id == predecessor_id
        {
            return Err(::validation_errors::ValidationError::equal(
                <crate::next_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::next_procedure_templates::parent_id::NAME,
                crate::next_procedure_templates::predecessor_id::NAME,
            ));
        }
        if let Some(successor_id) = <Self as diesel_builders::MayGetColumn<
            next_procedure_templates::successor_id,
        >>::may_get_column_ref(self)
            && parent_id == successor_id
        {
            return Err(::validation_errors::ValidationError::equal(
                <crate::next_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::next_procedure_templates::parent_id::NAME,
                crate::next_procedure_templates::successor_id::NAME,
            ));
        }
        Ok(())
    }
}
impl ::diesel_builders::ValidateColumn<next_procedure_templates::predecessor_id>
    for <next_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column_in_context(
        &self,
        predecessor_id: &::rosetta_uuid::Uuid,
    ) -> Result<(), Self::Error> {
        use diesel::Column;
        if let Some(parent_id) = <Self as diesel_builders::MayGetColumn<
            next_procedure_templates::parent_id,
        >>::may_get_column_ref(self)
            && parent_id == predecessor_id
        {
            return Err(::validation_errors::ValidationError::equal(
                <crate::next_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::next_procedure_templates::parent_id::NAME,
                crate::next_procedure_templates::predecessor_id::NAME,
            ));
        }
        if let Some(successor_id) = <Self as diesel_builders::MayGetColumn<
            next_procedure_templates::successor_id,
        >>::may_get_column_ref(self)
            && predecessor_id == successor_id
        {
            return Err(::validation_errors::ValidationError::equal(
                <crate::next_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
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
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column_in_context(
        &self,
        successor_id: &::rosetta_uuid::Uuid,
    ) -> Result<(), Self::Error> {
        use diesel::Column;
        if let Some(parent_id) = <Self as diesel_builders::MayGetColumn<
            next_procedure_templates::parent_id,
        >>::may_get_column_ref(self)
            && parent_id == successor_id
        {
            return Err(::validation_errors::ValidationError::equal(
                <crate::next_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::next_procedure_templates::parent_id::NAME,
                crate::next_procedure_templates::successor_id::NAME,
            ));
        }
        if let Some(predecessor_id) = <Self as diesel_builders::MayGetColumn<
            next_procedure_templates::predecessor_id,
        >>::may_get_column_ref(self)
            && predecessor_id == successor_id
        {
            return Err(::validation_errors::ValidationError::equal(
                <crate::next_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::next_procedure_templates::predecessor_id::NAME,
                crate::next_procedure_templates::successor_id::NAME,
            ));
        }
        Ok(())
    }
}
impl ::diesel_builders::GetColumn<aps_entities::entities::id> for NextProcedureTemplate {
    fn get_column_ref(
        &self,
    ) -> &<next_procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_ownables::ownables::id> for NextProcedureTemplate {
    fn get_column_ref(
        &self,
    ) -> &<next_procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
