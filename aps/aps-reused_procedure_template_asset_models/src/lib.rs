//! Auto-generated crate for the `reused_procedure_template_asset_models` table.
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
/// Struct representing a row in the `reused_procedure_template_asset_models`
/// table.
#[diesel(primary_key(procedure_template_id, procedure_template_asset_model_id))]
# [diesel (table_name = reused_procedure_template_asset_models)]
pub struct ReusedProcedureTemplateAssetModel {
    /// Field representing the `procedure_template_id` column in table
    /// `reused_procedure_template_asset_models`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_template_asset_model_id` column in
    /// table `reused_procedure_template_asset_models`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_asset_model_id: ::rosetta_uuid::Uuid,
}
:: diesel_builders :: prelude :: fk ! ((reused_procedure_template_asset_models :: procedure_template_id) -> (:: aps_procedure_templates :: procedure_templates :: id));
:: diesel_builders :: prelude :: fk ! ((reused_procedure_template_asset_models :: procedure_template_asset_model_id) -> (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id));
