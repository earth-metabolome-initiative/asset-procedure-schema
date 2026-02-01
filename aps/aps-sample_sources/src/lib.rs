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
    :: serde :: Serialize,
    :: serde :: Deserialize,
    :: diesel :: Queryable,
    :: diesel :: Selectable,
    :: diesel :: Identifiable,
    :: diesel :: Associations,
    :: diesel_builders :: prelude :: TableModel,
)]
/// Struct representing a row in the `sample_sources` table.
#[table_model(ancestors(
    aps_entities::entities,
    aps_ownables::ownables,
    aps_assets::assets,
    aps_physical_assets::physical_assets
))]
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (belongs_to (aps_physical_assets :: PhysicalAsset , foreign_key = id))]
# [diesel (belongs_to (aps_sample_source_models :: SampleSourceModel , foreign_key = sample_source_model_id))]
# [table_model (foreign_key ((id ,) , (:: aps_physical_assets :: physical_assets :: id)))]
# [table_model (foreign_key ((sample_source_model_id ,) , (:: aps_sample_source_models :: sample_source_models :: id)))]
#[table_model(default(aps_entities::entities::table_name_id, "sample_sources"))]
# [diesel (table_name = sample_sources)]
pub struct SampleSource {
    /// Field representing the `id` column in table `sample_sources`.
    #[same_as(aps_physical_assets::physical_assets::id)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `sample_source_model_id` column in table
    /// `sample_sources`.
    #[same_as(aps_physical_assets::physical_assets::physical_asset_model_id)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    sample_source_model_id: ::rosetta_uuid::Uuid,
}
impl ::diesel_builders::GetColumn<aps_assets::assets::id> for SampleSource {
    fn get_column_ref(
        &self,
    ) -> &<sample_sources::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_entities::entities::id> for SampleSource {
    fn get_column_ref(
        &self,
    ) -> &<sample_sources::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_ownables::ownables::id> for SampleSource {
    fn get_column_ref(
        &self,
    ) -> &<sample_sources::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_physical_assets::physical_assets::id> for SampleSource {
    fn get_column_ref(
        &self,
    ) -> &<sample_sources::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
