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
    serde :: Serialize,
    serde :: Deserialize,
    diesel :: Queryable,
    diesel :: Selectable,
    diesel :: Identifiable,
    diesel_builders :: prelude :: TableModel,
)]
/// Struct representing a row in the `samples` table.
#[table_model(ancestors(aps_assets::assets, aps_physical_assets::physical_assets))]
#[table_model(default(aps_assets::assets::asset_table_id, "samples"))]
# [diesel (table_name = samples)]
pub struct Sample {
    /// Field representing the `id` column in table `samples`.
    #[same_as(aps_physical_assets::physical_assets::id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `sample_model_id` column in table `samples`.
    #[same_as(aps_physical_assets::physical_assets::physical_asset_model_id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    sample_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `sample_source_id` column in table `samples`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    sample_source_id: Option<::rosetta_uuid::Uuid>,
    /// Field representing the `sample_source_model_id` column in table
    /// `samples`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    sample_source_model_id: ::rosetta_uuid::Uuid,
}
:: diesel_builders :: prelude :: fk ! ((samples :: id) -> (:: aps_physical_assets :: physical_assets :: id));
:: diesel_builders :: prelude :: fk ! ((samples :: sample_model_id) -> (:: aps_sample_models :: sample_models :: id));
:: diesel_builders :: prelude :: fk ! ((samples :: sample_source_id) -> (:: aps_sample_sources :: sample_sources :: id));
:: diesel_builders :: prelude :: fk ! ((samples :: sample_source_model_id) -> (:: aps_sample_source_models :: sample_source_models :: id));
:: diesel_builders :: prelude :: fk ! ((samples :: sample_source_id , samples :: sample_source_model_id) -> (:: aps_assets :: assets :: id , :: aps_assets :: assets :: model_id));
