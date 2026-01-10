//! Auto-generated crate for the `photograph_procedures` table.
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
/// Struct representing a row in the `photograph_procedures` table.
#[table_model(ancestors(aps_procedures::procedures))]
#[table_model(default(aps_procedures::procedures::procedure_table_id, "photograph_procedures"))]
# [diesel (table_name = photograph_procedures)]
pub struct PhotographProcedure {
    /// Field representing the `id` column in table `photograph_procedures`.
    #[same_as(aps_procedures::procedures::id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `photograph_procedure_template_id` column in
    /// table `photograph_procedures`.
    #[same_as(aps_procedures::procedures::procedure_template_id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    photograph_procedure_template_id: ::rosetta_uuid::Uuid,
    /// Field representing the `photographed_asset_id` column in table
    /// `photograph_procedures`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    photographed_asset_id: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `photographed_asset_model_id` column in table
    /// `photograph_procedures`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    photographed_asset_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_template_photographed_asset_model_id`
    /// column in table `photograph_procedures`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::procedure_template_asset_model_id,
        procedure_photographed_asset_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_photographed_asset_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_photographed_asset_id` column in table
    /// `photograph_procedures`.
    #[discretionary(aps_procedure_asset_models::procedure_asset_models)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_photographed_asset_id: ::rosetta_uuid::Uuid,
    /// Field representing the `photographed_with_id` column in table
    /// `photograph_procedures`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    photographed_with_id: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `photographed_with_model_id` column in table
    /// `photograph_procedures`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    photographed_with_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_template_photographed_with_model_id`
    /// column in table `photograph_procedures`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::procedure_template_asset_model_id,
        procedure_photographed_with_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_photographed_with_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_photographed_with_id` column in table
    /// `photograph_procedures`.
    #[discretionary(aps_procedure_asset_models::procedure_asset_models)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_photographed_with_id: ::rosetta_uuid::Uuid,
    /// Field representing the `photograph_id` column in table
    /// `photograph_procedures`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    photograph_id: ::rosetta_uuid::Uuid,
    /// Field representing the `photograph_model_id` column in table
    /// `photograph_procedures`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    photograph_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_template_photograph_model_id` column
    /// in table `photograph_procedures`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::procedure_template_asset_model_id,
        procedure_photograph_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_photograph_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_photograph_id` column in table
    /// `photograph_procedures`.
    #[discretionary(aps_procedure_asset_models::procedure_asset_models)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_photograph_id: ::rosetta_uuid::Uuid,
}
:: diesel_builders :: prelude :: fk ! ((photograph_procedures :: id) -> (:: aps_procedures :: procedures :: id));
:: diesel_builders :: prelude :: fk ! ((photograph_procedures :: photograph_procedure_template_id) -> (:: aps_photograph_procedure_templates :: photograph_procedure_templates :: id));
:: diesel_builders :: prelude :: fk ! ((photograph_procedures :: photographed_asset_id) -> (:: aps_physical_assets :: physical_assets :: id));
:: diesel_builders :: prelude :: fk ! ((photograph_procedures :: photographed_asset_model_id) -> (:: aps_physical_asset_models :: physical_asset_models :: id));
:: diesel_builders :: prelude :: fk ! ((photograph_procedures :: procedure_template_photographed_asset_model_id) -> (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id));
:: diesel_builders :: prelude :: fk ! ((photograph_procedures :: photographed_with_id) -> (:: aps_cameras :: cameras :: id));
:: diesel_builders :: prelude :: fk ! ((photograph_procedures :: photographed_with_model_id) -> (:: aps_camera_models :: camera_models :: id));
:: diesel_builders :: prelude :: fk ! ((photograph_procedures :: procedure_template_photographed_with_model_id) -> (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id));
:: diesel_builders :: prelude :: fk ! ((photograph_procedures :: photograph_id) -> (:: aps_photographs :: photographs :: id));
:: diesel_builders :: prelude :: fk ! ((photograph_procedures :: photograph_model_id) -> (:: aps_digital_asset_models :: digital_asset_models :: id));
:: diesel_builders :: prelude :: fk ! ((photograph_procedures :: procedure_template_photograph_model_id) -> (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id));
impl diesel_builders::GetColumn<aps_procedures::procedures::id> for PhotographProcedure {
    fn get_column_ref(&self) -> &<photograph_procedures::id as diesel_builders::Typed>::ColumnType {
        &self.id
    }
}
