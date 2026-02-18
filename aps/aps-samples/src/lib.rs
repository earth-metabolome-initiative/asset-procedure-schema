//! Auto-generated crate for the `samples` table.
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
/// mandatory source-model compatibility.
#[table_model(ancestors(
    aps_entities::entities,
    aps_ownables::ownables,
    aps_namespaced_ownables::namespaced_ownables,
    aps_assets::assets,
    aps_physical_assets::physical_assets
))]
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (belongs_to (aps_physical_assets :: PhysicalAsset , foreign_key = id))]
# [diesel (belongs_to (aps_sample_sources :: SampleSource , foreign_key = sample_source_id))]
# [diesel (belongs_to (aps_sample_source_models :: SampleSourceModel , foreign_key = sample_source_model_id))]
# [table_model (foreign_key ((id ,) , (:: aps_physical_assets :: physical_assets :: id)))]
# [table_model (foreign_key ((sample_model_id ,) , (:: aps_sample_models :: sample_models :: id)))]
# [table_model (foreign_key ((sample_source_id ,) , (:: aps_sample_sources :: sample_sources :: id)))]
# [table_model (foreign_key ((sample_source_model_id ,) , (:: aps_sample_source_models :: sample_source_models :: id)))]
# [table_model (foreign_key ((sample_source_id , sample_source_model_id ,) , (:: aps_assets :: assets :: id , :: aps_assets :: assets :: model_id)))]
#[table_model(default(aps_entities::entities::table_name_id, "samples"))]
# [diesel (table_name = samples)]
pub struct Sample {
    /// Stable asset identifier inherited from `physical_assets`.
    #[same_as(aps_physical_assets::physical_assets::id)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Sample model instantiated by this physical sample.
    #[same_as(aps_physical_assets::physical_assets::physical_asset_model_id)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    sample_model_id: ::rosetta_uuid::Uuid,
    /// Optional concrete source asset the sample was taken from.
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    sample_source_id: Option<::rosetta_uuid::Uuid>,
    /// Required source-model family associated with the sample and source
    /// relation.
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    sample_source_model_id: ::rosetta_uuid::Uuid,
}
impl ::diesel_builders::GetColumn<aps_assets::assets::id> for Sample {
    fn get_column_ref(&self) -> &<samples::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_entities::entities::id> for Sample {
    fn get_column_ref(&self) -> &<samples::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_namespaced_ownables::namespaced_ownables::id> for Sample {
    fn get_column_ref(&self) -> &<samples::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_ownables::ownables::id> for Sample {
    fn get_column_ref(&self) -> &<samples::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_physical_assets::physical_assets::id> for Sample {
    fn get_column_ref(&self) -> &<samples::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
