//! Auto-generated crate for the `centrifuge_procedures` table.
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
/// Struct representing a row in the `centrifuge_procedures` table.
#[table_model(ancestors(
    aps_entities::entities,
    aps_ownables::ownables,
    aps_procedures::procedures
))]
# [diesel (belongs_to (aps_volumetric_containers :: VolumetricContainer , foreign_key = centrifuged_container_id))]
# [diesel (belongs_to (aps_volumetric_container_models :: VolumetricContainerModel , foreign_key = centrifuged_container_model_id))]
# [diesel (belongs_to (aps_centrifuge_models :: CentrifugeModel , foreign_key = centrifuged_with_model_id))]
# [diesel (belongs_to (aps_centrifuges :: Centrifuge , foreign_key = centrifuged_with_id))]
# [table_model (foreign_key ((id ,) , (:: aps_procedures :: procedures :: id)))]
# [table_model (foreign_key ((centrifuge_procedure_template_id ,) , (:: aps_centrifuge_procedure_templates :: centrifuge_procedure_templates :: id)))]
# [table_model (foreign_key ((centrifuged_container_id ,) , (:: aps_volumetric_containers :: volumetric_containers :: id)))]
# [table_model (foreign_key ((centrifuged_container_model_id ,) , (:: aps_volumetric_container_models :: volumetric_container_models :: id)))]
# [table_model (foreign_key ((procedure_template_centrifuged_container_model_id ,) , (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id)))]
# [table_model (foreign_key ((centrifuged_with_model_id ,) , (:: aps_centrifuge_models :: centrifuge_models :: id)))]
# [table_model (foreign_key ((centrifuged_with_id ,) , (:: aps_centrifuges :: centrifuges :: id)))]
# [table_model (foreign_key ((procedure_template_centrifuged_with_model_id ,) , (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id)))]
# [table_model (foreign_key ((centrifuged_with_model_id , centrifuged_container_model_id ,) , (:: aps_asset_compatibility_rules :: asset_compatibility_rules :: left_asset_model_id , :: aps_asset_compatibility_rules :: asset_compatibility_rules :: right_asset_model_id)))]
#[table_model(default(aps_entities::entities::table_name_id, "centrifuge_procedures"))]
# [diesel (table_name = centrifuge_procedures)]
pub struct CentrifugeProcedure {
    /// Identifier of the centrifuge id, which is also a foreign key to the
    /// general procedure.
    #[same_as(aps_procedures::procedures::id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// We enforce that the model of this procedure_id must be a centrifuge
    /// procedure_id template.
    #[same_as(aps_procedures::procedures::procedure_template_id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    centrifuge_procedure_template_id: ::rosetta_uuid::Uuid,
    /// The container that is being centrifuged, which must be a volumetric
    /// container.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    centrifuged_container_id: ::rosetta_uuid::Uuid,
    /// The model of the container that is being centrifuged.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::asset_model_id,
        procedure_centrifuged_container_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    centrifuged_container_model_id: ::rosetta_uuid::Uuid,
    /// The procedure_id template asset model associated to the
    /// `centrifuged_container`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::procedure_template_asset_model_id,
        procedure_centrifuged_container_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_centrifuged_container_model_id: ::rosetta_uuid::Uuid,
    /// The procedure_id asset associated to the `centrifuged_container`.
    #[discretionary(aps_procedure_asset_models::procedure_asset_models)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_centrifuged_container_id: ::rosetta_uuid::Uuid,
    /// The centrifuge model used for the centrifuge procedure.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::asset_model_id,
        procedure_centrifuged_with_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    centrifuged_with_model_id: ::rosetta_uuid::Uuid,
    /// might not have been recorded at the time of performing the procedure.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    centrifuged_with_id: Option<::rosetta_uuid::Uuid>,
    /// The procedure_id template asset model associated to the
    /// `centrifuged_with_model`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::procedure_template_asset_model_id,
        procedure_centrifuged_with_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_centrifuged_with_model_id: ::rosetta_uuid::Uuid,
    /// The procedure_id asset associated to the `centrifuged_with`.
    #[discretionary(aps_procedure_asset_models::procedure_asset_models)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_centrifuged_with_id: ::rosetta_uuid::Uuid,
}
impl ::diesel_builders::GetColumn<aps_entities::entities::id> for CentrifugeProcedure {
    fn get_column_ref(
        &self,
    ) -> &<centrifuge_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_ownables::ownables::id> for CentrifugeProcedure {
    fn get_column_ref(
        &self,
    ) -> &<centrifuge_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_procedures::procedures::id> for CentrifugeProcedure {
    fn get_column_ref(
        &self,
    ) -> &<centrifuge_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl procedure_like::ProcedureTableLike for centrifuge_procedures::table {
    type ProcedureTemplateTable =
        aps_centrifuge_procedure_templates::centrifuge_procedure_templates::table;
}
