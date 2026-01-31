//! Auto-generated crate for the `procedure_template_asset_models` table.
#[derive(
    Clone,
    Debug,
    Hash,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    :: serde :: Serialize,
    :: serde :: Deserialize,
    :: diesel :: Queryable,
    :: diesel :: Selectable,
    :: diesel :: Identifiable,
    :: diesel :: Associations,
    :: diesel_builders :: prelude :: TableModel,
)]
/// Table to store procedure template asset models
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (belongs_to (aps_procedure_templates :: ProcedureTemplate , foreign_key = procedure_template_id))]
# [diesel (belongs_to (aps_asset_models :: AssetModel , foreign_key = asset_model_id))]
# [table_model (foreign_key ((procedure_template_id ,) , (:: aps_procedure_templates :: procedure_templates :: id)))]
# [table_model (foreign_key ((based_on_id ,) , (procedure_template_asset_models :: id)))]
# [table_model (foreign_key ((asset_model_id ,) , (:: aps_asset_models :: asset_models :: id)))]
# [table_model (foreign_key ((based_on_id , asset_model_id ,) , (procedure_template_asset_models :: id , procedure_template_asset_models :: asset_model_id)))]
# [diesel (table_name = procedure_template_asset_models)]
pub struct ProcedureTemplateAssetModel {
    /// Identifier of the procedure template asset model
    # [table_model (default = :: rosetta_uuid :: Uuid :: utc_v7 ())]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// The name of the procedure template asset model
    name: String,
    /// Procedure template this asset model is associated with
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_id: ::rosetta_uuid::Uuid,
    /// which this procedure template asset model is based on
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    based_on_id: Option<::rosetta_uuid::Uuid>,
    /// The asset model this procedure template asset model is associated with
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
impl ::diesel_builders::ValidateColumn<procedure_template_asset_models::name>
    for <procedure_template_asset_models::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column(name: &String) -> Result<(), Self::Error> {
        use diesel::Column;
        if name.is_empty() {
            return Err (:: validation_errors :: ValidationError :: empty (< crate :: procedure_template_asset_models :: table as :: diesel_builders :: TableExt > :: TABLE_NAME , crate :: procedure_template_asset_models :: name :: NAME)) ;
        }
        if name.len() < 255usize {
            return Err (:: validation_errors :: ValidationError :: exceeds_max_length (< crate :: procedure_template_asset_models :: table as :: diesel_builders :: TableExt > :: TABLE_NAME , crate :: procedure_template_asset_models :: name :: NAME , 255usize)) ;
        }
        Ok(())
    }
}
