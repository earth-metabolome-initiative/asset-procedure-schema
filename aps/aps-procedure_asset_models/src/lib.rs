//! Auto-generated crate for the `procedure_asset_models` table.
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
# [diesel (table_name = procedure_asset_models)]
pub struct ProcedureAssetModel {
    /// Undocumented column
    # [table_model (default = :: rosetta_uuid :: Uuid :: new_v4 ())]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Undocumented column
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_id: ::rosetta_uuid::Uuid,
    /// Undocumented column
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_id: ::rosetta_uuid::Uuid,
    /// Undocumented column
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    asset_model_id: ::rosetta_uuid::Uuid,
    /// Undocumented column
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_asset_model_id: ::rosetta_uuid::Uuid,
    /// Undocumented column
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    ancestor_model_id: ::rosetta_uuid::Uuid,
}
::diesel_builders::prelude::unique_index!(
    procedure_asset_models::id,
    procedure_asset_models::procedure_template_asset_model_id
);
::diesel_builders::prelude::unique_index!(
    procedure_asset_models::id,
    procedure_asset_models::asset_model_id
);
:: diesel_builders :: prelude :: fk ! ((procedure_asset_models :: procedure_id) -> (:: aps_procedures :: procedures :: id));
:: diesel_builders :: prelude :: fk ! ((procedure_asset_models :: procedure_template_id) -> (:: aps_procedure_templates :: procedure_templates :: id));
:: diesel_builders :: prelude :: fk ! ((procedure_asset_models :: asset_model_id) -> (:: aps_asset_models :: asset_models :: id));
:: diesel_builders :: prelude :: fk ! ((procedure_asset_models :: procedure_template_asset_model_id) -> (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id));
:: diesel_builders :: prelude :: fk ! ((procedure_asset_models :: ancestor_model_id) -> (:: aps_asset_models :: asset_models :: id));
:: diesel_builders :: prelude :: fk ! ((procedure_asset_models :: asset_model_id , procedure_asset_models :: ancestor_model_id) -> (:: aps_asset_model_ancestors :: asset_model_ancestors :: descendant_model_id , :: aps_asset_model_ancestors :: asset_model_ancestors :: ancestor_model_id));
