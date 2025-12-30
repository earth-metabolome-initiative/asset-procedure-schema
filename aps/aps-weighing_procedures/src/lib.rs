//! Auto-generated crate for the `weighing_procedures` table.
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
#[table_model(ancestors(aps_procedures::procedures))]
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (table_name = weighing_procedures)]
pub struct WeighingProcedure {
    /// Undocumented column
    #[same_as(aps_procedures::procedures::id)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Undocumented column
    #[same_as(aps_procedures::procedures::procedure_template_id)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    weighing_procedure_template_id: ::rosetta_uuid::Uuid,
    /// Undocumented column
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    weighed_asset_id: ::rosetta_uuid::Uuid,
    /// Undocumented column
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::asset_model_id,
        procedure_weighed_asset_id
    )]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    weighed_asset_model_id: ::rosetta_uuid::Uuid,
    /// Undocumented column
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::procedure_template_asset_model_id,
        procedure_weighed_asset_id
    )]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_weighed_asset_model_id: ::rosetta_uuid::Uuid,
    /// Undocumented column
    #[discretionary(aps_procedure_asset_models::procedure_asset_models)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_weighed_asset_id: ::rosetta_uuid::Uuid,
    /// Undocumented column
    mass: f32,
    /// Undocumented column
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    weighed_with_id: Option<::rosetta_uuid::Uuid>,
    /// Undocumented column
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::asset_model_id,
        procedure_weighed_with_id
    )]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    weighed_with_model_id: ::rosetta_uuid::Uuid,
    /// Undocumented column
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::procedure_template_asset_model_id,
        procedure_weighed_with_id
    )]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_weighed_with_model_id: ::rosetta_uuid::Uuid,
    /// Undocumented column
    #[discretionary(aps_procedure_asset_models::procedure_asset_models)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_weighed_with_id: ::rosetta_uuid::Uuid,
}
:: diesel_builders :: prelude :: fk ! ((weighing_procedures :: id) -> (:: aps_procedures :: procedures :: id));
:: diesel_builders :: prelude :: fk ! ((weighing_procedures :: weighing_procedure_template_id) -> (:: aps_weighing_procedure_templates :: weighing_procedure_templates :: id));
:: diesel_builders :: prelude :: fk ! ((weighing_procedures :: weighed_asset_id) -> (:: aps_physical_assets :: physical_assets :: id));
:: diesel_builders :: prelude :: fk ! ((weighing_procedures :: weighed_asset_model_id) -> (:: aps_physical_asset_models :: physical_asset_models :: id));
:: diesel_builders :: prelude :: fk ! ((weighing_procedures :: procedure_template_weighed_asset_model_id) -> (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id));
:: diesel_builders :: prelude :: fk ! ((weighing_procedures :: weighed_with_id) -> (:: aps_weighing_devices :: weighing_devices :: id));
:: diesel_builders :: prelude :: fk ! ((weighing_procedures :: weighed_with_model_id) -> (:: aps_weighing_device_models :: weighing_device_models :: id));
:: diesel_builders :: prelude :: fk ! ((weighing_procedures :: procedure_template_weighed_with_model_id) -> (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id));
impl ::diesel_builders::ValidateColumn<weighing_procedures::mass>
    for <weighing_procedures::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError<&'static str>;
    #[inline]
    fn validate_column(mass: &f32) -> Result<(), Self::Error> {
        use diesel::Column;
        if mass <= &0f32 {
            return Err(validation_errors::prelude::ValidationError::strictly_greater_than_value(
                crate::weighing_procedures::mass::NAME,
                0f64,
            ));
        }
        Ok(())
    }
}
