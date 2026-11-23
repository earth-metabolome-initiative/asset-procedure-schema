//! Command-line linter for SQL migration naming and registration conventions.

use std::{
    collections::BTreeSet,
    env, fs,
    path::{Path, PathBuf},
    process::ExitCode,
};

/// Relative path to the device migrations root.
const DEVICES_RELATIVE_PATH: &str = "sql/shared-schema/003-shared-assets/004-devices";
/// Relative path to the shared-assets migration root.
const SHARED_ASSETS_RELATIVE_PATH: &str = "sql/shared-schema/003-shared-assets";

/// Supported command for device migration linting.
const DEVICES_COMMAND: &str = "devices";
/// Supported command for shared-assets migration documentation linting.
const SHARED_ASSETS_DOCS_COMMAND: &str = "shared-assets-docs";
/// Optional flag to require complete device table families for each stem.
const REQUIRE_FULL_DEVICE_CHAIN_FLAG: &str = "--require-full-device-chain";
/// Optional path override for command target.
const PATH_FLAG: &str = "--path";

/// Represents one migration folder with its `up.sql` file.
#[derive(Debug, Clone)]
struct Migration {
    /// The migration folder name (for example `001-weighing-devices`).
    dir_name: String,
    /// Absolute path to the migration `up.sql` file.
    up_sql_path: PathBuf,
}

/// Runtime options for device migration linting.
#[derive(Debug, Clone, Copy, Default)]
struct DeviceLintOptions {
    /// Require full device table family for each declared stem:
    /// `<stem>_models`, `commercial_<stem>_models`,
    /// `commercial_<stem>_lots`, `<stem>s`.
    require_full_device_chain: bool,
}

/// Runs the SQL migration linter binary.
fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}

/// Executes argument parsing and dispatches the selected lint command.
fn run() -> Result<(), String> {
    let mut args = env::args().skip(1);

    let Some(command) = args.next() else {
        return Err(usage());
    };

    if command == "-h" || command == "--help" || command == "help" {
        return Err(usage());
    }

    let mut root =
        env::current_dir().map_err(|error| format!("Failed to read current directory: {error}"))?;
    let mut target_path: Option<PathBuf> = None;

    let remaining_args = args.collect::<Vec<_>>();
    let mut options = DeviceLintOptions::default();
    let mut index = 0_usize;
    while index < remaining_args.len() {
        let argument = &remaining_args[index];
        if argument == "--root" {
            let Some(value) = remaining_args.get(index + 1) else {
                return Err("Missing value for --root".to_string());
            };
            root = PathBuf::from(value);
            index += 2;
            continue;
        }
        if argument == PATH_FLAG {
            let Some(value) = remaining_args.get(index + 1) else {
                return Err(format!("Missing value for {PATH_FLAG}"));
            };
            target_path = Some(PathBuf::from(value));
            index += 2;
            continue;
        }
        if argument == REQUIRE_FULL_DEVICE_CHAIN_FLAG {
            options.require_full_device_chain = true;
            index += 1;
            continue;
        }

        return Err(format!("Unknown argument '{argument}'\n\n{}", usage()));
    }

    match command.as_str() {
        DEVICES_COMMAND => lint_devices(&root, target_path.as_deref(), options),
        SHARED_ASSETS_DOCS_COMMAND => lint_shared_assets_docs(&root, target_path.as_deref()),
        _ => Err(format!("Unknown command '{command}'\n\n{}", usage())),
    }
}

/// Returns command-line usage instructions.
fn usage() -> String {
    format!(
        "Usage:\n  cargo run -p sql-migration-lint -- {DEVICES_COMMAND} [--root <path>] [{PATH_FLAG} <path>] [{REQUIRE_FULL_DEVICE_CHAIN_FLAG}]\n  cargo run -p sql-migration-lint -- {SHARED_ASSETS_DOCS_COMMAND} [--root <path>] [{PATH_FLAG} <path>]\n"
    )
}

/// Lints the device migration folder naming and table registration conventions.
fn lint_devices(
    root: &Path,
    target_path: Option<&Path>,
    options: DeviceLintOptions,
) -> Result<(), String> {
    let devices_dir = resolve_command_target(root, target_path, DEVICES_RELATIVE_PATH);
    let migrations = load_device_migrations_recursive(&devices_dir)?;
    let check_incremental_prefixes = target_path.is_none();

    if migrations.is_empty() {
        return Err(format!(
            "No device migrations with up.sql found under {}",
            devices_dir.display()
        ));
    }

    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    if check_incremental_prefixes {
        validate_incremental_prefixes(&migrations, &mut errors);
    }

    for migration in &migrations {
        validate_migration(migration, &mut errors, &mut warnings, options)?;
    }

    if !warnings.is_empty() {
        println!("Devices migration lint warnings:");
        for warning in &warnings {
            println!("  - {warning}");
        }
    }

    if errors.is_empty() {
        println!("Devices migration lint passed.");
        return Ok(());
    }

    let mut message = String::from("Devices migration lint failed:\n");
    for error in errors {
        message.push_str("  - ");
        message.push_str(&error);
        message.push('\n');
    }
    Err(message)
}

/// Lints shared-assets migrations for required documentation sections and
/// in-file statement/column comments.
fn lint_shared_assets_docs(root: &Path, target_path: Option<&Path>) -> Result<(), String> {
    let shared_assets_dir = resolve_command_target(root, target_path, SHARED_ASSETS_RELATIVE_PATH);
    let migrations = load_up_sql_files_from_target(&shared_assets_dir)?;

    if migrations.is_empty() {
        return Err(format!(
            "No shared-assets migrations with up.sql found under {}",
            shared_assets_dir.display()
        ));
    }

    let mut errors = Vec::new();

    for migration_path in &migrations {
        let sql = fs::read_to_string(migration_path)
            .map_err(|error| format!("Failed to read {}: {error}", migration_path.display()))?;
        let migration_label =
            migration_path.strip_prefix(root).unwrap_or(migration_path).display().to_string();
        validate_shared_assets_documentation(&migration_label, &sql, &mut errors);
    }

    if errors.is_empty() {
        println!("Shared-assets migration documentation lint passed.");
        return Ok(());
    }

    let mut message = String::from("Shared-assets migration documentation lint failed:\n");
    for error in errors {
        message.push_str("  - ");
        message.push_str(&error);
        message.push('\n');
    }
    Err(message)
}

/// Resolves the target path for a command from default + optional override.
fn resolve_command_target(
    root: &Path,
    target_path: Option<&Path>,
    default_relative: &str,
) -> PathBuf {
    match target_path {
        Some(path) if path.is_absolute() => path.to_path_buf(),
        Some(path) => root.join(path),
        None => root.join(default_relative),
    }
}

/// Loads devices migrations recursively from a directory tree, or one `up.sql`.
fn load_device_migrations_recursive(target: &Path) -> Result<Vec<Migration>, String> {
    if target.is_file() {
        if target.file_name().and_then(|name| name.to_str()) != Some("up.sql") {
            return Err(format!(
                "Expected an up.sql file for devices lint target, got {}",
                target.display()
            ));
        }
        let Some(parent) = target.parent() else {
            return Err(format!("Failed to locate parent directory for {}", target.display()));
        };
        let migration = migration_from_up_sql(parent, target)?;
        return Ok(vec![migration]);
    }

    if !target.is_dir() {
        return Err(format!(
            "Devices lint target does not exist or is not accessible: {}",
            target.display()
        ));
    }

    let up_sql_files = load_up_sql_files_recursive(target)?;
    let mut migrations = Vec::new();

    for up_sql_path in up_sql_files {
        let Some(parent) = up_sql_path.parent() else {
            return Err(format!("Failed to locate parent directory for {}", up_sql_path.display()));
        };
        migrations.push(migration_from_up_sql(parent, &up_sql_path)?);
    }

    migrations.sort_by(|left, right| left.up_sql_path.cmp(&right.up_sql_path));
    Ok(migrations)
}

/// Creates one migration model from directory + up.sql path.
fn migration_from_up_sql(dir: &Path, up_sql_path: &Path) -> Result<Migration, String> {
    let Some(dir_name) = dir.file_name().and_then(|name| name.to_str()) else {
        return Err(format!("Failed to derive migration folder name from {}", dir.display()));
    };

    Ok(Migration { dir_name: dir_name.to_string(), up_sql_path: up_sql_path.to_path_buf() })
}

/// Recursively collects all `up.sql` files under `root`.
fn load_up_sql_files_recursive(root: &Path) -> Result<Vec<PathBuf>, String> {
    let mut files = Vec::new();
    collect_up_sql_files_recursive(root, &mut files)?;
    files.sort();
    Ok(files)
}

/// Loads one or many `up.sql` files from a target directory or file.
fn load_up_sql_files_from_target(target: &Path) -> Result<Vec<PathBuf>, String> {
    if target.is_file() {
        if target.file_name().and_then(|name| name.to_str()) != Some("up.sql") {
            return Err(format!(
                "Expected an up.sql file for docs lint target, got {}",
                target.display()
            ));
        }
        return Ok(vec![target.to_path_buf()]);
    }

    if target.is_dir() {
        return load_up_sql_files_recursive(target);
    }

    Err(format!("Docs lint target does not exist or is not accessible: {}", target.display()))
}

/// Recursively traverses directories and collects `up.sql` files.
fn collect_up_sql_files_recursive(dir: &Path, files: &mut Vec<PathBuf>) -> Result<(), String> {
    let entries =
        fs::read_dir(dir).map_err(|error| format!("Failed to read {}: {error}", dir.display()))?;

    for entry_result in entries {
        let entry = entry_result
            .map_err(|error| format!("Failed to iterate entries in {}: {error}", dir.display()))?;
        let path = entry.path();
        let file_type = entry.file_type().map_err(|error| {
            format!("Failed to inspect entry '{}' in {}: {error}", path.display(), dir.display())
        })?;

        if file_type.is_dir() {
            collect_up_sql_files_recursive(&path, files)?;
            continue;
        }

        if file_type.is_file() && path.file_name().and_then(|name| name.to_str()) == Some("up.sql")
        {
            files.push(path);
        }
    }

    Ok(())
}

/// Validates that migration prefixes are contiguous (`001..N`) without gaps.
fn validate_incremental_prefixes(migrations: &[Migration], errors: &mut Vec<String>) {
    for (index, migration) in migrations.iter().enumerate() {
        let expected = format!("{:03}", index + 1);
        let actual = migration.dir_name.split_once('-').map(|(prefix, _)| prefix).unwrap_or("");

        if actual != expected {
            errors.push(format!(
                "[{}] numeric prefix '{}' should be '{}' (incremental without gaps)",
                migration.dir_name, actual, expected
            ));
        }
    }
}

/// Applies all lint checks for one migration.
fn validate_migration(
    migration: &Migration,
    errors: &mut Vec<String>,
    warnings: &mut Vec<String>,
    options: DeviceLintOptions,
) -> Result<(), String> {
    validate_folder_format(migration, errors);

    let sql = fs::read_to_string(&migration.up_sql_path)
        .map_err(|error| format!("Failed to read {}: {error}", migration.up_sql_path.display()))?;

    let created_tables = extract_created_tables(&sql);
    let registered_tables = extract_registered_tables(&sql);

    if created_tables.is_empty() {
        return Ok(());
    }

    let declared_stems = stems_from_migration_dir(&migration.dir_name);
    if declared_stems.is_empty() {
        errors.push(format!("[{}] could not derive any stem from folder name", migration.dir_name));
        return Ok(());
    }

    if options.require_full_device_chain {
        validate_full_device_chain(migration, &declared_stems, &created_tables, errors);
    }

    if declared_stems.len() > 1 {
        warnings.push(format!(
            "[{}] migration folder declares multiple entities ({}) ; suggestion: keep one entity per migration and split folders",
            migration.dir_name,
            declared_stems.join(", ")
        ));
    }

    let mut created_stems = BTreeSet::new();
    let mut derived_table_stems = Vec::new();

    for table in &created_tables {
        let Some(table_stem) = derive_table_stem(table) else {
            errors.push(format!(
                "[{}] {}",
                migration.dir_name,
                unsupported_table_name_message(table, &declared_stems)
            ));
            continue;
        };

        created_stems.insert(table_stem.clone());
        derived_table_stems.push((table, table_stem));

        if !registered_tables.contains(table) {
            errors.push(format!(
                "[{}] missing in-file table_names registration for '{}'",
                migration.dir_name, table
            ));
        }
    }

    if created_stems.len() > 1 {
        let created_stems_text = created_stems.iter().cloned().collect::<Vec<_>>().join(", ");
        warnings.push(format!(
            "[{}] migration creates multiple entities ({created_stems_text}); suggestion: split into one migration per entity",
            migration.dir_name
        ));
    }

    for (table, table_stem) in derived_table_stems {
        if !declared_stems.iter().any(|stem| stem == &table_stem) {
            if created_stems.len() > 1 {
                warnings.push(format!(
                    "[{}] table '{}' belongs to entity '{}' but folder declares '{}'; suggestion: move this table to its own migration folder",
                    migration.dir_name,
                    table,
                    table_stem,
                    declared_stems.join("+")
                ));
            } else {
                let folder_stem = &declared_stems[0];
                let expected_models = format!("{folder_stem}_models");
                let expected_physical = pluralize_identifier(folder_stem);
                errors.push(format!(
                    "[{}] table '{}' uses stem '{}' but folder stem is '{}'; expected one of '{}', 'commercial_{}_models', 'commercial_{}_lots', '{}'",
                    migration.dir_name,
                    table,
                    table_stem,
                    folder_stem,
                    expected_models,
                    folder_stem,
                    folder_stem,
                    expected_physical
                ));
            }
        }
    }

    Ok(())
}

/// Validates that each declared stem has a complete device table family.
fn validate_full_device_chain(
    migration: &Migration,
    declared_stems: &[String],
    created_tables: &[String],
    errors: &mut Vec<String>,
) {
    let created = created_tables.iter().cloned().collect::<BTreeSet<_>>();

    for stem in declared_stems {
        let expected_tables = [
            format!("{stem}_models"),
            format!("commercial_{stem}_models"),
            format!("commercial_{stem}_lots"),
            pluralize_identifier(stem),
        ];

        let missing = expected_tables
            .iter()
            .filter(|table| !created.contains(*table))
            .cloned()
            .collect::<Vec<_>>();

        if !missing.is_empty() {
            errors.push(format!(
                "[{}] stem '{}' is missing required tables: {}",
                migration.dir_name,
                stem,
                missing.join(", ")
            ));
        }
    }
}

/// Validates migration folder shape `NNN-kebab-case`.
fn validate_folder_format(migration: &Migration, errors: &mut Vec<String>) {
    let Some((prefix, slug)) = migration.dir_name.split_once('-') else {
        errors.push(format!(
            "[{}] invalid folder format; expected NNN-kebab-case",
            migration.dir_name
        ));
        return;
    };

    let prefix_valid =
        prefix.len() == 3 && prefix.chars().all(|character| character.is_ascii_digit());
    let slug_valid = !slug.is_empty()
        && slug.chars().all(|character| {
            character.is_ascii_lowercase()
                || character.is_ascii_digit()
                || character == '-'
                || character == '+'
        })
        && !slug.starts_with('+')
        && !slug.ends_with('+')
        && !slug.contains("++");

    if !prefix_valid || !slug_valid {
        errors.push(format!(
            "[{}] invalid folder format; expected NNN-kebab-case",
            migration.dir_name
        ));
    }
}

/// Extracts table names from `CREATE TABLE ...` statements.
fn extract_created_tables(sql: &str) -> Vec<String> {
    let mut tables = Vec::new();

    for line in sql.lines() {
        let trimmed = line.trim_start();
        let Some(rest) = trimmed.strip_prefix("CREATE TABLE ") else {
            continue;
        };

        let Some(first_token) = rest.split_whitespace().next() else {
            continue;
        };

        let table_name =
            first_token.trim_matches('"').trim_end_matches('(').trim_end_matches(';').to_string();

        if !table_name.is_empty() {
            tables.push(table_name);
        }
    }

    tables
}

/// Extracts table ids registered in `table_names` inserts.
fn extract_registered_tables(sql: &str) -> BTreeSet<String> {
    let mut tables = BTreeSet::new();

    for line in sql.lines() {
        let trimmed = line.trim_start();
        if !trimmed.starts_with("INSERT INTO table_names") {
            continue;
        }

        let Some(values_start) = trimmed.find("VALUES ('") else {
            continue;
        };

        let value_tail = &trimmed[(values_start + "VALUES ('".len())..];
        let Some(value_end) = value_tail.find("')") else {
            continue;
        };

        let table_name = &value_tail[..value_end];
        if !table_name.is_empty() {
            tables.insert(table_name.to_string());
        }
    }

    tables
}

/// Validates documentation completeness for one shared-assets migration file.
fn validate_shared_assets_documentation(
    migration_label: &str,
    sql: &str,
    errors: &mut Vec<String>,
) {
    validate_required_doc_sections(migration_label, sql, errors);
    validate_statement_comments(migration_label, sql, errors);
    validate_column_comments(migration_label, sql, errors);
}

/// Validates required top-level documentation sections and ASCII placement art.
fn validate_required_doc_sections(migration_label: &str, sql: &str, errors: &mut Vec<String>) {
    let required_markers = [
        "Migration:",
        "Purpose:",
        "APS placement",
        "Security context",
        "SQL/RLS semantics",
        "Zanzibar semantics",
    ];

    for marker in required_markers {
        if !sql.contains(marker) {
            errors.push(format!(
                "[{}] missing documentation section marker '{}'",
                migration_label, marker
            ));
        }
    }

    if !sql.lines().any(|line| line.contains("+--")) {
        errors.push(format!(
            "[{}] missing APS ASCII placement diagram (expected at least one '+--' line)",
            migration_label
        ));
    }
}

/// Validates that each top-level SQL statement has a preceding comment.
fn validate_statement_comments(migration_label: &str, sql: &str, errors: &mut Vec<String>) {
    let lines = sql.lines().collect::<Vec<_>>();

    for (index, line) in lines.iter().enumerate() {
        let trimmed = line.trim_start();
        if !is_supported_statement_start(trimmed) {
            continue;
        }

        if !has_preceding_comment(&lines, index) {
            errors.push(format!(
                "[{}] SQL statement '{}' is missing a preceding '--' comment",
                migration_label, trimmed
            ));
        }
    }
}

/// Validates that each table column line has a preceding comment.
fn validate_column_comments(migration_label: &str, sql: &str, errors: &mut Vec<String>) {
    let lines = sql.lines().collect::<Vec<_>>();
    let mut current_table_name = String::new();
    let mut in_create_table = false;

    for (index, line) in lines.iter().enumerate() {
        let trimmed = line.trim_start();

        if let Some(table_name) = parse_create_table_name(trimmed) {
            in_create_table = true;
            current_table_name = table_name;
            continue;
        }

        if !in_create_table {
            continue;
        }

        if is_create_table_terminator(trimmed) {
            in_create_table = false;
            current_table_name.clear();
            continue;
        }

        if trimmed.is_empty() || trimmed.starts_with("--") {
            continue;
        }

        if is_column_definition(trimmed) && !has_preceding_comment(&lines, index) {
            errors.push(format!(
                "[{}] table '{}' column line '{}' is missing a preceding '--' comment",
                migration_label, current_table_name, trimmed
            ));
        }
    }
}

/// Returns true when the line starts one of the supported statement forms.
fn is_supported_statement_start(trimmed_line: &str) -> bool {
    [
        "CREATE TABLE ",
        "INSERT INTO ",
        "ALTER TABLE ",
        "CREATE UNIQUE INDEX ",
        "CREATE INDEX ",
        "CREATE POLICY ",
        "CREATE OR REPLACE ",
        "DROP TABLE ",
        "UPDATE ",
        "DELETE FROM ",
    ]
    .iter()
    .any(|prefix| trimmed_line.starts_with(prefix))
}

/// Returns true when a table definition line terminates `CREATE TABLE (...)`.
fn is_create_table_terminator(trimmed_line: &str) -> bool {
    let trimmed_end = trimmed_line.trim_end();
    trimmed_end == ");" || trimmed_end == ")"
}

/// Parses table name from a `CREATE TABLE ...` line.
fn parse_create_table_name(trimmed_line: &str) -> Option<String> {
    let rest = trimmed_line.strip_prefix("CREATE TABLE ")?;
    let first_token = rest.split_whitespace().next()?;
    let table_name =
        first_token.trim_matches('"').trim_end_matches('(').trim_end_matches(';').to_string();
    (!table_name.is_empty()).then_some(table_name)
}

/// Returns true if a line appears to define a table column.
fn is_column_definition(trimmed_line: &str) -> bool {
    if trimmed_line.is_empty() || trimmed_line.starts_with("--") {
        return false;
    }

    let uppercase = trimmed_line.to_ascii_uppercase();
    for keyword in [
        "CONSTRAINT ",
        "PRIMARY KEY",
        "FOREIGN KEY",
        "UNIQUE ",
        "CHECK ",
        "REFERENCES ",
        "ON DELETE",
        "ON UPDATE",
    ] {
        if uppercase.starts_with(keyword) {
            return false;
        }
    }

    if is_create_table_terminator(trimmed_line) {
        return false;
    }

    let mut tokens = trimmed_line.split_whitespace();
    let Some(first_token) = tokens.next() else {
        return false;
    };
    let Some(second_token) = tokens.next() else {
        return false;
    };

    let cleaned = first_token.trim_matches('"').trim_end_matches(',');
    if cleaned.is_empty() {
        return false;
    }

    let Some(first_character) = cleaned.chars().next() else {
        return false;
    };
    if !(first_character.is_ascii_lowercase() || first_character == '_') {
        return false;
    }

    let normalized_type = second_token.trim_matches(',').trim_matches('"').to_ascii_uppercase();
    let looks_like_data_type = matches!(
        normalized_type.as_str(),
        "UUID"
            | "TEXT"
            | "SMALLINT"
            | "INTEGER"
            | "BIGINT"
            | "REAL"
            | "DOUBLE"
            | "NUMERIC"
            | "BOOLEAN"
            | "DATE"
            | "TIMESTAMP"
            | "TIMESTAMPTZ"
            | "JSONB"
            | "BYTEA"
            | "SERIAL"
            | "BIGSERIAL"
    ) || normalized_type.starts_with("VARCHAR(")
        || normalized_type.starts_with("CHAR(")
        || normalized_type.starts_with("NUMERIC(");

    if !looks_like_data_type {
        return false;
    }

    cleaned.chars().all(|character| {
        character.is_ascii_lowercase() || character.is_ascii_digit() || character == '_'
    })
}

/// Returns true when the nearest previous non-empty line is a SQL comment.
fn has_preceding_comment(lines: &[&str], index: usize) -> bool {
    for previous in lines[..index].iter().rev() {
        let trimmed = previous.trim_start();
        if trimmed.is_empty() {
            continue;
        }

        return trimmed.starts_with("--");
    }

    false
}

/// Derives allowed stems from migration directory name.
///
/// Expected format: `NNN-<stem-slug>[+<stem-slug>...]`, where each stem slug is
/// kebab-case. Each stem slug is normalized to snake_case and singularized.
fn stems_from_migration_dir(migration_dir: &str) -> Vec<String> {
    let Some((_, slug)) = migration_dir.split_once('-') else {
        return Vec::new();
    };

    slug.split('+')
        .filter(|segment| !segment.is_empty())
        .map(|segment| singularize_identifier(&segment.replace('-', "_")))
        .collect()
}

/// Derives the stem from a table name according to supported table families.
fn derive_table_stem(table_name: &str) -> Option<String> {
    if let Some(rest) = table_name.strip_prefix("commercial_") {
        if let Some(stem) = rest.strip_suffix("_models") {
            return Some(stem.to_string());
        }
        if let Some(stem) = rest.strip_suffix("_lots") {
            return Some(stem.to_string());
        }
        return None;
    }

    if let Some(stem) = table_name.strip_suffix("_models") {
        return Some(stem.to_string());
    }

    // Physical entity tables must be pluralized stems.
    let stem = singularize_identifier(table_name);
    let expected_physical_table = pluralize_identifier(&stem);
    if table_name == expected_physical_table {
        return Some(stem);
    }

    None
}

/// Returns a human-readable error explaining why a table name is unsupported
/// and suggests a fix based on migration folder stem(s).
fn unsupported_table_name_message(table_name: &str, declared_stems: &[String]) -> String {
    if table_name.starts_with("commercial_") {
        let example_stem = declared_stems.first().map(String::as_str).unwrap_or("entity");
        return format!(
            "table '{}' uses 'commercial_' prefix but is not valid; expected suffix '_models' or '_lots' (for example 'commercial_{}_models')",
            table_name, example_stem
        );
    }

    let derived_stem = singularize_identifier(table_name);
    let suggested_plural = pluralize_identifier(&derived_stem);
    let folder_stems = declared_stems.join(", ");

    if table_name != suggested_plural {
        if declared_stems.iter().any(|stem| stem == &derived_stem) {
            return format!(
                "table '{}' should be plural for physical entity tables; use '{}' (folder stem '{}')",
                table_name, suggested_plural, derived_stem
            );
        }

        let preferred_example = declared_stems
            .first()
            .map(|stem| pluralize_identifier(stem))
            .unwrap_or_else(|| suggested_plural.clone());
        return format!(
            "table '{}' should be plural for physical entity tables and aligned with folder stem(s) [{}]; try '{}'",
            table_name, folder_stems, preferred_example
        );
    }

    format!(
        "table '{}' is not in a supported naming family; expected '<stem>_models', 'commercial_<stem>_models', 'commercial_<stem>_lots', or plural physical '<stem>s'",
        table_name
    )
}

/// Singularizes an identifier by singularizing its final underscore-separated
/// token.
fn singularize_identifier(identifier: &str) -> String {
    let mut tokens =
        identifier.split('_').map(std::string::ToString::to_string).collect::<Vec<_>>();

    if let Some(last) = tokens.last_mut() {
        *last = singularize_word(last);
    }

    tokens.join("_")
}

/// Pluralizes an identifier by pluralizing its final underscore-separated
/// token.
fn pluralize_identifier(identifier: &str) -> String {
    let mut tokens =
        identifier.split('_').map(std::string::ToString::to_string).collect::<Vec<_>>();

    if let Some(last) = tokens.last_mut() {
        *last = pluralize_word(last);
    }

    tokens.join("_")
}

/// Naive singularization aligned with project naming.
fn singularize_word(word: &str) -> String {
    if word.ends_with("ies") && word.len() > 3 {
        let mut base = word.to_string();
        base.truncate(base.len() - 3);
        base.push('y');
        return base;
    }

    if word.ends_with('s') && !word.ends_with("ss") && word.len() > 1 {
        let mut base = word.to_string();
        let _ = base.pop();
        return base;
    }

    word.to_string()
}

/// Naive pluralization aligned with project naming.
fn pluralize_word(word: &str) -> String {
    let mut chars = word.chars().rev();
    let Some(last) = chars.next() else {
        return String::new();
    };

    if last == 'y' {
        let previous = chars.next();
        let previous_is_vowel = matches!(previous, Some('a' | 'e' | 'i' | 'o' | 'u'));
        if !previous_is_vowel {
            let mut without_y = word.to_string();
            let _ = without_y.pop();
            without_y.push_str("ies");
            return without_y;
        }
    }

    format!("{word}s")
}

#[cfg(test)]
mod tests {
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::*;

    #[test]
    fn singularization_handles_plural_suffixes() {
        assert_eq!(singularize_word("batteries"), "battery");
        assert_eq!(singularize_word("days"), "day");
        assert_eq!(singularize_word("glass"), "glass");
    }

    #[test]
    fn pluralization_handles_consonant_y() {
        assert_eq!(pluralize_word("battery"), "batteries");
        assert_eq!(pluralize_word("day"), "days");
    }

    #[test]
    fn extractors_capture_tables_and_registrations() {
        let sql = r#"
CREATE TABLE camera_models (
    id UUID PRIMARY KEY
);
INSERT INTO table_names (id) VALUES ('camera_models') ON CONFLICT DO NOTHING;
"#;

        let created = extract_created_tables(sql);
        let registered = extract_registered_tables(sql);

        assert_eq!(created, vec!["camera_models"]);
        assert!(registered.contains("camera_models"));
    }

    #[test]
    fn derives_stems_from_single_and_multi_stem_folders() {
        assert_eq!(stems_from_migration_dir("001-weighing-devices"), vec!["weighing_device"]);
        assert_eq!(
            stems_from_migration_dir("003-ball-mill-machines+beads"),
            vec!["ball_mill_machine", "bead"]
        );
    }

    #[test]
    fn derives_table_stems_for_supported_families() {
        assert_eq!(
            derive_table_stem("commercial_ball_mill_machine_lots"),
            Some("ball_mill_machine".to_string())
        );
        assert_eq!(derive_table_stem("phone_devices"), Some("phone_device".to_string()));
        assert_eq!(derive_table_stem("phone_device"), None);
    }

    #[test]
    fn unsupported_message_mentions_plural_and_suggestion() {
        let message =
            unsupported_table_name_message("ball_mill_machine", &["ball_mill_machine".to_string()]);
        assert!(message.contains("should be plural"));
        assert!(message.contains("ball_mill_machines"));
    }

    #[test]
    fn full_device_chain_reports_missing_tables() {
        let migration = Migration {
            dir_name: "001-weighing-devices".to_string(),
            up_sql_path: PathBuf::from("unused"),
        };

        let declared_stems = vec!["weighing_device".to_string()];
        let created_tables =
            vec!["weighing_device_models".to_string(), "weighing_devices".to_string()];
        let mut errors = Vec::new();

        validate_full_device_chain(&migration, &declared_stems, &created_tables, &mut errors);

        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("commercial_weighing_device_models"));
        assert!(errors[0].contains("commercial_weighing_device_lots"));
    }

    #[test]
    fn shared_assets_docs_checker_accepts_fully_documented_migration() {
        let sql = r#"
-- Migration: Shared Assets / Example
-- Purpose:
--   Example purpose.
-- APS placement:
--   entities
--     +-- ownables
-- Security context:
-- SQL/RLS semantics:
-- Zanzibar semantics:
-- Creates example table.
CREATE TABLE examples (
    -- Stable identifier.
    id UUID PRIMARY KEY,
    -- User-facing name.
    name TEXT NOT NULL
);
-- Registers table name.
INSERT INTO table_names (id) VALUES ('examples') ON CONFLICT DO NOTHING;
"#;

        let mut errors = Vec::new();
        validate_shared_assets_documentation("example/up.sql", sql, &mut errors);
        assert!(errors.is_empty(), "{errors:?}");
    }

    #[test]
    fn shared_assets_docs_checker_reports_missing_markers_and_comments() {
        let sql = r#"
-- Migration: Shared Assets / Example
-- Purpose:
--   Example purpose.
-- APS placement:
CREATE TABLE examples (
    id UUID PRIMARY KEY
);
INSERT INTO table_names (id) VALUES ('examples') ON CONFLICT DO NOTHING;
"#;

        let mut errors = Vec::new();
        validate_shared_assets_documentation("example/up.sql", sql, &mut errors);

        assert!(!errors.is_empty());
        assert!(errors.iter().any(|error| error.contains("Security context")));
        assert!(errors.iter().any(|error| error.contains("ASCII placement diagram")));
        assert!(
            errors.iter().any(|error| error.contains("SQL statement 'INSERT INTO table_names"))
        );
        assert!(errors.iter().any(|error| error.contains("column line 'id UUID PRIMARY KEY'")));
    }

    #[test]
    fn column_definition_detection_ignores_check_expression_lines() {
        assert!(is_column_definition("id UUID PRIMARY KEY"));
        assert!(!is_column_definition("container_model_id <> contained_asset_model_id"));
    }

    #[test]
    fn docs_target_loader_accepts_single_up_sql_file() {
        let temp_dir = new_temp_test_dir();
        let up_sql = temp_dir.join("up.sql");
        fs::write(&up_sql, "-- test").expect("write up.sql");

        let files = load_up_sql_files_from_target(&up_sql).expect("load target");
        assert_eq!(files, vec![up_sql]);
    }

    #[test]
    fn devices_loader_accepts_single_migration_folder() {
        let temp_dir = new_temp_test_dir();
        let migration_dir = temp_dir.join("001-example-devices");
        fs::create_dir_all(&migration_dir).expect("create migration dir");
        let up_sql = migration_dir.join("up.sql");
        fs::write(&up_sql, "-- test").expect("write up.sql");

        let migrations =
            load_device_migrations_recursive(&migration_dir).expect("load devices target");
        assert_eq!(migrations.len(), 1);
        assert_eq!(migrations[0].dir_name, "001-example-devices");
        assert_eq!(migrations[0].up_sql_path, up_sql);
    }

    #[test]
    fn devices_loader_accepts_up_sql_file_target() {
        let temp_dir = new_temp_test_dir();
        let migration_dir = temp_dir.join("001-example-devices");
        fs::create_dir_all(&migration_dir).expect("create migration dir");
        let up_sql = migration_dir.join("up.sql");
        fs::write(&up_sql, "-- test").expect("write up.sql");

        let migrations = load_device_migrations_recursive(&up_sql).expect("load devices target");
        assert_eq!(migrations.len(), 1);
        assert_eq!(migrations[0].dir_name, "001-example-devices");
        assert_eq!(migrations[0].up_sql_path, up_sql);
    }

    #[test]
    fn devices_loader_recurses_under_target_directory() {
        let temp_dir = new_temp_test_dir();
        let top = temp_dir.join("001-top-devices");
        fs::create_dir_all(&top).expect("create top migration dir");
        fs::write(top.join("up.sql"), "-- top").expect("write top up.sql");

        let nested_root = temp_dir.join("nested");
        let nested = nested_root.join("002-nested-devices");
        fs::create_dir_all(&nested).expect("create nested migration dir");
        fs::write(nested.join("up.sql"), "-- nested").expect("write nested up.sql");

        let migrations =
            load_device_migrations_recursive(&temp_dir).expect("load devices target recursively");
        assert_eq!(migrations.len(), 2);
        assert!(migrations.iter().any(|migration| migration.dir_name == "001-top-devices"));
        assert!(migrations.iter().any(|migration| migration.dir_name == "002-nested-devices"));
    }

    fn new_temp_test_dir() -> PathBuf {
        let unique = SystemTime::now().duration_since(UNIX_EPOCH).expect("time works").as_nanos();
        let path = env::temp_dir().join(format!("sql_migration_lint_test_{unique}"));
        fs::create_dir_all(&path).expect("create temp dir");
        path
    }
}
