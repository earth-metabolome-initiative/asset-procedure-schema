//! Auto-generated crate for the `harvesting_procedures` table.
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
/// Struct representing a row in the `harvesting_procedures` table.
#[table_model(ancestors(aps_procedures::procedures))]
#[table_model(default(aps_procedures::procedures::procedure_table_id, "harvesting_procedures"))]
# [diesel (table_name = harvesting_procedures)]
pub struct HarvestingProcedure {
    /// Field representing the `id` column in table `harvesting_procedures`.
    #[same_as(aps_procedures::procedures::id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `harvesting_procedure_template_id` column in
    /// table `harvesting_procedures`.
    #[same_as(aps_procedures::procedures::procedure_template_id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    harvesting_procedure_template_id: ::rosetta_uuid::Uuid,
    /// Field representing the `sample_source_id` column in table
    /// `harvesting_procedures`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    sample_source_id: ::rosetta_uuid::Uuid,
    /// Field representing the `sample_source_model_id` column in table
    /// `harvesting_procedures`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::asset_model_id,
        procedure_sample_source_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    sample_source_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_template_sample_source_model_id`
    /// column in table `harvesting_procedures`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::procedure_template_asset_model_id,
        procedure_sample_source_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_sample_source_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_sample_source_id` column in table
    /// `harvesting_procedures`.
    #[discretionary(aps_procedure_asset_models::procedure_asset_models)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_sample_source_id: ::rosetta_uuid::Uuid,
    /// Field representing the `sample_id` column in table
    /// `harvesting_procedures`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    sample_id: ::rosetta_uuid::Uuid,
    /// Field representing the `sample_model_id` column in table
    /// `harvesting_procedures`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::asset_model_id,
        procedure_sample_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    sample_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_template_sample_model_id` column in
    /// table `harvesting_procedures`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::procedure_template_asset_model_id,
        procedure_sample_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_sample_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_sample_id` column in table
    /// `harvesting_procedures`.
    #[discretionary(aps_procedure_asset_models::procedure_asset_models)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_sample_id: ::rosetta_uuid::Uuid,
}
:: diesel_builders :: prelude :: fk ! ((harvesting_procedures :: id) -> (:: aps_procedures :: procedures :: id));
:: diesel_builders :: prelude :: fk ! ((harvesting_procedures :: harvesting_procedure_template_id) -> (:: aps_harvesting_procedure_templates :: harvesting_procedure_templates :: id));
:: diesel_builders :: prelude :: fk ! ((harvesting_procedures :: sample_source_id) -> (:: aps_sample_sources :: sample_sources :: id));
:: diesel_builders :: prelude :: fk ! ((harvesting_procedures :: sample_source_model_id) -> (:: aps_sample_source_models :: sample_source_models :: id));
:: diesel_builders :: prelude :: fk ! ((harvesting_procedures :: procedure_template_sample_source_model_id) -> (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id));
:: diesel_builders :: prelude :: fk ! ((harvesting_procedures :: sample_id) -> (:: aps_samples :: samples :: id));
:: diesel_builders :: prelude :: fk ! ((harvesting_procedures :: sample_model_id) -> (:: aps_sample_models :: sample_models :: id));
:: diesel_builders :: prelude :: fk ! ((harvesting_procedures :: procedure_template_sample_model_id) -> (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id));
impl diesel_builders::GetColumn<aps_procedures::procedures::id> for HarvestingProcedure {
    fn get_column_ref(&self) -> &<harvesting_procedures::id as diesel_builders::Typed>::ColumnType {
        &self.id
    }
}
