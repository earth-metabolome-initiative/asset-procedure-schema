//! Auto-generated crate for the `ownables` table.
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
/// Table storing ownables (base entity for ownable assets, procedures, etc.)
#[table_model(ancestors(aps_entities::entities))]
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (belongs_to (aps_entities :: Entity , foreign_key = id))]
# [diesel (belongs_to (aps_owners :: Owner , foreign_key = owner_id))]
# [table_model (foreign_key ((id ,) , (:: aps_entities :: entities :: id)))]
# [table_model (foreign_key ((owner_id ,) , (:: aps_owners :: owners :: id)))]
# [table_model (foreign_key ((creator_id ,) , (:: aps_users :: users :: id)))]
# [table_model (foreign_key ((editor_id ,) , (:: aps_users :: users :: id)))]
#[table_model(default(aps_entities::entities::table_name_id, "ownables"))]
# [diesel (table_name = ownables)]
pub struct Ownable {
    /// Surrogate primary key for the ownable entity
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Owner of the ownable entity
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    owner_id: ::rosetta_uuid::Uuid,
    /// Creator of the ownable entity
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    creator_id: ::rosetta_uuid::Uuid,
    /// Editor of the ownable entity
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    editor_id: ::rosetta_uuid::Uuid,
}
impl ::diesel_builders::GetColumn<aps_entities::entities::id> for Ownable {
    fn get_column_ref(&self) -> &<ownables::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
