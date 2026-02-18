//! Auto-generated crate for the `freeze_dryers` table.
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
/// Physical freeze dryers tracked in APS inventory.
#[table_model(ancestors(
    aps_entities::entities,
    aps_ownables::ownables,
    aps_namespaced_ownables::namespaced_ownables,
    aps_assets::assets,
    aps_physical_assets::physical_assets
))]
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (belongs_to (aps_physical_assets :: PhysicalAsset , foreign_key = id))]
# [diesel (belongs_to (aps_commercial_freeze_dryer_lots :: CommercialFreezeDryerLot , foreign_key = commercial_freeze_dryer_lot_id))]
# [table_model (foreign_key ((id ,) , (:: aps_physical_assets :: physical_assets :: id)))]
# [table_model (foreign_key ((commercial_freeze_dryer_lot_id ,) , (:: aps_commercial_freeze_dryer_lots :: commercial_freeze_dryer_lots :: id)))]
#[table_model(default(aps_entities::entities::table_name_id, "freeze_dryers"))]
# [diesel (table_name = freeze_dryers)]
pub struct FreezeDryer {
    /// Stable asset identifier inherited from physical_assets.
    #[same_as(aps_physical_assets::physical_assets::id)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Commercial lot model instantiated by this physical asset.
    #[same_as(aps_physical_assets::physical_assets::physical_asset_model_id)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    commercial_freeze_dryer_lot_id: ::rosetta_uuid::Uuid,
}
impl ::diesel_builders::GetColumn<aps_assets::assets::id> for FreezeDryer {
    fn get_column_ref(&self) -> &<freeze_dryers::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_entities::entities::id> for FreezeDryer {
    fn get_column_ref(&self) -> &<freeze_dryers::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_namespaced_ownables::namespaced_ownables::id>
    for FreezeDryer
{
    fn get_column_ref(&self) -> &<freeze_dryers::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_ownables::ownables::id> for FreezeDryer {
    fn get_column_ref(&self) -> &<freeze_dryers::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_physical_assets::physical_assets::id> for FreezeDryer {
    fn get_column_ref(&self) -> &<freeze_dryers::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
