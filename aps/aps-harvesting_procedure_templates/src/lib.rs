//! Auto-generated crate for the `harvesting_procedure_templates` table.
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
/// Struct representing a row in the `harvesting_procedure_templates` table.
#[table_model(ancestors(aps_procedure_templates::procedure_templates))]
# [diesel (belongs_to (aps_procedure_templates :: ProcedureTemplate , foreign_key = id))]
# [diesel (belongs_to (aps_sample_source_models :: SampleSourceModel , foreign_key = sample_source_model_id))]
# [table_model (foreign_key ((id ,) , (:: aps_procedure_templates :: procedure_templates :: id)))]
# [table_model (foreign_key ((sample_source_model_id ,) , (:: aps_sample_source_models :: sample_source_models :: id)))]
# [table_model (foreign_key ((sample_model_id ,) , (:: aps_sample_models :: sample_models :: id)))]
# [table_model (foreign_key ((id , procedure_template_sample_source_model_id ,) , (:: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_id , :: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_asset_model_id)))]
# [table_model (foreign_key ((id , procedure_template_sample_model_id ,) , (:: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_id , :: aps_reused_procedure_template_asset_models :: reused_procedure_template_asset_models :: procedure_template_asset_model_id)))]
#[table_model(default(
    aps_procedure_templates::procedure_templates::procedure_template_table_id,
    "harvesting_procedure_templates"
))]
# [diesel (table_name = harvesting_procedure_templates)]
pub struct HarvestingProcedureTemplate {
    /// Identifier of the harvesting procedure_id template, which is also a
    /// foreign key to the general procedure_id template.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Sample source model from which the sample is taken.
    #[same_as(
        aps_procedure_template_asset_models::procedure_template_asset_models::asset_model_id,
        procedure_template_sample_source_model_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    sample_source_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_template_sample_source_model_id`
    /// column in table `harvesting_procedure_templates`.
    #[discretionary(aps_procedure_template_asset_models::procedure_template_asset_models)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_sample_source_model_id: ::rosetta_uuid::Uuid,
    /// Sample model harvested from the sample source model.
    #[same_as(
        aps_procedure_template_asset_models::procedure_template_asset_models::asset_model_id,
        procedure_template_sample_model_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    sample_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `procedure_template_sample_model_id` column in
    /// table `harvesting_procedure_templates`.
    #[discretionary(aps_procedure_template_asset_models::procedure_template_asset_models)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_sample_model_id: ::rosetta_uuid::Uuid,
}
::diesel_builders::prelude::unique_index!(
    harvesting_procedure_templates::id,
    harvesting_procedure_templates::procedure_template_sample_source_model_id
);
::diesel_builders::prelude::unique_index!(
    harvesting_procedure_templates::id,
    harvesting_procedure_templates::procedure_template_sample_model_id
);
impl ::diesel_builders::GetColumn<aps_procedure_templates::procedure_templates::id>
    for HarvestingProcedureTemplate
{
    fn get_column_ref(
        &self,
    ) -> &<harvesting_procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
