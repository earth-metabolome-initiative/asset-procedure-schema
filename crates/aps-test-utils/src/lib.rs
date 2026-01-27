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
    let root = std::path::PathBuf::from(
        std::env::var("CARGO_MANIFEST_DIR").expect("Failed to get CARGO_MANIFEST_DIR"),
    )
    .join("../../");
    let translated = Pg2Sqlite::ups(root)
        .expect("Failed to get PostgreSQL migrations")
        .translate(&Pg2SqliteOptions::default().remove_unsupported_check_constraints())
        .expect("Failed to translate the PostgreSQL schema");
    let mut connection = SqliteConnection::establish(":memory:")
        .expect("Failed to create in-memory SQLite database");

    // Enable foreign key constraints
    diesel::sql_query("PRAGMA foreign_keys = ON")
        .execute(&mut connection)
        .expect("Failed to enable foreign key constraints");

    // Enable recursive triggers
    diesel::sql_query("PRAGMA recursive_triggers = ON")
        .execute(&mut connection)
        .expect("Failed to enable recursive triggers");

    // Set journal mode to WAL for better performance
    diesel::sql_query("PRAGMA journal_mode = WAL")
        .execute(&mut connection)
        .expect("Failed to set journal mode to WAL");

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

/// Creates and returns a physical asset model with the given name in the
/// provided connection for testing purposes.
///
/// # Arguments
///
/// * `name` - The name of the physical asset model to be created.
/// * `user` - The user creating the physical asset model.
/// * `conn` - A mutable reference to the database connection where the physical
///   asset model will be created.
///
/// # Panics
/// * If the physical asset model creation fails.
///
/// # Example
///
/// ```rust
/// use aps_test_utils::{aps_conn, physical_asset_model, user};
/// let mut conn = aps_conn();
/// let test_user = user(&mut conn);
/// let _test_physical_asset_model =
///     physical_asset_model("Test Physical Model", &test_user, &mut conn);
/// ```
pub fn physical_asset_model<C>(
    name: &str,
    user: &aps_users::User,
    conn: &mut C,
) -> aps_physical_asset_models::PhysicalAssetModel
where
    TableBuilder<users::table>: Insert<C>,
    TableBuilder<aps_physical_asset_models::physical_asset_models::table>: Insert<C>,
{
    aps_physical_asset_models::physical_asset_models::table::builder()
        .try_name(name)
        .expect("Failed to set physical asset model name")
        .try_description("A test physical asset model")
        .expect("Failed to set physical asset model description")
        .creator_id(user.get_column::<users::id>())
        .editor_id(user.get_column::<users::id>())
        .insert(conn)
        .expect("Failed to create test physical asset model")
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

/// Creates a procedure template which has child procedures, creating
/// a linear sequence of steps which share some of the procedure template
/// asset models between them.
///
/// To facilitate the semantic understanding of the procedure template and its
/// procedures, we use the example of making a pizza, which involves multiple
/// steps that can be represented as procedures.
///
/// # Arguments
///
/// * `user` - The user creating the procedure template and procedures.
/// * `conn` - A mutable reference to the database connection where the
///   procedure template and procedures will be created.
///
/// # Panics
///
/// * If the procedure template or procedure creation fails.
///
/// # Example
///
/// ```rust
/// use aps_test_utils::{aps_conn, pizza_procedure_template, user};
/// let mut conn = aps_conn();
/// let test_user = user(&mut conn);
/// let _pizza_template = pizza_procedure_template(&test_user, &mut conn);
/// ```
pub fn pizza_procedure_template(
    user: &aps_users::User,
    conn: &mut SqliteConnection,
) -> aps_procedure_templates::ProcedureTemplate {
    use procedure_traits::ProcedureTemplateNode;

    // First, we create the parent procedure template for making a pizza.
    let make_pizza = procedure_template("Make a Pizza", user, conn);

    // We create the asset models involved in the procedure template.
    let dough_model = asset_model("Pizza Dough", user, conn);
    let toppings_model = asset_model("Pizza Toppings", user, conn);
    let pizza_model = asset_model("Pizza", user, conn);
    let oven_model = asset_model("Oven", user, conn);

    // We create each step.

    // First step: Prepare the dough.
    let prepare_dough = procedure_template("Prepare Dough", user, conn);
    let [dough_ptam] = prepare_dough.requires_n([&dough_model], conn).unwrap();

    // Second step: Add toppings.
    let add_toppings = procedure_template("Add Toppings", user, conn);
    let [_toppings_model_ptam, pizza_model_ptam] =
        add_toppings.requires_n([&toppings_model, &pizza_model], conn).unwrap();
    add_toppings.reuses([dough_ptam], conn).unwrap();

    // Bake the pizza.
    let bake_pizza = procedure_template("Bake Pizza", user, conn);
    let [_oven_model_ptam] = bake_pizza.requires_n([&oven_model], conn).unwrap();
    bake_pizza.reuses([pizza_model_ptam], conn).unwrap();

    // Next, we create the individual steps, and we use the traits defined in
    // the `procedure-traits` crate to set up the relationships between them.
    make_pizza.child(&prepare_dough, user, conn).unwrap();
    make_pizza.child(&add_toppings, user, conn).unwrap();
    make_pizza.child(&bake_pizza, user, conn).unwrap();
    make_pizza
        .extend([prepare_dough, add_toppings, bake_pizza], user, conn)
        .map_err(|e| {
            println!("Error extending 'Make a Pizza' procedure template: {:?}", e);
            if let diesel::result::Error::DatabaseError(_, info) = &e {
                println!("Database error details: {}", info.message());
                println!("Database error details: {}", info.details().unwrap_or("No details"));
                println!("Database error hint: {}", info.hint().unwrap_or("No hint"));
                println!(
                    "Database error constraint: {}",
                    info.constraint_name().unwrap_or("No constraint")
                );
                println!("Database error table: {}", info.table_name().unwrap_or("No table"));
                println!("Database error column: {}", info.column_name().unwrap_or("No column"));
            }
            panic!("Failed to extend 'Make a Pizza' procedure template: {}", e);
        })
        .unwrap();
    make_pizza
}
