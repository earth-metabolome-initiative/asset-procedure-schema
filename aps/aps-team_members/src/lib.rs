//! Auto-generated crate for the `team_members` table.
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
/// Table storing team members
# [diesel (belongs_to (aps_teams :: Team , foreign_key = team_id))]
# [diesel (belongs_to (aps_users :: User , foreign_key = member_id))]
#[diesel(primary_key(team_id, member_id))]
# [table_model (foreign_key ((team_id ,) , (:: aps_teams :: teams :: id)))]
# [table_model (foreign_key ((member_id ,) , (:: aps_users :: users :: id)))]
# [diesel (table_name = team_members)]
pub struct TeamMember {
    /// The team to which the member belongs
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    team_id: ::rosetta_uuid::Uuid,
    /// The member who is part of the team
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    member_id: ::rosetta_uuid::Uuid,
}
