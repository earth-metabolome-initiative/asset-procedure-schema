//! Submodule providing the `UpdateTimestampTrigger` constraint, which enforces
//! that if a table has a specific timestamp column (default `edited_at`),
//! it must have a trigger to automatically update it on modification.

use sql_rules::{
    error::{Error, RuleErrorInfo},
    prelude::*,
};
use sql_traits::traits::{ColumnLike, DatabaseLike, FunctionLike, TableLike, TriggerLike};

/// Struct defining a constraint that enforces the existence of triggers to
/// update a timestamp column (e.g., `edited_at`).
///
/// # Example
///
/// ```rust
/// use sql_procedure_rules::prelude::*;
///
/// // Register with default column "edited_at"
/// let mut constrainer = GenericConstrainer::<ParserDB>::default();
/// constrainer.register_table_rule(UpdateTimestampTrigger::default().into());
///
/// // Register with custom columns
/// constrainer
///     .register_table_rule(UpdateTimestampTrigger::new(vec!["updated_at".to_string()]).into());
/// ```
pub struct UpdateTimestampTrigger<DB> {
    column_names: Vec<String>,
    marker: std::marker::PhantomData<DB>,
}

impl<DB> UpdateTimestampTrigger<DB> {
    /// Creates a new rule for the specified column names.
    pub fn new(column_names: Vec<String>) -> Self {
        Self { column_names, marker: std::marker::PhantomData }
    }
}

impl<DB> Default for UpdateTimestampTrigger<DB> {
    fn default() -> Self {
        Self::new(vec!["edited_at".to_string()])
    }
}

impl<DB> From<UpdateTimestampTrigger<DB>> for Box<dyn TableRule<Database = DB>>
where
    DB: DatabaseLike + 'static,
{
    fn from(rule: UpdateTimestampTrigger<DB>) -> Self {
        Box::new(rule)
    }
}

impl<DB> TableRule for UpdateTimestampTrigger<DB>
where
    DB: DatabaseLike,
{
    type Database = DB;

    fn validate_table(
        &self,
        database: &Self::Database,
        table: &<Self::Database as DatabaseLike>::Table,
    ) -> Result<(), Error<Self::Database>> {
        let table_name = table.table_name();

        for column_name in &self.column_names {
            // 1. Check if the table has the target column
            let has_column = table.columns(database).any(|col| col.column_name() == column_name);

            if !has_column {
                continue;
            }

            // 2. Check for trigger existence
            // Convention: trigger_update_<table_name>_<column_name>
            let expected_trigger_name = format!("trigger_update_{}_{}", table_name, column_name);

            let found_trigger =
                table.triggers(database).find(|t| t.name() == expected_trigger_name.as_str());

            let trigger = match found_trigger {
                Some(t) => t,
                None => {
                    let info: RuleErrorInfo = RuleErrorInfo::builder()
                        .rule("UpdateTimestampTrigger")
                        .unwrap()
                        .object(table_name.to_owned())
                        .unwrap()
                        .message(format!(
                            "Table '{}' has column '{}' but is missing the required trigger '{}'.",
                            table_name, column_name, expected_trigger_name
                        ))
                        .unwrap()
                        .resolution(generate_resolution_sql(table_name, column_name))
                        .unwrap()
                        .try_into()
                        .unwrap();
                    return Err(Error::Table(Box::new(table.clone()), info.into()));
                }
            };

            // 3. Check for function existence and name
            // Convention: update_<table_name>_<column_name>
            let expected_function_name = format!("update_{}_{}", table_name, column_name);
            let function_opt = trigger.function(database);

            let function = match function_opt {
                Some(f) => f,
                None => {
                    let info: RuleErrorInfo = RuleErrorInfo::builder()
                        .rule("UpdateTimestampTrigger")
                        .unwrap()
                        .object(expected_trigger_name.clone())
                        .unwrap()
                        .message(format!(
                            "Trigger '{}' does not have an associated function.",
                            expected_trigger_name
                        ))
                        .unwrap()
                        .resolution(generate_resolution_sql(table_name, column_name))
                        .unwrap()
                        .try_into()
                        .unwrap();
                    return Err(Error::Table(Box::new(table.clone()), info.into()));
                }
            };

            if function.name() != expected_function_name {
                let info: RuleErrorInfo = RuleErrorInfo::builder()
                    .rule("UpdateTimestampTrigger")
                    .unwrap()
                    .object(expected_trigger_name.clone())
                    .unwrap()
                    .message(format!(
                        "Trigger '{}' calls function '{}', but expected '{}'.",
                        expected_trigger_name,
                        function.name(),
                        expected_function_name
                    ))
                    .unwrap()
                    .resolution(generate_resolution_sql(table_name, column_name))
                    .unwrap()
                    .try_into()
                    .unwrap();
                return Err(Error::Table(Box::new(table.clone()), info.into()));
            }

            // 4. Validate function body
            // We expect: NEW.<column_name> = CURRENT_TIMESTAMP;
            // removing all whitespace for robust underlying check
            let body_str_opt = function.body();
            let body_str = match body_str_opt {
                Some(b) => b,
                None => continue, // Cannot validate body if not present in model
            };

            let normalized_body: String = body_str.chars().filter(|c| !c.is_whitespace()).collect();
            let expected_stmt = format!("NEW.{} = CURRENT_TIMESTAMP", column_name);
            let expected_stmt_normalized: String =
                expected_stmt.chars().filter(|c| !c.is_whitespace()).collect();

            if !normalized_body.contains(&expected_stmt_normalized) {
                let info: RuleErrorInfo = RuleErrorInfo::builder()
                    .rule("UpdateTimestampTrigger")
                    .unwrap()
                    .object(function.name().to_string())
                    .unwrap()
                    .message(format!(
                        "Function '{}' body does not contain the expected assignment: '{}'.",
                        function.name(),
                        expected_stmt
                    ))
                    .unwrap()
                    .resolution(generate_resolution_sql(table_name, column_name))
                    .unwrap()
                    .try_into()
                    .unwrap();
                return Err(Error::Table(Box::new(table.clone()), info.into()));
            }
        }

        Ok(())
    }
}

fn generate_resolution_sql(table_name: &str, column_name: &str) -> String {
    format!(
        r#"
CREATE OR REPLACE FUNCTION update_{table_name}_{column_name}() RETURNS TRIGGER AS $$
BEGIN
    NEW.{column_name} = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_update_{table_name}_{column_name}
BEFORE UPDATE ON {table_name}
FOR EACH ROW EXECUTE FUNCTION update_{table_name}_{column_name}();
"#
    )
}
