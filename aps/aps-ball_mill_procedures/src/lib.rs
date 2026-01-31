//! Auto-generated crate for the `ball_mill_procedures` table.
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
/// Struct representing a row in the `ball_mill_procedures` table.
#[table_model(ancestors(aps_ownables::ownables, aps_procedures::procedures))]
# [diesel (belongs_to (aps_bead_models :: BeadModel , foreign_key = bead_model_id))]
# [diesel (belongs_to (aps_ball_mill_machine_models :: BallMillMachineModel , foreign_key = milled_with_model_id))]
# [diesel (belongs_to (aps_ball_mill_machines :: BallMillMachine , foreign_key = milled_with_id))]
# [diesel (belongs_to (aps_volumetric_containers :: VolumetricContainer , foreign_key = milled_container_id))]
# [diesel (belongs_to (aps_volumetric_container_models :: VolumetricContainerModel , foreign_key = milled_container_model_id))]
# [table_model (foreign_key ((id ,) , (:: aps_procedures :: procedures :: id)))]
# [table_model (foreign_key ((ball_mill_procedure_template_id ,) , (:: aps_ball_mill_procedure_templates :: ball_mill_procedure_templates :: id)))]
# [table_model (foreign_key ((bead_model_id ,) , (:: aps_bead_models :: bead_models :: id)))]
# [table_model (foreign_key ((procedure_template_bead_model_id ,) , (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id)))]
# [table_model (foreign_key ((milled_with_model_id ,) , (:: aps_ball_mill_machine_models :: ball_mill_machine_models :: id)))]
# [table_model (foreign_key ((procedure_template_milled_with_model_id ,) , (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id)))]
# [table_model (foreign_key ((milled_with_id ,) , (:: aps_ball_mill_machines :: ball_mill_machines :: id)))]
# [table_model (foreign_key ((milled_container_id ,) , (:: aps_volumetric_containers :: volumetric_containers :: id)))]
# [table_model (foreign_key ((milled_container_model_id ,) , (:: aps_volumetric_container_models :: volumetric_container_models :: id)))]
# [table_model (foreign_key ((procedure_template_milled_container_model_id ,) , (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id)))]
# [table_model (foreign_key ((milled_with_model_id , milled_container_model_id ,) , (:: aps_asset_compatibility_rules :: asset_compatibility_rules :: left_asset_model_id , :: aps_asset_compatibility_rules :: asset_compatibility_rules :: right_asset_model_id)))]
# [table_model (foreign_key ((milled_with_model_id , bead_model_id ,) , (:: aps_asset_compatibility_rules :: asset_compatibility_rules :: left_asset_model_id , :: aps_asset_compatibility_rules :: asset_compatibility_rules :: right_asset_model_id)))]
# [table_model (foreign_key ((bead_model_id , milled_container_model_id ,) , (:: aps_asset_compatibility_rules :: asset_compatibility_rules :: left_asset_model_id , :: aps_asset_compatibility_rules :: asset_compatibility_rules :: right_asset_model_id)))]
#[table_model(default(aps_ownables::ownables::ownable_table_id, "ball_mill_procedures"))]
# [diesel (table_name = ball_mill_procedures)]
pub struct BallMillProcedure {
    /// Field representing the `id` column in table `ball_mill_procedures`.
    #[same_as(aps_procedures::procedures::id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `ball_mill_procedure_template_id` column in table
    /// `ball_mill_procedures`.
    #[same_as(aps_procedures::procedures::procedure_template_id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    ball_mill_procedure_template_id: ::rosetta_uuid::Uuid,
    /// The beads model used for the procedure.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::asset_model_id,
        procedure_bead_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    bead_model_id: ::rosetta_uuid::Uuid,
    /// The procedure_id template asset model associated to the `bead_model`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::procedure_template_asset_model_id,
        procedure_bead_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_bead_model_id: ::rosetta_uuid::Uuid,
    /// The procedure_id asset associated to the `bead_model`.
    #[discretionary(aps_procedure_asset_models::procedure_asset_models)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_bead_id: ::rosetta_uuid::Uuid,
    /// The device used for the ball mill procedure.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::asset_model_id,
        procedure_milled_with_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    milled_with_model_id: ::rosetta_uuid::Uuid,
    /// The procedure_id template asset model associated to the
    /// `milled_with_model`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::procedure_template_asset_model_id,
        procedure_milled_with_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_milled_with_model_id: ::rosetta_uuid::Uuid,
    /// The procedure_id asset associated to the `milled_with_model`.
    #[discretionary(aps_procedure_asset_models::procedure_asset_models)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_milled_with_id: ::rosetta_uuid::Uuid,
    /// machine might not have been recorded at the time of performing the
    /// procedure.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    milled_with_id: Option<::rosetta_uuid::Uuid>,
    /// The container that is being milled.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    milled_container_id: ::rosetta_uuid::Uuid,
    /// The container model that is being milled.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::asset_model_id,
        procedure_milled_container_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    milled_container_model_id: ::rosetta_uuid::Uuid,
    /// The procedure_id template asset model associated to the
    /// `milled_container`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::procedure_template_asset_model_id,
        procedure_milled_container_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_milled_container_model_id: ::rosetta_uuid::Uuid,
    /// The procedure_id asset associated to the `milled_container`.
    #[discretionary(aps_procedure_asset_models::procedure_asset_models)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_milled_container_id: ::rosetta_uuid::Uuid,
}
impl ::diesel_builders::GetColumn<aps_ownables::ownables::id> for BallMillProcedure {
    fn get_column_ref(
        &self,
    ) -> &<ball_mill_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_procedures::procedures::id> for BallMillProcedure {
    fn get_column_ref(
        &self,
    ) -> &<ball_mill_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl procedure_like::ProcedureTableLike for ball_mill_procedures::table {
    type ProcedureTemplateTable =
        aps_ball_mill_procedure_templates::ball_mill_procedure_templates::table;
}
