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
    :: serde :: Serialize,
    :: serde :: Deserialize,
    :: diesel :: Queryable,
    :: diesel :: Selectable,
    :: diesel :: Identifiable,
    :: diesel :: Associations,
    :: diesel_builders :: prelude :: TableModel,
)]
/// Struct representing a row in the `container_sealing_procedures` table.
#[table_model(ancestors(
    aps_entities::entities,
    aps_ownables::ownables,
    aps_procedures::procedures
))]
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (belongs_to (aps_volumetric_containers :: VolumetricContainer , foreign_key = capped_container_id))]
# [diesel (belongs_to (aps_volumetric_container_models :: VolumetricContainerModel , foreign_key = sealable_container_model_id))]
# [diesel (belongs_to (aps_container_sealer_models :: ContainerSealerModel , foreign_key = sealed_with_model_id))]
# [table_model (foreign_key ((id ,) , (:: aps_procedures :: procedures :: id)))]
# [table_model (foreign_key ((capping_procedure_template_id ,) , (:: aps_container_sealing_procedure_templates :: container_sealing_procedure_templates :: id)))]
# [table_model (foreign_key ((capped_container_id ,) , (:: aps_volumetric_containers :: volumetric_containers :: id)))]
# [table_model (foreign_key ((sealable_container_model_id ,) , (:: aps_volumetric_container_models :: volumetric_container_models :: id)))]
# [table_model (foreign_key ((procedure_template_sealable_container_model_id ,) , (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id)))]
# [table_model (foreign_key ((sealed_with_model_id ,) , (:: aps_container_sealer_models :: container_sealer_models :: id)))]
# [table_model (foreign_key ((procedure_template_sealed_with_model_id ,) , (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id)))]
# [table_model (foreign_key ((sealable_container_model_id , sealed_with_model_id ,) , (:: aps_asset_compatibility_rules :: asset_compatibility_rules :: left_asset_model_id , :: aps_asset_compatibility_rules :: asset_compatibility_rules :: right_asset_model_id)))]
#[table_model(default(aps_entities::entities::table_name_id, "container_sealing_procedures"))]
# [diesel (table_name = container_sealing_procedures)]
pub struct ContainerSealingProcedure {
    /// Identifier of the capping id, which is also a foreign key to the general
    /// procedure.
    #[same_as(aps_procedures::procedures::id)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// We enforce that the model of this procedure_id must be a capping
    /// procedure_id template.
    #[same_as(aps_procedures::procedures::procedure_template_id)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    capping_procedure_template_id: ::rosetta_uuid::Uuid,
    /// The container being capped, which must be a volumetric container.
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    capped_container_id: ::rosetta_uuid::Uuid,
    /// The model of the container being capped.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::asset_model_id,
        procedure_capped_container_id
    )]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    sealable_container_model_id: ::rosetta_uuid::Uuid,
    /// The procedure_id template asset model describing the `capped_container`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::procedure_template_asset_model_id,
        procedure_capped_container_id
    )]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_sealable_container_model_id: ::rosetta_uuid::Uuid,
    /// The procedure_id asset describing the `capped_container`.
    #[discretionary(aps_procedure_asset_models::procedure_asset_models)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_capped_container_id: ::rosetta_uuid::Uuid,
    /// The cap being used, which must be a cap model.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::asset_model_id,
        procedure_capped_with_id
    )]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    sealed_with_model_id: ::rosetta_uuid::Uuid,
    /// The procedure_id template asset model describing the
    /// `capped_with_model`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::procedure_template_asset_model_id,
        procedure_capped_with_id
    )]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_sealed_with_model_id: ::rosetta_uuid::Uuid,
    /// The procedure_id asset describing the `capped_with_model`.
    #[discretionary(aps_procedure_asset_models::procedure_asset_models)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_capped_with_id: ::rosetta_uuid::Uuid,
}
impl ::diesel_builders::GetColumn<aps_entities::entities::id> for ContainerSealingProcedure {
    fn get_column_ref(
        &self,
    ) -> &<container_sealing_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_ownables::ownables::id> for ContainerSealingProcedure {
    fn get_column_ref(
        &self,
    ) -> &<container_sealing_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_procedures::procedures::id> for ContainerSealingProcedure {
    fn get_column_ref(
        &self,
    ) -> &<container_sealing_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl procedure_like::ProcedureTableLike for container_sealing_procedures::table {
    type ProcedureTemplateTable =
        aps_container_sealing_procedure_templates::container_sealing_procedure_templates::table;
}
