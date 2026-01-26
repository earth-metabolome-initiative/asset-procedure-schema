//! Auto-generated crate for the `aliquoting_procedures` table.
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
/// Struct representing a row in the `aliquoting_procedures` table.
#[table_model(ancestors(aps_procedures::procedures))]
# [diesel (belongs_to (aps_volume_measuring_devices :: VolumeMeasuringDevice , foreign_key = aliquoted_with_id))]
# [diesel (belongs_to (aps_volume_measuring_device_models :: VolumeMeasuringDeviceModel , foreign_key = aliquoted_with_model_id))]
# [table_model (foreign_key ((id ,) , (:: aps_procedures :: procedures :: id)))]
# [table_model (foreign_key ((aliquoting_procedure_template_id ,) , (:: aps_aliquoting_procedure_templates :: aliquoting_procedure_templates :: id)))]
# [table_model (foreign_key ((aliquoted_with_id ,) , (:: aps_volume_measuring_devices :: volume_measuring_devices :: id)))]
# [table_model (foreign_key ((aliquoted_with_model_id ,) , (:: aps_volume_measuring_device_models :: volume_measuring_device_models :: id)))]
# [table_model (foreign_key ((procedure_template_aliquoted_with_model_id ,) , (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id)))]
# [table_model (foreign_key ((aliquoted_from_id ,) , (:: aps_volumetric_containers :: volumetric_containers :: id)))]
# [table_model (foreign_key ((procedure_template_aliquoted_from_model_id ,) , (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id)))]
# [table_model (foreign_key ((aliquoted_into_id ,) , (:: aps_volumetric_containers :: volumetric_containers :: id)))]
# [table_model (foreign_key ((procedure_template_aliquoted_into_model_id ,) , (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id)))]
#[table_model(default(aps_procedures::procedures::procedure_table_id, "aliquoting_procedures"))]
# [diesel (table_name = aliquoting_procedures)]
pub struct AliquotingProcedure {
    /// Field representing the `id` column in table `aliquoting_procedures`.
    #[same_as(aps_procedures::procedures::id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// We enforce that the model of this procedure must be an aliquoting
    /// procedure template.
    #[same_as(aps_procedures::procedures::procedure_template_id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    aliquoting_procedure_template_id: ::rosetta_uuid::Uuid,
    /// The identifier of the instrument used for aliquoting.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    aliquoted_with_id: Option<::rosetta_uuid::Uuid>,
    /// The identifier of the instrument model used for aliquoting.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::asset_model_id,
        procedure_aliquoted_with_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    aliquoted_with_model_id: ::rosetta_uuid::Uuid,
    /// The procedure template asset model associated to the `aliquoted_with`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::procedure_template_asset_model_id,
        procedure_aliquoted_with_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_aliquoted_with_model_id: ::rosetta_uuid::Uuid,
    /// The procedure asset associated to the `aliquoted_with`.
    #[discretionary(aps_procedure_asset_models::procedure_asset_models)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_aliquoted_with_id: ::rosetta_uuid::Uuid,
    /// The container being aliquoted, which must be a volumetric container
    /// model.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    aliquoted_from_id: ::rosetta_uuid::Uuid,
    /// The procedure template asset model associated to the `aliquoted_from`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::procedure_template_asset_model_id,
        procedure_aliquoted_from_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_aliquoted_from_model_id: ::rosetta_uuid::Uuid,
    /// The procedure asset associated to the `aliquoted_from`.
    #[discretionary(aps_procedure_asset_models::procedure_asset_models)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_aliquoted_from_id: ::rosetta_uuid::Uuid,
    /// The container receiving the aliquot, which must be a volumetric
    /// container model.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    aliquoted_into_id: ::rosetta_uuid::Uuid,
    /// The procedure template asset model associated to the `aliquoted_into`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::procedure_template_asset_model_id,
        procedure_aliquoted_into_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_aliquoted_into_model_id: ::rosetta_uuid::Uuid,
    /// The procedure asset associated to the `aliquoted_into`.
    #[discretionary(aps_procedure_asset_models::procedure_asset_models)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_aliquoted_into_id: ::rosetta_uuid::Uuid,
}
impl ::diesel_builders::GetColumn<aps_procedures::procedures::id> for AliquotingProcedure {
    fn get_column_ref(
        &self,
    ) -> &<aliquoting_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl procedure_like::ProcedureTableLike for aliquoting_procedures::table {
    type ProcedureTemplateTable =
        aps_aliquoting_procedure_templates::aliquoting_procedure_templates::table;
}
