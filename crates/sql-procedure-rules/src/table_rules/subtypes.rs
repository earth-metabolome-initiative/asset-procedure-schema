use super::*;

declare_rule!(
    TypedTemplateSubtype,
    "Typed template slots bind their owner, model and role and use `guard_procedure_template_subtype`.",
    "typed_template_subtype",
    "CREATE TABLE procedure_templates (id INT PRIMARY KEY); CREATE TABLE typed (id INT PRIMARY KEY REFERENCES procedure_templates(id));",
    "CREATE FUNCTION guard_procedure_template_subtype() RETURNS TRIGGER LANGUAGE plpgsql AS $$ BEGIN RETURN NEW; END $$; CREATE TABLE procedure_templates (id INT PRIMARY KEY); CREATE TABLE typed (id INT PRIMARY KEY REFERENCES procedure_templates(id)); CREATE TRIGGER guard BEFORE INSERT OR UPDATE OR DELETE ON typed FOR EACH ROW EXECUTE FUNCTION guard_procedure_template_subtype();"
);
declare_rule!(
    TypedProcedureSubtype,
    "Typed procedures copy their template and status under a deferred key and use `guard_procedure_subtype`.",
    "typed_procedure_subtype",
    "CREATE TABLE procedures (id INT PRIMARY KEY); CREATE TABLE typed (id INT PRIMARY KEY REFERENCES procedures(id));",
    "CREATE FUNCTION guard_procedure_subtype() RETURNS TRIGGER LANGUAGE plpgsql AS $$ BEGIN RETURN NEW; END $$; CREATE TABLE procedure_templates (id INT PRIMARY KEY); CREATE TABLE typed_templates (id INT PRIMARY KEY REFERENCES procedure_templates(id)); CREATE TABLE procedures (id INT PRIMARY KEY, organization_id INT, procedure_template_id INT, status_id TEXT, UNIQUE (id, procedure_template_id, status_id, organization_id)); CREATE TABLE typed (id INT PRIMARY KEY REFERENCES procedures(id), organization_id INT, template_id INT REFERENCES typed_templates(id), status_id TEXT, FOREIGN KEY (id, template_id, status_id, organization_id) REFERENCES procedures(id, procedure_template_id, status_id, organization_id) DEFERRABLE INITIALLY DEFERRED); CREATE TRIGGER guard BEFORE INSERT OR UPDATE OR DELETE ON typed FOR EACH ROW EXECUTE FUNCTION guard_procedure_subtype();"
);

impl<DB: DatabaseLike> TableRule for TypedTemplateSubtype<DB> {
    type Database = DB;
    fn check_table(
        &self,
        database: &DB,
        table: &DB::Table,
        reporter: &mut Reporter<'_, DB>,
    ) -> Result<(), Interruption> {
        if !descendant(database, table, PROCEDURE_TEMPLATES_TABLE_NAME)? {
            return Ok(());
        }
        for column in table.columns(database)? {
            let Some(prefix) = column.column_name().strip_suffix("_slot_id") else {
                continue;
            };
            let model = format!("{prefix}_model_id");
            let role = format!("{prefix}_slot_role_id");
            let mut complete = true;
            for required in [&model, &role] {
                if !has_column(database, table, required)? {
                    complete = false;
                    reporter.table(
                        table,
                        format!("Slot {} requires {required}.", column.column_name()),
                        "Add the slot companion column.".into(),
                    )?;
                }
            }
            if !complete {
                continue;
            }
            let mut found = false;
            for foreign_key in table.foreign_keys(database)? {
                if key_matches(
                    database,
                    foreign_key,
                    "procedure_template_slots",
                    &[column.column_name(), "id", &model, &role, "organization_id"],
                    &[
                        "id",
                        "procedure_template_id",
                        "asset_model_id",
                        "role_id",
                        "organization_id",
                    ],
                )? {
                    found = true;
                    break;
                }
            }
            if !found {
                reporter.table(table, format!("Slot {} requires its slot ownership foreign key.", column.column_name()), format!("Add FOREIGN KEY ({}, id, {model}, {role}, organization_id) REFERENCES procedure_template_slots (id, procedure_template_id, asset_model_id, role_id, organization_id).", column.column_name()))?;
            }
        }
        guard(database, table, "guard_procedure_template_subtype", reporter)
    }
}

impl<DB: DatabaseLike> TableRule for TypedProcedureSubtype<DB> {
    type Database = DB;
    fn check_table(
        &self,
        database: &DB,
        table: &DB::Table,
        reporter: &mut Reporter<'_, DB>,
    ) -> Result<(), Interruption> {
        if !descendant(database, table, PROCEDURES_TABLE_NAME)? {
            return Ok(());
        }
        if !has_column(database, table, "status_id")? {
            reporter.table(
                table,
                "Typed procedure requires status_id.".into(),
                "Add the copied status column.".into(),
            )?;
            return Ok(());
        }
        let mut typed_template = false;
        let mut status_copy = false;
        for template_key in table.foreign_keys(database)? {
            if !descendant(
                database,
                template_key.referenced_table(database)?,
                PROCEDURE_TEMPLATES_TABLE_NAME,
            )? {
                continue;
            }
            let mut columns = template_key.host_columns(database)?;
            let Some(template_column) = columns.next() else {
                continue;
            };
            if columns.next().is_some()
                || !template_key
                    .referenced_columns(database)?
                    .map(ColumnLike::column_name)
                    .eq(["id"])
            {
                continue;
            }
            typed_template = true;
            for status_key in table.foreign_keys(database)? {
                if status_key.deferral(database) == ConstraintDeferral::InitiallyDeferred
                    && key_matches(
                        database,
                        status_key,
                        PROCEDURES_TABLE_NAME,
                        &["id", template_column.column_name(), "status_id", "organization_id"],
                        &["id", "procedure_template_id", "status_id", "organization_id"],
                    )?
                {
                    status_copy = true;
                    break;
                }
            }
        }
        if !typed_template {
            reporter.table(
                table,
                "Typed procedure requires a typed template foreign key.".into(),
                "Reference the typed template primary key.".into(),
            )?;
        } else if !status_copy {
            reporter.table(table, "Typed procedure requires a deferred status-copy foreign key.".into(), "Reference procedures (id, procedure_template_id, status_id, organization_id) DEFERRABLE INITIALLY DEFERRED, with a trigger copying status updates.".into())?;
        }
        guard(database, table, "guard_procedure_subtype", reporter)
    }
}
