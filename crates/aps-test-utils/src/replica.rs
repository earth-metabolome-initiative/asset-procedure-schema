//! The SQLite replica of connetto-schema and the APS slices, as pg2sqlite
//! translates them for the acting role `app`.

use std::{
    fs::File,
    hash::{DefaultHasher, Hash, Hasher},
    io::ErrorKind,
    path::{Path, PathBuf},
    sync::{
        Arc, LazyLock, Mutex, Once,
        atomic::{AtomicBool, Ordering},
    },
};

use diesel::{
    Connection, QueryResult, SqliteConnection,
    connection::SimpleConnection,
    sql_types::{Binary, Bool, Integer, Nullable, Text},
};
use pg2sqlite::{
    prelude::{
        ArrayRepresentation, CaseFoldFunctions, HelperFunction, JsonbFunctions, Pg2Sqlite,
        Pg2SqliteOptions, SessionVariableMapping, UuidRepresentation,
    },
    programs::unprepared_programs,
};
use serde::{Deserialize, Serialize};
use sql_traits::structs::{Deployment, SessionSetting, SetPacking, SettingValue};
use sqlparser::ast::DataType;

/// The role every replica write runs as.
pub const ACTING_ROLE: &str = "app";
/// The session settings the replica answers through functions named by
/// [`setting_function`].
const SETTINGS: [&str; 5] = [
    "app.user_id",
    "app.installation_id",
    "app.original_actor_id",
    "app.link_ids",
    "app.verified_email",
];

diesel::define_sql_function! {
    /// Answers `app.user_id`.
    fn app_user_id() -> Nullable<Text>;
}
diesel::define_sql_function! {
    /// Answers `app.installation_id`.
    fn app_installation_id() -> Nullable<Text>;
}
diesel::define_sql_function! {
    /// Answers `app.original_actor_id`.
    fn app_original_actor_id() -> Nullable<Text>;
}
diesel::define_sql_function! {
    /// Answers `app.link_ids`.
    fn app_link_ids() -> Nullable<Text>;
}
diesel::define_sql_function! {
    /// Answers `app.verified_email`.
    fn app_verified_email() -> Nullable<Text>;
}
diesel::define_sql_function! {
    /// Answers whether the running write is a delivery from the server.
    fn connetto_write_exempt() -> Bool;
}
diesel::define_sql_function! {
    /// Marks the start (`1`) and end (`-1`) of the writes a trigger derives.
    fn server_derived_write(step: Integer) -> Integer;
}
diesel::define_sql_function! {
    /// Answers a random version 4 UUID as its 16 bytes.
    fn uuidv4() -> Binary;
}
diesel::define_sql_function! {
    /// PostgreSQL's Unicode `lower`.
    fn pg_lower(text: Nullable<Text>) -> Nullable<Text>;
}
diesel::define_sql_function! {
    /// PostgreSQL's Unicode `upper`.
    fn pg_upper(text: Nullable<Text>) -> Nullable<Text>;
}

/// The SQLite function answering `setting`.
fn setting_function(setting: &str) -> String {
    setting.replace('.', "_")
}

/// The functions and collation the replica's `jsonb` columns use.
fn jsonb_functions() -> JsonbFunctions {
    JsonbFunctions::new("pg_jsonb", "pg_jsonb_text", "pg_jsonb_order")
}

/// The deployment the replica serves, declaring what each setting holds and
/// that every resource a role administers it may also write and read.
fn deployment() -> Deployment {
    let unset = |name: &str, value| {
        SessionSetting::value(name, value).expect("session setting").may_be_unset()
    };
    Deployment::new(
        ACTING_ROLE,
        [ACTING_ROLE],
        [
            unset("app.user_id", SettingValue::Caller(DataType::Uuid)),
            unset(
                "app.installation_id",
                SettingValue::Principal { kind: "installation".into(), identity: DataType::Uuid },
            ),
            unset("app.original_actor_id", SettingValue::Scalar(DataType::Uuid)),
            unset(
                "app.link_ids",
                SettingValue::Set {
                    element: DataType::Uuid,
                    packing: SetPacking::Delimited(",".into()),
                },
            ),
            unset("app.verified_email", SettingValue::Scalar(DataType::Text)),
        ],
    )
    .expect("deployment")
    .with_set_within("authz.resources_with('admin')", "authz.resources_with('writer')")
    .expect("a set expression")
    .with_set_within("authz.resources_with('writer')", "authz.resources_with('reader')")
    .expect("a set expression")
}

/// The PostgreSQL major the replica stands in for, the default of
/// `tests/Dockerfile`.
const SERVER_POSTGRES_MAJOR: u16 = 18;

/// The options the replica translates under.
fn options() -> Pg2SqliteOptions {
    let mut options = Pg2SqliteOptions::default()
        .with_uuid_representation(UuidRepresentation::Blob)
        .with_uuid_function_name("uuidv4")
        .with_rls_audit_table_name("rls_audit")
        .with_rls_exemption_function("connetto_write_exempt")
        .with_deployment(deployment())
        .with_acting_role(ACTING_ROLE)
        .with_server_derived_write_function("server_derived_write")
        .with_jsonb_functions(jsonb_functions())
        .with_array_representation(ArrayRepresentation::Json)
        .with_sqlitegis_enabled()
        .with_case_fold_functions(CaseFoldFunctions::new("pg_lower", "pg_upper"))
        .with_postgres_major(SERVER_POSTGRES_MAJOR);
    for setting in SETTINGS {
        options = options
            .with_session_variable(SessionVariableMapping::new(setting, setting_function(setting)));
    }
    options
}

/// The repository root, two levels above this crate.
fn repository() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// Every file the replica translates, in the order PostgreSQL applies them:
/// the connetto-schema and APS cluster roles, the connetto-schema slices,
/// then every APS slice in path order.
fn sources() -> Vec<PathBuf> {
    let root = repository();
    let mut connetto: Vec<PathBuf> = std::fs::read_dir(root.join("connetto-schema/sql"))
        .expect("connetto-schema slices, is the submodule checked out?")
        .map(|entry| entry.expect("connetto-schema slice").path().join("up.sql"))
        .filter(|path| path.exists())
        .collect();
    connetto.sort();
    let mut aps = Vec::new();
    let mut directories = vec![root.join("sql")];
    while let Some(directory) = directories.pop() {
        for entry in std::fs::read_dir(directory).expect("APS slice directory") {
            let path = entry.expect("APS slice").path();
            if path.is_dir() {
                directories.push(path);
            } else if path.file_name().is_some_and(|name| name == "up.sql") {
                aps.push(path);
            }
        }
    }
    aps.sort();
    let mut all =
        vec![root.join("connetto-schema/cluster/roles.sql"), root.join("cluster/roles.sql")];
    all.extend(connetto);
    all.extend(aps);
    all
}

/// The translated replica, as text.
#[derive(Serialize, Deserialize)]
struct ReplicaScript {
    /// The statements creating the replica.
    statements: Vec<String>,
    /// The statements run before every `COMMIT` of a writing transaction.
    before_commit: Vec<String>,
    /// The helpers every connection registers as table-valued functions
    /// before running any statement.
    functions: Vec<ReplicaFunction>,
}

/// A helper the statements call as a table-valued function, as text.
#[derive(Serialize, Deserialize)]
struct ReplicaFunction {
    /// The name a call site reads the function by.
    name: String,
    /// The parameters, in the order a call site passes them.
    parameters: Vec<String>,
    /// The columns of each row the function answers.
    columns: Vec<String>,
    /// The query answering a call.
    body: String,
}

impl<S: ToString> From<&HelperFunction<S>> for ReplicaFunction {
    fn from(function: &HelperFunction<S>) -> Self {
        Self {
            name: function.name.clone(),
            parameters: function.parameters.clone(),
            columns: function.columns.clone(),
            body: function.body.to_string(),
        }
    }
}

impl ReplicaFunction {
    /// The helper as pg2sqlite registers and prepares it.
    fn helper(&self) -> HelperFunction<&str> {
        HelperFunction {
            name: self.name.clone(),
            parameters: self.parameters.clone(),
            columns: self.columns.clone(),
            body: self.body.as_str(),
        }
    }
}

/// Translates every source slice.
fn translate(sources: &[PathBuf]) -> ReplicaScript {
    let translator = sources.iter().fold(Pg2Sqlite::default(), |translator, source| {
        translator
            .file(source)
            .unwrap_or_else(|error| panic!("{} does not parse: {error}", source.display()))
    });
    let report = translator
        .translate_with_report(&options())
        .unwrap_or_else(|error| panic!("pg2sqlite refuses the schema: {error}"));
    ReplicaScript {
        statements: report.script.iter().map(ToString::to_string).collect(),
        before_commit: report.script.before_commit.iter().map(ToString::to_string).collect(),
        functions: report.script.functions.iter().map(ReplicaFunction::from).collect(),
    }
}

/// The `Cargo.lock` entries of every package from a registry or a git
/// repository, pg2sqlite and what it builds on among them, leaving out the
/// workspace crates, whose entries change with every dependency they gain.
fn external_packages() -> Vec<String> {
    let path = repository().join("Cargo.lock");
    let lock = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("reading {}: {error}", path.display()));
    lock.split("[[package]]")
        .filter(|package| package.contains("\nsource = "))
        .map(str::to_owned)
        .collect()
}

/// The translated replica, read from the cache under the workspace `target/`
/// or translated and cached there.
///
/// Translating the whole schema takes pg2sqlite minutes, so the script is
/// cached under a key hashing every source slice, the external packages of
/// `Cargo.lock`, which pin pg2sqlite, and this file, which holds the
/// translation options and the cached form. A file lock lets one test
/// process translate while the others wait for its result.
fn cached_script() -> ReplicaScript {
    let sources = sources();
    let mut hasher = DefaultHasher::new();
    for path in &sources {
        let content = std::fs::read(path)
            .unwrap_or_else(|error| panic!("reading {}: {error}", path.display()));
        path.strip_prefix(repository()).unwrap_or(path).hash(&mut hasher);
        content.hash(&mut hasher);
    }
    external_packages().hash(&mut hasher);
    include_str!("replica.rs").hash(&mut hasher);
    let directory = repository().join("target/aps-replica");
    std::fs::create_dir_all(&directory).expect("replica cache directory");
    let cached = directory.join(format!("{:016x}.json", hasher.finish()));
    let lock = File::create(directory.join("lock")).expect("replica cache lock file");
    lock.lock().expect("replica cache lock");
    let script = match std::fs::read(&cached) {
        Ok(bytes) => serde_json::from_slice(&bytes).expect("cached replica script"),
        Err(error) if error.kind() == ErrorKind::NotFound => {
            let script = translate(&sources);
            let written = directory.join("translating.json");
            std::fs::write(&written, serde_json::to_vec(&script).expect("replica script JSON"))
                .expect("writing the replica cache");
            std::fs::rename(&written, &cached).expect("publishing the replica cache");
            script
        }
        Err(error) => panic!("reading {}: {error}", cached.display()),
    };
    lock.unlock().expect("replica cache unlock");
    script
}

/// The translation every replica of this process loads.
static SCRIPT: LazyLock<ReplicaScript> = LazyLock::new(cached_script);

/// Whether this process already checked that SQLite prepares every program
/// of the translation.
static PREPARED: Once = Once::new();

/// An in-memory SQLite replica, acting as `app` for the user
/// [`Replica::act_as`] last named.
pub struct Replica {
    connection: SqliteConnection,
    exempt: Arc<AtomicBool>,
    user: Arc<Mutex<Option<String>>>,
}

impl Replica {
    /// A fresh replica with the schema's seed rows and no acting user.
    ///
    /// # Panics
    ///
    /// Panics when pg2sqlite refuses the schema, SQLite refuses a statement
    /// of its translation, or SQLite cannot prepare a write firing its
    /// triggers or a read of one of its views.
    #[must_use]
    pub fn new() -> Self {
        sqlitegis::sqlite::register_on_every_new_connection();
        let mut connection =
            SqliteConnection::establish(":memory:").expect("in-memory SQLite database");
        jsonb_functions().register(&mut connection).expect("jsonb functions");
        pg_lower_utils::register_impl(&mut connection, |text: Option<String>| {
            text.map(|text| text.to_lowercase())
        })
        .expect("pg_lower");
        pg_upper_utils::register_impl(&mut connection, |text: Option<String>| {
            text.map(|text| text.to_uppercase())
        })
        .expect("pg_upper");
        uuidv4_utils::register_nondeterministic_impl(&mut connection, || {
            rosetta_uuid::Uuid::new_v4().as_bytes().to_vec()
        })
        .expect("uuidv4");
        let user = Arc::new(Mutex::new(None));
        let answer = Arc::clone(&user);
        app_user_id_utils::register_nondeterministic_impl(&mut connection, move || {
            answer.lock().expect("acting user").clone()
        })
        .expect("app_user_id");
        app_installation_id_utils::register_nondeterministic_impl(
            &mut connection,
            || None::<String>,
        )
        .expect("app_installation_id");
        app_original_actor_id_utils::register_nondeterministic_impl(&mut connection, || {
            None::<String>
        })
        .expect("app_original_actor_id");
        app_link_ids_utils::register_nondeterministic_impl(&mut connection, || None::<String>)
            .expect("app_link_ids");
        app_verified_email_utils::register_nondeterministic_impl(
            &mut connection,
            || None::<String>,
        )
        .expect("app_verified_email");
        // The schema's own seed rows are the server's writes, so the script
        // runs exempt.
        let exempt = Arc::new(AtomicBool::new(true));
        let flag = Arc::clone(&exempt);
        connetto_write_exempt_utils::register_nondeterministic_impl(&mut connection, move || {
            flag.load(Ordering::Relaxed)
        })
        .expect("connetto_write_exempt");
        server_derived_write_utils::register_nondeterministic_impl(&mut connection, |step: i32| {
            step
        })
        .expect("server_derived_write");
        let helpers: Vec<HelperFunction<&str>> =
            SCRIPT.functions.iter().map(ReplicaFunction::helper).collect();
        for helper in &helpers {
            helper
                .register(&mut connection)
                .unwrap_or_else(|error| panic!("SQLite refuses helper {}: {error}", helper.name));
        }
        for statement in &SCRIPT.statements {
            connection
                .batch_execute(statement)
                .unwrap_or_else(|error| panic!("SQLite refuses {statement}: {error}"));
        }
        PREPARED.call_once(|| {
            let unprepared =
                unprepared_programs(&mut connection, &helpers).expect("replica programs");
            assert!(
                unprepared.is_empty(),
                "SQLite cannot prepare {}",
                unprepared.iter().map(ToString::to_string).collect::<Vec<_>>().join("; ")
            );
        });
        exempt.store(false, Ordering::Relaxed);
        Self { connection, exempt, user }
    }

    /// Makes `user` the caller `app.user_id` names, or no one.
    pub fn act_as(&mut self, user: Option<rosetta_uuid::Uuid>) {
        *self.user.lock().expect("acting user") = user.map(|user| user.to_string());
    }

    /// The connection, for reads.
    pub fn connection(&mut self) -> &mut SqliteConnection {
        &mut self.connection
    }

    /// Runs `write` as the acting user in one transaction ending with the
    /// checks the schema defers to commit, as a client writes.
    ///
    /// # Errors
    ///
    /// Returns the error of `write` or of a deferred check, after rolling the
    /// transaction back.
    pub fn write<T>(
        &mut self,
        write: impl FnOnce(&mut SqliteConnection) -> QueryResult<T>,
    ) -> QueryResult<T> {
        self.connection.transaction(|connection| {
            let value = write(connection)?;
            for statement in &SCRIPT.before_commit {
                connection.batch_execute(statement)?;
            }
            Ok(value)
        })
    }

    /// Runs `delivery` as rows the server sends, which the replica's write
    /// guards let through, in one transaction.
    ///
    /// # Errors
    ///
    /// Returns the error of `delivery`, after rolling the transaction back.
    pub fn deliver<T>(
        &mut self,
        delivery: impl FnOnce(&mut SqliteConnection) -> QueryResult<T>,
    ) -> QueryResult<T> {
        self.exempt.store(true, Ordering::Relaxed);
        let outcome = self.write(delivery);
        self.exempt.store(false, Ordering::Relaxed);
        outcome
    }
}

impl Default for Replica {
    fn default() -> Self {
        Self::new()
    }
}
