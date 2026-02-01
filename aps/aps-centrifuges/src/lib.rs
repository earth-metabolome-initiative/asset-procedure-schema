//! Auto-generated crate for the `centrifuges` table.
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
/// Struct representing a row in the `centrifuges` table.
#[table_model(ancestors(
    aps_entities::entities,
    aps_ownables::ownables,
    aps_assets::assets,
    aps_physical_assets::physical_assets
))]
# [diesel (belongs_to (aps_physical_assets :: PhysicalAsset , foreign_key = id))]
# [diesel (belongs_to (aps_commercial_centrifuge_lots :: CommercialCentrifugeLot , foreign_key = commercial_centrifuge_lot_id))]
# [table_model (foreign_key ((id ,) , (:: aps_physical_assets :: physical_assets :: id)))]
# [table_model (foreign_key ((commercial_centrifuge_lot_id ,) , (:: aps_commercial_centrifuge_lots :: commercial_centrifuge_lots :: id)))]
#[table_model(default(aps_entities::entities::table_name_id, "centrifuges"))]
# [diesel (table_name = centrifuges)]
pub struct Centrifuge {
    /// Field representing the `id` column in table `centrifuges`.
    #[same_as(aps_physical_assets::physical_assets::id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `commercial_centrifuge_lot_id` column in table
    /// `centrifuges`.
    #[same_as(aps_physical_assets::physical_assets::physical_asset_model_id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    commercial_centrifuge_lot_id: ::rosetta_uuid::Uuid,
}
impl ::diesel_builders::GetColumn<aps_assets::assets::id> for Centrifuge {
    fn get_column_ref(&self) -> &<centrifuges::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_entities::entities::id> for Centrifuge {
    fn get_column_ref(&self) -> &<centrifuges::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_ownables::ownables::id> for Centrifuge {
    fn get_column_ref(&self) -> &<centrifuges::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_physical_assets::physical_assets::id> for Centrifuge {
    fn get_column_ref(&self) -> &<centrifuges::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
