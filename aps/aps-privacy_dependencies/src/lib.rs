//! Auto-generated crate for the `privacy_dependencies` table.
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
    :: diesel_builders :: prelude :: TableModel,
)]
/// Used to enforce privacy inheritance: Child.minimum_role_id >=
/// Parent.minimum_role_id
# [table_model (error = :: validation_errors :: ValidationError)]
#[diesel(primary_key(child_id, parent_id))]
# [table_model (foreign_key ((child_id ,) , (:: aps_entities :: entities :: id)))]
# [table_model (foreign_key ((parent_id ,) , (:: aps_entities :: entities :: id)))]
# [diesel (table_name = privacy_dependencies)]
pub struct PrivacyDependency {
    /// Field representing the `child_id` column in table
    /// `privacy_dependencies`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    child_id: ::rosetta_uuid::Uuid,
    /// Field representing the `parent_id` column in table
    /// `privacy_dependencies`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    parent_id: ::rosetta_uuid::Uuid,
}
impl ::diesel_builders::ValidateColumn<privacy_dependencies::child_id>
    for <privacy_dependencies::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column_in_context(
        &self,
        child_id: &::rosetta_uuid::Uuid,
    ) -> Result<(), Self::Error> {
        use diesel::Column;
        if let Some(parent_id) = <Self as diesel_builders::MayGetColumn<
            privacy_dependencies::parent_id,
        >>::may_get_column_ref(self)
        {
            if child_id == parent_id {
                return Err(::validation_errors::ValidationError::equal(
                    <crate::privacy_dependencies::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::privacy_dependencies::child_id::NAME,
                    crate::privacy_dependencies::parent_id::NAME,
                ));
            }
        }
        Ok(())
    }
}
impl ::diesel_builders::ValidateColumn<privacy_dependencies::parent_id>
    for <privacy_dependencies::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column_in_context(
        &self,
        parent_id: &::rosetta_uuid::Uuid,
    ) -> Result<(), Self::Error> {
        use diesel::Column;
        if let Some(child_id) = <Self as diesel_builders::MayGetColumn<
            privacy_dependencies::child_id,
        >>::may_get_column_ref(self)
        {
            if child_id == parent_id {
                return Err(::validation_errors::ValidationError::equal(
                    <crate::privacy_dependencies::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::privacy_dependencies::child_id::NAME,
                    crate::privacy_dependencies::parent_id::NAME,
                ));
            }
        }
        Ok(())
    }
}
