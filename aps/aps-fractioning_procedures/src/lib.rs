//! Auto-generated crate for the `fractioning_procedures` table.
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
/// Struct representing a row in the `fractioning_procedures` table.
#[table_model(ancestors(
    aps_entities::entities,
    aps_ownables::ownables,
    aps_procedures::procedures
))]
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (belongs_to (aps_weighing_devices :: WeighingDevice , foreign_key = weighed_with_id))]
# [diesel (belongs_to (aps_weighing_device_models :: WeighingDeviceModel , foreign_key = weighed_with_model_id))]
# [table_model (foreign_key ((id ,) , (:: aps_procedures :: procedures :: id)))]
# [table_model (foreign_key ((fractioning_procedure_template_id ,) , (:: aps_fractioning_procedure_templates :: fractioning_procedure_templates :: id)))]
# [table_model (foreign_key ((fragment_container_id ,) , (:: aps_volumetric_containers :: volumetric_containers :: id)))]
# [table_model (foreign_key ((fragment_container_model_id ,) , (:: aps_volumetric_container_models :: volumetric_container_models :: id)))]
# [table_model (foreign_key ((procedure_template_fragment_container_model_id ,) , (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id)))]
# [table_model (foreign_key ((fragment_placed_into_id ,) , (:: aps_volumetric_containers :: volumetric_containers :: id)))]
# [table_model (foreign_key ((fragment_placed_into_model_id ,) , (:: aps_volumetric_container_models :: volumetric_container_models :: id)))]
# [table_model (foreign_key ((procedure_template_fragment_placed_into_model_id ,) , (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id)))]
# [table_model (foreign_key ((weighed_with_id ,) , (:: aps_weighing_devices :: weighing_devices :: id)))]
# [table_model (foreign_key ((weighed_with_model_id ,) , (:: aps_weighing_device_models :: weighing_device_models :: id)))]
# [table_model (foreign_key ((procedure_template_weighed_with_model_id ,) , (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id)))]
#[table_model(default(aps_entities::entities::table_name_id, "fractioning_procedures"))]
# [diesel (table_name = fractioning_procedures)]
pub struct FractioningProcedure {
    /// Identifier of the fractioning id, which is also a foreign key to the
    /// general procedure.
    #[same_as(aps_procedures::procedures::id)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// The template of this procedure_id should be a fractioning procedure_id
    /// template.
    #[same_as(aps_procedures::procedures::procedure_template_id)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    fractioning_procedure_template_id: ::rosetta_uuid::Uuid,
    /// The source container from which the fraction is taken, which must be a
    /// volumetric container model.
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    fragment_container_id: ::rosetta_uuid::Uuid,
    /// The model of the source container from which the fraction is taken.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::asset_model_id,
        procedure_fragment_container_id
    )]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    fragment_container_model_id: ::rosetta_uuid::Uuid,
    /// The procedure_id template asset model associated to the
    /// `fragment_container`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::procedure_template_asset_model_id,
        procedure_fragment_container_id
    )]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_fragment_container_model_id: ::rosetta_uuid::Uuid,
    /// The procedure_id asset associated to the `fragment_container`.
    #[discretionary(aps_procedure_asset_models::procedure_asset_models)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_fragment_container_id: ::rosetta_uuid::Uuid,
    /// The destination container to which the fraction is transferred, which
    /// must be a volumetric container model.
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    fragment_placed_into_id: ::rosetta_uuid::Uuid,
    /// The model of the destination container to which the fraction is
    /// transferred.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::asset_model_id,
        procedure_fragment_placed_into_id
    )]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    fragment_placed_into_model_id: ::rosetta_uuid::Uuid,
    /// The procedure_id template asset model associated to the
    /// `fragment_placed_into`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::procedure_template_asset_model_id,
        procedure_fragment_placed_into_id
    )]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_fragment_placed_into_model_id: ::rosetta_uuid::Uuid,
    /// The procedure_id asset associated to the `fragment_placed_into`.
    #[discretionary(aps_procedure_asset_models::procedure_asset_models)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_fragment_placed_into_id: ::rosetta_uuid::Uuid,
    /// Mass in kilograms. The actual amount of the fraction collected.
    mass: f32,
    /// as the weighing device might not necessarily be tracked.
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    weighed_with_id: Option<::rosetta_uuid::Uuid>,
    /// The model of the scale used to measure the fraction mass.
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
    /// The procedure_id asset associated to the `weighed_with`.
    #[discretionary(aps_procedure_asset_models::procedure_asset_models)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_weighed_with_id: ::rosetta_uuid::Uuid,
}
impl ::diesel_builders::ValidateColumn<fractioning_procedures::mass>
    for <fractioning_procedures::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column(mass: &f32) -> Result<(), Self::Error> {
        use diesel::Column;
        if mass <= &0f32 {
            return Err(::validation_errors::ValidationError::strictly_greater_than_value(
                <crate::fractioning_procedures::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::fractioning_procedures::mass::NAME,
                0f64,
            ));
        }
        Ok(())
    }
}
impl ::diesel_builders::GetColumn<aps_entities::entities::id> for FractioningProcedure {
    fn get_column_ref(
        &self,
    ) -> &<fractioning_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_ownables::ownables::id> for FractioningProcedure {
    fn get_column_ref(
        &self,
    ) -> &<fractioning_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_procedures::procedures::id> for FractioningProcedure {
    fn get_column_ref(
        &self,
    ) -> &<fractioning_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl procedure_like::ProcedureTableLike for fractioning_procedures::table {
    type ProcedureTemplateTable =
        aps_fractioning_procedure_templates::fractioning_procedure_templates::table;
}
