//! Test utilities for APS crates.

use aps_asset_models::*;
use aps_users::*;
use diesel::{Connection, SqliteConnection, connection::SimpleConnection};
use diesel_builders::{TableBuilder, prelude::*};
use pg2sqlite::{
    prelude::{Pg2Sqlite, Pg2SqliteOptions},
    traits::TranslationOptions,
};

/// Creates an in-memory SQLite connection by translating the PostgreSQL
/// schema from the asset-procedure-schema repository.
///
/// # Panics
///
/// * If cloning the repository fails.
/// * If translating the schema fails.
/// * If applying the translated migrations fails.
///
/// # Example
///
/// ```rust
/// use aps_test_utils::aps_conn;
/// let _conn = aps_conn();
/// ```
pub fn aps_conn() -> SqliteConnection {
    let translated = Pg2Sqlite::from_git(
        "https://github.com/earth-metabolome-initiative/asset-procedure-schema",
    )
    .expect("Failed to clone the asset-procedure-schema repository")
    .translate(&Pg2SqliteOptions::default().remove_unsupported_check_constraints())
    .expect("Failed to translate the PostgreSQL schema");
    let mut connection = SqliteConnection::establish(":memory:")
        .expect("Failed to create in-memory SQLite database");

    for translated_migration in translated.iter() {
        let sql = translated_migration.to_string();
        connection
            .batch_execute(&sql)
            .unwrap_or_else(|_| panic!("Failed to apply translated migration:\n{}", sql));
    }

    connection
}

/// Creates and returns a standard user in the provided connection for testing
/// purposes.
///
/// # Arguments
///
/// * `conn` - A mutable reference to the database connection where the user
///  will be created.
///
/// # Panics
/// * If the user creation fails.
///
/// # Example
///
/// ```rust
/// use aps_test_utils::user;
/// let mut conn = aps_test_utils::aps_conn();
/// let _test_user = user(&mut conn);
/// ```
pub fn user<C>(conn: &mut C) -> aps_users::User
where
    TableBuilder<users::table>: Insert<C>,
{
    users::table::builder().insert(conn).expect("Failed to create test user")
}

/// Creates and returns an asset model with the given name in the provided
/// connection for testing purposes.
///
/// # Arguments
///
/// * `name` - The name of the asset model to be created.
/// * `conn` - A mutable reference to the database connection where the asset
///   model will be created.
///
/// # Panics
/// * If the asset model creation fails.
/// 
/// # Example
/// 
/// ```rust
/// use aps_test_utils::asset_model;
/// let mut conn = aps_test_utils::aps_conn();
/// let _test_asset_model = asset_model("Test Model", &mut conn);
/// ```
pub fn asset_model<C>(name: &str, conn: &mut C) -> AssetModel
where
    TableBuilder<users::table>: Insert<C>,
    TableBuilder<asset_models::table>: Insert<C>,
{
    let creator = user(conn);
    asset_models::table::builder()
        .try_name(name)
        .expect("Failed to set asset model name")
        .try_description("A test asset model")
        .expect("Failed to set asset model description")
        .creator_id(&creator)
        .editor_id(&creator)
        .insert(conn)
        .expect("Failed to create test asset model")
}
