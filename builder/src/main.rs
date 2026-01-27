//! Builder executable to generate the Directus database code.
use std::{path::Path, process::Command};

use proc_macro2::TokenStream;
use quote::quote;
use sql_rules::prelude::*;
use sql_traits::{prelude::ParserDB, traits::DatabaseLike};
use sqlparser::ast::CreateTable;
use synql::prelude::*;
use time_requirements::{prelude::TimeTracker, task::Task};
mod visualize_dags;
mod visualize_workspace;
use sql_procedure_rules::table_rules::{PROCEDURE_TEMPLATES_TABLE_NAME, PROCEDURES_TABLE_NAME};

/// Callback function to generate code for procedure-like tables.
///
/// # Panic
///
/// Panics if the procedures table or the expected procedure template table is
/// not found in the database.
fn generate_procedure_like(
    table: &CreateTable,
    db: &ParserDB,
    workspace: &Workspace,
) -> Result<Option<TokenStream>, synql::Error> {
    let Some(procedures_table) = db.table(None, PROCEDURES_TABLE_NAME) else {
        panic!("Procedures table not found in database");
    };

    if !table.is_descendant_of(db, procedures_table) && table != procedures_table {
        return Ok(None);
    }

    let expected_procedure_template_table = if table == procedures_table {
        PROCEDURE_TEMPLATES_TABLE_NAME.to_string()
    } else {
        format!("{}_templates", table.table_name().replace("procedures", "procedure"))
    };

    let Some(procedure_template_table) = db.table(None, &expected_procedure_template_table) else {
        panic!(
            "Procedure template table '{}' not found in database",
            expected_procedure_template_table
        )
    };

    let Some(procedure_templates_table) = db.table(None, PROCEDURE_TEMPLATES_TABLE_NAME) else {
        panic!("Procedure templates table not found in database");
    };

    assert!(
        procedure_template_table.is_descendant_of(db, procedure_templates_table)
            || procedure_template_table == procedure_templates_table,
        "Procedure template table '{}' is not a descendant of '{}'",
        expected_procedure_template_table,
        PROCEDURE_TEMPLATES_TABLE_NAME
    );

    let procedure_table_snake_ident = table.table_snake_ident();
    let procedure_template_crate_ident = procedure_template_table.crate_ident(workspace);
    let procedure_template_snake_ident = procedure_template_table.table_snake_ident();

    Ok(Some(quote! {
        impl procedure_like::ProcedureTableLike for #procedure_table_snake_ident::table {
            type ProcedureTemplateTable = #procedure_template_crate_ident::#procedure_template_snake_ident::table;
        }
    }))
}

/// Callback function to generate the TOML dependency for procedure-like tables.
fn generate_toml_procedure_like(
    table: &CreateTable,
    db: &ParserDB,
) -> Result<Option<TomlDependency>, synql::Error> {
    let Some(procedures_table) = db.table(None, PROCEDURES_TABLE_NAME) else {
        panic!("Procedures table not found in database");
    };

    if !table.is_descendant_of(db, procedures_table) && table != procedures_table {
        return Ok(None);
    }

    Ok(Some(TomlDependency::new("procedure-like").path("../../crates/procedure-like")?))
}

/// Executable to generate the code for the Directus database.
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut tracker = TimeTracker::new("APS Generation");

    let task = Task::new("Database Introspection");
    let db = ParserDB::try_from(Path::new("../")).expect("Failed to parse database schema");
    assert!(db.has_tables(), "Database should have tables");
    tracker.add_completed_task(task);

    // Validate the database schema with all available constraints
    let validation_task = Task::new("Schema Validation");
    let mut constrainer = DefaultConstrainer::<ParserDB>::default();
    sql_procedure_rules::register_procedure_constraints(&mut constrainer);
    constrainer.validate_schema(&db).expect("Database schema should pass all constraints");
    tracker.add_completed_task(validation_task);

    // Generate the code associated with the database
    let synql: SynQL<ParserDB> =
        SynQL::new_with_crate_base_path(&db, "../".as_ref(), "aps".as_ref())
            .name("aps")
            .sink_crate("aps")
            .dag_sink_crate_prefix("aps-dag-")
            .generate_workspace_toml()
            .generate_rustfmt()
            .callback(generate_procedure_like)
            .toml_callback(generate_toml_procedure_like)
            .members([
                TomlDependency::new("builder").path("builder")?,
                TomlDependency::new("procedure-traits").path("crates/procedure-traits")?,
                TomlDependency::new("sql-procedure-rules").path("crates/sql-procedure-rules")?,
                TomlDependency::new("asset-traits").path("crates/asset-traits")?,
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
