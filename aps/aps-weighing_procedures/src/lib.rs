//! Auto-generated crate for the `weighing_procedures` table.
#[derive(
    Copy,
    Clone,
    Debug,
    PartialOrd,
    PartialEq,
    :: serde :: Serialize,
    :: serde :: Deserialize,
    :: diesel :: Queryable,
    :: diesel :: Selectable,
    :: diesel :: Identifiable,
    :: diesel :: Associations,
    :: diesel_builders :: prelude :: TableModel,
)]
/// Struct representing a row in the `weighing_procedures` table.
#[table_model(ancestors(aps_procedures::procedures))]
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (belongs_to (aps_physical_assets :: PhysicalAsset , foreign_key = weighed_asset_id))]
# [diesel (belongs_to (aps_physical_asset_models :: PhysicalAssetModel , foreign_key = weighed_asset_model_id))]
# [diesel (belongs_to (aps_weighing_devices :: WeighingDevice , foreign_key = weighed_with_id))]
# [diesel (belongs_to (aps_weighing_device_models :: WeighingDeviceModel , foreign_key = weighed_with_model_id))]
# [table_model (foreign_key ((id ,) , (:: aps_procedures :: procedures :: id)))]
# [table_model (foreign_key ((weighing_procedure_template_id ,) , (:: aps_weighing_procedure_templates :: weighing_procedure_templates :: id)))]
# [table_model (foreign_key ((weighed_asset_id ,) , (:: aps_physical_assets :: physical_assets :: id)))]
# [table_model (foreign_key ((weighed_asset_model_id ,) , (:: aps_physical_asset_models :: physical_asset_models :: id)))]
# [table_model (foreign_key ((procedure_template_weighed_asset_model_id ,) , (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id)))]
# [table_model (foreign_key ((weighed_with_id ,) , (:: aps_weighing_devices :: weighing_devices :: id)))]
# [table_model (foreign_key ((weighed_with_model_id ,) , (:: aps_weighing_device_models :: weighing_device_models :: id)))]
# [table_model (foreign_key ((procedure_template_weighed_with_model_id ,) , (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id)))]
#[table_model(default(aps_procedures::procedures::procedure_table_id, "weighing_procedures"))]
# [diesel (table_name = weighing_procedures)]
pub struct WeighingProcedure {
    /// Field representing the `id` column in table `weighing_procedures`.
    #[same_as(aps_procedures::procedures::id)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// We enforce that the model of this procedure_id must be a weighing
    /// procedure_id template.
    #[same_as(aps_procedures::procedures::procedure_template_id)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    weighing_procedure_template_id: ::rosetta_uuid::Uuid,
    /// The asset being weighed.
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    weighed_asset_id: ::rosetta_uuid::Uuid,
    /// The model of the container being weighed.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::asset_model_id,
        procedure_weighed_asset_id
    )]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    weighed_asset_model_id: ::rosetta_uuid::Uuid,
    /// The procedure_id template asset model associated to the
    /// `weighed_container`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::procedure_template_asset_model_id,
        procedure_weighed_asset_id
    )]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_weighed_asset_model_id: ::rosetta_uuid::Uuid,
    /// The procedure_id asset associated to the `weighed_container`.
    #[discretionary(aps_procedure_asset_models::procedure_asset_models)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_weighed_asset_id: ::rosetta_uuid::Uuid,
    /// Mass in kilograms. The measured weight, which must be strictly positive.
    mass: f32,
    /// are several situations where the weighing device is not tracked.
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    weighed_with_id: Option<::rosetta_uuid::Uuid>,
    /// The model of the weighing device used for weighing.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::asset_model_id,
        procedure_weighed_with_id
    )]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    weighed_with_model_id: ::rosetta_uuid::Uuid,
    /// The procedure_id template asset model associated to the
    /// `weighed_with_model`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::procedure_template_asset_model_id,
        procedure_weighed_with_id
    )]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_weighed_with_model_id: ::rosetta_uuid::Uuid,
    /// The procedure_id asset associated to the `weighed_with_model`.
    #[discretionary(aps_procedure_asset_models::procedure_asset_models)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_weighed_with_id: ::rosetta_uuid::Uuid,
}
impl ::diesel_builders::ValidateColumn<weighing_procedures::mass>
    for <weighing_procedures::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column(mass: &f32) -> Result<(), Self::Error> {
        use diesel::Column;
        if mass <= &0f32 {
            return Err(::validation_errors::ValidationError::strictly_greater_than_value(
                "weighing_procedures",
                crate::weighing_procedures::mass::NAME,
                0f64,
            ));
        }
        Ok(())
    }
}
impl ::diesel_builders::GetColumn<aps_procedures::procedures::id> for WeighingProcedure {
    fn get_column_ref(
        &self,
    ) -> &<weighing_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl procedure_like::ProcedureTableLike for weighing_procedures::table {
    type ProcedureTemplateTable =
        aps_weighing_procedure_templates::weighing_procedure_templates::table;
}
