//! Auto-generated crate for the `supernatant_procedures` table.
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
/// Struct representing a row in the `supernatant_procedures` table.
#[table_model(ancestors(aps_procedures::procedures))]
# [diesel (belongs_to (aps_volume_measuring_devices :: VolumeMeasuringDevice , foreign_key = transferred_with_id))]
# [diesel (belongs_to (aps_volume_measuring_device_models :: VolumeMeasuringDeviceModel , foreign_key = transferred_with_model_id))]
# [table_model (foreign_key ((id ,) , (:: aps_procedures :: procedures :: id)))]
# [table_model (foreign_key ((supernatant_procedure_template_id ,) , (:: aps_supernatant_procedure_templates :: supernatant_procedure_templates :: id)))]
# [table_model (foreign_key ((stratified_source_id ,) , (:: aps_volumetric_containers :: volumetric_containers :: id)))]
# [table_model (foreign_key ((stratified_source_model_id ,) , (:: aps_volumetric_container_models :: volumetric_container_models :: id)))]
# [table_model (foreign_key ((procedure_template_stratified_source_model_id ,) , (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id)))]
# [table_model (foreign_key ((supernatant_destination_id ,) , (:: aps_volumetric_containers :: volumetric_containers :: id)))]
# [table_model (foreign_key ((supernatant_destination_model_id ,) , (:: aps_volumetric_container_models :: volumetric_container_models :: id)))]
# [table_model (foreign_key ((procedure_template_supernatant_destination_model_id ,) , (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id)))]
# [table_model (foreign_key ((transferred_with_id ,) , (:: aps_volume_measuring_devices :: volume_measuring_devices :: id)))]
# [table_model (foreign_key ((transferred_with_model_id ,) , (:: aps_volume_measuring_device_models :: volume_measuring_device_models :: id)))]
# [table_model (foreign_key ((procedure_template_transferred_with_model_id ,) , (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id)))]
#[table_model(default(aps_procedures::procedures::procedure_table_id, "supernatant_procedures"))]
# [diesel (table_name = supernatant_procedures)]
pub struct SupernatantProcedure {
    /// Field representing the `id` column in table `supernatant_procedures`.
    #[same_as(aps_procedures::procedures::id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// We enforce that the model of this procedure_id must be a supernatant
    /// procedure_id template.
    #[same_as(aps_procedures::procedures::procedure_template_id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    supernatant_procedure_template_id: ::rosetta_uuid::Uuid,
    /// The source container from which the supernatant is taken.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    stratified_source_id: ::rosetta_uuid::Uuid,
    /// The model of the source container from which the supernatant is taken.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::asset_model_id,
        procedure_stratified_source_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    stratified_source_model_id: ::rosetta_uuid::Uuid,
    /// The procedure_id template asset model associated to the
    /// `stratified_source`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::procedure_template_asset_model_id,
        procedure_stratified_source_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_stratified_source_model_id: ::rosetta_uuid::Uuid,
    /// The procedure_id asset associated to the `stratified_source`.
    #[discretionary(aps_procedure_asset_models::procedure_asset_models)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_stratified_source_id: ::rosetta_uuid::Uuid,
    /// The destination container to which the supernatant is transferred.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    supernatant_destination_id: ::rosetta_uuid::Uuid,
    /// The model of the destination container to which the supernatant is
    /// transferred.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::asset_model_id,
        procedure_supernatant_destination_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    supernatant_destination_model_id: ::rosetta_uuid::Uuid,
    /// The procedure_id template asset model associated to the
    /// `supernatant_destination`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::procedure_template_asset_model_id,
        procedure_supernatant_destination_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_supernatant_destination_model_id: ::rosetta_uuid::Uuid,
    /// The procedure_id asset associated to the `supernatant_destination`.
    #[discretionary(aps_procedure_asset_models::procedure_asset_models)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_supernatant_destination_id: ::rosetta_uuid::Uuid,
    /// The device used for the aliquoting procedure.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    transferred_with_id: ::rosetta_uuid::Uuid,
    /// The model of the device used for the aliquoting procedure.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::asset_model_id,
        procedure_transferred_with_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    transferred_with_model_id: ::rosetta_uuid::Uuid,
    /// The procedure_id template asset model associated to the
    /// `transferred_with`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::procedure_template_asset_model_id,
        procedure_transferred_with_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_transferred_with_model_id: ::rosetta_uuid::Uuid,
    /// The procedure_id asset associated to the `transferred_with`.
    #[discretionary(aps_procedure_asset_models::procedure_asset_models)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_transferred_with_id: ::rosetta_uuid::Uuid,
}
impl ::diesel_builders::GetColumn<aps_procedures::procedures::id> for SupernatantProcedure {
    fn get_column_ref(
        &self,
    ) -> &<supernatant_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl procedure_like::ProcedureTableLike for supernatant_procedures::table {
    type ProcedureTemplateTable =
        aps_supernatant_procedure_templates::supernatant_procedure_templates::table;
}
