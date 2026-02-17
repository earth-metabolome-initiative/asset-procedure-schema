//! Test to validate the schema against the rules.

#[cfg(test)]
mod tests {
    use std::path::Path;

    use sql_procedure_rules::register_procedure_constraints;
    use sql_rules::prelude::*;
    use sql_traits::prelude::ParserDB;
    use sqlparser::dialect::PostgreSqlDialect;

    #[test]
    fn validate_schema() {
        // Parse the database schema from the root of the repository
        let db = ParserDB::from_path::<PostgreSqlDialect>(Path::new("../../"))
            .expect("Failed to parse database schema");
        assert!(db.has_tables(), "Database should have tables");

        // Validate the database schema with all available constraints
        let mut constrainer = DefaultConstrainer::<ParserDB>::default();
        register_procedure_constraints(&mut constrainer);

        // We panic if the validation fails to show the error
        if let Err(e) = constrainer.validate_schema(&db) {
            panic!("{}", e);
        }
    }
}
