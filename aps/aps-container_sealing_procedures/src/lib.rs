//! Auto-generated crate for the `container_sealing_procedures` table.
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
/// Struct representing a row in the `container_sealing_procedures` table.
#[table_model(ancestors(aps_procedures::procedures))]
#[table_model(default(
    aps_procedures::procedures::procedure_table_id,
    "container_sealing_procedures"
))]
# [diesel (table_name = container_sealing_procedures)]
pub struct ContainerSealingProcedure {
    /// Field representing the `id` column in table
    /// `container_sealing_procedures`.
    #[same_as(aps_procedures::procedures::id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `capping_procedure_template_id` column in table
    /// `container_sealing_procedures`.
    #[same_as(aps_procedures::procedures::procedure_template_id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    capping_procedure_template_id: ::rosetta_uuid::Uuid,
    /// Field representing the `capped_container_id` column in table
    /// `container_sealing_procedures`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    capped_container_id: ::rosetta_uuid::Uuid,
    /// Field representing the `sealable_container_model_id` column in table
    /// `container_sealing_procedures`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::asset_model_id,
        procedure_capped_container_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    sealable_container_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_template_sealable_container_model_id`
    /// column in table `container_sealing_procedures`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::procedure_template_asset_model_id,
        procedure_capped_container_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_sealable_container_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_capped_container_id` column in table
    /// `container_sealing_procedures`.
    #[discretionary(aps_procedure_asset_models::procedure_asset_models)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_capped_container_id: ::rosetta_uuid::Uuid,
    /// Field representing the `sealed_with_model_id` column in table
    /// `container_sealing_procedures`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::asset_model_id,
        procedure_capped_with_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    sealed_with_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_template_sealed_with_model_id` column
    /// in table `container_sealing_procedures`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::procedure_template_asset_model_id,
        procedure_capped_with_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_sealed_with_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_capped_with_id` column in table
    /// `container_sealing_procedures`.
    #[discretionary(aps_procedure_asset_models::procedure_asset_models)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_capped_with_id: ::rosetta_uuid::Uuid,
}
:: diesel_builders :: prelude :: fk ! ((container_sealing_procedures :: id) -> (:: aps_procedures :: procedures :: id));
:: diesel_builders :: prelude :: fk ! ((container_sealing_procedures :: capping_procedure_template_id) -> (:: aps_container_sealing_procedure_templates :: container_sealing_procedure_templates :: id));
:: diesel_builders :: prelude :: fk ! ((container_sealing_procedures :: capped_container_id) -> (:: aps_volumetric_containers :: volumetric_containers :: id));
:: diesel_builders :: prelude :: fk ! ((container_sealing_procedures :: sealable_container_model_id) -> (:: aps_volumetric_container_models :: volumetric_container_models :: id));
:: diesel_builders :: prelude :: fk ! ((container_sealing_procedures :: procedure_template_sealable_container_model_id) -> (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id));
:: diesel_builders :: prelude :: fk ! ((container_sealing_procedures :: sealed_with_model_id) -> (:: aps_container_sealer_models :: container_sealer_models :: id));
:: diesel_builders :: prelude :: fk ! ((container_sealing_procedures :: procedure_template_sealed_with_model_id) -> (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id));
:: diesel_builders :: prelude :: fk ! ((container_sealing_procedures :: sealable_container_model_id , container_sealing_procedures :: sealed_with_model_id) -> (:: aps_asset_compatibility_rules :: asset_compatibility_rules :: left_asset_model_id , :: aps_asset_compatibility_rules :: asset_compatibility_rules :: right_asset_model_id));
