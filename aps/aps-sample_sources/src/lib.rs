//! Auto-generated crate for the `sample_sources` table.
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
/// Struct representing a row in the `sample_sources` table.
#[table_model(ancestors(aps_assets::assets, aps_physical_assets::physical_assets))]
#[table_model(default(aps_assets::assets::asset_table_id, "sample_sources"))]
# [diesel (table_name = sample_sources)]
pub struct SampleSource {
    /// Field representing the `id` column in table `sample_sources`.
    #[same_as(aps_physical_assets::physical_assets::id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `sample_source_model_id` column in table
    /// `sample_sources`.
    #[same_as(aps_physical_assets::physical_assets::physical_asset_model_id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    sample_source_model_id: ::rosetta_uuid::Uuid,
}
:: diesel_builders :: prelude :: fk ! ((sample_sources :: id) -> (:: aps_physical_assets :: physical_assets :: id));
:: diesel_builders :: prelude :: fk ! ((sample_sources :: sample_source_model_id) -> (:: aps_sample_source_models :: sample_source_models :: id));
impl diesel_builders::GetColumn<aps_assets::assets::id> for SampleSource {
    fn get_column_ref(&self) -> &<sample_sources::id as diesel_builders::Typed>::ColumnType {
        &self.id
    }
}
impl diesel_builders::GetColumn<aps_physical_assets::physical_assets::id> for SampleSource {
    fn get_column_ref(&self) -> &<sample_sources::id as diesel_builders::Typed>::ColumnType {
        &self.id
    }
}
