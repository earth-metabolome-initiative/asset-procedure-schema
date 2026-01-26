//! Auto-generated crate for the `procedure_asset_models` table.
#[derive(
    Copy,
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
/// Struct representing a row in the `procedure_asset_models` table.
# [diesel (belongs_to (aps_procedure_templates :: ProcedureTemplate , foreign_key = procedure_template_id))]
# [table_model (foreign_key ((procedure_id ,) , (:: aps_procedures :: procedures :: id)))]
# [table_model (foreign_key ((procedure_template_id ,) , (:: aps_procedure_templates :: procedure_templates :: id)))]
# [table_model (foreign_key ((asset_model_id ,) , (:: aps_asset_models :: asset_models :: id)))]
# [table_model (foreign_key ((procedure_template_asset_model_id ,) , (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id)))]
# [table_model (foreign_key ((ancestor_model_id ,) , (:: aps_asset_models :: asset_models :: id)))]
# [table_model (foreign_key ((asset_model_id , ancestor_model_id ,) , (:: aps_asset_model_ancestors :: asset_model_ancestors :: descendant_model_id , :: aps_asset_model_ancestors :: asset_model_ancestors :: ancestor_model_id)))]
# [diesel (table_name = procedure_asset_models)]
pub struct ProcedureAssetModel {
    /// The ID of this procedure_id asset.
    # [table_model (default = :: rosetta_uuid :: Uuid :: new_v4 ())]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// The ID of the procedure_id this asset is used in.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_id: ::rosetta_uuid::Uuid,
    /// The procedure_id template of the procedure_id this asset is used in.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_id: ::rosetta_uuid::Uuid,
    /// The asset model of the asset used in this procedure.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    asset_model_id: ::rosetta_uuid::Uuid,
    /// We enforce that there must be a procedure_id template asset for this
    /// asset.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_asset_model_id: ::rosetta_uuid::Uuid,
    /// The ancestor asset model defined in the procedure_id template asset.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    ancestor_model_id: ::rosetta_uuid::Uuid,
}
::diesel_builders::prelude::unique_index!(
    procedure_asset_models::id,
    procedure_asset_models::procedure_template_asset_model_id
);
::diesel_builders::prelude::unique_index!(
    procedure_asset_models::id,
    procedure_asset_models::asset_model_id
);
