//! Test utilities for APS crates.

use aps_asset_models::*;
use aps_procedure_templates::*;
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
///   will be created.
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
/// * `user` - The user creating the asset model.
/// * `conn` - A mutable reference to the database connection where the asset
///   model will be created.
///
/// # Panics
/// * If the asset model creation fails.
///
/// # Example
///
/// ```rust
/// use aps_test_utils::{aps_conn, asset_model, user};
/// let mut conn = aps_conn();
/// let test_user = user(&mut conn);
/// let _test_asset_model = asset_model("Test Model", &test_user, &mut conn);
/// ```
pub fn asset_model<C>(name: &str, user: &aps_users::User, conn: &mut C) -> AssetModel
where
    TableBuilder<users::table>: Insert<C>,
    TableBuilder<asset_models::table>: Insert<C>,
{
    asset_models::table::builder()
        .try_name(name)
        .expect("Failed to set asset model name")
        .try_description("A test asset model")
        .expect("Failed to set asset model description")
        .creator_id(user.get_column::<users::id>())
        .editor_id(user.get_column::<users::id>())
        .insert(conn)
        .expect("Failed to create test asset model")
}

/// Creates and returns a container model with the given name in the provided
/// connection for testing purposes.
///
/// # Arguments
///
/// * `name` - The name of the container model to be created.
/// * `user` - The user creating the container model.
/// * `conn` - A mutable reference to the database connection where the
///   container model will be created.
///
/// # Panics
/// * If the container model creation fails.
///
/// # Example
///
/// ```rust
/// use aps_test_utils::{aps_conn, container_model, user};
/// let mut conn = aps_conn();
/// let test_user = user(&mut conn);
/// let _test_container_model = container_model("Test Container", &test_user, &mut conn);
/// ```
pub fn container_model<C>(
    name: &str,
    user: &aps_users::User,
    conn: &mut C,
) -> aps_container_models::ContainerModel
where
    TableBuilder<users::table>: Insert<C>,
    TableBuilder<aps_container_models::container_models::table>: Insert<C>,
{
    aps_container_models::container_models::table::builder()
        .try_name(name)
        .expect("Failed to set container model name")
        .try_description("A test container model")
        .expect("Failed to set container model description")
        .creator_id(user.get_column::<users::id>())
        .editor_id(user.get_column::<users::id>())
        .insert(conn)
        .expect("Failed to create test container model")
}

/// Creates and returns a procedure template with the given name in the
/// provided connection for testing purposes.
///
/// # Arguments
///
/// * `name` - The name of the procedure template to be created.
/// * `user` - The user creating the procedure template.
/// * `conn` - A mutable reference to the database connection where the
///   procedure template will be created.
///
/// # Panics
/// * If the procedure template creation fails.
///
/// # Example
///
/// ```rust
/// use aps_test_utils::{aps_conn, procedure_template, user};
/// let mut conn = aps_conn();
/// let test_user = user(&mut conn);
/// let _test_procedure_template = procedure_template("Test Procedure", &test_user, &mut conn);
/// ```
pub fn procedure_template<C>(
    name: &str,
    user: &aps_users::User,
    conn: &mut C,
) -> aps_procedure_templates::ProcedureTemplate
where
    TableBuilder<users::table>: Insert<C>,
    TableBuilder<aps_procedure_templates::procedure_templates::table>: Insert<C>,
{
    aps_procedure_templates::procedure_templates::table::builder()
        .try_name(name)
        .expect("Failed to set procedure template name")
        .try_description("A test procedure template")
        .expect("Failed to set procedure template description")
        .creator_id(user.get_column::<users::id>())
        .editor_id(user.get_column::<users::id>())
        .insert(conn)
        .expect("Failed to create test procedure template")
}
