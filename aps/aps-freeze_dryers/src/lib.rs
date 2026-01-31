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
/// Struct representing a row in the `freeze_dryers` table.
#[table_model(ancestors(
    aps_ownables::ownables,
    aps_assets::assets,
    aps_physical_assets::physical_assets
))]
# [diesel (belongs_to (aps_physical_assets :: PhysicalAsset , foreign_key = id))]
# [diesel (belongs_to (aps_commercial_freeze_dryer_lots :: CommercialFreezeDryerLot , foreign_key = commercial_freeze_dryer_lot_id))]
# [table_model (foreign_key ((id ,) , (:: aps_physical_assets :: physical_assets :: id)))]
# [table_model (foreign_key ((commercial_freeze_dryer_lot_id ,) , (:: aps_commercial_freeze_dryer_lots :: commercial_freeze_dryer_lots :: id)))]
#[table_model(default(aps_ownables::ownables::ownable_table_id, "freeze_dryers"))]
# [diesel (table_name = freeze_dryers)]
pub struct FreezeDryer {
    /// Field representing the `id` column in table `freeze_dryers`.
    #[same_as(aps_physical_assets::physical_assets::id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `commercial_freeze_dryer_lot_id` column in table
    /// `freeze_dryers`.
    #[same_as(aps_physical_assets::physical_assets::physical_asset_model_id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    commercial_freeze_dryer_lot_id: ::rosetta_uuid::Uuid,
}
impl ::diesel_builders::GetColumn<aps_assets::assets::id> for FreezeDryer {
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
