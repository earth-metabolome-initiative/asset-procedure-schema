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
    serde :: Serialize,
    serde :: Deserialize,
    diesel :: Queryable,
    diesel :: Selectable,
    diesel :: Identifiable,
    diesel_builders :: prelude :: TableModel,
)]
/// Struct representing a row in the `ball_mill_procedures` table.
#[table_model(ancestors(aps_procedures::procedures))]
#[table_model(default(aps_procedures::procedures::procedure_table_id, "ball_mill_procedures"))]
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
    /// Field representing the `bead_model_id` column in table
    /// `ball_mill_procedures`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::asset_model_id,
        procedure_bead_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    bead_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_template_bead_model_id` column in
    /// table `ball_mill_procedures`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::procedure_template_asset_model_id,
        procedure_bead_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_bead_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_bead_id` column in table
    /// `ball_mill_procedures`.
    #[discretionary(aps_procedure_asset_models::procedure_asset_models)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_bead_id: ::rosetta_uuid::Uuid,
    /// Field representing the `milled_with_model_id` column in table
    /// `ball_mill_procedures`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::asset_model_id,
        procedure_milled_with_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    milled_with_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_template_milled_with_model_id` column
    /// in table `ball_mill_procedures`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::procedure_template_asset_model_id,
        procedure_milled_with_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_milled_with_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_milled_with_id` column in table
    /// `ball_mill_procedures`.
    #[discretionary(aps_procedure_asset_models::procedure_asset_models)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_milled_with_id: ::rosetta_uuid::Uuid,
    /// Field representing the `milled_with_id` column in table
    /// `ball_mill_procedures`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    milled_with_id: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `milled_container_id` column in table
    /// `ball_mill_procedures`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    milled_container_id: ::rosetta_uuid::Uuid,
    /// Field representing the `milled_container_model_id` column in table
    /// `ball_mill_procedures`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::asset_model_id,
        procedure_milled_container_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    milled_container_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_template_milled_container_model_id`
    /// column in table `ball_mill_procedures`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::procedure_template_asset_model_id,
        procedure_milled_container_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_milled_container_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_milled_container_id` column in table
    /// `ball_mill_procedures`.
    #[discretionary(aps_procedure_asset_models::procedure_asset_models)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_milled_container_id: ::rosetta_uuid::Uuid,
}
:: diesel_builders :: prelude :: fk ! ((ball_mill_procedures :: id) -> (:: aps_procedures :: procedures :: id));
:: diesel_builders :: prelude :: fk ! ((ball_mill_procedures :: ball_mill_procedure_template_id) -> (:: aps_ball_mill_procedure_templates :: ball_mill_procedure_templates :: id));
:: diesel_builders :: prelude :: fk ! ((ball_mill_procedures :: bead_model_id) -> (:: aps_bead_models :: bead_models :: id));
:: diesel_builders :: prelude :: fk ! ((ball_mill_procedures :: procedure_template_bead_model_id) -> (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id));
:: diesel_builders :: prelude :: fk ! ((ball_mill_procedures :: milled_with_model_id) -> (:: aps_ball_mill_machine_models :: ball_mill_machine_models :: id));
:: diesel_builders :: prelude :: fk ! ((ball_mill_procedures :: procedure_template_milled_with_model_id) -> (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id));
:: diesel_builders :: prelude :: fk ! ((ball_mill_procedures :: milled_with_id) -> (:: aps_ball_mill_machines :: ball_mill_machines :: id));
:: diesel_builders :: prelude :: fk ! ((ball_mill_procedures :: milled_container_id) -> (:: aps_volumetric_containers :: volumetric_containers :: id));
:: diesel_builders :: prelude :: fk ! ((ball_mill_procedures :: milled_container_model_id) -> (:: aps_volumetric_container_models :: volumetric_container_models :: id));
:: diesel_builders :: prelude :: fk ! ((ball_mill_procedures :: procedure_template_milled_container_model_id) -> (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id));
:: diesel_builders :: prelude :: fk ! ((ball_mill_procedures :: milled_with_model_id , ball_mill_procedures :: milled_container_model_id) -> (:: aps_asset_compatibility_rules :: asset_compatibility_rules :: left_asset_model_id , :: aps_asset_compatibility_rules :: asset_compatibility_rules :: right_asset_model_id));
:: diesel_builders :: prelude :: fk ! ((ball_mill_procedures :: milled_with_model_id , ball_mill_procedures :: bead_model_id) -> (:: aps_asset_compatibility_rules :: asset_compatibility_rules :: left_asset_model_id , :: aps_asset_compatibility_rules :: asset_compatibility_rules :: right_asset_model_id));
:: diesel_builders :: prelude :: fk ! ((ball_mill_procedures :: bead_model_id , ball_mill_procedures :: milled_container_model_id) -> (:: aps_asset_compatibility_rules :: asset_compatibility_rules :: left_asset_model_id , :: aps_asset_compatibility_rules :: asset_compatibility_rules :: right_asset_model_id));
