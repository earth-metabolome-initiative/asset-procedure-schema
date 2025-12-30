//! Auto-generated crate for the `procedure_template_asset_models` table.
#[derive(
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
/// Undocumented table
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (table_name = procedure_template_asset_models)]
pub struct ProcedureTemplateAssetModel {
    /// Undocumented column
    # [table_model (default = :: rosetta_uuid :: Uuid :: new_v4 ())]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Undocumented column
    name: String,
    /// Undocumented column
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_id: ::rosetta_uuid::Uuid,
    /// Undocumented column
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    based_on_id: Option<::rosetta_uuid::Uuid>,
    /// Undocumented column
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    asset_model_id: ::rosetta_uuid::Uuid,
}
::diesel_builders::prelude::unique_index!(
    procedure_template_asset_models::procedure_template_id,
    procedure_template_asset_models::name
);
::diesel_builders::prelude::unique_index!(
    procedure_template_asset_models::id,
    procedure_template_asset_models::procedure_template_id
);
::diesel_builders::prelude::unique_index!(
    procedure_template_asset_models::id,
    procedure_template_asset_models::asset_model_id
);
:: diesel_builders :: prelude :: fk ! ((procedure_template_asset_models :: procedure_template_id) -> (:: aps_procedure_templates :: procedure_templates :: id));
:: diesel_builders :: prelude :: fk ! ((procedure_template_asset_models :: based_on_id) -> (procedure_template_asset_models :: id));
:: diesel_builders :: prelude :: fk ! ((procedure_template_asset_models :: asset_model_id) -> (:: aps_asset_models :: asset_models :: id));
:: diesel_builders :: prelude :: fk ! ((procedure_template_asset_models :: based_on_id , procedure_template_asset_models :: asset_model_id) -> (procedure_template_asset_models :: id , procedure_template_asset_models :: asset_model_id));
impl ::diesel_builders::ValidateColumn<procedure_template_asset_models::name>
    for <procedure_template_asset_models::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError<&'static str>;
    #[inline]
    fn validate_column(name: &String) -> Result<(), Self::Error> {
        use diesel::Column;
        if name.is_empty() {
            return Err(validation_errors::prelude::ValidationError::empty(
                crate::procedure_template_asset_models::name::NAME,
            ));
        }
        Ok(())
    }
}
