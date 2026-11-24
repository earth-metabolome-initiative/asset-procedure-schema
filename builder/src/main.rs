//! Builder executable that validates the APS schema and generates the `aps/`
//! crates.
use std::{path::Path, process::Command};

use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use sql_traits::{
    prelude::{IdentifierCase, ParserDB, TargetName},
    traits::{DatabaseLike, TableLike},
};
use sqlparser::ast::CreateTable;
use synql::prelude::*;
use time_requirements::{prelude::TimeTracker, task::Task};
mod gate;
mod shape;
mod typed_rows;
mod visualize_dags;
mod visualize_workspace;
use sql_procedure_rules::table_rules::{PROCEDURE_TEMPLATES_TABLE_NAME, PROCEDURES_TABLE_NAME};

/// The diesel repository.
const DIESEL_GIT: &str = "https://github.com/diesel-rs/diesel";
/// The diesel branch carrying SQLite table-valued functions and their
/// run-time declaration, diesel-rs/diesel#5395 and #5396.
const DIESEL_TABLE_FUNCTIONS: &str = "upstream/sqlite-table-valued-functions-dynamic";

/// Returns the table named `name`.
///
/// # Panics
///
/// Panics if the schema has no table of that name.
fn table<'db>(db: &'db ParserDB, name: &str) -> Result<&'db CreateTable, synql::Error> {
    Ok(db
        .table_by_target(TargetName::new(name, false), IdentifierCase::AsWritten)?
        .unwrap_or_else(|| panic!("Table '{name}' not found in database")))
}

/// Whether `candidate` is `procedures` or one of its typed subtypes.
fn is_procedure_like(candidate: &CreateTable, db: &ParserDB) -> Result<bool, synql::Error> {
    let procedures_table = table(db, PROCEDURES_TABLE_NAME)?;
    Ok(candidate == procedures_table || candidate.is_descendant_of(db, procedures_table)?)
}

/// Callback function to generate code for procedure-like tables.
///
/// # Panics
///
/// Panics if the procedures table or the expected procedure template table is
/// not found in the database.
fn generate_procedure_like(
    procedure_table: &CreateTable,
    db: &ParserDB,
    workspace: &Workspace,
) -> Result<Option<TokenStream>, synql::Error> {
    if !is_procedure_like(procedure_table, db)? {
        return Ok(None);
    }

    let expected_procedure_template_table = if procedure_table.table_name() == PROCEDURES_TABLE_NAME
    {
        PROCEDURE_TEMPLATES_TABLE_NAME.to_string()
    } else {
        format!("{}_templates", procedure_table.table_name().replace("procedures", "procedure"))
    };
    let procedure_template_table = table(db, &expected_procedure_template_table)?;
    let procedure_templates_table = table(db, PROCEDURE_TEMPLATES_TABLE_NAME)?;
    assert!(
        procedure_template_table == procedure_templates_table
            || procedure_template_table.is_descendant_of(db, procedure_templates_table)?,
        "Procedure template table '{expected_procedure_template_table}' is not a descendant of '{PROCEDURE_TEMPLATES_TABLE_NAME}'"
    );

    let procedure_table_snake_ident = procedure_table.table_snake_ident();
    let procedure_template_crate_ident = procedure_template_table.crate_ident(workspace);
    let procedure_template_snake_ident = procedure_template_table.table_snake_ident();

    Ok(Some(quote! {
        impl procedure_like::ProcedureTableLike for #procedure_table_snake_ident::table {
            type ProcedureTemplateTable = #procedure_template_crate_ident::#procedure_template_snake_ident::table;
        }
    }))
}

/// Callback emitting the shape const of a typed procedure, a typed template or
/// an asset model table.
fn generate_shape(
    table: &CreateTable,
    db: &ParserDB,
    _workspace: &Workspace,
) -> Result<Option<TokenStream>, synql::Error> {
    Ok(shape::shape_const(table, db)?)
}

/// Callback adding `procedure-like` to the crates whose generated code names
/// it, the procedure-like tables and the tables with a shape.
fn generate_toml_procedure_like(
    table: &CreateTable,
    db: &ParserDB,
) -> Result<Option<TomlDependency>, synql::Error> {
    if !is_procedure_like(table, db)? && shape::shape_const(table, db)?.is_none() {
        return Ok(None);
    }
    Ok(Some(TomlDependency::new("procedure-like").path("../../crates/procedure-like")?))
}

/// Sink callback emitting the lookups of the shapes by `table_name_id` over
/// the tables the sink crate re-exports.
fn generate_shape_lookups(
    tables: &[&CreateTable],
    db: &ParserDB,
    workspace: &Workspace,
) -> Result<Option<TokenStream>, synql::Error> {
    let tables = tables
        .iter()
        .map(|table| (*table, table.crate_ident(workspace).into_token_stream()))
        .collect::<Vec<_>>();
    Ok(Some(shape::shape_lookups(&tables, db)?))
}

/// Sink callback adding `procedure-like`, whose shape types the lookups
/// return.
fn generate_sink_toml_procedure_like(
    _db: &ParserDB,
) -> Result<Option<TomlDependency>, synql::Error> {
    Ok(Some(TomlDependency::new("procedure-like").path("../../crates/procedure-like")?))
}

/// Sink callback emitting the `TypedRows` trait over the tables the sink
/// crate re-exports.
fn generate_typed_rows(
    tables: &[&CreateTable],
    db: &ParserDB,
    _workspace: &Workspace,
) -> Result<Option<TokenStream>, synql::Error> {
    Ok(Some(typed_rows::typed_rows(tables, db)?))
}

/// Sink callbacks adding the workspace crates the `TypedRows` implementation
/// names.
fn generate_sink_toml_typed_rows(
    name: &'static str,
) -> impl Fn(&ParserDB) -> Result<Option<TomlDependency>, synql::Error> {
    move |_db| Ok(Some(TomlDependency::new(name).workspace(true)?))
}

/// Validates the schema and generates the `aps/` workspace crates.
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut tracker = TimeTracker::new("APS Generation");

    let task = Task::new("Database Introspection");
    let schema = gate::Schema::load(Path::new(".."));
    assert!(schema.database.has_tables(), "Database should have tables");
    tracker.add_completed_task(task);

    let validation_task = Task::new("Schema Validation");
    let findings = schema.findings();
    for finding in &findings {
        eprintln!("{finding}\n");
    }
    let divergences = schema.divergences();
    for note in &divergences {
        eprintln!("rls2fga [{}] {}: {note}\n", note.severity(), note.subject());
    }
    if !findings.is_empty() || !divergences.is_empty() {
        return Err(format!(
            "{} SQLippy findings, {} rls2fga divergences",
            findings.len(),
            divergences.len()
        )
        .into());
    }
    tracker.add_completed_task(validation_task);
    let db = schema.database;

    // Generate the code associated with the database
    let synql: SynQL<ParserDB> =
        SynQL::new_with_crate_base_path(&db, "../".as_ref(), "aps".as_ref())
            .name("aps")
            .external_crate(
                ExternalCrate::new("sqlparser")
                    .expect("sqlparser is a valid crate name")
                    .version("0.64")?
                    .git("https://github.com/apache/datafusion-sqlparser-rs", "main")?
                    .into(),
            )
            .external_crate(
                ExternalCrate::new("sql-values")
                    .expect("sql-values is a valid crate name")
                    .version("0.6")?
                    .git("https://github.com/connetto-rs/sql-values", "main")?
                    .into(),
            )
            // pg2sqlite, which `aps-test-utils` builds, registers its runtime
            // functions through diesel's table-valued function branch.
            .crates_io_patch(
                TomlDependency::new("diesel").git(DIESEL_GIT, Some(DIESEL_TABLE_FUNCTIONS))?,
            )
            .crates_io_patch(
                TomlDependency::new("sqlstate").git(DIESEL_GIT, Some(DIESEL_TABLE_FUNCTIONS))?,
            )
            .sink_crate("aps")
            .dag_sink_crate_prefix("aps-dag-")
            .group_tables(1.0)
            .generate_workspace_toml()
            .generate_rustfmt()
            .callback(generate_procedure_like)
            .callback(generate_shape)
            .toml_callback(generate_toml_procedure_like)
            .sink_callback(generate_shape_lookups)
            .sink_callback(generate_typed_rows)
            .sink_toml_callback(generate_sink_toml_procedure_like)
            .sink_toml_callback(generate_sink_toml_typed_rows("diesel"))
            .sink_toml_callback(generate_sink_toml_typed_rows("diesel-builders"))
            .sink_toml_callback(generate_sink_toml_typed_rows("rosetta-uuid"))
            .sink_toml_callback(generate_sink_toml_typed_rows("sqlitegis"))
            .members([
                TomlDependency::new("builder").path("builder")?,
                TomlDependency::new("sql-procedure-rules").path("crates/sql-procedure-rules")?,
                TomlDependency::new("sql-migration-lint").path("crates/sql-migration-lint")?,
                TomlDependency::new("aps-traits").path("crates/aps-traits")?,
                TomlDependency::new("aps-test-utils").path("crates/aps-test-utils")?,
                TomlDependency::new("procedure-template-visitor")
                    .path("crates/procedure-template-visitor")?,
                TomlDependency::new("procedure-template-visualization")
                    .path("crates/procedure-template-visualization")?,
                TomlDependency::new("procedure-like").path("crates/procedure-like")?,
            ])
            .into();

    tracker.extend(synql.generate().expect("Unable to generate workspace"));

    // Formats the generated TOML using `taplo fmt`
    let task = Task::new("TOML Formatting");
    Command::new("taplo")
        .arg("fmt")
        .current_dir("../")
        .status()
        .expect("Failed to format generated TOML");
    tracker.add_completed_task(task);

    // Formats the generated code
    let task = Task::new("Code Formatting (1)");
    Command::new("cargo")
        .arg("fmt")
        .current_dir("../")
        .status()
        .expect("Failed to format generated code");
    tracker.add_completed_task(task);

    // Fix clippy warnings in the generated code where possible
    let task = Task::new("Clippy Fixes");
    Command::new("cargo")
        .arg("clippy")
        .arg("--fix")
        .arg("--allow-dirty")
        .arg("--allow-staged")
        .current_dir("../")
        .status()
        .expect("Failed to fix clippy warnings in generated code");
    tracker.add_completed_task(task);

    // Formats the generated code
    let task = Task::new("Code Formatting (2)");
    Command::new("cargo")
        .arg("fmt")
        .current_dir("../")
        .status()
        .expect("Failed to format generated code");
    tracker.add_completed_task(task);

    // We visualize the workspace dependencies as an ERD
    let task = Task::new("Workspace Dependency Visualization");
    let erd = visualize_workspace::workspace_dependencies(&db)
        .expect("Failed to visualize workspace dependencies");
    // We write the ERD to a file
    std::fs::write("workspace_dependencies.mmd", erd.to_string())
        .expect("Failed to write workspace dependencies ERD");
    tracker.add_completed_task(task);

    // We visualize the DAG structures present in the database
    let task = Task::new("DAG Structure Visualization");
    visualize_dags::visualize_dags(&db).expect("Failed to visualize DAG structures");
    tracker.add_completed_task(task);

    // We print the report
    tracker.write(Path::new("TIME_REQUIREMENTS.md")).unwrap();

    Ok(())
}
