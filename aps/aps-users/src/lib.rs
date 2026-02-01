//! Auto-generated crate for the `users` table.
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
/// Table storing users, extending owners
#[table_model(ancestors(aps_entities::entities, aps_owners::owners))]
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (belongs_to (aps_owners :: Owner , foreign_key = id))]
# [table_model (foreign_key ((id ,) , (:: aps_owners :: owners :: id)))]
#[table_model(default(aps_entities::entities::table_name_id, "users"))]
# [diesel (table_name = users)]
pub struct User {
    /// Primary key references owners(id)
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
}
impl ::diesel_builders::GetColumn<aps_entities::entities::id> for User {
    fn get_column_ref(&self) -> &<users::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_owners::owners::id> for User {
    fn get_column_ref(&self) -> &<users::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
