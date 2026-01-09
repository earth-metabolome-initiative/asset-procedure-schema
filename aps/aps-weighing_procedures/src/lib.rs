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
/// Struct representing a row in the `weighing_procedures` table.
#[table_model(ancestors(aps_procedures::procedures))]
# [table_model (error = :: validation_errors :: ValidationError)]
#[table_model(default(aps_procedures::procedures::procedure_table_id, "weighing_procedures"))]
# [diesel (table_name = weighing_procedures)]
pub struct WeighingProcedure {
    /// Field representing the `id` column in table `weighing_procedures`.
    #[same_as(aps_procedures::procedures::id)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `weighing_procedure_template_id` column in table
    /// `weighing_procedures`.
    #[same_as(aps_procedures::procedures::procedure_template_id)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    weighing_procedure_template_id: ::rosetta_uuid::Uuid,
    /// Field representing the `weighed_asset_id` column in table
    /// `weighing_procedures`.
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    weighed_asset_id: ::rosetta_uuid::Uuid,
    /// Field representing the `weighed_asset_model_id` column in table
    /// `weighing_procedures`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::asset_model_id,
        procedure_weighed_asset_id
    )]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    weighed_asset_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_template_weighed_asset_model_id`
    /// column in table `weighing_procedures`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::procedure_template_asset_model_id,
        procedure_weighed_asset_id
    )]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_weighed_asset_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_weighed_asset_id` column in table
    /// `weighing_procedures`.
    #[discretionary(aps_procedure_asset_models::procedure_asset_models)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_weighed_asset_id: ::rosetta_uuid::Uuid,
    /// Field representing the `mass` column in table `weighing_procedures`.
    mass: f32,
    /// Field representing the `weighed_with_id` column in table
    /// `weighing_procedures`.
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    weighed_with_id: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `weighed_with_model_id` column in table
    /// `weighing_procedures`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::asset_model_id,
        procedure_weighed_with_id
    )]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    weighed_with_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_template_weighed_with_model_id` column
    /// in table `weighing_procedures`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::procedure_template_asset_model_id,
        procedure_weighed_with_id
    )]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_weighed_with_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_weighed_with_id` column in table
    /// `weighing_procedures`.
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
