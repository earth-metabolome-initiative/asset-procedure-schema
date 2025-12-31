//! Auto-generated crate for the `packaging_procedure_templates` table.
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
/// Undocumented table
#[table_model(ancestors(aps_procedure_templates::procedure_templates))]
# [diesel (table_name = packaging_procedure_templates)]
pub struct PackagingProcedureTemplate {
    /// Undocumented column
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Undocumented column
    #[same_as(
        aps_procedure_template_asset_models::procedure_template_asset_models::asset_model_id,
        procedure_template_packaged_with_model_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    packaged_with_model_id: ::rosetta_uuid::Uuid,
    /// Undocumented column
    #[discretionary(aps_procedure_template_asset_models::procedure_template_asset_models)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_packaged_with_model_id: ::rosetta_uuid::Uuid,
    /// Undocumented column
    #[same_as(
        aps_procedure_template_asset_models::procedure_template_asset_models::asset_model_id,
        procedure_template_sample_model_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    sample_model_id: ::rosetta_uuid::Uuid,
    /// Undocumented column
    #[discretionary(aps_procedure_template_asset_models::procedure_template_asset_models)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_sample_model_id: ::rosetta_uuid::Uuid,
}
::diesel_builders::prelude::unique_index!(
    packaging_procedure_templates::id,
    packaging_procedure_templates::procedure_template_packaged_with_model_id
);
::diesel_builders::prelude::unique_index!(
    packaging_procedure_templates::id,
    packaging_procedure_templates::procedure_template_sample_model_id
);
:: diesel_builders :: prelude :: fk ! ((packaging_procedure_templates :: id) -> (:: aps_procedure_templates :: procedure_templates :: id));
:: diesel_builders :: prelude :: fk ! ((packaging_procedure_templates :: packaged_with_model_id) -> (:: aps_packaging_models :: packaging_models :: id));
:: diesel_builders :: prelude :: fk ! ((packaging_procedure_templates :: sample_model_id) -> (:: aps_physical_asset_models :: physical_asset_models :: id));
:: diesel_builders :: prelude :: fk ! ((packaging_procedure_templates :: packaged_with_model_id , packaging_procedure_templates :: sample_model_id) -> (:: aps_asset_compatibility_rules :: asset_compatibility_rules :: left_asset_model_id , :: aps_asset_compatibility_rules :: asset_compatibility_rules :: right_asset_model_id));
