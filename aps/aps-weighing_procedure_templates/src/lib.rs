//! Auto-generated crate for the `weighing_procedure_templates` table.
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
/// Struct representing a row in the `weighing_procedure_templates` table.
#[table_model(ancestors(aps_procedure_templates::procedure_templates))]
#[table_model(default(
    aps_procedure_templates::procedure_templates::procedure_template_table_id,
    "weighing_procedure_templates"
))]
# [diesel (table_name = weighing_procedure_templates)]
pub struct WeighingProcedureTemplate {
    /// Field representing the `id` column in table
    /// `weighing_procedure_templates`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `weighed_asset_model_id` column in table
    /// `weighing_procedure_templates`.
    #[same_as(
        aps_procedure_template_asset_models::procedure_template_asset_models::asset_model_id,
        procedure_template_weighed_asset_model_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    weighed_asset_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_template_weighed_asset_model_id`
    /// column in table `weighing_procedure_templates`.
    #[discretionary(aps_procedure_template_asset_models::procedure_template_asset_models)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_weighed_asset_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `weighed_with_model_id` column in table
    /// `weighing_procedure_templates`.
    #[same_as(
        aps_procedure_template_asset_models::procedure_template_asset_models::asset_model_id,
        procedure_template_weighed_with_model_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    weighed_with_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_template_weighed_with_model_id` column
    /// in table `weighing_procedure_templates`.
    #[discretionary(aps_procedure_template_asset_models::procedure_template_asset_models)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_weighed_with_model_id: ::rosetta_uuid::Uuid,
}
::diesel_builders::prelude::unique_index!(
    weighing_procedure_templates::id,
    weighing_procedure_templates::procedure_template_weighed_asset_model_id
);
::diesel_builders::prelude::unique_index!(
    weighing_procedure_templates::id,
    weighing_procedure_templates::procedure_template_weighed_with_model_id
);
:: diesel_builders :: prelude :: fk ! ((weighing_procedure_templates :: id) -> (:: aps_procedure_templates :: procedure_templates :: id));
:: diesel_builders :: prelude :: fk ! ((weighing_procedure_templates :: weighed_asset_model_id) -> (:: aps_physical_asset_models :: physical_asset_models :: id));
:: diesel_builders :: prelude :: fk ! ((weighing_procedure_templates :: weighed_with_model_id) -> (:: aps_weighing_device_models :: weighing_device_models :: id));
:: diesel_builders :: prelude :: fk ! ((weighing_procedure_templates :: id , weighing_procedure_templates :: procedure_template_weighed_asset_model_id) -> (:: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_id , :: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_asset_model_id));
:: diesel_builders :: prelude :: fk ! ((weighing_procedure_templates :: id , weighing_procedure_templates :: procedure_template_weighed_with_model_id) -> (:: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_id , :: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_asset_model_id));
