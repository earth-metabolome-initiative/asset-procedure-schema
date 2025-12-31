//! Auto-generated crate for the `volumetric_containers` table.
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
/// Undocumented table
#[table_model(ancestors(
    aps_assets::assets,
    aps_physical_assets::physical_assets,
    aps_containers::containers
))]
# [diesel (table_name = volumetric_containers)]
pub struct VolumetricContainer {
    /// Undocumented column
    #[same_as(aps_physical_assets::physical_assets::id)]
    #[same_as(aps_containers::containers::id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Undocumented column
    #[same_as(aps_containers::containers::container_model_id)]
    #[same_as(aps_physical_assets::physical_assets::physical_asset_model_id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    volumetric_container_model_id: ::rosetta_uuid::Uuid,
}
:: diesel_builders :: prelude :: fk ! ((volumetric_containers :: id) -> (:: aps_containers :: containers :: id));
:: diesel_builders :: prelude :: fk ! ((volumetric_containers :: volumetric_container_model_id) -> (:: aps_volumetric_container_models :: volumetric_container_models :: id));
