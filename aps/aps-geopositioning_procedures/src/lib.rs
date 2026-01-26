//! Auto-generated crate for the `geopositioning_procedures` table.
#[derive(
    Clone,
    Debug,
    PartialEq,
    :: serde :: Serialize,
    :: serde :: Deserialize,
    :: diesel :: Queryable,
    :: diesel :: Selectable,
    :: diesel :: Identifiable,
    :: diesel :: Associations,
    :: diesel_builders :: prelude :: TableModel,
)]
/// Struct representing a row in the `geopositioning_procedures` table.
#[table_model(ancestors(aps_procedures::procedures))]
# [diesel (belongs_to (aps_physical_assets :: PhysicalAsset , foreign_key = geopositioned_asset_id))]
# [diesel (belongs_to (aps_physical_asset_models :: PhysicalAssetModel , foreign_key = geopositioned_asset_model_id))]
# [diesel (belongs_to (aps_geopositioning_devices :: GeopositioningDevice , foreign_key = geopositioned_with_id))]
# [diesel (belongs_to (aps_geopositioning_device_models :: GeopositioningDeviceModel , foreign_key = geopositioned_with_model_id))]
# [table_model (foreign_key ((id ,) , (:: aps_procedures :: procedures :: id)))]
# [table_model (foreign_key ((geopositioning_procedure_template_id ,) , (:: aps_geopositioning_procedure_templates :: geopositioning_procedure_templates :: id)))]
# [table_model (foreign_key ((geopositioned_asset_id ,) , (:: aps_physical_assets :: physical_assets :: id)))]
# [table_model (foreign_key ((geopositioned_asset_model_id ,) , (:: aps_physical_asset_models :: physical_asset_models :: id)))]
# [table_model (foreign_key ((procedure_template_geopositioned_asset_model_id ,) , (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id)))]
# [table_model (foreign_key ((geopositioned_with_id ,) , (:: aps_geopositioning_devices :: geopositioning_devices :: id)))]
# [table_model (foreign_key ((geopositioned_with_model_id ,) , (:: aps_geopositioning_device_models :: geopositioning_device_models :: id)))]
# [table_model (foreign_key ((procedure_template_geopositioned_with_model_id ,) , (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id)))]
#[table_model(default(
    aps_procedures::procedures::procedure_table_id,
    "geopositioning_procedures"
))]
# [diesel (table_name = geopositioning_procedures)]
pub struct GeopositioningProcedure {
    /// Identifier of the geopositioning id, which is also a foreign key to the
    /// general procedure.
    #[same_as(aps_procedures::procedures::id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// The template of this procedure_id should be a geopositioning
    /// procedure_id template.
    #[same_as(aps_procedures::procedures::procedure_template_id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    geopositioning_procedure_template_id: ::rosetta_uuid::Uuid,
    /// The asset being geopositioned, which must be a physical asset.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    geopositioned_asset_id: ::rosetta_uuid::Uuid,
    /// The model of the asset being geopositioned.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::asset_model_id,
        procedure_geopositioned_asset_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    geopositioned_asset_model_id: ::rosetta_uuid::Uuid,
    /// The procedure_id template asset model associated to the
    /// `geopositioned_asset`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::procedure_template_asset_model_id,
        procedure_geopositioned_asset_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_geopositioned_asset_model_id: ::rosetta_uuid::Uuid,
    /// The procedure_id asset associated to the `geopositioned_asset`.
    #[discretionary(aps_procedure_asset_models::procedure_asset_models)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_geopositioned_asset_id: ::rosetta_uuid::Uuid,
    /// The positioning device used for geopositioning. This field is optional,
    /// as the positioning device might not necessarily be tracked.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    geopositioned_with_id: Option<::rosetta_uuid::Uuid>,
    /// The model of the positioning device used for geopositioning.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::asset_model_id,
        procedure_geopositioned_with_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    geopositioned_with_model_id: ::rosetta_uuid::Uuid,
    /// The procedure_id asset associated to the `geopositioned_with`.
    #[discretionary(aps_procedure_asset_models::procedure_asset_models)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_geopositioned_with_id: ::rosetta_uuid::Uuid,
    /// The procedure_id template asset model associated to the
    /// `geopositioned_with_model`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::procedure_template_asset_model_id,
        procedure_geopositioned_with_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_geopositioned_with_model_id: ::rosetta_uuid::Uuid,
    /// The latitude and longitude of the geopositioning.
    # [diesel (sql_type = :: postgis_diesel :: sql_types :: Geography)]
    location: postgis_diesel::types::GeometryContainer<postgis_diesel::types::Point>,
}
impl ::diesel_builders::GetColumn<aps_procedures::procedures::id> for GeopositioningProcedure {
    fn get_column_ref(
        &self,
    ) -> &<geopositioning_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl procedure_like::ProcedureTableLike for geopositioning_procedures::table {
    type ProcedureTemplateTable =
        aps_geopositioning_procedure_templates::geopositioning_procedure_templates::table;
}
