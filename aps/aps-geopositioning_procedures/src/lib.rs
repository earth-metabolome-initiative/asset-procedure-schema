//! Auto-generated crate for the `geopositioning_procedures` table.
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Serialize,
    serde :: Deserialize,
    diesel :: Queryable,
    diesel :: Selectable,
    diesel :: Identifiable,
    diesel_builders :: prelude :: TableModel,
)]
/// Struct representing a row in the `geopositioning_procedures` table.
#[table_model(ancestors(aps_procedures::procedures))]
#[table_model(default(
    aps_procedures::procedures::procedure_table_id,
    "geopositioning_procedures"
))]
# [diesel (table_name = geopositioning_procedures)]
pub struct GeopositioningProcedure {
    /// Field representing the `id` column in table `geopositioning_procedures`.
    #[same_as(aps_procedures::procedures::id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `geopositioning_procedure_template_id` column in
    /// table `geopositioning_procedures`.
    #[same_as(aps_procedures::procedures::procedure_template_id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    geopositioning_procedure_template_id: ::rosetta_uuid::Uuid,
    /// Field representing the `geopositioned_asset_id` column in table
    /// `geopositioning_procedures`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    geopositioned_asset_id: ::rosetta_uuid::Uuid,
    /// Field representing the `geopositioned_asset_model_id` column in table
    /// `geopositioning_procedures`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::asset_model_id,
        procedure_geopositioned_asset_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    geopositioned_asset_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_template_geopositioned_asset_model_id`
    /// column in table `geopositioning_procedures`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::procedure_template_asset_model_id,
        procedure_geopositioned_asset_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_geopositioned_asset_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_geopositioned_asset_id` column in
    /// table `geopositioning_procedures`.
    #[discretionary(aps_procedure_asset_models::procedure_asset_models)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_geopositioned_asset_id: ::rosetta_uuid::Uuid,
    /// Field representing the `geopositioned_with_id` column in table
    /// `geopositioning_procedures`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    geopositioned_with_id: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `geopositioned_with_model_id` column in table
    /// `geopositioning_procedures`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::asset_model_id,
        procedure_geopositioned_with_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    geopositioned_with_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_geopositioned_with_id` column in table
    /// `geopositioning_procedures`.
    #[discretionary(aps_procedure_asset_models::procedure_asset_models)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_geopositioned_with_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_template_geopositioned_with_model_id`
    /// column in table `geopositioning_procedures`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::procedure_template_asset_model_id,
        procedure_geopositioned_with_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_geopositioned_with_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `location` column in table
    /// `geopositioning_procedures`.
    # [diesel (sql_type = :: postgis_diesel :: sql_types :: Geography)]
    location: postgis_diesel::types::GeometryContainer<postgis_diesel::types::Point>,
}
:: diesel_builders :: prelude :: fk ! ((geopositioning_procedures :: id) -> (:: aps_procedures :: procedures :: id));
:: diesel_builders :: prelude :: fk ! ((geopositioning_procedures :: geopositioning_procedure_template_id) -> (:: aps_geopositioning_procedure_templates :: geopositioning_procedure_templates :: id));
:: diesel_builders :: prelude :: fk ! ((geopositioning_procedures :: geopositioned_asset_id) -> (:: aps_physical_assets :: physical_assets :: id));
:: diesel_builders :: prelude :: fk ! ((geopositioning_procedures :: geopositioned_asset_model_id) -> (:: aps_physical_asset_models :: physical_asset_models :: id));
:: diesel_builders :: prelude :: fk ! ((geopositioning_procedures :: procedure_template_geopositioned_asset_model_id) -> (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id));
:: diesel_builders :: prelude :: fk ! ((geopositioning_procedures :: geopositioned_with_id) -> (:: aps_geopositioning_devices :: geopositioning_devices :: id));
:: diesel_builders :: prelude :: fk ! ((geopositioning_procedures :: geopositioned_with_model_id) -> (:: aps_geopositioning_device_models :: geopositioning_device_models :: id));
:: diesel_builders :: prelude :: fk ! ((geopositioning_procedures :: procedure_template_geopositioned_with_model_id) -> (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id));
impl diesel_builders::GetColumn<aps_procedures::procedures::id> for GeopositioningProcedure {
    fn get_column_ref(
        &self,
    ) -> &<geopositioning_procedures::id as diesel_builders::Typed>::ColumnType {
        &self.id
    }
}
