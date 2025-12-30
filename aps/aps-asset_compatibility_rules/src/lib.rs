//! Auto-generated crate for the `asset_compatibility_rules` table.
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
#[diesel(primary_key(left_asset_model_id, right_asset_model_id))]
# [diesel (table_name = asset_compatibility_rules)]
pub struct AssetCompatibilityRule {
    /// Undocumented column
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    left_asset_model_id: ::rosetta_uuid::Uuid,
    /// Undocumented column
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    right_asset_model_id: ::rosetta_uuid::Uuid,
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
:: diesel_builders :: prelude :: fk ! ((asset_compatibility_rules :: left_asset_model_id) -> (:: aps_asset_models :: asset_models :: id));
:: diesel_builders :: prelude :: fk ! ((asset_compatibility_rules :: right_asset_model_id) -> (:: aps_asset_models :: asset_models :: id));
:: diesel_builders :: prelude :: fk ! ((asset_compatibility_rules :: creator_id) -> (:: aps_users :: users :: id));
impl ::diesel_builders::ValidateColumn<asset_compatibility_rules::left_asset_model_id>
    for <asset_compatibility_rules::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError<&'static str>;
    #[inline]
    fn validate_column_in_context(
        &self,
        left_asset_model_id: &::rosetta_uuid::Uuid,
    ) -> Result<(), Self::Error> {
        use diesel::Column;
        if let Some(right_asset_model_id) = <Self as diesel_builders::MayGetColumn<
            asset_compatibility_rules::right_asset_model_id,
        >>::may_get_column_ref(self)
        {
            if left_asset_model_id == right_asset_model_id {
                return Err(validation_errors::prelude::ValidationError::equal(
                    crate::asset_compatibility_rules::left_asset_model_id::NAME,
                    crate::asset_compatibility_rules::right_asset_model_id::NAME,
                ));
            }
        }
        Ok(())
    }
}
impl ::diesel_builders::ValidateColumn<asset_compatibility_rules::right_asset_model_id>
    for <asset_compatibility_rules::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError<&'static str>;
    #[inline]
    fn validate_column_in_context(
        &self,
        right_asset_model_id: &::rosetta_uuid::Uuid,
    ) -> Result<(), Self::Error> {
        use diesel::Column;
        if let Some(left_asset_model_id) = <Self as diesel_builders::MayGetColumn<
            asset_compatibility_rules::left_asset_model_id,
        >>::may_get_column_ref(self)
        {
            if left_asset_model_id == right_asset_model_id {
                return Err(validation_errors::prelude::ValidationError::equal(
                    crate::asset_compatibility_rules::left_asset_model_id::NAME,
                    crate::asset_compatibility_rules::right_asset_model_id::NAME,
                ));
            }
        }
        Ok(())
    }
}
