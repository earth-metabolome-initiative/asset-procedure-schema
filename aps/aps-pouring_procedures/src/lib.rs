//! Auto-generated crate for the `pouring_procedures` table.
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
/// Struct representing a row in the `pouring_procedures` table.
#[table_model(ancestors(aps_procedures::procedures))]
#[table_model(default(aps_procedures::procedures::procedure_table_id, "pouring_procedures"))]
# [diesel (table_name = pouring_procedures)]
pub struct PouringProcedure {
    /// Field representing the `id` column in table `pouring_procedures`.
    #[same_as(aps_procedures::procedures::id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `pouring_procedure_template_id` column in table
    /// `pouring_procedures`.
    #[same_as(aps_procedures::procedures::procedure_template_id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    pouring_procedure_template_id: ::rosetta_uuid::Uuid,
    /// Field representing the `poured_from_id` column in table
    /// `pouring_procedures`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    poured_from_id: ::rosetta_uuid::Uuid,
    /// Field representing the `poured_from_model_id` column in table
    /// `pouring_procedures`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::asset_model_id,
        procedure_poured_from_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    poured_from_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_template_poured_from_model_id` column
    /// in table `pouring_procedures`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::procedure_template_asset_model_id,
        procedure_poured_from_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_poured_from_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_poured_from_id` column in table
    /// `pouring_procedures`.
    #[discretionary(aps_procedure_asset_models::procedure_asset_models)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_poured_from_id: ::rosetta_uuid::Uuid,
    /// Field representing the `measured_with_id` column in table
    /// `pouring_procedures`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    measured_with_id: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `measured_with_model_id` column in table
    /// `pouring_procedures`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::asset_model_id,
        procedure_measured_with_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    measured_with_model_id: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `procedure_template_measured_with_model_id`
    /// column in table `pouring_procedures`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::procedure_template_asset_model_id,
        procedure_measured_with_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_measured_with_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_measured_with_id` column in table
    /// `pouring_procedures`.
    #[discretionary(aps_procedure_asset_models::procedure_asset_models)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_measured_with_id: ::rosetta_uuid::Uuid,
    /// Field representing the `poured_into_id` column in table
    /// `pouring_procedures`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    poured_into_id: ::rosetta_uuid::Uuid,
    /// Field representing the `poured_into_model_id` column in table
    /// `pouring_procedures`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::asset_model_id,
        procedure_poured_into_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    poured_into_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_template_poured_into_model_id` column
    /// in table `pouring_procedures`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::procedure_template_asset_model_id,
        procedure_poured_into_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_poured_into_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_poured_into_id` column in table
    /// `pouring_procedures`.
    #[discretionary(aps_procedure_asset_models::procedure_asset_models)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_poured_into_id: ::rosetta_uuid::Uuid,
}
:: diesel_builders :: prelude :: fk ! ((pouring_procedures :: id) -> (:: aps_procedures :: procedures :: id));
:: diesel_builders :: prelude :: fk ! ((pouring_procedures :: pouring_procedure_template_id) -> (:: aps_pouring_procedure_templates :: pouring_procedure_templates :: id));
:: diesel_builders :: prelude :: fk ! ((pouring_procedures :: poured_from_id) -> (:: aps_volumetric_containers :: volumetric_containers :: id));
:: diesel_builders :: prelude :: fk ! ((pouring_procedures :: poured_from_model_id) -> (:: aps_volumetric_container_models :: volumetric_container_models :: id));
:: diesel_builders :: prelude :: fk ! ((pouring_procedures :: procedure_template_poured_from_model_id) -> (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id));
:: diesel_builders :: prelude :: fk ! ((pouring_procedures :: measured_with_id) -> (:: aps_volume_measuring_devices :: volume_measuring_devices :: id));
:: diesel_builders :: prelude :: fk ! ((pouring_procedures :: measured_with_model_id) -> (:: aps_volume_measuring_device_models :: volume_measuring_device_models :: id));
:: diesel_builders :: prelude :: fk ! ((pouring_procedures :: procedure_template_measured_with_model_id) -> (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id));
:: diesel_builders :: prelude :: fk ! ((pouring_procedures :: poured_into_id) -> (:: aps_volumetric_containers :: volumetric_containers :: id));
:: diesel_builders :: prelude :: fk ! ((pouring_procedures :: poured_into_model_id) -> (:: aps_volumetric_container_models :: volumetric_container_models :: id));
:: diesel_builders :: prelude :: fk ! ((pouring_procedures :: procedure_template_poured_into_model_id) -> (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id));
impl diesel_builders::GetColumn<aps_procedures::procedures::id> for PouringProcedure {
    fn get_column_ref(&self) -> &<pouring_procedures::id as diesel_builders::Typed>::ColumnType {
        &self.id
    }
}
