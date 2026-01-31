//! Auto-generated crate for the `freezing_procedures` table.
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
/// Struct representing a row in the `freezing_procedures` table.
#[table_model(ancestors(aps_ownables::ownables, aps_procedures::procedures))]
# [diesel (belongs_to (aps_volumetric_containers :: VolumetricContainer , foreign_key = frozen_container_id))]
# [diesel (belongs_to (aps_volumetric_container_models :: VolumetricContainerModel , foreign_key = frozen_container_model_id))]
# [diesel (belongs_to (aps_freezers :: Freezer , foreign_key = frozen_with_id))]
# [diesel (belongs_to (aps_freezer_models :: FreezerModel , foreign_key = frozen_with_model_id))]
# [table_model (foreign_key ((id ,) , (:: aps_procedures :: procedures :: id)))]
# [table_model (foreign_key ((freezing_procedure_template_id ,) , (:: aps_freezing_procedure_templates :: freezing_procedure_templates :: id)))]
# [table_model (foreign_key ((frozen_container_id ,) , (:: aps_volumetric_containers :: volumetric_containers :: id)))]
# [table_model (foreign_key ((frozen_container_model_id ,) , (:: aps_volumetric_container_models :: volumetric_container_models :: id)))]
# [table_model (foreign_key ((procedure_template_frozen_container_model_id ,) , (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id)))]
# [table_model (foreign_key ((frozen_with_id ,) , (:: aps_freezers :: freezers :: id)))]
# [table_model (foreign_key ((frozen_with_model_id ,) , (:: aps_freezer_models :: freezer_models :: id)))]
# [table_model (foreign_key ((procedure_template_frozen_with_model_id ,) , (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id)))]
# [table_model (foreign_key ((frozen_with_model_id , frozen_container_model_id ,) , (:: aps_asset_compatibility_rules :: asset_compatibility_rules :: left_asset_model_id , :: aps_asset_compatibility_rules :: asset_compatibility_rules :: right_asset_model_id)))]
#[table_model(default(aps_ownables::ownables::ownable_table_id, "freezing_procedures"))]
# [diesel (table_name = freezing_procedures)]
pub struct FreezingProcedure {
    /// Identifier of the freezing id, which is also a foreign key to the
    /// general procedure.
    #[same_as(aps_procedures::procedures::id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// The template of this procedure_id should be a freezing procedure_id
    /// template.
    #[same_as(aps_procedures::procedures::procedure_template_id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    freezing_procedure_template_id: ::rosetta_uuid::Uuid,
    /// The container that is being frozen, which must be a volumetric
    /// container.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    frozen_container_id: ::rosetta_uuid::Uuid,
    /// The model of the container being frozen, which must be a container
    /// model.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::asset_model_id,
        procedure_frozen_container_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    frozen_container_model_id: ::rosetta_uuid::Uuid,
    /// The procedure_id template asset model associated to the
    /// `frozen_container`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::procedure_template_asset_model_id,
        procedure_frozen_container_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_frozen_container_model_id: ::rosetta_uuid::Uuid,
    /// The procedure_id asset associated to the `frozen_container`.
    #[discretionary(aps_procedure_asset_models::procedure_asset_models)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_frozen_container_id: ::rosetta_uuid::Uuid,
    /// The freezer used for the freezing procedure. This field is optional, as
    /// the freezer might not necessarily be tracked.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    frozen_with_id: Option<::rosetta_uuid::Uuid>,
    /// The model of the freezer used, which must be a freezer model.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::asset_model_id,
        procedure_frozen_with_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    frozen_with_model_id: ::rosetta_uuid::Uuid,
    /// The procedure_id template asset model associated to the
    /// `frozen_with_model`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::procedure_template_asset_model_id,
        procedure_frozen_with_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_frozen_with_model_id: ::rosetta_uuid::Uuid,
    /// The procedure_id asset associated to the `frozen_with`.
    #[discretionary(aps_procedure_asset_models::procedure_asset_models)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_frozen_with_id: ::rosetta_uuid::Uuid,
}
impl ::diesel_builders::GetColumn<aps_ownables::ownables::id> for FreezingProcedure {
    fn get_column_ref(
        &self,
    ) -> &<freezing_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_procedures::procedures::id> for FreezingProcedure {
    fn get_column_ref(
        &self,
    ) -> &<freezing_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl procedure_like::ProcedureTableLike for freezing_procedures::table {
    type ProcedureTemplateTable =
        aps_freezing_procedure_templates::freezing_procedure_templates::table;
}
