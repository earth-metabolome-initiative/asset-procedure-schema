//! Auto-generated crate for the `weighed_container_models` table.
#[derive(
    Copy,
    Clone,
    Debug,
    PartialOrd,
    PartialEq,
    serde :: Serialize,
    serde :: Deserialize,
    diesel :: Queryable,
    diesel :: Selectable,
    diesel :: Identifiable,
    diesel_builders :: prelude :: TableModel,
)]
/// Undocumented table
#[table_model(ancestors(
    aps_asset_models::asset_models,
    aps_physical_asset_models::physical_asset_models,
    aps_container_models::container_models
))]
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (table_name = weighed_container_models)]
pub struct WeighedContainerModel {
    /// Undocumented column
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Undocumented column
    mass: f32,
}
:: diesel_builders :: prelude :: fk ! ((weighed_container_models :: id) -> (:: aps_container_models :: container_models :: id));
impl ::diesel_builders::ValidateColumn<weighed_container_models::mass>
    for <weighed_container_models::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError<&'static str>;
    #[inline]
    fn validate_column(mass: &f32) -> Result<(), Self::Error> {
        use diesel::Column;
        if mass <= &0f32 {
            return Err(validation_errors::prelude::ValidationError::strictly_greater_than_value(
                crate::weighed_container_models::mass::NAME,
                0f64,
            ));
        }
        Ok(())
    }
}
