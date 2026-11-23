//! The replica keeps the server's tables to the server's deliveries.

use aps_test_utils::Replica;
use diesel::{RunQueryDsl, sql_query};
use rosetta_uuid::{Uuid, diesel_impls::Uuid as UuidSql};

/// Inserts the grantee and user rows of a new user.
fn insert_user(connection: &mut diesel::SqliteConnection, id: Uuid) -> diesel::QueryResult<()> {
    sql_query("INSERT INTO grantees (id, kind) VALUES (?, 'user')")
        .bind::<UuidSql, _>(id)
        .execute(connection)?;
    sql_query("INSERT INTO users (id) VALUES (?)").bind::<UuidSql, _>(id).execute(connection)?;
    Ok(())
}

#[test]
fn users_arrive_only_from_the_server() {
    let mut replica = Replica::new();
    let user = Uuid::new_v4();
    replica.act_as(Some(user));
    assert!(replica.write(|connection| insert_user(connection, user)).is_err());
    replica.deliver(|connection| insert_user(connection, user)).expect("delivered user");
}
