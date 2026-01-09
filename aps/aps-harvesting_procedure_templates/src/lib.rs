//! Auto-generated crate for the `harvesting_procedure_templates` table.
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
/// Struct representing a row in the `harvesting_procedure_templates` table.
#[table_model(ancestors(aps_procedure_templates::procedure_templates))]
#[table_model(default(
    aps_procedure_templates::procedure_templates::procedure_template_table_id,
    "harvesting_procedure_templates"
))]
# [diesel (table_name = harvesting_procedure_templates)]
pub struct HarvestingProcedureTemplate {
    /// Field representing the `id` column in table
    /// `harvesting_procedure_templates`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `sample_source_model_id` column in table
    /// `harvesting_procedure_templates`.
    #[same_as(
        aps_procedure_template_asset_models::procedure_template_asset_models::asset_model_id,
        procedure_template_sample_source_model_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    sample_source_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_template_sample_source_model_id`
    /// column in table `harvesting_procedure_templates`.
    #[discretionary(aps_procedure_template_asset_models::procedure_template_asset_models)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_sample_source_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `sample_model_id` column in table
    /// `harvesting_procedure_templates`.
    #[same_as(
        aps_procedure_template_asset_models::procedure_template_asset_models::asset_model_id,
        procedure_template_sample_model_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    sample_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_template_sample_model_id` column in
    /// table `harvesting_procedure_templates`.
    #[discretionary(aps_procedure_template_asset_models::procedure_template_asset_models)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_sample_model_id: ::rosetta_uuid::Uuid,
}
::diesel_builders::prelude::unique_index!(
    harvesting_procedure_templates::id,
    harvesting_procedure_templates::procedure_template_sample_source_model_id
);
::diesel_builders::prelude::unique_index!(
    harvesting_procedure_templates::id,
    harvesting_procedure_templates::procedure_template_sample_model_id
);
:: diesel_builders :: prelude :: fk ! ((harvesting_procedure_templates :: id) -> (:: aps_procedure_templates :: procedure_templates :: id));
:: diesel_builders :: prelude :: fk ! ((harvesting_procedure_templates :: sample_source_model_id) -> (:: aps_sample_source_models :: sample_source_models :: id));
:: diesel_builders :: prelude :: fk ! ((harvesting_procedure_templates :: sample_model_id) -> (:: aps_sample_models :: sample_models :: id));
