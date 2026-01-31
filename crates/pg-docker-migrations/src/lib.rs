#![doc = include_str!("../README.md")]

use diesel::Connection;
use testcontainers::{
    ContainerAsync, GenericImage, ImageExt, TestcontainersError,
    core::{IntoContainerPort, WaitFor},
    runners::AsyncRunner,
};

/// The default database user.
pub const DATABASE_USER: &str = "user";
/// The default database user password.
pub const DATABASE_PASSWORD: &str = "password";

/// Setup a docker container with a postgres database.
///
/// # Arguments
///
/// * `database_port` - The port of the database.
/// * `database_name` - The name of the database.
///
/// # Panics
///
/// * If the container cannot be started.
async fn postgis(
    database_port: u16,
    database_name: &str,
) -> Result<ContainerAsync<GenericImage>, TestcontainersError> {
    GenericImage::new("postgis/postgis", "18-3.6")
        .with_wait_for(WaitFor::message_on_stderr("database system is ready to accept connections"))
        .with_network("bridge")
        .with_env_var("DEBUG", "1")
        .with_env_var("POSTGRES_USER", DATABASE_USER)
        .with_env_var("POSTGRES_PASSWORD", DATABASE_PASSWORD)
        .with_env_var("POSTGRES_DB", database_name)
        .with_mapped_port(database_port, 5432_u16.tcp())
        .start()
        .await
}

/// Establish a connection to a postgres database.
///
/// # Arguments
///
/// * `database_port` - The port of the database.
/// * `database_name` - The name of the database.
///
/// # Errors
///
/// * If the connection cannot be established.
fn establish_connection_to_postgres<C: Connection>(
    database_port: u16,
    database_name: &str,
) -> Result<C, diesel::ConnectionError> {
    let database_url = format!(
        "postgres://{DATABASE_USER}:{DATABASE_PASSWORD}@localhost:{database_port}/{database_name}",
    );

    let mut number_of_attempts = 0;

    while let Err(e) = C::establish(&database_url) {
        eprintln!("Failed to establish connection: {e:?}");
        std::thread::sleep(std::time::Duration::from_secs(1));
        if number_of_attempts > 10 {
            eprintln!("Failed to establish connection after 10 attempts");
            std::process::exit(1);
        }
        number_of_attempts += 1;
    }

    C::establish(&database_url)
}

/// Setup a database with a custom migration dir.
///
/// # Arguments
///
/// * `database_name` - The name of the database.
/// * `port` - The port of the database.
///
/// # Errors
///
/// * If the connection cannot be established.
/// * If the container cannot be started.
///
/// # Example
///
/// ```rust
/// use diesel::PgConnection;
/// use pg_docker_migrations::connect;
///
/// #[tokio::main]
/// async fn main() {
///     let (docker, conn) = connect::<PgConnection>("test_db", 6437).await.unwrap();
/// }
/// ```
pub async fn connect<C: Connection>(
    database_name: &str,
    port: u16,
) -> Result<(ContainerAsync<GenericImage>, C), diesel::ConnectionError> {
    let docker = postgis(port, database_name).await.expect("Failed to start container");
    let conn = establish_connection_to_postgres(port, database_name)?;
    Ok((docker, conn))
}

/// Setups a docker container, connects to the database, and applies migrations from the specified directory.
/// 
/// # Arguments
/// 
/// * `database_name` - The name of the database.
/// * `port` - The port of the database.
/// * `migrations_dir` - The directory containing the migrations.
/// * `last_migration` - The last migration to apply, optional.
/// 
/// # Errors
/// 
/// * If the connection cannot be established.
/// * If the container cannot be started.
/// * If the migrations cannot be applied.
pub async fn setup_database_with_migrations<C: Connection, P: AsRef<std::path::Path>>(
    database_name: &str,
    port: u16,
    migrations_dir: P,
    last_migration: Option<P>,
) -> Result<(ContainerAsync<GenericImage>, C), Box<dyn std::error::Error>> {
    let (docker, conn) = connect::<C>(database_name, port).await?;
    todo!()
}

/// RLS-aware migration, which applies migrations.
pub async fn setup_database_with_rls_migrations<C: Connection, P: AsRef<std::path::Path>>(
    database_name: &str,
    port: u16,
    migrations_dir: P,
    last_migration: Option<P>,
    user: &Uuid
) -> Result<(ContainerAsync<GenericImage>, C), Box<dyn std::error::Error>> {
    let (docker, conn) = connect::<C>(database_name, port).await?;
    todo!()
}