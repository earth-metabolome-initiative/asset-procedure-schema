//! Auto-generated crate for the `owner_grants` table.
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
/// Grants
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (belongs_to (aps_roles :: Role , foreign_key = role_id))]
#[diesel(primary_key(grantee_owner_id, granted_owner_id))]
# [table_model (foreign_key ((grantee_owner_id ,) , (:: aps_owners :: owners :: id)))]
# [table_model (foreign_key ((granted_owner_id ,) , (:: aps_owners :: owners :: id)))]
# [table_model (foreign_key ((role_id ,) , (:: aps_roles :: roles :: id)))]
# [diesel (table_name = owner_grants)]
pub struct OwnerGrant {
    /// Field representing the `grantee_owner_id` column in table
    /// `owner_grants`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    grantee_owner_id: ::rosetta_uuid::Uuid,
    /// Field representing the `granted_owner_id` column in table
    /// `owner_grants`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    granted_owner_id: ::rosetta_uuid::Uuid,
    /// Field representing the `role_id` column in table `owner_grants`.
    #[infallible]
    role_id: i16,
}
impl ::diesel_builders::ValidateColumn<owner_grants::grantee_owner_id>
    for <owner_grants::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column_in_context(
        &self,
        grantee_owner_id: &::rosetta_uuid::Uuid,
    ) -> Result<(), Self::Error> {
        use diesel::Column;
        if let Some(granted_owner_id) = <Self as diesel_builders::MayGetColumn<
            owner_grants::granted_owner_id,
        >>::may_get_column_ref(self)
        {
            if grantee_owner_id == granted_owner_id {
                return Err(::validation_errors::ValidationError::equal(
                    <crate::owner_grants::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::owner_grants::grantee_owner_id::NAME,
                    crate::owner_grants::granted_owner_id::NAME,
                ));
            }
        }
        Ok(())
    }
}
impl ::diesel_builders::ValidateColumn<owner_grants::granted_owner_id>
    for <owner_grants::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column_in_context(
        &self,
        granted_owner_id: &::rosetta_uuid::Uuid,
    ) -> Result<(), Self::Error> {
        use diesel::Column;
        if let Some(grantee_owner_id) = <Self as diesel_builders::MayGetColumn<
            owner_grants::grantee_owner_id,
        >>::may_get_column_ref(self)
        {
            if grantee_owner_id == granted_owner_id {
                return Err(::validation_errors::ValidationError::equal(
                    <crate::owner_grants::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::owner_grants::grantee_owner_id::NAME,
                    crate::owner_grants::granted_owner_id::NAME,
                ));
            }
        }
        Ok(())
    }
}
