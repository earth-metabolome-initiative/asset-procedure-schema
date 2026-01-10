//! Auto-generated crate for the `disposal_procedures` table.
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
/// Struct representing a row in the `disposal_procedures` table.
#[table_model(ancestors(aps_procedures::procedures))]
#[table_model(default(aps_procedures::procedures::procedure_table_id, "disposal_procedures"))]
# [diesel (table_name = disposal_procedures)]
pub struct DisposalProcedure {
    /// Field representing the `id` column in table `disposal_procedures`.
    #[same_as(aps_procedures::procedures::id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `disposal_procedure_template_id` column in table
    /// `disposal_procedures`.
    #[same_as(aps_procedures::procedures::procedure_template_id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    disposal_procedure_template_id: ::rosetta_uuid::Uuid,
    /// Field representing the `disposed_asset_id` column in table
    /// `disposal_procedures`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    disposed_asset_id: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `disposed_asset_model_id` column in table
    /// `disposal_procedures`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::asset_model_id,
        procedure_disposed_asset_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    disposed_asset_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_template_disposed_asset_model_id`
    /// column in table `disposal_procedures`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::procedure_template_asset_model_id,
        procedure_disposed_asset_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_disposed_asset_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_disposed_asset_id` column in table
    /// `disposal_procedures`.
    #[discretionary(aps_procedure_asset_models::procedure_asset_models)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_disposed_asset_id: ::rosetta_uuid::Uuid,
}
:: diesel_builders :: prelude :: fk ! ((disposal_procedures :: id) -> (:: aps_procedures :: procedures :: id));
:: diesel_builders :: prelude :: fk ! ((disposal_procedures :: disposal_procedure_template_id) -> (:: aps_disposal_procedure_templates :: disposal_procedure_templates :: id));
:: diesel_builders :: prelude :: fk ! ((disposal_procedures :: disposed_asset_id) -> (:: aps_physical_assets :: physical_assets :: id));
:: diesel_builders :: prelude :: fk ! ((disposal_procedures :: disposed_asset_model_id) -> (:: aps_physical_asset_models :: physical_asset_models :: id));
:: diesel_builders :: prelude :: fk ! ((disposal_procedures :: procedure_template_disposed_asset_model_id) -> (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id));
impl diesel_builders::GetColumn<aps_procedures::procedures::id> for DisposalProcedure {
    fn get_column_ref(&self) -> &<disposal_procedures::id as diesel_builders::Typed>::ColumnType {
        &self.id
    }
}
