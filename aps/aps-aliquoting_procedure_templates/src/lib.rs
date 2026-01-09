//! Auto-generated crate for the `aliquoting_procedure_templates` table.
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
/// Struct representing a row in the `aliquoting_procedure_templates` table.
#[table_model(ancestors(aps_procedure_templates::procedure_templates))]
# [table_model (error = :: validation_errors :: ValidationError)]
#[table_model(default(
    aps_procedure_templates::procedure_templates::procedure_template_table_id,
    "aliquoting_procedure_templates"
))]
# [diesel (table_name = aliquoting_procedure_templates)]
pub struct AliquotingProcedureTemplate {
    /// Field representing the `id` column in table
    /// `aliquoting_procedure_templates`.
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `volume` column in table
    /// `aliquoting_procedure_templates`.
    volume: f32,
    /// Field representing the `aliquoted_from_model_id` column in table
    /// `aliquoting_procedure_templates`.
    #[same_as(
        aps_procedure_template_asset_models::procedure_template_asset_models::asset_model_id,
        procedure_template_aliquoted_from_model_id
    )]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    aliquoted_from_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_template_aliquoted_from_model_id`
    /// column in table `aliquoting_procedure_templates`.
    #[discretionary(aps_procedure_template_asset_models::procedure_template_asset_models)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_aliquoted_from_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `aliquoted_into_model_id` column in table
    /// `aliquoting_procedure_templates`.
    #[same_as(
        aps_procedure_template_asset_models::procedure_template_asset_models::asset_model_id,
        procedure_template_aliquoted_into_model_id
    )]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    aliquoted_into_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_template_aliquoted_into_model_id`
    /// column in table `aliquoting_procedure_templates`.
    #[discretionary(aps_procedure_template_asset_models::procedure_template_asset_models)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_aliquoted_into_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `aliquoted_with_model_id` column in table
    /// `aliquoting_procedure_templates`.
    #[same_as(
        aps_procedure_template_asset_models::procedure_template_asset_models::asset_model_id,
        procedure_template_aliquoted_with_model_id
    )]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    aliquoted_with_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_template_aliquoted_with_model_id`
    /// column in table `aliquoting_procedure_templates`.
    #[discretionary(aps_procedure_template_asset_models::procedure_template_asset_models)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_aliquoted_with_model_id: ::rosetta_uuid::Uuid,
}
::diesel_builders::prelude::unique_index!(
    aliquoting_procedure_templates::id,
    aliquoting_procedure_templates::procedure_template_aliquoted_from_model_id
);
::diesel_builders::prelude::unique_index!(
    aliquoting_procedure_templates::id,
    aliquoting_procedure_templates::procedure_template_aliquoted_into_model_id
);
::diesel_builders::prelude::unique_index!(
    aliquoting_procedure_templates::id,
    aliquoting_procedure_templates::procedure_template_aliquoted_with_model_id
);
:: diesel_builders :: prelude :: fk ! ((aliquoting_procedure_templates :: id) -> (:: aps_procedure_templates :: procedure_templates :: id));
:: diesel_builders :: prelude :: fk ! ((aliquoting_procedure_templates :: aliquoted_from_model_id) -> (:: aps_volumetric_container_models :: volumetric_container_models :: id));
:: diesel_builders :: prelude :: fk ! ((aliquoting_procedure_templates :: aliquoted_into_model_id) -> (:: aps_volumetric_container_models :: volumetric_container_models :: id));
:: diesel_builders :: prelude :: fk ! ((aliquoting_procedure_templates :: aliquoted_with_model_id) -> (:: aps_volume_measuring_device_models :: volume_measuring_device_models :: id));
impl ::diesel_builders::ValidateColumn<aliquoting_procedure_templates::volume>
    for <aliquoting_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError<&'static str>;
    #[inline]
    fn validate_column(volume: &f32) -> Result<(), Self::Error> {
        use diesel::Column;
        if volume <= &0f32 {
            return Err(validation_errors::prelude::ValidationError::strictly_greater_than_value(
                crate::aliquoting_procedure_templates::volume::NAME,
                0f64,
            ));
        }
        Ok(())
    }
}
