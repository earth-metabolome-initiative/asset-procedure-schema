//! Auto-generated crate for the `volumetric_container_models` table.
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
# [diesel (table_name = volumetric_container_models)]
pub struct VolumetricContainerModel {
    /// Undocumented column
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Undocumented column
    volume: f32,
}
:: diesel_builders :: prelude :: fk ! ((volumetric_container_models :: id) -> (:: aps_container_models :: container_models :: id));
impl ::diesel_builders::ValidateColumn<volumetric_container_models::volume>
    for <volumetric_container_models::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError<&'static str>;
    #[inline]
    fn validate_column(volume: &f32) -> Result<(), Self::Error> {
        use diesel::Column;
        if volume <= &0f32 {
            return Err(validation_errors::prelude::ValidationError::strictly_greater_than_value(
                crate::volumetric_container_models::volume::NAME,
                0f64,
            ));
        }
        Ok(())
    }
}
