//! Test utilities for APS crates.

use aps_asset_models::*;
use aps_container_models::container_models;
use aps_namespaced_ownables::*;
use aps_namespaces::*;
use aps_ownables::*;
use aps_physical_asset_models::physical_asset_models;
use aps_procedure_templates::*;
use aps_users::*;
use diesel::{Connection, SqliteConnection, connection::SimpleConnection};
use diesel_builders::{TableBuilder, prelude::*};
use pg2sqlite::{
    prelude::{Pg2Sqlite, Pg2SqliteOptions},
    traits::{TranslationOptions, UuidRepresentation},
};

#[declare_sql_function]
extern "SQL" {
    /// Generates a UUID v4
    fn uuidv4() -> Binary;
    /// Generates a UUID v7
    fn uuidv7() -> Binary;
}

/// Creates an in-memory SQLite connection by translating the PostgreSQL
/// schema from the asset-procedure-schema repository.
///
/// # Panics
///
/// * If cloning the repository fails.
/// * If translating the schema fails.
/// * If applying the translated migrations fails.
fn aps_conn_from_statements(translator: Pg2Sqlite) -> SqliteConnection {
    let translated = translator
        .translate(
            &Pg2SqliteOptions::default()
                .remove_unsupported_check_constraints()
                .with_uuid_representation(UuidRepresentation::Blob)
                .with_uuid_function_name("uuidv7".to_string()),
        )
        .expect("Failed to translate the PostgreSQL schema");

    let sql = translated.iter().map(ToString::to_string).collect::<Vec<_>>().join(";\n");

    println!("{sql}");

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

    uuidv4_utils::register_impl(&connection, rosetta_uuid::Uuid::new_v4)
        .expect("Failed to register uuidv4");
    uuidv7_utils::register_impl(&connection, rosetta_uuid::Uuid::utc_v7)
        .expect("Failed to register uuidv7");

    for translated_migration in translated.iter() {
        let sql = translated_migration.to_string();
        connection
            .batch_execute(&sql)
            .unwrap_or_else(|err| panic!("Failed to apply translated migration:\n{sql} - {err}"));
    }

    connection
}

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
    aps_conn_from_statements(
        Pg2Sqlite::ups(root)
            .expect("Failed to get local PostgreSQL migrations, are you in the APS repository?"),
    )
}

/// Creates an in-memory SQLite connection by translating the PostgreSQL
/// schema from the asset-procedure-schema repository, loading the migrations
/// directly from the GitHub repository.
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
/// use aps_test_utils::aps_git_conn;
/// let _conn = aps_git_conn();
/// ```
pub fn aps_git_conn() -> SqliteConnection {
    aps_conn_from_statements(
        Pg2Sqlite::from_git(
            "https://github.com/earth-metabolome-initiative/asset-procedure-schema",
        )
        .expect("Failed to get PostgreSQL migrations from Git repository"),
    )
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

/// Creates and returns a namespace in the provided connection for testing
/// purposes.
///
/// # Arguments
///
/// * `name` - The name of the namespace to be created.
/// * `user` - The user creating the namespace.
/// * `conn` - A mutable reference to the database connection where the
///   namespace will be created.
///
/// # Panics
///
/// * If the namespace creation fails.
///
/// # Example
///
/// ```rust
/// use aps_test_utils::{aps_conn, namespace, user};
/// let mut conn = aps_conn();
///
/// let test_user = user(&mut conn);
/// let _test_namespace = namespace("Test Namespace", &test_user, &mut conn);
/// ```
pub fn namespace<C>(
    name: &str,
    user: &aps_users::User,
    conn: &mut C,
) -> NestedModel<namespaces::table>
where
    TableBuilder<namespaces::table>: Insert<C>,
    (namespaces::name,): LoadNestedFirst<namespaces::table, C>,
{
    if let Ok(existing) = <(namespaces::name,)>::load_nested_first((name,), conn) {
        return existing;
    }

    namespaces::table::builder()
        .try_name(name)
        .expect("Failed to set namespace name")
        .owner_id(user.get_column::<users::id>())
        .creator_id(user.get_column::<users::id>())
        .editor_id(user.get_column::<users::id>())
        .insert_nested(conn)
        .expect("Failed to create test namespace")
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
pub fn asset_model<C>(
    name: &str,
    user: &aps_users::User,
    conn: &mut C,
) -> NestedModel<asset_models::table>
where
    TableBuilder<users::table>: Insert<C>,
    TableBuilder<asset_models::table>: Insert<C>,
    TableBuilder<namespaces::table>: Insert<C>,
    (namespaces::name,): LoadNestedFirst<namespaces::table, C>,
    (namespaced_ownables::namespace_id, (namespaced_ownables::name,)):
        LoadNestedFirst<asset_models::table, C>,
{
    let test_namespace = namespace("aps-test-utils", user, conn);

    // We try to load an existing asset model with the same name to avoid duplicates
    // in tests that create multiple asset models with the same name.
    if let Ok(existing) =
        <(namespaced_ownables::namespace_id, (namespaced_ownables::name,))>::load_nested_first(
            (test_namespace.get_column::<namespaces::id>(), (name,)),
            conn,
        )
    {
        return existing;
    }

    asset_models::table::builder()
        .try_name(name)
        .expect("Failed to set asset model name")
        .try_description("A test asset model")
        .expect("Failed to set asset model description")
        .creator_id(user.get_column::<users::id>())
        .editor_id(user.get_column::<users::id>())
        .owner_id(user.get_column::<users::id>())
        .namespace_id(test_namespace.get_column::<namespaces::id>())
        .insert_nested(conn)
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
) -> NestedModel<physical_asset_models::table>
where
    TableBuilder<physical_asset_models::table>: Insert<C>,
    TableBuilder<namespaces::table>: Insert<C>,
    (namespaces::name,): LoadNestedFirst<namespaces::table, C>,
    (namespaced_ownables::namespace_id, (namespaced_ownables::name,)):
        LoadNestedFirst<physical_asset_models::table, C>,
{
    let test_namespace = namespace("aps-test-utils", user, conn);
    if let Ok(existing) =
        <(namespaced_ownables::namespace_id, (namespaced_ownables::name,))>::load_nested_first(
            (test_namespace.get_column::<namespaces::id>(), (name,)),
            conn,
        )
    {
        return existing;
    }

    physical_asset_models::table::builder()
        .try_name(name)
        .expect("Failed to set physical asset model name")
        .try_description("A test physical asset model")
        .expect("Failed to set physical asset model description")
        .creator_id(user.get_column::<users::id>())
        .editor_id(user.get_column::<users::id>())
        .owner_id(user.get_column::<users::id>())
        .namespace_id(test_namespace.get_column::<namespaces::id>())
        .insert_nested(conn)
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
) -> NestedModel<container_models::table>
where
    TableBuilder<container_models::table>: Insert<C>,
    TableBuilder<namespaces::table>: Insert<C>,
    (namespaces::name,): LoadNestedFirst<namespaces::table, C>,
    (namespaced_ownables::namespace_id, (namespaced_ownables::name,)):
        LoadNestedFirst<container_models::table, C>,
{
    let test_namespace = namespace("aps-test-utils", user, conn);
    if let Ok(existing) =
        <(namespaced_ownables::namespace_id, (namespaced_ownables::name,))>::load_nested_first(
            (test_namespace.get_column::<namespaces::id>(), (name,)),
            conn,
        )
    {
        return existing;
    }

    container_models::table::builder()
        .try_name(name)
        .expect("Failed to set container model name")
        .try_description("A test container model")
        .expect("Failed to set container model description")
        .creator_id(user.get_column::<users::id>())
        .editor_id(user.get_column::<users::id>())
        .owner_id(user.get_column::<users::id>())
        .namespace_id(test_namespace.get_column::<namespaces::id>())
        .insert_nested(conn)
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
) -> NestedModel<procedure_templates::table>
where
    TableBuilder<procedure_templates::table>: Insert<C>,
    TableBuilder<namespaces::table>: Insert<C>,
    (namespaces::name,): LoadNestedFirst<namespaces::table, C>,
    (namespaced_ownables::namespace_id, (namespaced_ownables::name,)):
        LoadNestedFirst<procedure_templates::table, C>,
{
    let test_namespace = namespace("aps-test-utils", user, conn);
    if let Ok(existing) =
        <(namespaced_ownables::namespace_id, (namespaced_ownables::name,))>::load_nested_first(
            (test_namespace.get_column::<namespaces::id>(), (name,)),
            conn,
        )
    {
        return existing;
    }

    procedure_templates::table::builder()
        .try_name(name)
        .expect("Failed to set procedure template name")
        .try_description("A test procedure template")
        .expect("Failed to set procedure template description")
        .creator_id(user.get_column::<users::id>())
        .editor_id(user.get_column::<users::id>())
        .owner_id(user.get_column::<users::id>())
        .namespace_id(test_namespace.get_column::<namespaces::id>())
        .insert_nested(conn)
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
) -> NestedModel<procedure_templates::table> {
    use aps_traits::ProcedureTemplateNode;

    // First, we create the parent procedure template for making a pizza.
    let make_pizza: NestedModel<procedure_templates::table> =
        procedure_template("Make a Pizza", user, conn);

    // We create the asset models involved in the procedure template.
    let dough_model: NestedModel<asset_models::table> = asset_model("Pizza Dough", user, conn);
    let toppings_model: NestedModel<asset_models::table> =
        asset_model("Pizza Toppings", user, conn);
    let pizza_model: NestedModel<asset_models::table> = asset_model("Pizza", user, conn);
    let oven_model: NestedModel<asset_models::table> = asset_model("Oven", user, conn);

    // We create each step.

    // First step: Prepare the dough.
    let prepare_dough: NestedModel<procedure_templates::table> =
        procedure_template("Prepare Dough", user, conn);
    let [dough_ptam] = prepare_dough.requires_n([&dough_model], conn).unwrap();

    // Second step: Add toppings.
    let add_toppings: NestedModel<procedure_templates::table> =
        procedure_template("Add Toppings", user, conn);
    let [_toppings_model_ptam, pizza_model_ptam] =
        add_toppings.requires_n([&toppings_model, &pizza_model], conn).unwrap();
    add_toppings.reuses([dough_ptam], conn).unwrap();

    // Bake the pizza.
    let bake_pizza: NestedModel<procedure_templates::table> =
        procedure_template("Bake Pizza", user, conn);
    let [_oven_model_ptam] = bake_pizza.requires_n([&oven_model], conn).unwrap();
    bake_pizza.reuses([pizza_model_ptam], conn).unwrap();

    // Next, we create the individual steps, and we use the traits defined in
    // the `aps-traits` crate to set up the relationships between them.
    make_pizza
        .extend([prepare_dough, add_toppings, bake_pizza], user, conn)
        .expect("Failed to extend 'Make a Pizza' procedure template");
    make_pizza
}

/// Creates a procedure template which has child procedures, creating
/// a forking sequence of steps which share some of the procedure template
/// asset models between them.
///
/// To facilitate the semantic understanding of the procedure template and its
/// procedures, we use the example of making a pizza, which involves multiple
/// steps that can be represented as procedures. The forking step will be the
/// choice of different toppings.
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
/// use aps_test_utils::{aps_conn, pizza_four_season_procedure_template, user};
/// let mut conn = aps_conn();
/// let test_user = user(&mut conn);
/// let _pizza_template = pizza_four_season_procedure_template(&test_user, &mut conn);
/// ```
pub fn pizza_four_season_procedure_template(
    user: &aps_users::User,
    conn: &mut SqliteConnection,
) -> NestedModel<procedure_templates::table> {
    use aps_traits::ProcedureTemplateNode;

    // First, we create the parent procedure template for making a pizza.
    let make_pizza = procedure_template("Make a Four Seasons Pizza", user, conn);

    // We create the asset models involved in the procedure template.
    let dough_model: NestedModel<asset_models::table> = asset_model("Pizza Dough 2", user, conn);
    let mozzarella_model: NestedModel<asset_models::table> =
        asset_model("Mozzarella Cheese 2", user, conn);
    let mushrooms_model: NestedModel<asset_models::table> = asset_model("Mushrooms", user, conn);
    let artichokes_model: NestedModel<asset_models::table> = asset_model("Artichokes", user, conn);
    let ham_model: NestedModel<asset_models::table> = asset_model("Ham", user, conn);
    let tofu_model: NestedModel<asset_models::table> = asset_model("Tofu", user, conn);
    let olives_model: NestedModel<asset_models::table> = asset_model("Olives", user, conn);
    let veg_pizza_model: NestedModel<asset_models::table> =
        asset_model("Vegetarian Pizza", user, conn);
    let omni_pizza_model: NestedModel<asset_models::table> =
        asset_model("Omnivore Pizza", user, conn);
    let oven_model: NestedModel<asset_models::table> = asset_model("Oven 2", user, conn);

    // We create each step.

    // Step zero: Heat the oven.
    let heat_oven: NestedModel<procedure_templates::table> =
        procedure_template("Heat Oven", user, conn);
    let [oven_model_ptam] = heat_oven.requires_n([&oven_model], conn).unwrap();

    // Step half: Prepare the ingredients and cut them as needed.
    let prepare_ingredients: NestedModel<procedure_templates::table> =
        procedure_template("Prepare Ingredients", user, conn);
    let [
        mozzarella_model_ptam,
        mushrooms_model_ptam,
        artichokes_model_ptam,
        ham_model_ptam,
        tofu_model_ptam,
        olives_model_ptam,
    ] = prepare_ingredients
        .requires_n(
            [
                &mozzarella_model,
                &mushrooms_model,
                &artichokes_model,
                &ham_model,
                &tofu_model,
                &olives_model,
            ],
            conn,
        )
        .unwrap();

    // First step: Prepare the dough.
    let prepare_dough: NestedModel<procedure_templates::table> =
        procedure_template("Prepare Dough 2", user, conn);
    let [dough_ptam] = prepare_dough.requires_n([&dough_model], conn).unwrap();

    // Second step, option 1: Add vegetarian toppings.
    let add_vegetarian_toppings: NestedModel<procedure_templates::table> =
        procedure_template("Vegetarian Toppings", user, conn);
    let [veg_pizza_model_ptam] =
        add_vegetarian_toppings.requires_n([&veg_pizza_model], conn).unwrap();
    add_vegetarian_toppings
        .reuses(
            [
                &mozzarella_model_ptam,
                &mushrooms_model_ptam,
                &artichokes_model_ptam,
                &tofu_model_ptam,
                &olives_model_ptam,
                &dough_ptam,
            ],
            conn,
        )
        .unwrap();
    // Second step, option 2: Add omnivore toppings.
    let add_omnivore_toppings: NestedModel<procedure_templates::table> =
        procedure_template("Omnivore Toppings", user, conn);
    let [omni_pizza_model_ptam] =
        add_omnivore_toppings.requires_n([&omni_pizza_model], conn).unwrap();
    add_omnivore_toppings
        .reuses(
            [
                &mozzarella_model_ptam,
                &mushrooms_model_ptam,
                &artichokes_model_ptam,
                &ham_model_ptam,
                &olives_model_ptam,
            ],
            conn,
        )
        .unwrap();

    // Bake the pizza.
    let bake_veg_pizza: NestedModel<procedure_templates::table> =
        procedure_template("Bake Vegetarian Pizza", user, conn);
    bake_veg_pizza.reuses([&oven_model_ptam, &veg_pizza_model_ptam], conn).unwrap();
    let bake_omni_pizza: NestedModel<procedure_templates::table> =
        procedure_template("Bake Omnivore Pizza", user, conn);
    bake_omni_pizza.reuses([&oven_model_ptam, &omni_pizza_model_ptam], conn).unwrap();

    // Next, we create the individual steps, and we use the traits defined in
    // the `aps-traits` crate to set up the relationships between them.
    // First, we define the sequence for the vegetarian path.
    make_pizza
        .extend([&heat_oven, &prepare_dough, &add_vegetarian_toppings, &bake_veg_pizza], user, conn)
        .expect("Failed to extend 'Make a Pizza' procedure template, vegetarian path");
    // Then, we define the sequence for the omnivore path.
    make_pizza
        .extend([&prepare_dough, &add_omnivore_toppings, &bake_omni_pizza], user, conn)
        .expect("Failed to extend 'Make a Pizza' procedure template, omnivore path");
    make_pizza
}
