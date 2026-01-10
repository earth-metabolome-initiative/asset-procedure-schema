//! Auto-generated crate for the `aliquoting_procedures` table.
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
/// Struct representing a row in the `aliquoting_procedures` table.
#[table_model(ancestors(aps_procedures::procedures))]
#[table_model(default(aps_procedures::procedures::procedure_table_id, "aliquoting_procedures"))]
# [diesel (table_name = aliquoting_procedures)]
pub struct AliquotingProcedure {
    /// Field representing the `id` column in table `aliquoting_procedures`.
    #[same_as(aps_procedures::procedures::id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `aliquoting_procedure_template_id` column in
    /// table `aliquoting_procedures`.
    #[same_as(aps_procedures::procedures::procedure_template_id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    aliquoting_procedure_template_id: ::rosetta_uuid::Uuid,
    /// Field representing the `aliquoted_with_id` column in table
    /// `aliquoting_procedures`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    aliquoted_with_id: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `aliquoted_with_model_id` column in table
    /// `aliquoting_procedures`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::asset_model_id,
        procedure_aliquoted_with_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    aliquoted_with_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_template_aliquoted_with_model_id`
    /// column in table `aliquoting_procedures`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::procedure_template_asset_model_id,
        procedure_aliquoted_with_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_aliquoted_with_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_aliquoted_with_id` column in table
    /// `aliquoting_procedures`.
    #[discretionary(aps_procedure_asset_models::procedure_asset_models)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_aliquoted_with_id: ::rosetta_uuid::Uuid,
    /// Field representing the `aliquoted_from_id` column in table
    /// `aliquoting_procedures`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    aliquoted_from_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_template_aliquoted_from_model_id`
    /// column in table `aliquoting_procedures`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::procedure_template_asset_model_id,
        procedure_aliquoted_from_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_aliquoted_from_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_aliquoted_from_id` column in table
    /// `aliquoting_procedures`.
    #[discretionary(aps_procedure_asset_models::procedure_asset_models)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_aliquoted_from_id: ::rosetta_uuid::Uuid,
    /// Field representing the `aliquoted_into_id` column in table
    /// `aliquoting_procedures`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    aliquoted_into_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_template_aliquoted_into_model_id`
    /// column in table `aliquoting_procedures`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::procedure_template_asset_model_id,
        procedure_aliquoted_into_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_aliquoted_into_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_aliquoted_into_id` column in table
    /// `aliquoting_procedures`.
    #[discretionary(aps_procedure_asset_models::procedure_asset_models)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_aliquoted_into_id: ::rosetta_uuid::Uuid,
}
:: diesel_builders :: prelude :: fk ! ((aliquoting_procedures :: id) -> (:: aps_procedures :: procedures :: id));
:: diesel_builders :: prelude :: fk ! ((aliquoting_procedures :: aliquoting_procedure_template_id) -> (:: aps_aliquoting_procedure_templates :: aliquoting_procedure_templates :: id));
:: diesel_builders :: prelude :: fk ! ((aliquoting_procedures :: aliquoted_with_id) -> (:: aps_volume_measuring_devices :: volume_measuring_devices :: id));
:: diesel_builders :: prelude :: fk ! ((aliquoting_procedures :: aliquoted_with_model_id) -> (:: aps_volume_measuring_device_models :: volume_measuring_device_models :: id));
:: diesel_builders :: prelude :: fk ! ((aliquoting_procedures :: procedure_template_aliquoted_with_model_id) -> (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id));
:: diesel_builders :: prelude :: fk ! ((aliquoting_procedures :: aliquoted_from_id) -> (:: aps_volumetric_containers :: volumetric_containers :: id));
:: diesel_builders :: prelude :: fk ! ((aliquoting_procedures :: procedure_template_aliquoted_from_model_id) -> (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id));
:: diesel_builders :: prelude :: fk ! ((aliquoting_procedures :: aliquoted_into_id) -> (:: aps_volumetric_containers :: volumetric_containers :: id));
:: diesel_builders :: prelude :: fk ! ((aliquoting_procedures :: procedure_template_aliquoted_into_model_id) -> (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id));
impl diesel_builders::GetColumn<aps_procedures::procedures::id> for AliquotingProcedure {
    fn get_column_ref(&self) -> &<aliquoting_procedures::id as diesel_builders::Typed>::ColumnType {
        &self.id
    }
}
