//! Auto-generated crate for the `packaging_procedures` table.
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
/// Struct representing a row in the `packaging_procedures` table.
#[table_model(ancestors(aps_procedures::procedures))]
#[table_model(default(aps_procedures::procedures::procedure_table_id, "packaging_procedures"))]
# [diesel (table_name = packaging_procedures)]
pub struct PackagingProcedure {
    /// Field representing the `id` column in table `packaging_procedures`.
    #[same_as(aps_procedures::procedures::id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `packaging_procedure_template_id` column in table
    /// `packaging_procedures`.
    #[same_as(aps_procedures::procedures::procedure_template_id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    packaging_procedure_template_id: ::rosetta_uuid::Uuid,
    /// Field representing the `sample_id` column in table
    /// `packaging_procedures`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    sample_id: ::rosetta_uuid::Uuid,
    /// Field representing the `sample_model_id` column in table
    /// `packaging_procedures`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::asset_model_id,
        procedure_sample_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    sample_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_template_sample_model_id` column in
    /// table `packaging_procedures`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::procedure_template_asset_model_id,
        procedure_sample_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_sample_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_sample_id` column in table
    /// `packaging_procedures`.
    #[discretionary(aps_procedure_asset_models::procedure_asset_models)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_sample_id: ::rosetta_uuid::Uuid,
    /// Field representing the `packaged_with_model_id` column in table
    /// `packaging_procedures`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::asset_model_id,
        procedure_packaged_with_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    packaged_with_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_template_packaged_with_model_id`
    /// column in table `packaging_procedures`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::procedure_template_asset_model_id,
        procedure_packaged_with_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_packaged_with_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_packaged_with_id` column in table
    /// `packaging_procedures`.
    #[discretionary(aps_procedure_asset_models::procedure_asset_models)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_packaged_with_id: ::rosetta_uuid::Uuid,
}
:: diesel_builders :: prelude :: fk ! ((packaging_procedures :: id) -> (:: aps_procedures :: procedures :: id));
:: diesel_builders :: prelude :: fk ! ((packaging_procedures :: packaging_procedure_template_id) -> (:: aps_packaging_procedure_templates :: packaging_procedure_templates :: id));
:: diesel_builders :: prelude :: fk ! ((packaging_procedures :: sample_id) -> (:: aps_physical_assets :: physical_assets :: id));
:: diesel_builders :: prelude :: fk ! ((packaging_procedures :: sample_model_id) -> (:: aps_physical_asset_models :: physical_asset_models :: id));
:: diesel_builders :: prelude :: fk ! ((packaging_procedures :: procedure_template_sample_model_id) -> (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id));
:: diesel_builders :: prelude :: fk ! ((packaging_procedures :: packaged_with_model_id) -> (:: aps_packaging_models :: packaging_models :: id));
:: diesel_builders :: prelude :: fk ! ((packaging_procedures :: procedure_template_packaged_with_model_id) -> (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id));
:: diesel_builders :: prelude :: fk ! ((packaging_procedures :: packaged_with_model_id , packaging_procedures :: sample_model_id) -> (:: aps_asset_compatibility_rules :: asset_compatibility_rules :: left_asset_model_id , :: aps_asset_compatibility_rules :: asset_compatibility_rules :: right_asset_model_id));
impl diesel_builders::GetColumn<aps_procedures::procedures::id> for PackagingProcedure {
    fn get_column_ref(&self) -> &<packaging_procedures::id as diesel_builders::Typed>::ColumnType {
        &self.id
    }
}
