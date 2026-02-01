//! Auto-generated crate for the `photographs` table.
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
/// Struct representing a row in the `photographs` table.
#[table_model(ancestors(
    aps_entities::entities,
    aps_ownables::ownables,
    aps_assets::assets,
    aps_digital_assets::digital_assets
))]
# [diesel (belongs_to (aps_digital_assets :: DigitalAsset , foreign_key = id))]
# [table_model (foreign_key ((id ,) , (:: aps_digital_assets :: digital_assets :: id)))]
#[table_model(default(aps_entities::entities::table_name_id, "photographs"))]
# [diesel (table_name = photographs)]
pub struct Photograph {
    /// Field representing the `id` column in table `photographs`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
}
impl ::diesel_builders::GetColumn<aps_assets::assets::id> for Photograph {
    fn get_column_ref(&self) -> &<photographs::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_digital_assets::digital_assets::id> for Photograph {
    fn get_column_ref(&self) -> &<photographs::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_entities::entities::id> for Photograph {
    fn get_column_ref(&self) -> &<photographs::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_ownables::ownables::id> for Photograph {
    fn get_column_ref(&self) -> &<photographs::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
