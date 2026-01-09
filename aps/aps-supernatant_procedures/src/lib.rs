//! Auto-generated crate for the `supernatant_procedures` table.
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
/// Struct representing a row in the `supernatant_procedures` table.
#[table_model(ancestors(aps_procedures::procedures))]
#[table_model(default(aps_procedures::procedures::procedure_table_id, "supernatant_procedures"))]
# [diesel (table_name = supernatant_procedures)]
pub struct SupernatantProcedure {
    /// Field representing the `id` column in table `supernatant_procedures`.
    #[same_as(aps_procedures::procedures::id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `supernatant_procedure_template_id` column in
    /// table `supernatant_procedures`.
    #[same_as(aps_procedures::procedures::procedure_template_id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    supernatant_procedure_template_id: ::rosetta_uuid::Uuid,
    /// Field representing the `stratified_source_id` column in table
    /// `supernatant_procedures`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    stratified_source_id: ::rosetta_uuid::Uuid,
    /// Field representing the `stratified_source_model_id` column in table
    /// `supernatant_procedures`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::asset_model_id,
        procedure_stratified_source_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    stratified_source_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_template_stratified_source_model_id`
    /// column in table `supernatant_procedures`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::procedure_template_asset_model_id,
        procedure_stratified_source_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_stratified_source_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_stratified_source_id` column in table
    /// `supernatant_procedures`.
    #[discretionary(aps_procedure_asset_models::procedure_asset_models)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_stratified_source_id: ::rosetta_uuid::Uuid,
    /// Field representing the `supernatant_destination_id` column in table
    /// `supernatant_procedures`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    supernatant_destination_id: ::rosetta_uuid::Uuid,
    /// Field representing the `supernatant_destination_model_id` column in
    /// table `supernatant_procedures`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::asset_model_id,
        procedure_supernatant_destination_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    supernatant_destination_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the
    /// `procedure_template_supernatant_destination_model_id` column in table
    /// `supernatant_procedures`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::procedure_template_asset_model_id,
        procedure_supernatant_destination_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_supernatant_destination_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_supernatant_destination_id` column in
    /// table `supernatant_procedures`.
    #[discretionary(aps_procedure_asset_models::procedure_asset_models)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_supernatant_destination_id: ::rosetta_uuid::Uuid,
    /// Field representing the `transferred_with_id` column in table
    /// `supernatant_procedures`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    transferred_with_id: ::rosetta_uuid::Uuid,
    /// Field representing the `transferred_with_model_id` column in table
    /// `supernatant_procedures`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::asset_model_id,
        procedure_transferred_with_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    transferred_with_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_template_transferred_with_model_id`
    /// column in table `supernatant_procedures`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::procedure_template_asset_model_id,
        procedure_transferred_with_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_transferred_with_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_transferred_with_id` column in table
    /// `supernatant_procedures`.
    #[discretionary(aps_procedure_asset_models::procedure_asset_models)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_transferred_with_id: ::rosetta_uuid::Uuid,
}
:: diesel_builders :: prelude :: fk ! ((supernatant_procedures :: id) -> (:: aps_procedures :: procedures :: id));
:: diesel_builders :: prelude :: fk ! ((supernatant_procedures :: supernatant_procedure_template_id) -> (:: aps_supernatant_procedure_templates :: supernatant_procedure_templates :: id));
:: diesel_builders :: prelude :: fk ! ((supernatant_procedures :: stratified_source_id) -> (:: aps_volumetric_containers :: volumetric_containers :: id));
:: diesel_builders :: prelude :: fk ! ((supernatant_procedures :: stratified_source_model_id) -> (:: aps_volumetric_container_models :: volumetric_container_models :: id));
:: diesel_builders :: prelude :: fk ! ((supernatant_procedures :: procedure_template_stratified_source_model_id) -> (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id));
:: diesel_builders :: prelude :: fk ! ((supernatant_procedures :: supernatant_destination_id) -> (:: aps_volumetric_containers :: volumetric_containers :: id));
:: diesel_builders :: prelude :: fk ! ((supernatant_procedures :: supernatant_destination_model_id) -> (:: aps_volumetric_container_models :: volumetric_container_models :: id));
:: diesel_builders :: prelude :: fk ! ((supernatant_procedures :: procedure_template_supernatant_destination_model_id) -> (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id));
:: diesel_builders :: prelude :: fk ! ((supernatant_procedures :: transferred_with_id) -> (:: aps_volume_measuring_devices :: volume_measuring_devices :: id));
:: diesel_builders :: prelude :: fk ! ((supernatant_procedures :: transferred_with_model_id) -> (:: aps_volume_measuring_device_models :: volume_measuring_device_models :: id));
:: diesel_builders :: prelude :: fk ! ((supernatant_procedures :: procedure_template_transferred_with_model_id) -> (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id));
