//! Auto-generated crate for the `fractioning_procedures` table.
#[derive(
    Copy,
    Clone,
    Debug,
    PartialOrd,
    PartialEq,
    serde :: Serialize,
    serde :: Deserialize,
    diesel :: Queryable,
    diesel :: Selectable,
    diesel :: Identifiable,
    diesel_builders :: prelude :: TableModel,
)]
/// Struct representing a row in the `fractioning_procedures` table.
#[table_model(ancestors(aps_procedures::procedures))]
# [table_model (error = :: validation_errors :: ValidationError)]
#[table_model(default(aps_procedures::procedures::procedure_table_id, "fractioning_procedures"))]
# [diesel (table_name = fractioning_procedures)]
pub struct FractioningProcedure {
    /// Field representing the `id` column in table `fractioning_procedures`.
    #[same_as(aps_procedures::procedures::id)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `fractioning_procedure_template_id` column in
    /// table `fractioning_procedures`.
    #[same_as(aps_procedures::procedures::procedure_template_id)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    fractioning_procedure_template_id: ::rosetta_uuid::Uuid,
    /// Field representing the `fragment_container_id` column in table
    /// `fractioning_procedures`.
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    fragment_container_id: ::rosetta_uuid::Uuid,
    /// Field representing the `fragment_container_model_id` column in table
    /// `fractioning_procedures`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::asset_model_id,
        procedure_fragment_container_id
    )]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    fragment_container_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_template_fragment_container_model_id`
    /// column in table `fractioning_procedures`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::procedure_template_asset_model_id,
        procedure_fragment_container_id
    )]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_fragment_container_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_fragment_container_id` column in table
    /// `fractioning_procedures`.
    #[discretionary(aps_procedure_asset_models::procedure_asset_models)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_fragment_container_id: ::rosetta_uuid::Uuid,
    /// Field representing the `fragment_placed_into_id` column in table
    /// `fractioning_procedures`.
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    fragment_placed_into_id: ::rosetta_uuid::Uuid,
    /// Field representing the `fragment_placed_into_model_id` column in table
    /// `fractioning_procedures`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::asset_model_id,
        procedure_fragment_placed_into_id
    )]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    fragment_placed_into_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the
    /// `procedure_template_fragment_placed_into_model_id` column in table
    /// `fractioning_procedures`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::procedure_template_asset_model_id,
        procedure_fragment_placed_into_id
    )]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_fragment_placed_into_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_fragment_placed_into_id` column in
    /// table `fractioning_procedures`.
    #[discretionary(aps_procedure_asset_models::procedure_asset_models)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_fragment_placed_into_id: ::rosetta_uuid::Uuid,
    /// Field representing the `mass` column in table `fractioning_procedures`.
    mass: f32,
    /// Field representing the `weighed_with_id` column in table
    /// `fractioning_procedures`.
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    weighed_with_id: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `weighed_with_model_id` column in table
    /// `fractioning_procedures`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::asset_model_id,
        procedure_weighed_with_id
    )]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    weighed_with_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_template_weighed_with_model_id` column
    /// in table `fractioning_procedures`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::procedure_template_asset_model_id,
        procedure_weighed_with_id
    )]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_weighed_with_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_weighed_with_id` column in table
    /// `fractioning_procedures`.
    #[discretionary(aps_procedure_asset_models::procedure_asset_models)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_weighed_with_id: ::rosetta_uuid::Uuid,
}
:: diesel_builders :: prelude :: fk ! ((fractioning_procedures :: id) -> (:: aps_procedures :: procedures :: id));
:: diesel_builders :: prelude :: fk ! ((fractioning_procedures :: fractioning_procedure_template_id) -> (:: aps_fractioning_procedure_templates :: fractioning_procedure_templates :: id));
:: diesel_builders :: prelude :: fk ! ((fractioning_procedures :: fragment_container_id) -> (:: aps_volumetric_containers :: volumetric_containers :: id));
:: diesel_builders :: prelude :: fk ! ((fractioning_procedures :: fragment_container_model_id) -> (:: aps_volumetric_container_models :: volumetric_container_models :: id));
:: diesel_builders :: prelude :: fk ! ((fractioning_procedures :: procedure_template_fragment_container_model_id) -> (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id));
:: diesel_builders :: prelude :: fk ! ((fractioning_procedures :: fragment_placed_into_id) -> (:: aps_volumetric_containers :: volumetric_containers :: id));
:: diesel_builders :: prelude :: fk ! ((fractioning_procedures :: fragment_placed_into_model_id) -> (:: aps_volumetric_container_models :: volumetric_container_models :: id));
:: diesel_builders :: prelude :: fk ! ((fractioning_procedures :: procedure_template_fragment_placed_into_model_id) -> (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id));
:: diesel_builders :: prelude :: fk ! ((fractioning_procedures :: weighed_with_id) -> (:: aps_weighing_devices :: weighing_devices :: id));
:: diesel_builders :: prelude :: fk ! ((fractioning_procedures :: weighed_with_model_id) -> (:: aps_weighing_device_models :: weighing_device_models :: id));
:: diesel_builders :: prelude :: fk ! ((fractioning_procedures :: procedure_template_weighed_with_model_id) -> (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id));
impl ::diesel_builders::ValidateColumn<fractioning_procedures::mass>
    for <fractioning_procedures::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column(mass: &f32) -> Result<(), Self::Error> {
        use diesel::Column;
        if mass <= &0f32 {
            return Err(::validation_errors::ValidationError::strictly_greater_than_value(
                "fractioning_procedures",
                crate::fractioning_procedures::mass::NAME,
                0f64,
            ));
        }
        Ok(())
    }
}
impl diesel_builders::GetColumn<aps_procedures::procedures::id> for FractioningProcedure {
    fn get_column_ref(
        &self,
    ) -> &<fractioning_procedures::id as diesel_builders::Typed>::ColumnType {
        &self.id
    }
}
