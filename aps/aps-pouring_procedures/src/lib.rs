//! Auto-generated crate for the `pouring_procedures` table.
#[derive(
    Copy,
    Clone,
    Debug,
    Hash,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    :: serde :: Serialize,
    :: serde :: Deserialize,
    :: diesel :: Queryable,
    :: diesel :: Selectable,
    :: diesel :: Identifiable,
    :: diesel :: Associations,
    :: diesel_builders :: prelude :: TableModel,
)]
/// Struct representing a row in the `pouring_procedures` table.
#[table_model(ancestors(aps_ownables::ownables, aps_procedures::procedures))]
# [diesel (belongs_to (aps_volume_measuring_devices :: VolumeMeasuringDevice , foreign_key = measured_with_id))]
# [diesel (belongs_to (aps_volume_measuring_device_models :: VolumeMeasuringDeviceModel , foreign_key = measured_with_model_id))]
# [table_model (foreign_key ((id ,) , (:: aps_procedures :: procedures :: id)))]
# [table_model (foreign_key ((pouring_procedure_template_id ,) , (:: aps_pouring_procedure_templates :: pouring_procedure_templates :: id)))]
# [table_model (foreign_key ((poured_from_id ,) , (:: aps_volumetric_containers :: volumetric_containers :: id)))]
# [table_model (foreign_key ((poured_from_model_id ,) , (:: aps_volumetric_container_models :: volumetric_container_models :: id)))]
# [table_model (foreign_key ((procedure_template_poured_from_model_id ,) , (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id)))]
# [table_model (foreign_key ((measured_with_id ,) , (:: aps_volume_measuring_devices :: volume_measuring_devices :: id)))]
# [table_model (foreign_key ((measured_with_model_id ,) , (:: aps_volume_measuring_device_models :: volume_measuring_device_models :: id)))]
# [table_model (foreign_key ((procedure_template_measured_with_model_id ,) , (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id)))]
# [table_model (foreign_key ((poured_into_id ,) , (:: aps_volumetric_containers :: volumetric_containers :: id)))]
# [table_model (foreign_key ((poured_into_model_id ,) , (:: aps_volumetric_container_models :: volumetric_container_models :: id)))]
# [table_model (foreign_key ((procedure_template_poured_into_model_id ,) , (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id)))]
#[table_model(default(aps_ownables::ownables::ownable_table_id, "pouring_procedures"))]
# [diesel (table_name = pouring_procedures)]
pub struct PouringProcedure {
    /// The extended `procedure`.
    #[same_as(aps_procedures::procedures::id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// The procedure_id template of the extended `procedure`.
    #[same_as(aps_procedures::procedures::procedure_template_id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    pouring_procedure_template_id: ::rosetta_uuid::Uuid,
    /// The container from which the liquid is poured.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    poured_from_id: ::rosetta_uuid::Uuid,
    /// The model of the container from which the liquid is poured.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::asset_model_id,
        procedure_poured_from_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    poured_from_model_id: ::rosetta_uuid::Uuid,
    /// The procedure_id template asset model associated to the `poured_from`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::procedure_template_asset_model_id,
        procedure_poured_from_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_poured_from_model_id: ::rosetta_uuid::Uuid,
    /// The procedure_id asset associated to the `poured_from`.
    #[discretionary(aps_procedure_asset_models::procedure_asset_models)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_poured_from_id: ::rosetta_uuid::Uuid,
    /// The actual measuring device (if known) used to measure the liquid
    /// volume.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    measured_with_id: Option<::rosetta_uuid::Uuid>,
    /// The model of the measuring device.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::asset_model_id,
        procedure_measured_with_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    measured_with_model_id: Option<::rosetta_uuid::Uuid>,
    /// The procedure_id template asset model associated to the
    /// `measured_with_model`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::procedure_template_asset_model_id,
        procedure_measured_with_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_measured_with_model_id: ::rosetta_uuid::Uuid,
    /// The procedure_id asset associated to the `measured_with`.
    #[discretionary(aps_procedure_asset_models::procedure_asset_models)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_measured_with_id: ::rosetta_uuid::Uuid,
    /// The container into which the liquid is poured.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    poured_into_id: ::rosetta_uuid::Uuid,
    /// The model of the container into which the liquid is poured.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::asset_model_id,
        procedure_poured_into_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    poured_into_model_id: ::rosetta_uuid::Uuid,
    /// The procedure_id template asset model associated to the `poured_into`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::procedure_template_asset_model_id,
        procedure_poured_into_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_poured_into_model_id: ::rosetta_uuid::Uuid,
    /// The procedure_id asset associated to the `poured_into`.
    #[discretionary(aps_procedure_asset_models::procedure_asset_models)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_poured_into_id: ::rosetta_uuid::Uuid,
}
impl ::diesel_builders::GetColumn<aps_ownables::ownables::id> for PouringProcedure {
    fn get_column_ref(
        &self,
    ) -> &<pouring_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_procedures::procedures::id> for PouringProcedure {
    fn get_column_ref(
        &self,
    ) -> &<pouring_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl procedure_like::ProcedureTableLike for pouring_procedures::table {
    type ProcedureTemplateTable =
        aps_pouring_procedure_templates::pouring_procedure_templates::table;
}
