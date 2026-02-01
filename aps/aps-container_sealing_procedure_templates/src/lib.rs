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
    :: serde :: Serialize,
    :: serde :: Deserialize,
    :: diesel :: Queryable,
    :: diesel :: Selectable,
    :: diesel :: Identifiable,
    :: diesel :: Associations,
    :: diesel_builders :: prelude :: TableModel,
)]
/// Struct representing a row in the `container_sealing_procedure_templates`
/// table.
#[table_model(ancestors(
    aps_entities::entities,
    aps_ownables::ownables,
    aps_procedure_templates::procedure_templates
))]
# [diesel (belongs_to (aps_procedure_templates :: ProcedureTemplate , foreign_key = id))]
# [diesel (belongs_to (aps_volumetric_container_models :: VolumetricContainerModel , foreign_key = sealable_container_model_id))]
# [diesel (belongs_to (aps_container_sealer_models :: ContainerSealerModel , foreign_key = sealed_with_model_id))]
# [table_model (foreign_key ((id ,) , (:: aps_procedure_templates :: procedure_templates :: id)))]
# [table_model (foreign_key ((sealable_container_model_id ,) , (:: aps_volumetric_container_models :: volumetric_container_models :: id)))]
# [table_model (foreign_key ((sealed_with_model_id ,) , (:: aps_container_sealer_models :: container_sealer_models :: id)))]
# [table_model (foreign_key ((sealable_container_model_id , sealed_with_model_id ,) , (:: aps_asset_compatibility_rules :: asset_compatibility_rules :: left_asset_model_id , :: aps_asset_compatibility_rules :: asset_compatibility_rules :: right_asset_model_id)))]
# [table_model (foreign_key ((id , procedure_template_sealable_container_model_id ,) , (:: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_id , :: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_asset_model_id)))]
# [table_model (foreign_key ((id , procedure_template_sealed_with_model_id ,) , (:: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_id , :: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_asset_model_id)))]
#[table_model(default(
    aps_entities::entities::table_name_id,
    "container_sealing_procedure_templates"
))]
# [diesel (table_name = container_sealing_procedure_templates)]
pub struct ContainerSealingProcedureTemplate {
    /// Identifier of the capping procedure_id template, which is also a foreign
    /// key to the general procedure_id template.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// The container to be capped.
    #[same_as(
        aps_procedure_template_asset_models::procedure_template_asset_models::asset_model_id,
        procedure_template_sealable_container_model_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    sealable_container_model_id: ::rosetta_uuid::Uuid,
    /// The procedure_id template associated with the container model.
    #[discretionary(aps_procedure_template_asset_models::procedure_template_asset_models)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_sealable_container_model_id: ::rosetta_uuid::Uuid,
    /// The cap to be used for the container.
    #[same_as(
        aps_procedure_template_asset_models::procedure_template_asset_models::asset_model_id,
        procedure_template_sealed_with_model_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    sealed_with_model_id: ::rosetta_uuid::Uuid,
    /// The procedure_id template associated with the cap model.
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
impl ::diesel_builders::GetColumn<aps_entities::entities::id>
    for ContainerSealingProcedureTemplate
{
    fn get_column_ref(
        &self,
    ) -> &<container_sealing_procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType
    {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_ownables::ownables::id>
    for ContainerSealingProcedureTemplate
{
    fn get_column_ref(
        &self,
    ) -> &<container_sealing_procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType
    {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_procedure_templates::procedure_templates::id>
    for ContainerSealingProcedureTemplate
{
    fn get_column_ref(
        &self,
    ) -> &<container_sealing_procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType
    {
        &self.id
    }
}
