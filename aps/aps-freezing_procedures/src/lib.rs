//! Auto-generated crate for the `freezing_procedures` table.
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
/// Struct representing a row in the `freezing_procedures` table.
#[table_model(ancestors(aps_procedures::procedures))]
#[table_model(default(aps_procedures::procedures::procedure_table_id, "freezing_procedures"))]
# [diesel (table_name = freezing_procedures)]
pub struct FreezingProcedure {
    /// Field representing the `id` column in table `freezing_procedures`.
    #[same_as(aps_procedures::procedures::id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `freezing_procedure_template_id` column in table
    /// `freezing_procedures`.
    #[same_as(aps_procedures::procedures::procedure_template_id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    freezing_procedure_template_id: ::rosetta_uuid::Uuid,
    /// Field representing the `frozen_container_id` column in table
    /// `freezing_procedures`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    frozen_container_id: ::rosetta_uuid::Uuid,
    /// Field representing the `frozen_container_model_id` column in table
    /// `freezing_procedures`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::asset_model_id,
        procedure_frozen_container_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    frozen_container_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_template_frozen_container_model_id`
    /// column in table `freezing_procedures`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::procedure_template_asset_model_id,
        procedure_frozen_container_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_frozen_container_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_frozen_container_id` column in table
    /// `freezing_procedures`.
    #[discretionary(aps_procedure_asset_models::procedure_asset_models)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_frozen_container_id: ::rosetta_uuid::Uuid,
    /// Field representing the `frozen_with_id` column in table
    /// `freezing_procedures`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    frozen_with_id: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `frozen_with_model_id` column in table
    /// `freezing_procedures`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::asset_model_id,
        procedure_frozen_with_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    frozen_with_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_template_frozen_with_model_id` column
    /// in table `freezing_procedures`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::procedure_template_asset_model_id,
        procedure_frozen_with_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_frozen_with_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_frozen_with_id` column in table
    /// `freezing_procedures`.
    #[discretionary(aps_procedure_asset_models::procedure_asset_models)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_frozen_with_id: ::rosetta_uuid::Uuid,
}
:: diesel_builders :: prelude :: fk ! ((freezing_procedures :: id) -> (:: aps_procedures :: procedures :: id));
:: diesel_builders :: prelude :: fk ! ((freezing_procedures :: freezing_procedure_template_id) -> (:: aps_freezing_procedure_templates :: freezing_procedure_templates :: id));
:: diesel_builders :: prelude :: fk ! ((freezing_procedures :: frozen_container_id) -> (:: aps_volumetric_containers :: volumetric_containers :: id));
:: diesel_builders :: prelude :: fk ! ((freezing_procedures :: frozen_container_model_id) -> (:: aps_volumetric_container_models :: volumetric_container_models :: id));
:: diesel_builders :: prelude :: fk ! ((freezing_procedures :: procedure_template_frozen_container_model_id) -> (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id));
:: diesel_builders :: prelude :: fk ! ((freezing_procedures :: frozen_with_id) -> (:: aps_freezers :: freezers :: id));
:: diesel_builders :: prelude :: fk ! ((freezing_procedures :: frozen_with_model_id) -> (:: aps_freezer_models :: freezer_models :: id));
:: diesel_builders :: prelude :: fk ! ((freezing_procedures :: procedure_template_frozen_with_model_id) -> (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id));
:: diesel_builders :: prelude :: fk ! ((freezing_procedures :: frozen_with_model_id , freezing_procedures :: frozen_container_model_id) -> (:: aps_asset_compatibility_rules :: asset_compatibility_rules :: left_asset_model_id , :: aps_asset_compatibility_rules :: asset_compatibility_rules :: right_asset_model_id));
impl diesel_builders::GetColumn<aps_procedures::procedures::id> for FreezingProcedure {
    fn get_column_ref(&self) -> &<freezing_procedures::id as diesel_builders::Typed>::ColumnType {
        &self.id
    }
}
