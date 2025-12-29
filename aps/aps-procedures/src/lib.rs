//! Auto-generated crate for the `procedures` table.
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
# [diesel (table_name = procedures)]
pub struct Procedure {
    /// Undocumented column
    # [table_model (default = :: rosetta_uuid :: Uuid :: new_v4 ())]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Undocumented column
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_id: ::rosetta_uuid::Uuid,
    /// Undocumented column
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    parent_procedure_id: Option<::rosetta_uuid::Uuid>,
    /// Undocumented column
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    parent_procedure_template_id: Option<::rosetta_uuid::Uuid>,
    /// Undocumented column
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    predecessor_procedure_id: Option<::rosetta_uuid::Uuid>,
    /// Undocumented column
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    predecessor_procedure_template_id: Option<::rosetta_uuid::Uuid>,
    /// Undocumented column
    #[infallible]
    most_concrete_table: String,
    /// Undocumented column
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    created_by_id: ::rosetta_uuid::Uuid,
    /// Undocumented column
    # [table_model (default = :: rosetta_timestamp :: TimestampUTC :: default ())]
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    created_at: ::rosetta_timestamp::TimestampUTC,
    /// Undocumented column
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    updated_by_id: ::rosetta_uuid::Uuid,
    /// Undocumented column
    # [table_model (default = :: rosetta_timestamp :: TimestampUTC :: default ())]
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    updated_at: ::rosetta_timestamp::TimestampUTC,
}
::diesel_builders::prelude::unique_index!(procedures::id, procedures::procedure_template_id);
:: diesel_builders :: prelude :: fk ! ((procedures :: procedure_template_id) -> (:: aps_procedure_templates :: procedure_templates :: id));
:: diesel_builders :: prelude :: fk ! ((procedures :: parent_procedure_id) -> (procedures :: id));
:: diesel_builders :: prelude :: fk ! ((procedures :: parent_procedure_template_id) -> (:: aps_procedure_templates :: procedure_templates :: id));
:: diesel_builders :: prelude :: fk ! ((procedures :: predecessor_procedure_id) -> (procedures :: id));
:: diesel_builders :: prelude :: fk ! ((procedures :: predecessor_procedure_template_id) -> (:: aps_procedure_templates :: procedure_templates :: id));
:: diesel_builders :: prelude :: fk ! ((procedures :: created_by_id) -> (:: aps_users :: users :: id));
:: diesel_builders :: prelude :: fk ! ((procedures :: updated_by_id) -> (:: aps_users :: users :: id));
:: diesel_builders :: prelude :: fk ! ((procedures :: parent_procedure_id , procedures :: parent_procedure_template_id) -> (procedures :: id , procedures :: procedure_template_id));
:: diesel_builders :: prelude :: fk ! ((procedures :: predecessor_procedure_id , procedures :: predecessor_procedure_template_id) -> (procedures :: id , procedures :: procedure_template_id));
:: diesel_builders :: prelude :: fk ! ((procedures :: parent_procedure_template_id , procedures :: procedure_template_id) -> (:: aps_parent_procedure_templates :: parent_procedure_templates :: parent_id , :: aps_parent_procedure_templates :: parent_procedure_templates :: child_id));
:: diesel_builders :: prelude :: fk ! ((procedures :: parent_procedure_template_id , procedures :: predecessor_procedure_template_id , procedures :: procedure_template_id) -> (:: aps_next_procedure_templates :: next_procedure_templates :: parent_id , :: aps_next_procedure_templates :: next_procedure_templates :: predecessor_id , :: aps_next_procedure_templates :: next_procedure_templates :: successor_id));
impl ::diesel_builders::ValidateColumn<procedures::id>
    for <procedures::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError<&'static str>;
    #[inline]
    fn validate_column_in_context(&self, id: &::rosetta_uuid::Uuid) -> Result<(), Self::Error> {
        use diesel::Column;
        if let Some(parent_procedure_id) = <Self as diesel_builders::MayGetColumn<
            procedures::parent_procedure_id,
        >>::may_get_column_ref(self)
        {
            if parent_procedure_id
                .as_ref()
                .is_some_and(|parent_procedure_id| id == parent_procedure_id)
            {
                return Err(validation_errors::prelude::ValidationError::equal(
                    crate::procedures::id::NAME,
                    crate::procedures::parent_procedure_id::NAME,
                ));
            }
        }
        if let Some(predecessor_procedure_id) = <Self as diesel_builders::MayGetColumn<
            procedures::predecessor_procedure_id,
        >>::may_get_column_ref(self)
        {
            if predecessor_procedure_id
                .as_ref()
                .is_some_and(|predecessor_procedure_id| id == predecessor_procedure_id)
            {
                return Err(validation_errors::prelude::ValidationError::equal(
                    crate::procedures::id::NAME,
                    crate::procedures::predecessor_procedure_id::NAME,
                ));
            }
        }
        Ok(())
    }
}
impl ::diesel_builders::ValidateColumn<procedures::procedure_template_id>
    for <procedures::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError<&'static str>;
    #[inline]
    fn validate_column_in_context(
        &self,
        procedure_template_id: &::rosetta_uuid::Uuid,
    ) -> Result<(), Self::Error> {
        use diesel::Column;
        if let Some(parent_procedure_template_id) = <Self as diesel_builders::MayGetColumn<
            procedures::parent_procedure_template_id,
        >>::may_get_column_ref(self)
        {
            if parent_procedure_template_id.as_ref().is_some_and(|parent_procedure_template_id| {
                procedure_template_id == parent_procedure_template_id
            }) {
                return Err(validation_errors::prelude::ValidationError::equal(
                    crate::procedures::procedure_template_id::NAME,
                    crate::procedures::parent_procedure_template_id::NAME,
                ));
            }
        }
        if let Some(predecessor_procedure_template_id) = <Self as diesel_builders::MayGetColumn<
            procedures::predecessor_procedure_template_id,
        >>::may_get_column_ref(self)
        {
            if predecessor_procedure_template_id.as_ref().is_some_and(
                |predecessor_procedure_template_id| {
                    procedure_template_id == predecessor_procedure_template_id
                },
            ) {
                return Err(validation_errors::prelude::ValidationError::equal(
                    crate::procedures::procedure_template_id::NAME,
                    crate::procedures::predecessor_procedure_template_id::NAME,
                ));
            }
        }
        Ok(())
    }
}
impl ::diesel_builders::ValidateColumn<procedures::parent_procedure_id>
    for <procedures::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError<&'static str>;
    #[inline]
    fn validate_column_in_context(
        &self,
        parent_procedure_id: &::rosetta_uuid::Uuid,
    ) -> Result<(), Self::Error> {
        use diesel::Column;
        if let Some(id) =
            <Self as diesel_builders::MayGetColumn<procedures::id>>::may_get_column_ref(self)
        {
            if id == parent_procedure_id {
                return Err(validation_errors::prelude::ValidationError::equal(
                    crate::procedures::id::NAME,
                    crate::procedures::parent_procedure_id::NAME,
                ));
            }
        }
        Ok(())
    }
}
impl ::diesel_builders::ValidateColumn<procedures::parent_procedure_template_id>
    for <procedures::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError<&'static str>;
    #[inline]
    fn validate_column_in_context(
        &self,
        parent_procedure_template_id: &::rosetta_uuid::Uuid,
    ) -> Result<(), Self::Error> {
        use diesel::Column;
        if let Some(procedure_template_id) = <Self as diesel_builders::MayGetColumn<
            procedures::procedure_template_id,
        >>::may_get_column_ref(self)
        {
            if procedure_template_id == parent_procedure_template_id {
                return Err(validation_errors::prelude::ValidationError::equal(
                    crate::procedures::procedure_template_id::NAME,
                    crate::procedures::parent_procedure_template_id::NAME,
                ));
            }
        }
        Ok(())
    }
}
impl ::diesel_builders::ValidateColumn<procedures::predecessor_procedure_id>
    for <procedures::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError<&'static str>;
    #[inline]
    fn validate_column_in_context(
        &self,
        predecessor_procedure_id: &::rosetta_uuid::Uuid,
    ) -> Result<(), Self::Error> {
        use diesel::Column;
        if let Some(id) =
            <Self as diesel_builders::MayGetColumn<procedures::id>>::may_get_column_ref(self)
        {
            if id == predecessor_procedure_id {
                return Err(validation_errors::prelude::ValidationError::equal(
                    crate::procedures::id::NAME,
                    crate::procedures::predecessor_procedure_id::NAME,
                ));
            }
        }
        Ok(())
    }
}
impl ::diesel_builders::ValidateColumn<procedures::predecessor_procedure_template_id>
    for <procedures::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError<&'static str>;
    #[inline]
    fn validate_column_in_context(
        &self,
        predecessor_procedure_template_id: &::rosetta_uuid::Uuid,
    ) -> Result<(), Self::Error> {
        use diesel::Column;
        if let Some(procedure_template_id) = <Self as diesel_builders::MayGetColumn<
            procedures::procedure_template_id,
        >>::may_get_column_ref(self)
        {
            if procedure_template_id == predecessor_procedure_template_id {
                return Err(validation_errors::prelude::ValidationError::equal(
                    crate::procedures::procedure_template_id::NAME,
                    crate::procedures::predecessor_procedure_template_id::NAME,
                ));
            }
        }
        Ok(())
    }
}
impl ::diesel_builders::ValidateColumn<procedures::created_at>
    for <procedures::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError<&'static str>;
    #[inline]
    fn validate_column_in_context(
        &self,
        created_at: &::rosetta_timestamp::TimestampUTC,
    ) -> Result<(), Self::Error> {
        use diesel::Column;
        if let Some(updated_at) =
            <Self as diesel_builders::MayGetColumn<procedures::updated_at>>::may_get_column_ref(
                self,
            )
        {
            if created_at > updated_at {
                return Err(validation_errors::prelude::ValidationError::smaller_than(
                    crate::procedures::created_at::NAME,
                    crate::procedures::updated_at::NAME,
                ));
            }
        }
        Ok(())
    }
}
impl ::diesel_builders::ValidateColumn<procedures::updated_at>
    for <procedures::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError<&'static str>;
    #[inline]
    fn validate_column_in_context(
        &self,
        updated_at: &::rosetta_timestamp::TimestampUTC,
    ) -> Result<(), Self::Error> {
        use diesel::Column;
        if let Some(created_at) =
            <Self as diesel_builders::MayGetColumn<procedures::created_at>>::may_get_column_ref(
                self,
            )
        {
            if created_at > updated_at {
                return Err(validation_errors::prelude::ValidationError::smaller_than(
                    crate::procedures::created_at::NAME,
                    crate::procedures::updated_at::NAME,
                ));
            }
        }
        Ok(())
    }
}
