//! Auto-generated crate for the `reused_procedure_template_asset_models` table.
#[derive(
    Copy,
    Clone,
    Eq,
    PartialEq,
    serde :: Serialize,
    serde :: Deserialize,
    diesel :: Queryable,
    diesel :: Selectable,
    diesel :: Identifiable,
    diesel_builders :: prelude :: TableModel,
)]
/// Undocumented table
#[diesel(primary_key(procedure_template_asset_model_id, procedure_template_id))]
# [diesel (table_name = reused_procedure_template_asset_models)]
pub struct ReusedProcedureTemplateAssetModel {
    /// Undocumented column
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_asset_model_id: ::rosetta_uuid::Uuid,
    /// Undocumented column
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_id: ::rosetta_uuid::Uuid,
}
:: diesel_builders :: prelude :: fk ! ((reused_procedure_template_asset_models :: procedure_template_asset_model_id) -> (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id));
:: diesel_builders :: prelude :: fk ! ((reused_procedure_template_asset_models :: procedure_template_id) -> (:: aps_procedure_templates :: procedure_templates :: id));
