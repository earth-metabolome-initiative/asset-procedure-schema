//! Builder executable to generate the Directus database code.
use sql_constraints::prelude::*;
use sql_traits::prelude::ParserDB;
use sql_traits::traits::DatabaseLike;
use std::path::Path;
use std::process::Command;
use synql::prelude::*;
use time_requirements::{prelude::TimeTracker, report::Report, task::Task};

/// Executable to generate the code for the Directus database.
pub fn main() {
    let mut tracker = TimeTracker::new("Directus Schema Generation");

    let mut tracking_test = TimeTracker::new("EMI Workspace Generation Test");

    let task = Task::new("Database Parsing");
    let db = ParserDB::try_from(Path::new("../")).expect("Failed to parse database schema");
    assert!(db.has_tables(), "Database should have tables");
    tracking_test.add_completed_task(task);

    // Validate the database schema with all available constraints
    let validation_task = Task::new("Schema Validation");
    let mut constrainer = DefaultConstrainer::<ParserDB>::default();
    sql_procedure_constraints::register_procedure_constraints(&mut constrainer);
    constrainer
        .validate_schema(&db)
        .expect("Database schema should pass all constraints");
    tracking_test.add_completed_task(validation_task);

    // Generate the code associated with the database
    let synql: SynQL<ParserDB> =
        SynQL::new_with_crate_base_path(&db, "../".as_ref(), "aps".as_ref())
            .name("aps")
            .sink_crate("aps")
            .generate_workspace_toml()
            .generate_rustfmt()
            .into();

    tracker.extend(synql.generate().expect("Unable to generate workspace"));

    // Formats the generated code
    let task = Task::new("Code Formatting");
    Command::new("cargo")
        .arg("fmt")
        .current_dir("../")
        .status()
        .expect("Failed to format generated code");
    tracker.add_completed_task(task);

    // Formats the generated TOML using `taplo fmt`
    let task = Task::new("TOML Formatting");
    Command::new("taplo")
        .arg("fmt")
        .current_dir("../")
        .status()
        .expect("Failed to format generated TOML");
    tracker.add_completed_task(task);

    // We print the report
    Report::new(tracker)
        .write(
            Path::new("TIME_REQUIREMENTS.md"),
            Path::new("TIME_REQUIREMENTS.png"),
        )
        .unwrap();
}
