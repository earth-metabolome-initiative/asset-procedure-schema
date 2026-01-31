//! Auto-generated crate for the `freeze_drying_procedures` table.
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
/// Struct representing a row in the `freeze_drying_procedures` table.
#[table_model(ancestors(aps_ownables::ownables, aps_procedures::procedures))]
# [diesel (belongs_to (aps_volumetric_containers :: VolumetricContainer , foreign_key = freeze_dried_container_id))]
# [diesel (belongs_to (aps_volumetric_container_models :: VolumetricContainerModel , foreign_key = freeze_dried_container_model_id))]
# [diesel (belongs_to (aps_freeze_dryers :: FreezeDryer , foreign_key = freeze_dried_with_id))]
# [diesel (belongs_to (aps_freeze_dryer_models :: FreezeDryerModel , foreign_key = freeze_dried_with_model_id))]
# [table_model (foreign_key ((id ,) , (:: aps_procedures :: procedures :: id)))]
# [table_model (foreign_key ((freeze_drying_procedure_template_id ,) , (:: aps_freeze_drying_procedure_templates :: freeze_drying_procedure_templates :: id)))]
# [table_model (foreign_key ((freeze_dried_container_id ,) , (:: aps_volumetric_containers :: volumetric_containers :: id)))]
# [table_model (foreign_key ((freeze_dried_container_model_id ,) , (:: aps_volumetric_container_models :: volumetric_container_models :: id)))]
# [table_model (foreign_key ((procedure_template_freeze_dried_container_model_id ,) , (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id)))]
# [table_model (foreign_key ((freeze_dried_with_id ,) , (:: aps_freeze_dryers :: freeze_dryers :: id)))]
# [table_model (foreign_key ((freeze_dried_with_model_id ,) , (:: aps_freeze_dryer_models :: freeze_dryer_models :: id)))]
# [table_model (foreign_key ((procedure_template_freeze_dried_with_model_id ,) , (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id)))]
# [table_model (foreign_key ((freeze_dried_with_model_id , freeze_dried_container_model_id ,) , (:: aps_asset_compatibility_rules :: asset_compatibility_rules :: left_asset_model_id , :: aps_asset_compatibility_rules :: asset_compatibility_rules :: right_asset_model_id)))]
#[table_model(default(aps_ownables::ownables::ownable_table_id, "freeze_drying_procedures"))]
# [diesel (table_name = freeze_drying_procedures)]
pub struct FreezeDryingProcedure {
    /// Identifier of the freeze drying id, which is also a foreign key to the
    /// general procedure.
    #[same_as(aps_procedures::procedures::id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// The template of this procedure_id should be a freeze drying procedure_id
    /// template.
    #[same_as(aps_procedures::procedures::procedure_template_id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    freeze_drying_procedure_template_id: ::rosetta_uuid::Uuid,
    /// The container that is being freeze dried, which must be a volumetric
    /// container.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    freeze_dried_container_id: ::rosetta_uuid::Uuid,
    /// The container model that is being freeze dried, which must be a
    /// volumetric container model.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::asset_model_id,
        procedure_freeze_dried_container_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    freeze_dried_container_model_id: ::rosetta_uuid::Uuid,
    /// The procedure_id template asset model associated to the
    /// `freeze_dried_container`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::procedure_template_asset_model_id,
        procedure_freeze_dried_container_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_freeze_dried_container_model_id: ::rosetta_uuid::Uuid,
    /// The procedure_id asset associated to the `freeze_dried_container`.
    #[discretionary(aps_procedure_asset_models::procedure_asset_models)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_freeze_dried_container_id: ::rosetta_uuid::Uuid,
    /// The freeze drier used for the freeze drying procedure. This field is
    /// optional, as the freeze drier might not necessarily be tracked.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    freeze_dried_with_id: Option<::rosetta_uuid::Uuid>,
    /// The model of the freeze drier used, which must be a freeze drier model.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::asset_model_id,
        procedure_freeze_dried_with_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    freeze_dried_with_model_id: ::rosetta_uuid::Uuid,
    /// The procedure_id template asset model associated to the
    /// `freeze_dried_with`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::procedure_template_asset_model_id,
        procedure_freeze_dried_with_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_freeze_dried_with_model_id: ::rosetta_uuid::Uuid,
    /// The procedure_id asset associated to the `freeze_dried_with`.
    #[discretionary(aps_procedure_asset_models::procedure_asset_models)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_freeze_dried_with_id: ::rosetta_uuid::Uuid,
}
impl ::diesel_builders::GetColumn<aps_ownables::ownables::id> for FreezeDryingProcedure {
    fn get_column_ref(
        &self,
    ) -> &<freeze_drying_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_procedures::procedures::id> for FreezeDryingProcedure {
    fn get_column_ref(
        &self,
    ) -> &<freeze_drying_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl procedure_like::ProcedureTableLike for freeze_drying_procedures::table {
    type ProcedureTemplateTable =
        aps_freeze_drying_procedure_templates::freeze_drying_procedure_templates::table;
}
