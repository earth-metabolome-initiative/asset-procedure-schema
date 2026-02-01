//! Auto-generated crate for the `owners` table.
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
/// Table storing owners (base entity for users, teams, projects)
#[table_model(ancestors(aps_entities::entities))]
# [diesel (belongs_to (aps_entities :: Entity , foreign_key = id))]
# [table_model (foreign_key ((id ,) , (:: aps_entities :: entities :: id)))]
#[table_model(default(aps_entities::entities::table_name_id, "owners"))]
# [diesel (table_name = owners)]
pub struct Owner {
    /// Surrogate primary key for the owner entity
    # [table_model (default = :: rosetta_uuid :: Uuid :: utc_v7 ())]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
}
impl ::diesel_builders::GetColumn<aps_entities::entities::id> for Owner {
    fn get_column_ref(&self) -> &<owners::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
