//! Auto-generated crate for the `freeze_drying_procedures` table.
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
/// Struct representing a row in the `freeze_drying_procedures` table.
#[table_model(ancestors(aps_procedures::procedures))]
#[table_model(default(aps_procedures::procedures::procedure_table_id, "freeze_drying_procedures"))]
# [diesel (table_name = freeze_drying_procedures)]
pub struct FreezeDryingProcedure {
    /// Field representing the `id` column in table `freeze_drying_procedures`.
    #[same_as(aps_procedures::procedures::id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `freeze_drying_procedure_template_id` column in
    /// table `freeze_drying_procedures`.
    #[same_as(aps_procedures::procedures::procedure_template_id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    freeze_drying_procedure_template_id: ::rosetta_uuid::Uuid,
    /// Field representing the `freeze_dried_container_id` column in table
    /// `freeze_drying_procedures`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    freeze_dried_container_id: ::rosetta_uuid::Uuid,
    /// Field representing the `freeze_dried_container_model_id` column in table
    /// `freeze_drying_procedures`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::asset_model_id,
        procedure_freeze_dried_container_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    freeze_dried_container_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the
    /// `procedure_template_freeze_dried_container_model_id` column in table
    /// `freeze_drying_procedures`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::procedure_template_asset_model_id,
        procedure_freeze_dried_container_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_freeze_dried_container_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_freeze_dried_container_id` column in
    /// table `freeze_drying_procedures`.
    #[discretionary(aps_procedure_asset_models::procedure_asset_models)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_freeze_dried_container_id: ::rosetta_uuid::Uuid,
    /// Field representing the `freeze_dried_with_id` column in table
    /// `freeze_drying_procedures`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    freeze_dried_with_id: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `freeze_dried_with_model_id` column in table
    /// `freeze_drying_procedures`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::asset_model_id,
        procedure_freeze_dried_with_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    freeze_dried_with_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_template_freeze_dried_with_model_id`
    /// column in table `freeze_drying_procedures`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::procedure_template_asset_model_id,
        procedure_freeze_dried_with_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_freeze_dried_with_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_freeze_dried_with_id` column in table
    /// `freeze_drying_procedures`.
    #[discretionary(aps_procedure_asset_models::procedure_asset_models)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_freeze_dried_with_id: ::rosetta_uuid::Uuid,
}
:: diesel_builders :: prelude :: fk ! ((freeze_drying_procedures :: id) -> (:: aps_procedures :: procedures :: id));
:: diesel_builders :: prelude :: fk ! ((freeze_drying_procedures :: freeze_drying_procedure_template_id) -> (:: aps_freeze_drying_procedure_templates :: freeze_drying_procedure_templates :: id));
:: diesel_builders :: prelude :: fk ! ((freeze_drying_procedures :: freeze_dried_container_id) -> (:: aps_volumetric_containers :: volumetric_containers :: id));
:: diesel_builders :: prelude :: fk ! ((freeze_drying_procedures :: freeze_dried_container_model_id) -> (:: aps_volumetric_container_models :: volumetric_container_models :: id));
:: diesel_builders :: prelude :: fk ! ((freeze_drying_procedures :: procedure_template_freeze_dried_container_model_id) -> (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id));
:: diesel_builders :: prelude :: fk ! ((freeze_drying_procedures :: freeze_dried_with_id) -> (:: aps_freeze_dryers :: freeze_dryers :: id));
:: diesel_builders :: prelude :: fk ! ((freeze_drying_procedures :: freeze_dried_with_model_id) -> (:: aps_freeze_dryer_models :: freeze_dryer_models :: id));
:: diesel_builders :: prelude :: fk ! ((freeze_drying_procedures :: procedure_template_freeze_dried_with_model_id) -> (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id));
:: diesel_builders :: prelude :: fk ! ((freeze_drying_procedures :: freeze_dried_with_model_id , freeze_drying_procedures :: freeze_dried_container_model_id) -> (:: aps_asset_compatibility_rules :: asset_compatibility_rules :: left_asset_model_id , :: aps_asset_compatibility_rules :: asset_compatibility_rules :: right_asset_model_id));
impl diesel_builders::GetColumn<aps_procedures::procedures::id> for FreezeDryingProcedure {
    fn get_column_ref(
        &self,
    ) -> &<freeze_drying_procedures::id as diesel_builders::Typed>::ColumnType {
        &self.id
    }
}
