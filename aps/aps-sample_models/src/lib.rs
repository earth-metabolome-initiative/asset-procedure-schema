//! Auto-generated crate for the `sample_models` table.
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
/// Struct representing a row in the `sample_models` table.
#[table_model(ancestors(
    aps_asset_models::asset_models,
    aps_physical_asset_models::physical_asset_models
))]
#[table_model(default(aps_asset_models::asset_models::asset_model_table_id, "sample_models"))]
# [diesel (table_name = sample_models)]
pub struct SampleModel {
    /// Field representing the `id` column in table `sample_models`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `sample_source_model_id` column in table
    /// `sample_models`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    sample_source_model_id: ::rosetta_uuid::Uuid,
}
::diesel_builders::prelude::unique_index!(sample_models::id, sample_models::sample_source_model_id);
:: diesel_builders :: prelude :: fk ! ((sample_models :: id) -> (:: aps_physical_asset_models :: physical_asset_models :: id));
:: diesel_builders :: prelude :: fk ! ((sample_models :: sample_source_model_id) -> (:: aps_sample_source_models :: sample_source_models :: id));
impl diesel_builders::GetColumn<aps_asset_models::asset_models::id> for SampleModel {
    fn get_column_ref(&self) -> &<sample_models::id as diesel_builders::Typed>::ColumnType {
        &self.id
    }
}
impl diesel_builders::GetColumn<aps_physical_asset_models::physical_asset_models::id>
    for SampleModel
{
    fn get_column_ref(&self) -> &<sample_models::id as diesel_builders::Typed>::ColumnType {
        &self.id
    }
}
