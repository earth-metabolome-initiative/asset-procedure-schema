//! Builder executable to generate the Directus database code.
use std::{path::Path, process::Command};

use sql_constraints::prelude::*;
use sql_traits::{prelude::ParserDB, traits::DatabaseLike};
use synql::prelude::*;
use time_requirements::{prelude::TimeTracker, report::Report, task::Task};
mod visualize_dags;
mod visualize_workspace;

/// Executable to generate the code for the Directus database.
pub fn main() {
    let mut tracker = TimeTracker::new("APS Generation");

    let task = Task::new("Database Introspection");
    let db = ParserDB::try_from(Path::new("../")).expect("Failed to parse database schema");
    assert!(db.has_tables(), "Database should have tables");
    tracker.add_completed_task(task);

    // Validate the database schema with all available constraints
    let validation_task = Task::new("Schema Validation");
    let mut constrainer = DefaultConstrainer::<ParserDB>::default();
    sql_procedure_constraints::register_procedure_constraints(&mut constrainer);
    constrainer.validate_schema(&db).expect("Database schema should pass all constraints");
    tracker.add_completed_task(validation_task);

    // Generate the code associated with the database
    let synql: SynQL<ParserDB> =
        SynQL::new_with_crate_base_path(&db, "../".as_ref(), "aps".as_ref())
            .name("aps")
            .sink_crate("aps")
            .generate_workspace_toml()
            .generate_rustfmt()
            .member("builder")
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
    Report::new(tracker)
        .write(Path::new("TIME_REQUIREMENTS.md"), Path::new("TIME_REQUIREMENTS.png"))
        .unwrap();
}
