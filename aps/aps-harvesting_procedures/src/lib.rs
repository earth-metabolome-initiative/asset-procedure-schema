//! Auto-generated crate for the `harvesting_procedures` table.
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
/// Struct representing a row in the `harvesting_procedures` table.
#[table_model(ancestors(
    aps_entities::entities,
    aps_ownables::ownables,
    aps_procedures::procedures
))]
# [diesel (belongs_to (aps_sample_sources :: SampleSource , foreign_key = sample_source_id))]
# [diesel (belongs_to (aps_sample_source_models :: SampleSourceModel , foreign_key = sample_source_model_id))]
# [diesel (belongs_to (aps_samples :: Sample , foreign_key = sample_id))]
# [diesel (belongs_to (aps_sample_models :: SampleModel , foreign_key = sample_model_id))]
# [table_model (foreign_key ((id ,) , (:: aps_procedures :: procedures :: id)))]
# [table_model (foreign_key ((harvesting_procedure_template_id ,) , (:: aps_harvesting_procedure_templates :: harvesting_procedure_templates :: id)))]
# [table_model (foreign_key ((sample_source_id ,) , (:: aps_sample_sources :: sample_sources :: id)))]
# [table_model (foreign_key ((sample_source_model_id ,) , (:: aps_sample_source_models :: sample_source_models :: id)))]
# [table_model (foreign_key ((procedure_template_sample_source_model_id ,) , (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id)))]
# [table_model (foreign_key ((sample_id ,) , (:: aps_samples :: samples :: id)))]
# [table_model (foreign_key ((sample_model_id ,) , (:: aps_sample_models :: sample_models :: id)))]
# [table_model (foreign_key ((procedure_template_sample_model_id ,) , (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id)))]
#[table_model(default(aps_entities::entities::table_name_id, "harvesting_procedures"))]
# [diesel (table_name = harvesting_procedures)]
pub struct HarvestingProcedure {
    /// Identifier of the harvesting id, which is also a foreign key to the
    /// general procedure.
    #[same_as(aps_procedures::procedures::id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// The template of this procedure_id should be a harvesting procedure_id
    /// template.
    #[same_as(aps_procedures::procedures::procedure_template_id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    harvesting_procedure_template_id: ::rosetta_uuid::Uuid,
    /// The sample source from which the sample is harvested, which must be a
    /// sample source asset.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    sample_source_id: ::rosetta_uuid::Uuid,
    /// The model of the sample source from which the sample is harvested.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::asset_model_id,
        procedure_sample_source_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    sample_source_model_id: ::rosetta_uuid::Uuid,
    /// The procedure_id template asset model associated to the `sample_source`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::procedure_template_asset_model_id,
        procedure_sample_source_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_sample_source_model_id: ::rosetta_uuid::Uuid,
    /// The procedure_id asset associated to the `sample_source`.
    #[discretionary(aps_procedure_asset_models::procedure_asset_models)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_sample_source_id: ::rosetta_uuid::Uuid,
    /// The sample harvested from the sample source, which must be a sample
    /// asset.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    sample_id: ::rosetta_uuid::Uuid,
    /// The model of the sample harvested from the sample source.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::asset_model_id,
        procedure_sample_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    sample_model_id: ::rosetta_uuid::Uuid,
    /// The procedure_id template asset model associated to the `sample`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::procedure_template_asset_model_id,
        procedure_sample_id
    )]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_sample_model_id: ::rosetta_uuid::Uuid,
    /// The procedure_id asset associated to the `sample`.
    #[discretionary(aps_procedure_asset_models::procedure_asset_models)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_sample_id: ::rosetta_uuid::Uuid,
}
impl ::diesel_builders::GetColumn<aps_entities::entities::id> for HarvestingProcedure {
    fn get_column_ref(
        &self,
    ) -> &<harvesting_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_ownables::ownables::id> for HarvestingProcedure {
    fn get_column_ref(
        &self,
    ) -> &<harvesting_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_procedures::procedures::id> for HarvestingProcedure {
    fn get_column_ref(
        &self,
    ) -> &<harvesting_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl procedure_like::ProcedureTableLike for harvesting_procedures::table {
    type ProcedureTemplateTable =
        aps_harvesting_procedure_templates::harvesting_procedure_templates::table;
}
