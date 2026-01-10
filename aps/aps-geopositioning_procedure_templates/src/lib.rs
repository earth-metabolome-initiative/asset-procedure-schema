//! Auto-generated crate for the `geopositioning_procedure_templates` table.
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
/// Struct representing a row in the `geopositioning_procedure_templates` table.
#[table_model(ancestors(aps_procedure_templates::procedure_templates))]
#[table_model(default(
    aps_procedure_templates::procedure_templates::procedure_template_table_id,
    "geopositioning_procedure_templates"
))]
# [diesel (table_name = geopositioning_procedure_templates)]
pub struct GeopositioningProcedureTemplate {
    /// Field representing the `id` column in table
    /// `geopositioning_procedure_templates`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `geopositioned_with_model_id` column in table
    /// `geopositioning_procedure_templates`.
    #[same_as(
        aps_procedure_template_asset_models::procedure_template_asset_models::asset_model_id,
        procedure_template_geopositioned_with_model_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    geopositioned_with_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_template_geopositioned_with_model_id`
    /// column in table `geopositioning_procedure_templates`.
    #[discretionary(aps_procedure_template_asset_models::procedure_template_asset_models)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_geopositioned_with_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `geopositioned_asset_model_id` column in table
    /// `geopositioning_procedure_templates`.
    #[same_as(
        aps_procedure_template_asset_models::procedure_template_asset_models::asset_model_id,
        procedure_template_geopositioned_asset_model_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    geopositioned_asset_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_template_geopositioned_asset_model_id`
    /// column in table `geopositioning_procedure_templates`.
    #[discretionary(aps_procedure_template_asset_models::procedure_template_asset_models)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_geopositioned_asset_model_id: ::rosetta_uuid::Uuid,
}
::diesel_builders::prelude::unique_index!(
    geopositioning_procedure_templates::id,
    geopositioning_procedure_templates::procedure_template_geopositioned_with_model_id
);
::diesel_builders::prelude::unique_index!(
    geopositioning_procedure_templates::id,
    geopositioning_procedure_templates::procedure_template_geopositioned_asset_model_id
);
:: diesel_builders :: prelude :: fk ! ((geopositioning_procedure_templates :: id) -> (:: aps_procedure_templates :: procedure_templates :: id));
:: diesel_builders :: prelude :: fk ! ((geopositioning_procedure_templates :: geopositioned_with_model_id) -> (:: aps_geopositioning_device_models :: geopositioning_device_models :: id));
:: diesel_builders :: prelude :: fk ! ((geopositioning_procedure_templates :: geopositioned_asset_model_id) -> (:: aps_physical_asset_models :: physical_asset_models :: id));
:: diesel_builders :: prelude :: fk ! ((geopositioning_procedure_templates :: id , geopositioning_procedure_templates :: procedure_template_geopositioned_with_model_id) -> (:: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_id , :: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_asset_model_id));
:: diesel_builders :: prelude :: fk ! ((geopositioning_procedure_templates :: id , geopositioning_procedure_templates :: procedure_template_geopositioned_asset_model_id) -> (:: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_id , :: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_asset_model_id));
impl diesel_builders::GetColumn<aps_procedure_templates::procedure_templates::id>
    for GeopositioningProcedureTemplate
{
    fn get_column_ref(
        &self,
    ) -> &<geopositioning_procedure_templates::id as diesel_builders::Typed>::ColumnType {
        &self.id
    }
}
