//! Submodule providing the `ReusedProcedureTemplateAssetModelsTrigger`
//! constraint, which enforces that for each table in the procedure template DAG
//! (identified by having columns referencing
//! `procedure_template_asset_models`), there must be a trigger that
//! automatically populates the `reused_procedure_template_asset_models` table.

use std::collections::HashSet;

use sql_rules::{
    error::{Error, RuleErrorInfo},
    prelude::*,
};
use sql_traits::traits::{ColumnLike, DatabaseLike, FunctionLike, TableLike, TriggerLike};
use sqlparser::{
    ast::{Expr, SetExpr, Statement, Values},
    dialect::PostgreSqlDialect,
    parser::Parser,
};

use crate::{
    column_rules::PROCEDURE_TEMPLATE_ASSET_MODELS_TABLE_NAME,
    table_rules::{
        PROCEDURE_TEMPLATES_TABLE_NAME,
        reused_procedure_template_asset_models_foreign_key::REUSED_PROCEDURE_TEMPLATE_ASSET_MODELS_TABLE_NAME,
    },
};

/// Struct defining a constraint that enforces the existence of triggers to
/// populate `reused_procedure_template_asset_models`.
///
/// # Example
///
/// ```rust
/// use sql_procedure_rules::prelude::*;
///
/// let mut constrainer = GenericConstrainer::<ParserDB>::default();
/// constrainer.register_table_rule(ReusedProcedureTemplateAssetModelsTrigger::default().into());
///
/// // Invalid: Table has columns referencing procedure_template_asset_models but no trigger
/// let invalid_schema = ParserDB::try_from(
///     r#"
/// CREATE TABLE procedure_templates (id INT PRIMARY KEY);
/// CREATE TABLE procedure_template_asset_models (id INT PRIMARY KEY);
/// CREATE TABLE reused_procedure_template_asset_models (
///     procedure_template_id INT,
///     procedure_template_asset_model_id INT
/// );
///
/// CREATE TABLE my_procedure_templates (
///     id INT PRIMARY KEY REFERENCES procedure_templates(id),
///     procedure_template_model_id INT REFERENCES procedure_template_asset_models(id)
/// );
/// "#,
/// )
/// .unwrap();
/// assert!(constrainer.validate_schema(&invalid_schema).is_err());
///
/// // Valid: Table has trigger and function correctly implemented
/// let valid_schema = ParserDB::try_from(
///     r#"
/// CREATE TABLE procedure_templates (id INT PRIMARY KEY);
/// CREATE TABLE procedure_template_asset_models (id INT PRIMARY KEY);
/// CREATE TABLE reused_procedure_template_asset_models (
///     procedure_template_id INT,
///     procedure_template_asset_model_id INT
/// );
///
/// CREATE TABLE my_procedure_templates (
///     id INT PRIMARY KEY REFERENCES procedure_templates(id),
///     procedure_template_model_id INT REFERENCES procedure_template_asset_models(id)
/// );
///
/// CREATE FUNCTION my_procedure_templates_rptam_insert_fn() RETURNS TRIGGER AS $$
/// BEGIN
///     INSERT INTO reused_procedure_template_asset_models (procedure_template_id, procedure_template_asset_model_id)
///     VALUES (NEW.id, NEW.procedure_template_model_id)
///     ON CONFLICT DO NOTHING;
///     RETURN NEW;
/// END;
/// $$ LANGUAGE plpgsql;
///
/// CREATE TRIGGER my_procedure_templates_rptam_insert_trigger
/// AFTER INSERT ON my_procedure_templates
/// FOR EACH ROW EXECUTE FUNCTION my_procedure_templates_rptam_insert_fn();
/// "#,
/// )
/// .unwrap();
/// assert!(constrainer.validate_schema(&valid_schema).is_ok());
/// ```
pub struct ReusedProcedureTemplateAssetModelsTrigger<DB>(std::marker::PhantomData<DB>);

impl<DB> Default for ReusedProcedureTemplateAssetModelsTrigger<DB> {
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<DB> From<ReusedProcedureTemplateAssetModelsTrigger<DB>> for Box<dyn TableRule<Database = DB>>
where
    DB: DatabaseLike + 'static,
{
    fn from(rule: ReusedProcedureTemplateAssetModelsTrigger<DB>) -> Self {
        Box::new(rule)
    }
}

impl<DB> TableRule for ReusedProcedureTemplateAssetModelsTrigger<DB>
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

        // 0. Ensure required tables exist
        let Some(procedure_templates_table) = database.table(None, PROCEDURE_TEMPLATES_TABLE_NAME)
        else {
            return Err(Error::Unapplicable(format!(
                "Table '{}' does not exist",
                PROCEDURE_TEMPLATES_TABLE_NAME
            )));
        };

        let Some(procedure_template_asset_models_table) =
            database.table(None, PROCEDURE_TEMPLATE_ASSET_MODELS_TABLE_NAME)
        else {
            return Err(Error::Unapplicable(format!(
                "Table '{}' does not exist",
                PROCEDURE_TEMPLATE_ASSET_MODELS_TABLE_NAME
            )));
        };

        // Check if this table is a descendant of procedure_templates
        if !table.is_descendant_of(database, procedure_templates_table)
            && table != procedure_templates_table
        {
            return Ok(());
        }

        // 1. Identify columns referencing `procedure_template_asset_models`
        let ptam_columns: Vec<_> = table
            .columns(database)
            .filter(|col| {
                table.referenced_tables_via_column(database, col).iter().any(|t| {
                    (*t).table_name() == procedure_template_asset_models_table.table_name()
                })
            })
            .map(|col| col.column_name().to_string())
            .collect();

        if ptam_columns.is_empty() {
            return Ok(());
        }

        // 2. Check for trigger existence
        let expected_trigger_name = format!("{}_rptam_insert_trigger", table_name);

        let found_trigger =
            table.triggers(database).find(|t| t.name() == expected_trigger_name.as_str());

        let trigger = match found_trigger {
            Some(t) => t,
            None => {
                let info: RuleErrorInfo = RuleErrorInfo::builder()
                    .rule("ReusedProcedureTemplateAssetModelsTrigger")
                    .unwrap()
                    .object(table_name.to_owned())
                    .unwrap()
                    .message(format!(
                        "Table '{}' has columns referencing '{}' but is missing the required trigger '{}'.",
                        table_name,
                        PROCEDURE_TEMPLATE_ASSET_MODELS_TABLE_NAME,
                        expected_trigger_name
                    ))
                    .unwrap()
                    .resolution(generate_full_sql(table_name, &ptam_columns))
                    .unwrap()
                    .try_into()
                    .unwrap();
                return Err(Error::Table(Box::new(table.clone()), info.into()));
            }
        };

        // 3. Check for function existence and name
        let expected_function_name = format!("{}_rptam_insert_fn", table_name);
        let function_opt = trigger.function(database);

        // Handling Option from function()
        let function = match function_opt {
            Some(f) => f,
            None => {
                let info: RuleErrorInfo = RuleErrorInfo::builder()
                    .rule("ReusedProcedureTemplateAssetModelsTrigger")
                    .unwrap()
                    .object(expected_trigger_name.clone())
                    .unwrap()
                    .message(format!(
                        "Trigger '{}' does not have an associated function.",
                        expected_trigger_name
                    ))
                    .unwrap()
                    .resolution(generate_full_sql(table_name, &ptam_columns))
                    .unwrap()
                    .try_into()
                    .unwrap();
                return Err(Error::Table(Box::new(table.clone()), info.into()));
            }
        };

        // Use ends_with because function names might be qualified
        if !function.name().ends_with(&expected_function_name) {
            let info: RuleErrorInfo = RuleErrorInfo::builder()
                .rule("ReusedProcedureTemplateAssetModelsTrigger")
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
                .resolution(generate_full_sql(table_name, &ptam_columns))
                .unwrap()
                .try_into()
                .unwrap();
            return Err(Error::Table(Box::new(table.clone()), info.into()));
        }

        // 4. Parse function body

        let body_str_opt = function.body();
        let body_str = match body_str_opt {
            Some(b) => b,
            None => {
                // No body implies we can't check it.
                return Ok(());
            }
        };

        let dialect = PostgreSqlDialect {};

        // Extract statements from PL/pgSQL function body: BEGIN ... END;
        let statements_sql =
            if body_str.trim().starts_with("BEGIN") && body_str.trim().ends_with("END;") {
                let start = body_str.find("BEGIN").unwrap() + "BEGIN".len();
                let end = body_str.rfind("END;").unwrap();
                body_str[start..end].trim()
            } else {
                body_str
            };

        let ast = match Parser::parse_sql(&dialect, statements_sql) {
            Ok(ast) => ast,
            Err(e) => {
                let info: RuleErrorInfo = RuleErrorInfo::builder()
                    .rule("ReusedProcedureTemplateAssetModelsTrigger")
                    .unwrap()
                    .object(function.name().to_string())
                    .unwrap()
                    .message(format!(
                        "Failed to parse body of function '{}': {}",
                        function.name(),
                        e
                    ))
                    .unwrap()
                    .resolution("Fix SQL syntax".to_string())
                    .unwrap()
                    .try_into()
                    .unwrap();
                return Err(Error::Table(Box::new(table.clone()), info.into()));
            }
        };

        // 5. Verify INSERTs
        let mut covered_columns = HashSet::new();

        for statement in ast {
            if let Statement::Insert(insert) = statement {
                // Check table name
                if insert.table.to_string() != REUSED_PROCEDURE_TEMPLATE_ASSET_MODELS_TABLE_NAME {
                    continue;
                }

                // Identify indices of relevant columns
                let mut pid_idx = None;
                let mut ptam_idx = None;

                for (i, col) in insert.columns.iter().enumerate() {
                    if col.value == "procedure_template_id" {
                        pid_idx = Some(i);
                    } else if col.value == "procedure_template_asset_model_id" {
                        ptam_idx = Some(i);
                    }
                }

                if pid_idx.is_none() || ptam_idx.is_none() {
                    continue;
                }

                // Check values
                if let Some(query) = insert.source
                    && let SetExpr::Values(Values { rows, .. }) = *query.body
                {
                    for row in rows {
                        let pid_expr = &row[pid_idx.unwrap()];
                        let ptam_expr = &row[ptam_idx.unwrap()];

                        if is_new_field(pid_expr, "id")
                            && let Some(col_name) = extract_new_field(ptam_expr)
                        {
                            covered_columns.insert(col_name);
                        }
                    }
                }
            }
        }

        for col in &ptam_columns {
            if !covered_columns.contains(col) {
                let info: RuleErrorInfo = RuleErrorInfo::builder()
                    .rule("ReusedProcedureTemplateAssetModelsTrigger")
                    .unwrap()
                    .object(function.name().to_string())
                    .unwrap()
                    .message(format!(
                        "Function '{}' is missing an INSERT statement into '{}' for column '{}'.",
                        function.name(),
                        REUSED_PROCEDURE_TEMPLATE_ASSET_MODELS_TABLE_NAME,
                        col
                    ))
                    .unwrap()
                    .resolution(generate_full_sql(table_name, &ptam_columns))
                    .unwrap()
                    .try_into()
                    .unwrap();
                return Err(Error::Table(Box::new(table.clone()), info.into()));
            }
        }

        Ok(())
    }
}

fn generate_full_sql(table_name: &str, ptam_columns: &[String]) -> String {
    let function_name = format!("{}_rptam_insert_fn", table_name);
    let trigger_name = format!("{}_rptam_insert_trigger", table_name);

    let mut inserts = String::new();
    for col in ptam_columns {
        inserts.push_str(&format!(
            "\tINSERT INTO reused_procedure_template_asset_models (procedure_template_id, procedure_template_asset_model_id) VALUES (NEW.id, NEW.{}) ON CONFLICT DO NOTHING;\n",
            col
        ));
    }

    format!(
        "CREATE OR REPLACE FUNCTION {}() RETURNS TRIGGER AS $$
BEGIN
{}  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER {}
AFTER INSERT ON {}
FOR EACH ROW EXECUTE FUNCTION {}();",
        function_name, inserts, trigger_name, table_name, function_name
    )
}

fn is_new_field(expr: &Expr, field: &str) -> bool {
    match expr {
        Expr::CompoundIdentifier(ids) => {
            ids.len() == 2 && ids[0].value.to_uppercase() == "NEW" && ids[1].value == field
        }
        _ => false,
    }
}

fn extract_new_field(expr: &Expr) -> Option<String> {
    match expr {
        Expr::CompoundIdentifier(ids) => {
            if ids.len() == 2 && ids[0].value.to_uppercase() == "NEW" {
                Some(ids[1].value.clone())
            } else {
                None
            }
        }
        _ => None,
    }
}
