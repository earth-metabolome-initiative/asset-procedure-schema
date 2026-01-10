//! Auto-generated crate for the `container_sealing_procedure_templates` table.
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
/// Struct representing a row in the `container_sealing_procedure_templates`
/// table.
#[table_model(ancestors(aps_procedure_templates::procedure_templates))]
#[table_model(default(
    aps_procedure_templates::procedure_templates::procedure_template_table_id,
    "container_sealing_procedure_templates"
))]
# [diesel (table_name = container_sealing_procedure_templates)]
pub struct ContainerSealingProcedureTemplate {
    /// Field representing the `id` column in table
    /// `container_sealing_procedure_templates`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `sealable_container_model_id` column in table
    /// `container_sealing_procedure_templates`.
    #[same_as(
        aps_procedure_template_asset_models::procedure_template_asset_models::asset_model_id,
        procedure_template_sealable_container_model_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    sealable_container_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_template_sealable_container_model_id`
    /// column in table `container_sealing_procedure_templates`.
    #[discretionary(aps_procedure_template_asset_models::procedure_template_asset_models)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_sealable_container_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `sealed_with_model_id` column in table
    /// `container_sealing_procedure_templates`.
    #[same_as(
        aps_procedure_template_asset_models::procedure_template_asset_models::asset_model_id,
        procedure_template_sealed_with_model_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    sealed_with_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_template_sealed_with_model_id` column
    /// in table `container_sealing_procedure_templates`.
    #[discretionary(aps_procedure_template_asset_models::procedure_template_asset_models)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_sealed_with_model_id: ::rosetta_uuid::Uuid,
}
::diesel_builders::prelude::unique_index!(
    container_sealing_procedure_templates::id,
    container_sealing_procedure_templates::procedure_template_sealable_container_model_id
);
::diesel_builders::prelude::unique_index!(
    container_sealing_procedure_templates::id,
    container_sealing_procedure_templates::procedure_template_sealed_with_model_id
);
:: diesel_builders :: prelude :: fk ! ((container_sealing_procedure_templates :: id) -> (:: aps_procedure_templates :: procedure_templates :: id));
:: diesel_builders :: prelude :: fk ! ((container_sealing_procedure_templates :: sealable_container_model_id) -> (:: aps_volumetric_container_models :: volumetric_container_models :: id));
:: diesel_builders :: prelude :: fk ! ((container_sealing_procedure_templates :: sealed_with_model_id) -> (:: aps_container_sealer_models :: container_sealer_models :: id));
:: diesel_builders :: prelude :: fk ! ((container_sealing_procedure_templates :: sealable_container_model_id , container_sealing_procedure_templates :: sealed_with_model_id) -> (:: aps_asset_compatibility_rules :: asset_compatibility_rules :: left_asset_model_id , :: aps_asset_compatibility_rules :: asset_compatibility_rules :: right_asset_model_id));
:: diesel_builders :: prelude :: fk ! ((container_sealing_procedure_templates :: id , container_sealing_procedure_templates :: procedure_template_sealable_container_model_id) -> (:: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_id , :: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_asset_model_id));
:: diesel_builders :: prelude :: fk ! ((container_sealing_procedure_templates :: id , container_sealing_procedure_templates :: procedure_template_sealed_with_model_id) -> (:: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_id , :: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_asset_model_id));
impl diesel_builders::GetColumn<aps_procedure_templates::procedure_templates::id>
    for ContainerSealingProcedureTemplate
{
    fn get_column_ref(
        &self,
    ) -> &<container_sealing_procedure_templates::id as diesel_builders::Typed>::ColumnType {
        &self.id
    }
}
