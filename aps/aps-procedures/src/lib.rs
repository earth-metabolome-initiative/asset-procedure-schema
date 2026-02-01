//! Auto-generated crate for the `procedures` table.
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
/// Struct representing a row in the `procedures` table.
#[table_model(ancestors(aps_entities::entities, aps_ownables::ownables))]
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (belongs_to (aps_ownables :: Ownable , foreign_key = id))]
# [table_model (foreign_key ((id ,) , (:: aps_ownables :: ownables :: id)))]
# [table_model (foreign_key ((procedure_template_id ,) , (:: aps_procedure_templates :: procedure_templates :: id)))]
# [table_model (foreign_key ((parent_procedure_id ,) , (procedures :: id)))]
# [table_model (foreign_key ((parent_procedure_template_id ,) , (:: aps_procedure_templates :: procedure_templates :: id)))]
# [table_model (foreign_key ((predecessor_procedure_id ,) , (procedures :: id)))]
# [table_model (foreign_key ((predecessor_procedure_template_id ,) , (:: aps_procedure_templates :: procedure_templates :: id)))]
# [table_model (foreign_key ((parent_procedure_id , parent_procedure_template_id ,) , (procedures :: id , procedures :: procedure_template_id)))]
# [table_model (foreign_key ((predecessor_procedure_id , predecessor_procedure_template_id ,) , (procedures :: id , procedures :: procedure_template_id)))]
# [table_model (foreign_key ((parent_procedure_template_id , procedure_template_id ,) , (:: aps_parent_procedure_templates :: parent_procedure_templates :: parent_id , :: aps_parent_procedure_templates :: parent_procedure_templates :: child_id)))]
# [table_model (foreign_key ((parent_procedure_template_id , predecessor_procedure_template_id , procedure_template_id ,) , (:: aps_next_procedure_templates :: next_procedure_templates :: parent_id , :: aps_next_procedure_templates :: next_procedure_templates :: predecessor_id , :: aps_next_procedure_templates :: next_procedure_templates :: successor_id)))]
#[table_model(default(aps_entities::entities::table_name_id, "procedures"))]
# [diesel (table_name = procedures)]
pub struct Procedure {
    /// The ID of this procedure.
    # [table_model (default = :: rosetta_uuid :: Uuid :: utc_v7 ())]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// The procedure_id template of this procedure.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_id: ::rosetta_uuid::Uuid,
    /// The parent_id procedure_id (if any) of this procedure.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    parent_procedure_id: Option<::rosetta_uuid::Uuid>,
    /// The parent_id procedure_id template (if any) of this procedure.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    parent_procedure_template_id: Option<::rosetta_uuid::Uuid>,
    /// The predecessor_id procedure_id (if any) of this procedure.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    predecessor_procedure_id: Option<::rosetta_uuid::Uuid>,
    /// The predecessor_id procedure_id template (if any) of this procedure.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    predecessor_procedure_template_id: Option<::rosetta_uuid::Uuid>,
}
::diesel_builders::prelude::unique_index!(procedures::id, procedures::procedure_template_id);
impl ::diesel_builders::ValidateColumn<procedures::id>
    for <procedures::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column_in_context(&self, id: &::rosetta_uuid::Uuid) -> Result<(), Self::Error> {
        use diesel::Column;
        if let Some(parent_procedure_id) = <Self as diesel_builders::MayGetColumn<
            procedures::parent_procedure_id,
        >>::may_get_column_ref(self)
            && parent_procedure_id
                .as_ref()
                .is_some_and(|parent_procedure_id| id == parent_procedure_id)
        {
            return Err(::validation_errors::ValidationError::equal(
                <crate::procedures::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::procedures::id::NAME,
                crate::procedures::parent_procedure_id::NAME,
            ));
        }
        if let Some(predecessor_procedure_id) = <Self as diesel_builders::MayGetColumn<
            procedures::predecessor_procedure_id,
        >>::may_get_column_ref(self)
            && predecessor_procedure_id
                .as_ref()
                .is_some_and(|predecessor_procedure_id| id == predecessor_procedure_id)
        {
            return Err(::validation_errors::ValidationError::equal(
                <crate::procedures::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::procedures::id::NAME,
                crate::procedures::predecessor_procedure_id::NAME,
            ));
        }
        Ok(())
    }
}
impl ::diesel_builders::ValidateColumn<procedures::procedure_template_id>
    for <procedures::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column_in_context(
        &self,
        procedure_template_id: &::rosetta_uuid::Uuid,
    ) -> Result<(), Self::Error> {
        use diesel::Column;
        if let Some(parent_procedure_template_id) = <Self as diesel_builders::MayGetColumn<
            procedures::parent_procedure_template_id,
        >>::may_get_column_ref(self)
            && parent_procedure_template_id.as_ref().is_some_and(|parent_procedure_template_id| {
                procedure_template_id == parent_procedure_template_id
            })
        {
            return Err(::validation_errors::ValidationError::equal(
                <crate::procedures::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::procedures::procedure_template_id::NAME,
                crate::procedures::parent_procedure_template_id::NAME,
            ));
        }
        if let Some(predecessor_procedure_template_id) = <Self as diesel_builders::MayGetColumn<
            procedures::predecessor_procedure_template_id,
        >>::may_get_column_ref(self)
            && predecessor_procedure_template_id.as_ref().is_some_and(
                |predecessor_procedure_template_id| {
                    procedure_template_id == predecessor_procedure_template_id
                },
            )
        {
            return Err(::validation_errors::ValidationError::equal(
                <crate::procedures::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::procedures::procedure_template_id::NAME,
                crate::procedures::predecessor_procedure_template_id::NAME,
            ));
        }
        Ok(())
    }
}
impl ::diesel_builders::ValidateColumn<procedures::parent_procedure_id>
    for <procedures::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column_in_context(
        &self,
        parent_procedure_id: &::rosetta_uuid::Uuid,
    ) -> Result<(), Self::Error> {
        use diesel::Column;
        if let Some(id) =
            <Self as diesel_builders::MayGetColumn<procedures::id>>::may_get_column_ref(self)
            && id == parent_procedure_id
        {
            return Err(::validation_errors::ValidationError::equal(
                <crate::procedures::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::procedures::id::NAME,
                crate::procedures::parent_procedure_id::NAME,
            ));
        }
        Ok(())
    }
}
impl ::diesel_builders::ValidateColumn<procedures::parent_procedure_template_id>
    for <procedures::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column_in_context(
        &self,
        parent_procedure_template_id: &::rosetta_uuid::Uuid,
    ) -> Result<(), Self::Error> {
        use diesel::Column;
        if let Some(procedure_template_id) = <Self as diesel_builders::MayGetColumn<
            procedures::procedure_template_id,
        >>::may_get_column_ref(self)
            && procedure_template_id == parent_procedure_template_id
        {
            return Err(::validation_errors::ValidationError::equal(
                <crate::procedures::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::procedures::procedure_template_id::NAME,
                crate::procedures::parent_procedure_template_id::NAME,
            ));
        }
        Ok(())
    }
}
impl ::diesel_builders::ValidateColumn<procedures::predecessor_procedure_id>
    for <procedures::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column_in_context(
        &self,
        predecessor_procedure_id: &::rosetta_uuid::Uuid,
    ) -> Result<(), Self::Error> {
        use diesel::Column;
        if let Some(id) =
            <Self as diesel_builders::MayGetColumn<procedures::id>>::may_get_column_ref(self)
            && id == predecessor_procedure_id
        {
            return Err(::validation_errors::ValidationError::equal(
                <crate::procedures::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::procedures::id::NAME,
                crate::procedures::predecessor_procedure_id::NAME,
            ));
        }
        Ok(())
    }
}
impl ::diesel_builders::ValidateColumn<procedures::predecessor_procedure_template_id>
    for <procedures::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column_in_context(
        &self,
        predecessor_procedure_template_id: &::rosetta_uuid::Uuid,
    ) -> Result<(), Self::Error> {
        use diesel::Column;
        if let Some(procedure_template_id) = <Self as diesel_builders::MayGetColumn<
            procedures::procedure_template_id,
        >>::may_get_column_ref(self)
            && procedure_template_id == predecessor_procedure_template_id
        {
            return Err(::validation_errors::ValidationError::equal(
                <crate::procedures::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::procedures::procedure_template_id::NAME,
                crate::procedures::predecessor_procedure_template_id::NAME,
            ));
        }
        Ok(())
    }
}
impl ::diesel_builders::GetColumn<aps_entities::entities::id> for Procedure {
    fn get_column_ref(&self) -> &<procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_ownables::ownables::id> for Procedure {
    fn get_column_ref(&self) -> &<procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl procedure_like::ProcedureTableLike for procedures::table {
    type ProcedureTemplateTable = aps_procedure_templates::procedure_templates::table;
}
