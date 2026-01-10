//! Auto-generated crate for the `pouring_procedure_templates` table.
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
/// Struct representing a row in the `pouring_procedure_templates` table.
#[table_model(ancestors(aps_procedure_templates::procedure_templates))]
# [table_model (error = :: validation_errors :: ValidationError)]
#[table_model(default(
    aps_procedure_templates::procedure_templates::procedure_template_table_id,
    "pouring_procedure_templates"
))]
# [diesel (table_name = pouring_procedure_templates)]
pub struct PouringProcedureTemplate {
    /// Field representing the `id` column in table
    /// `pouring_procedure_templates`.
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `measured_with_model_id` column in table
    /// `pouring_procedure_templates`.
    #[same_as(
        aps_procedure_template_asset_models::procedure_template_asset_models::asset_model_id,
        procedure_template_measured_with_model_id
    )]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    measured_with_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_template_measured_with_model_id`
    /// column in table `pouring_procedure_templates`.
    #[discretionary(aps_procedure_template_asset_models::procedure_template_asset_models)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_measured_with_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `poured_from_model_id` column in table
    /// `pouring_procedure_templates`.
    #[same_as(
        aps_procedure_template_asset_models::procedure_template_asset_models::asset_model_id,
        procedure_template_poured_from_model_id
    )]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    poured_from_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_template_poured_from_model_id` column
    /// in table `pouring_procedure_templates`.
    #[discretionary(aps_procedure_template_asset_models::procedure_template_asset_models)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_poured_from_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `poured_into_model_id` column in table
    /// `pouring_procedure_templates`.
    #[same_as(
        aps_procedure_template_asset_models::procedure_template_asset_models::asset_model_id,
        procedure_template_poured_into_model_id
    )]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    poured_into_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_template_poured_into_model_id` column
    /// in table `pouring_procedure_templates`.
    #[discretionary(aps_procedure_template_asset_models::procedure_template_asset_models)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_poured_into_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `volume` column in table
    /// `pouring_procedure_templates`.
    volume: f32,
}
::diesel_builders::prelude::unique_index!(
    pouring_procedure_templates::id,
    pouring_procedure_templates::procedure_template_measured_with_model_id
);
::diesel_builders::prelude::unique_index!(
    pouring_procedure_templates::id,
    pouring_procedure_templates::procedure_template_poured_from_model_id
);
::diesel_builders::prelude::unique_index!(
    pouring_procedure_templates::id,
    pouring_procedure_templates::procedure_template_poured_into_model_id
);
:: diesel_builders :: prelude :: fk ! ((pouring_procedure_templates :: id) -> (:: aps_procedure_templates :: procedure_templates :: id));
:: diesel_builders :: prelude :: fk ! ((pouring_procedure_templates :: measured_with_model_id) -> (:: aps_volume_measuring_device_models :: volume_measuring_device_models :: id));
:: diesel_builders :: prelude :: fk ! ((pouring_procedure_templates :: poured_from_model_id) -> (:: aps_volumetric_container_models :: volumetric_container_models :: id));
:: diesel_builders :: prelude :: fk ! ((pouring_procedure_templates :: poured_into_model_id) -> (:: aps_volumetric_container_models :: volumetric_container_models :: id));
:: diesel_builders :: prelude :: fk ! ((pouring_procedure_templates :: id , pouring_procedure_templates :: procedure_template_measured_with_model_id) -> (:: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_id , :: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_asset_model_id));
:: diesel_builders :: prelude :: fk ! ((pouring_procedure_templates :: id , pouring_procedure_templates :: procedure_template_poured_into_model_id) -> (:: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_id , :: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_asset_model_id));
:: diesel_builders :: prelude :: fk ! ((pouring_procedure_templates :: id , pouring_procedure_templates :: procedure_template_poured_from_model_id) -> (:: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_id , :: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_asset_model_id));
impl ::diesel_builders::ValidateColumn<pouring_procedure_templates::volume>
    for <pouring_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column(volume: &f32) -> Result<(), Self::Error> {
        use diesel::Column;
        if volume <= &0f32 {
            return Err(::validation_errors::ValidationError::strictly_greater_than_value(
                "pouring_procedure_templates",
                crate::pouring_procedure_templates::volume::NAME,
                0f64,
            ));
        }
        Ok(())
    }
}
impl diesel_builders::GetColumn<aps_procedure_templates::procedure_templates::id>
    for PouringProcedureTemplate
{
    fn get_column_ref(
        &self,
    ) -> &<pouring_procedure_templates::id as diesel_builders::Typed>::ColumnType {
        &self.id
    }
}
