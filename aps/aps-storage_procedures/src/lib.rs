//! Auto-generated crate for the `storage_procedures` table.
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
/// Struct representing a row in the `storage_procedures` table.
#[table_model(ancestors(aps_procedures::procedures))]
#[table_model(default(aps_procedures::procedures::procedure_table_id, "storage_procedures"))]
# [diesel (table_name = storage_procedures)]
pub struct StorageProcedure {
    /// Field representing the `id` column in table `storage_procedures`.
    #[same_as(aps_procedures::procedures::id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `storage_procedure_template_id` column in table
    /// `storage_procedures`.
    #[same_as(aps_procedures::procedures::procedure_template_id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    storage_procedure_template_id: ::rosetta_uuid::Uuid,
    /// Field representing the `stored_asset_id` column in table
    /// `storage_procedures`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    stored_asset_id: ::rosetta_uuid::Uuid,
    /// Field representing the `stored_asset_model_id` column in table
    /// `storage_procedures`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::asset_model_id,
        procedure_stored_asset_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    stored_asset_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_template_stored_asset_model_id` column
    /// in table `storage_procedures`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::procedure_template_asset_model_id,
        procedure_stored_asset_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_stored_asset_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_stored_asset_id` column in table
    /// `storage_procedures`.
    #[discretionary(aps_procedure_asset_models::procedure_asset_models)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_stored_asset_id: ::rosetta_uuid::Uuid,
    /// Field representing the `stored_into_id` column in table
    /// `storage_procedures`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    stored_into_id: ::rosetta_uuid::Uuid,
    /// Field representing the `stored_into_model_id` column in table
    /// `storage_procedures`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::asset_model_id,
        procedure_stored_into_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    stored_into_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_template_stored_into_model_id` column
    /// in table `storage_procedures`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::procedure_template_asset_model_id,
        procedure_stored_into_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_stored_into_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_stored_into_id` column in table
    /// `storage_procedures`.
    #[discretionary(aps_procedure_asset_models::procedure_asset_models)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_stored_into_id: ::rosetta_uuid::Uuid,
}
:: diesel_builders :: prelude :: fk ! ((storage_procedures :: id) -> (:: aps_procedures :: procedures :: id));
:: diesel_builders :: prelude :: fk ! ((storage_procedures :: storage_procedure_template_id) -> (:: aps_storage_procedure_templates :: storage_procedure_templates :: id));
:: diesel_builders :: prelude :: fk ! ((storage_procedures :: stored_asset_id) -> (:: aps_physical_assets :: physical_assets :: id));
:: diesel_builders :: prelude :: fk ! ((storage_procedures :: stored_asset_model_id) -> (:: aps_physical_asset_models :: physical_asset_models :: id));
:: diesel_builders :: prelude :: fk ! ((storage_procedures :: procedure_template_stored_asset_model_id) -> (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id));
:: diesel_builders :: prelude :: fk ! ((storage_procedures :: stored_into_id) -> (:: aps_containers :: containers :: id));
:: diesel_builders :: prelude :: fk ! ((storage_procedures :: stored_into_model_id) -> (:: aps_container_models :: container_models :: id));
:: diesel_builders :: prelude :: fk ! ((storage_procedures :: procedure_template_stored_into_model_id) -> (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id));
:: diesel_builders :: prelude :: fk ! ((storage_procedures :: stored_into_model_id , storage_procedures :: stored_asset_model_id) -> (:: aps_asset_compatibility_rules :: asset_compatibility_rules :: left_asset_model_id , :: aps_asset_compatibility_rules :: asset_compatibility_rules :: right_asset_model_id));
