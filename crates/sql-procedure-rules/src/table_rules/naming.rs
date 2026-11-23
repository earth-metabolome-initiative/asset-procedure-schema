use super::*;

declare_rule!(
    ProcedureDescendantNaming,
    "Procedure subtypes have names ending in `_procedures`.",
    "procedure_descendant_naming",
    "CREATE TABLE procedures (id INT PRIMARY KEY); CREATE TABLE weighing (id INT PRIMARY KEY REFERENCES procedures(id));",
    "CREATE TABLE procedures (id INT PRIMARY KEY); CREATE TABLE weighing_procedures (id INT PRIMARY KEY REFERENCES procedures(id));"
);
declare_rule!(
    ProcedureTemplateDescendantNaming,
    "Template subtypes have names ending in `_procedure_templates`.",
    "procedure_template_descendant_naming",
    "CREATE TABLE procedure_templates (id INT PRIMARY KEY); CREATE TABLE weighing (id INT PRIMARY KEY REFERENCES procedure_templates(id));",
    "CREATE TABLE procedure_templates (id INT PRIMARY KEY); CREATE TABLE weighing_procedure_templates (id INT PRIMARY KEY REFERENCES procedure_templates(id));"
);
declare_rule!(
    MatchingProcedureAndTemplateTables,
    "Each typed procedure has a matching typed template table and conversely.",
    "matching_procedure_and_template_tables",
    "CREATE TABLE procedures (id INT PRIMARY KEY); CREATE TABLE weighing_procedures (id INT PRIMARY KEY REFERENCES procedures(id));",
    "CREATE TABLE procedures (id INT PRIMARY KEY); CREATE TABLE procedure_templates (id INT PRIMARY KEY); CREATE TABLE weighing_procedures (id INT PRIMARY KEY REFERENCES procedures(id)); CREATE TABLE weighing_procedure_templates (id INT PRIMARY KEY REFERENCES procedure_templates(id));"
);

fn naming<DB: DatabaseLike>(
    database: &DB,
    table: &DB::Table,
    root: &str,
    suffix: &str,
    reporter: &mut Reporter<'_, DB>,
) -> Result<(), Interruption> {
    if descendant(database, table, root)? && !table.table_name().ends_with(suffix) {
        reporter.table(
            table,
            format!("Subtype name must end with {suffix}."),
            format!("Rename the table with the {suffix} suffix."),
        )?;
    }
    Ok(())
}

impl<DB: DatabaseLike> TableRule for ProcedureDescendantNaming<DB> {
    type Database = DB;
    fn check_table(
        &self,
        database: &DB,
        table: &DB::Table,
        reporter: &mut Reporter<'_, DB>,
    ) -> Result<(), Interruption> {
        naming(database, table, PROCEDURES_TABLE_NAME, "_procedures", reporter)
    }
}

impl<DB: DatabaseLike> TableRule for ProcedureTemplateDescendantNaming<DB> {
    type Database = DB;
    fn check_table(
        &self,
        database: &DB,
        table: &DB::Table,
        reporter: &mut Reporter<'_, DB>,
    ) -> Result<(), Interruption> {
        naming(database, table, PROCEDURE_TEMPLATES_TABLE_NAME, "_procedure_templates", reporter)
    }
}

impl<DB: DatabaseLike> TableRule for MatchingProcedureAndTemplateTables<DB> {
    type Database = DB;
    fn check_table(
        &self,
        database: &DB,
        table: &DB::Table,
        reporter: &mut Reporter<'_, DB>,
    ) -> Result<(), Interruption> {
        let (suffix, counterpart, root) = if descendant(database, table, PROCEDURES_TABLE_NAME)? {
            ("_procedures", "_procedure_templates", PROCEDURE_TEMPLATES_TABLE_NAME)
        } else if descendant(database, table, PROCEDURE_TEMPLATES_TABLE_NAME)? {
            ("_procedure_templates", "_procedures", PROCEDURES_TABLE_NAME)
        } else {
            return Ok(());
        };
        let Some(prefix) = table.table_name().strip_suffix(suffix) else {
            return Ok(());
        };
        let expected = format!("{prefix}{counterpart}");
        let counterpart = database.tables().find(|candidate| {
            candidate.table_name() == expected && candidate.table_schema() == table.table_schema()
        });
        let matches = if let Some(counterpart) = counterpart {
            descendant(database, counterpart, root)?
        } else {
            false
        };
        if !matches {
            reporter.table(
                table,
                format!("Table requires matching subtype table {expected}."),
                format!("Create {expected} extending {root}."),
            )?;
        }
        Ok(())
    }
}
