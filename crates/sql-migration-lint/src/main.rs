//! Command-line linter for SQL migration naming and registration conventions.

use std::{
    collections::BTreeSet,
    env, fs,
    path::{Path, PathBuf},
    process::ExitCode,
};

/// Relative path to the device migrations root.
const DEVICES_RELATIVE_PATH: &str = "sql/shared-schema/003-shared-assets/004-devices";

/// Supported command for device migration linting.
const DEVICES_COMMAND: &str = "devices";
/// Optional flag to require complete device table families for each stem.
const REQUIRE_FULL_DEVICE_CHAIN_FLAG: &str = "--require-full-device-chain";

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
        if argument == REQUIRE_FULL_DEVICE_CHAIN_FLAG {
            options.require_full_device_chain = true;
            index += 1;
            continue;
        }

        return Err(format!("Unknown argument '{argument}'\n\n{}", usage()));
    }

    match command.as_str() {
        DEVICES_COMMAND => lint_devices(&root, options),
        _ => Err(format!("Unknown command '{command}'\n\n{}", usage())),
    }
}

/// Returns command-line usage instructions.
fn usage() -> String {
    format!(
        "Usage:\n  cargo run -p sql-migration-lint -- {DEVICES_COMMAND} [--root <path>] [{REQUIRE_FULL_DEVICE_CHAIN_FLAG}]\n"
    )
}

/// Lints the device migration folder naming and table registration conventions.
fn lint_devices(root: &Path, options: DeviceLintOptions) -> Result<(), String> {
    let devices_dir = root.join(DEVICES_RELATIVE_PATH);
    let migrations = load_migrations(&devices_dir)?;

    if migrations.is_empty() {
        return Err(format!(
            "No device migrations with up.sql found under {}",
            devices_dir.display()
        ));
    }

    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    validate_incremental_prefixes(&migrations, &mut errors);

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

/// Loads migration directories that contain an `up.sql` file.
fn load_migrations(devices_dir: &Path) -> Result<Vec<Migration>, String> {
    let entries = fs::read_dir(devices_dir)
        .map_err(|error| format!("Failed to read {}: {error}", devices_dir.display()))?;

    let mut migrations = Vec::new();

    for entry_result in entries {
        let entry = entry_result.map_err(|error| {
            format!("Failed to iterate entries in {}: {error}", devices_dir.display())
        })?;

        let file_type = entry.file_type().map_err(|error| {
            format!(
                "Failed to inspect entry '{}' in {}: {error}",
                entry.path().display(),
                devices_dir.display()
            )
        })?;

        if !file_type.is_dir() {
            continue;
        }

        let up_sql_path = entry.path().join("up.sql");
        if !up_sql_path.is_file() {
            continue;
        }

        let dir_name = entry.file_name().to_string_lossy().to_string();
        migrations.push(Migration { dir_name, up_sql_path });
    }

    migrations.sort_by(|left, right| left.dir_name.cmp(&right.dir_name));
    Ok(migrations)
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
}
