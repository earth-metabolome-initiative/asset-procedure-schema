//! Auto-generated crate for the `photograph_procedures` table.
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
/// Struct representing a row in the `photograph_procedures` table.
#[table_model(ancestors(aps_procedures::procedures))]
# [diesel (belongs_to (aps_physical_assets :: PhysicalAsset , foreign_key = photographed_asset_id))]
# [diesel (belongs_to (aps_physical_asset_models :: PhysicalAssetModel , foreign_key = photographed_asset_model_id))]
# [diesel (belongs_to (aps_cameras :: Camera , foreign_key = photographed_with_id))]
# [diesel (belongs_to (aps_camera_models :: CameraModel , foreign_key = photographed_with_model_id))]
# [diesel (belongs_to (aps_photographs :: Photograph , foreign_key = photograph_id))]
# [diesel (belongs_to (aps_digital_asset_models :: DigitalAssetModel , foreign_key = photograph_model_id))]
# [table_model (foreign_key ((id ,) , (:: aps_procedures :: procedures :: id)))]
# [table_model (foreign_key ((photograph_procedure_template_id ,) , (:: aps_photograph_procedure_templates :: photograph_procedure_templates :: id)))]
# [table_model (foreign_key ((photographed_asset_id ,) , (:: aps_physical_assets :: physical_assets :: id)))]
# [table_model (foreign_key ((photographed_asset_model_id ,) , (:: aps_physical_asset_models :: physical_asset_models :: id)))]
# [table_model (foreign_key ((procedure_template_photographed_asset_model_id ,) , (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id)))]
# [table_model (foreign_key ((photographed_with_id ,) , (:: aps_cameras :: cameras :: id)))]
# [table_model (foreign_key ((photographed_with_model_id ,) , (:: aps_camera_models :: camera_models :: id)))]
# [table_model (foreign_key ((procedure_template_photographed_with_model_id ,) , (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id)))]
# [table_model (foreign_key ((photograph_id ,) , (:: aps_photographs :: photographs :: id)))]
# [table_model (foreign_key ((photograph_model_id ,) , (:: aps_digital_asset_models :: digital_asset_models :: id)))]
# [table_model (foreign_key ((procedure_template_photograph_model_id ,) , (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id)))]
#[table_model(default(aps_procedures::procedures::procedure_table_id, "photograph_procedures"))]
# [diesel (table_name = photograph_procedures)]
pub struct PhotographProcedure {
    /// Identifier of the photograph_id id, which is also a foreign key to the
    /// general procedure.
    #[same_as(aps_procedures::procedures::id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// The template of this procedure_id should be a photograph_id procedure_id
    /// template.
    #[same_as(aps_procedures::procedures::procedure_template_id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    photograph_procedure_template_id: ::rosetta_uuid::Uuid,
    /// The asset being photographed, which must be a physical asset.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    photographed_asset_id: Option<::rosetta_uuid::Uuid>,
    /// The model of the asset being photographed.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    photographed_asset_model_id: ::rosetta_uuid::Uuid,
    /// The procedure_id template asset model associated to the
    /// `photographed_asset`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::procedure_template_asset_model_id,
        procedure_photographed_asset_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_photographed_asset_model_id: ::rosetta_uuid::Uuid,
    /// The procedure_id asset associated to the `photographed_asset`.
    #[discretionary(aps_procedure_asset_models::procedure_asset_models)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_photographed_asset_id: ::rosetta_uuid::Uuid,
    /// The positioning device used for photograph. This field is optional, as
    /// the positioning device might not necessarily be tracked.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    photographed_with_id: Option<::rosetta_uuid::Uuid>,
    /// The model of the positioning device used for photograph.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    photographed_with_model_id: ::rosetta_uuid::Uuid,
    /// The procedure_id template asset model associated to the
    /// `photographed_with_model`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::procedure_template_asset_model_id,
        procedure_photographed_with_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_photographed_with_model_id: ::rosetta_uuid::Uuid,
    /// The procedure_id asset associated to the `photographed_with`.
    #[discretionary(aps_procedure_asset_models::procedure_asset_models)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_photographed_with_id: ::rosetta_uuid::Uuid,
    /// The resulting photograph.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    photograph_id: ::rosetta_uuid::Uuid,
    /// The model of the resulting photograph.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    photograph_model_id: ::rosetta_uuid::Uuid,
    /// The procedure_id template asset model associated to the
    /// `photograph_model`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::procedure_template_asset_model_id,
        procedure_photograph_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_photograph_model_id: ::rosetta_uuid::Uuid,
    /// The procedure_id asset associated to the `photograph`.
    #[discretionary(aps_procedure_asset_models::procedure_asset_models)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_photograph_id: ::rosetta_uuid::Uuid,
}
impl ::diesel_builders::GetColumn<aps_procedures::procedures::id> for PhotographProcedure {
    fn get_column_ref(
        &self,
    ) -> &<photograph_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl procedure_like::ProcedureTableLike for photograph_procedures::table {
    type ProcedureTemplateTable =
        aps_photograph_procedure_templates::photograph_procedure_templates::table;
}
