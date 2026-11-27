//! Validates the migration sequence applied by `tests/run.sh`.

use std::path::{Path, PathBuf};

use sql_procedure_rules::register_procedure_constraints;
use sql_traits::prelude::ParserDB;
use sqlippy::prelude::*;
use sqlparser::dialect::PostgreSqlDialect;

fn migrations(directory: &Path, paths: &mut Vec<PathBuf>) {
    for entry in std::fs::read_dir(directory).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            migrations(&path, paths);
        } else if path.file_name().is_some_and(|name| name == "up.sql") {
            paths.push(path);
        }
    }
}

#[test]
fn validate_schema() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let mut connetto = Vec::new();
    migrations(&root.join("connetto-schema/sql"), &mut connetto);
    connetto.sort();
    let mut domain = Vec::new();
    migrations(&root.join("sql"), &mut domain);
    domain.sort();
    assert!(!connetto.is_empty() && !domain.is_empty());
    let paths: Vec<_> =
        [root.join("connetto-schema/cluster/roles.sql"), root.join("cluster/roles.sql")]
            .into_iter()
            .chain(connetto)
            .chain(domain)
            .collect();
    let paths: Vec<_> = paths.iter().map(PathBuf::as_path).collect();
    let database = ParserDB::from_paths::<PostgreSqlDialect>(&paths).unwrap();
    let mut constrainer = GenericConstrainer::<ParserDB>::default();
    register_procedure_constraints(&mut constrainer);
    constrainer.validate_schema(&database).unwrap();
}
