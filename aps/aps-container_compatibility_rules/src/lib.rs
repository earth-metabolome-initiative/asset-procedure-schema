//! Auto-generated crate for the `container_compatibility_rules` table.
#[derive(
    Copy,
    Clone,
    Debug,
    Hash,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    serde :: Serialize,
    serde :: Deserialize,
    diesel :: Queryable,
    diesel :: Selectable,
    diesel :: Identifiable,
    diesel_builders :: prelude :: TableModel,
)]
/// Struct representing a row in the `container_compatibility_rules` table.
# [table_model (error = :: validation_errors :: ValidationError)]
#[diesel(primary_key(container_model_id, contained_asset_model_id))]
# [diesel (table_name = container_compatibility_rules)]
pub struct ContainerCompatibilityRule {
    /// Field representing the `container_model_id` column in table
    /// `container_compatibility_rules`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    container_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `contained_asset_model_id` column in table
    /// `container_compatibility_rules`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    contained_asset_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `quantity` column in table
    /// `container_compatibility_rules`.
    #[table_model(default = 1i16)]
    quantity: Option<i16>,
    /// Field representing the `created_by_id` column in table
    /// `container_compatibility_rules`.
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    created_by_id: ::rosetta_uuid::Uuid,
    /// Field representing the `created_at` column in table
    /// `container_compatibility_rules`.
    # [table_model (default = :: rosetta_timestamp :: TimestampUTC :: default ())]
    #[infallible]
    # [diesel (sql_type = :: rosetta_timestamp :: diesel_impls :: TimestampUTC)]
    created_at: ::rosetta_timestamp::TimestampUTC,
}
:: diesel_builders :: prelude :: fk ! ((container_compatibility_rules :: container_model_id) -> (:: aps_container_models :: container_models :: id));
:: diesel_builders :: prelude :: fk ! ((container_compatibility_rules :: contained_asset_model_id) -> (:: aps_physical_asset_models :: physical_asset_models :: id));
:: diesel_builders :: prelude :: fk ! ((container_compatibility_rules :: created_by_id) -> (:: aps_users :: users :: id));
impl ::diesel_builders::ValidateColumn<container_compatibility_rules::container_model_id>
    for <container_compatibility_rules::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column_in_context(
        &self,
        container_model_id: &::rosetta_uuid::Uuid,
    ) -> Result<(), Self::Error> {
        use diesel::Column;
        if let Some(contained_asset_model_id) = <Self as diesel_builders::MayGetColumn<
            container_compatibility_rules::contained_asset_model_id,
        >>::may_get_column_ref(self)
            && container_model_id == contained_asset_model_id
        {
            return Err(::validation_errors::ValidationError::equal(
                "container_compatibility_rules",
                crate::container_compatibility_rules::container_model_id::NAME,
                crate::container_compatibility_rules::contained_asset_model_id::NAME,
            ));
        }
        Ok(())
    }
}
impl ::diesel_builders::ValidateColumn<container_compatibility_rules::contained_asset_model_id>
    for <container_compatibility_rules::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column_in_context(
        &self,
        contained_asset_model_id: &::rosetta_uuid::Uuid,
    ) -> Result<(), Self::Error> {
        use diesel::Column;
        if let Some(container_model_id) = <Self as diesel_builders::MayGetColumn<
            container_compatibility_rules::container_model_id,
        >>::may_get_column_ref(self)
            && container_model_id == contained_asset_model_id
        {
            return Err(::validation_errors::ValidationError::equal(
                "container_compatibility_rules",
                crate::container_compatibility_rules::container_model_id::NAME,
                crate::container_compatibility_rules::contained_asset_model_id::NAME,
            ));
        }
        Ok(())
    }
}
impl ::diesel_builders::ValidateColumn<container_compatibility_rules::quantity>
    for <container_compatibility_rules::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column(quantity: &i16) -> Result<(), Self::Error> {
        use diesel::Column;
        if quantity <= &0i16 {
            return Err(::validation_errors::ValidationError::strictly_greater_than_value(
                "container_compatibility_rules",
                crate::container_compatibility_rules::quantity::NAME,
                0f64,
            ));
        }
        Ok(())
    }
}
