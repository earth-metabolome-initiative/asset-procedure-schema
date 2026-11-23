//! Isolated procedure rule contracts.

use sql_procedure_rules::{column_rules::AssetModelColumnNaming, table_rules::*};
use sql_traits::prelude::ParserDB;
use sqlippy::prelude::*;
use sqlparser::dialect::PostgreSqlDialect;

const ROOTS: &str = "CREATE TABLE procedure_templates (id UUID PRIMARY KEY); CREATE TABLE procedures (id UUID PRIMARY KEY, organization_id UUID, procedure_template_id UUID, status_id TEXT, UNIQUE (id, procedure_template_id, status_id), UNIQUE (id, procedure_template_id, status_id, organization_id));";
const TEMPLATE: &str = "CREATE TABLE weighing_procedure_templates (id UUID PRIMARY KEY REFERENCES procedure_templates(id), organization_id UUID, weighed_slot_id UUID, weighed_model_id UUID, weighed_slot_role_id TEXT, FOREIGN KEY (weighed_slot_id, id, weighed_model_id, weighed_slot_role_id, organization_id) REFERENCES procedure_template_slots(id, procedure_template_id, asset_model_id, role_id, organization_id)); CREATE TRIGGER guard BEFORE INSERT OR UPDATE OR DELETE ON weighing_procedure_templates FOR EACH ROW EXECUTE FUNCTION guard_procedure_template_subtype();";
const SLOTS: &str = "CREATE TABLE procedure_template_slots (id UUID PRIMARY KEY, organization_id UUID, procedure_template_id UUID, asset_model_id UUID, role_id TEXT, UNIQUE (id, procedure_template_id, asset_model_id, role_id), UNIQUE (id, procedure_template_id, asset_model_id, role_id, organization_id));";
const PROCEDURE: &str = "CREATE TABLE weighing_procedures (id UUID PRIMARY KEY REFERENCES procedures(id), organization_id UUID, weighing_procedure_template_id UUID REFERENCES weighing_procedure_templates(id), status_id TEXT, FOREIGN KEY (id, weighing_procedure_template_id, status_id, organization_id) REFERENCES procedures(id, procedure_template_id, status_id, organization_id) ON UPDATE CASCADE); CREATE TRIGGER guard BEFORE INSERT OR UPDATE OR DELETE ON weighing_procedures FOR EACH ROW EXECUTE FUNCTION guard_procedure_subtype();";
const SUPPORT: &str = "CREATE ROLE app; CREATE ROLE unrelated; CREATE FUNCTION guard_procedure_template_subtype() RETURNS TRIGGER LANGUAGE plpgsql AS $$ BEGIN RETURN NEW; END $$; CREATE FUNCTION guard_procedure_subtype() RETURNS TRIGGER LANGUAGE plpgsql AS $$ BEGIN RETURN NEW; END $$; CREATE FUNCTION wrong_guard() RETURNS TRIGGER LANGUAGE plpgsql AS $$ BEGIN RETURN NEW; END $$;";

fn check(rule: impl TableRule<Database = ParserDB> + 'static, sql: &str, violation: Option<&str>) {
    let database = ParserDB::parse::<PostgreSqlDialect>(&format!("{SUPPORT}{sql}")).unwrap();
    let mut constrainer = GenericConstrainer::default();
    constrainer.register_table_rule(Box::new(rule));
    match violation {
        None => constrainer.validate_schema(&database).unwrap(),
        Some(message) => {
            let error = constrainer.validate_schema(&database).unwrap_err().to_string();
            assert!(error.contains(message), "{error}");
        }
    }
}

#[test]
fn procedure_names() {
    for (name, violation) in
        [("weighing_procedures", None), ("weighing", Some("must end with _procedures"))]
    {
        check(
            ProcedureDescendantNaming::default(),
            &format!(
                "{ROOTS} CREATE TABLE {name} (id UUID PRIMARY KEY REFERENCES procedures(id));"
            ),
            violation,
        );
    }
}

#[test]
fn template_names() {
    for (name, violation) in [
        ("weighing_procedure_templates", None),
        ("weighing", Some("must end with _procedure_templates")),
    ] {
        check(
            ProcedureTemplateDescendantNaming::default(),
            &format!(
                "{ROOTS} CREATE TABLE {name} (id UUID PRIMARY KEY REFERENCES procedure_templates(id));"
            ),
            violation,
        );
    }
}

#[test]
fn matching_tables() {
    let template = "CREATE TABLE weighing_procedure_templates (id UUID PRIMARY KEY REFERENCES procedure_templates(id));";
    let procedure =
        "CREATE TABLE weighing_procedures (id UUID PRIMARY KEY REFERENCES procedures(id));";
    check(
        MatchingProcedureAndTemplateTables::default(),
        &format!("{ROOTS}{template}{procedure}"),
        None,
    );
    for schema in [template, procedure] {
        check(
            MatchingProcedureAndTemplateTables::default(),
            &format!("{ROOTS}{schema}"),
            Some("matching subtype table"),
        );
    }
}

#[test]
fn template_slots() {
    let schema = format!("{ROOTS}{SLOTS}{TEMPLATE}");
    check(TypedTemplateSubtype::default(), &schema, None);
    for (before, after, violation) in [
        ("weighed_model_id UUID,", "", "requires weighed_model_id"),
        ("weighed_slot_role_id TEXT,", "", "requires weighed_slot_role_id"),
        (
            "FOREIGN KEY (weighed_slot_id, id, weighed_model_id, weighed_slot_role_id, organization_id) REFERENCES procedure_template_slots(id, procedure_template_id, asset_model_id, role_id, organization_id)",
            "CHECK (true)",
            "slot ownership foreign key",
        ),
        (
            "(weighed_slot_id, id, weighed_model_id, weighed_slot_role_id, organization_id)",
            "(id, weighed_slot_id, weighed_model_id, weighed_slot_role_id, organization_id)",
            "slot ownership foreign key",
        ),
        (
            "weighed_slot_role_id, organization_id) REFERENCES procedure_template_slots(id, procedure_template_id, asset_model_id, role_id, organization_id)",
            "weighed_slot_role_id) REFERENCES procedure_template_slots(id, procedure_template_id, asset_model_id, role_id)",
            "slot ownership foreign key",
        ),
        ("guard_procedure_template_subtype()", "wrong_guard()", "guard_procedure_template_subtype"),
    ] {
        let invalid = schema.replace(before, after);
        let invalid = if after.is_empty() {
            invalid.replace("FOREIGN KEY (weighed_slot_id, id, weighed_model_id, weighed_slot_role_id, organization_id) REFERENCES procedure_template_slots(id, procedure_template_id, asset_model_id, role_id, organization_id)", "CHECK (true)")
        } else {
            invalid
        };
        check(TypedTemplateSubtype::default(), &invalid, Some(violation));
    }
}

#[test]
fn procedure_status() {
    let schema = format!("{ROOTS}{SLOTS}{TEMPLATE}{PROCEDURE}");
    check(TypedProcedureSubtype::default(), &schema, None);
    for (before, after, violation) in [
        (
            "status_id TEXT, FOREIGN KEY (id, weighing_procedure_template_id, status_id, organization_id) REFERENCES procedures(id, procedure_template_id, status_id, organization_id) ON UPDATE CASCADE",
            "CHECK (true)",
            "requires status_id",
        ),
        ("ON UPDATE CASCADE", "ON UPDATE RESTRICT", "status-copy foreign key"),
        (
            "(id, weighing_procedure_template_id, status_id, organization_id)",
            "(weighing_procedure_template_id, id, status_id, organization_id)",
            "status-copy foreign key",
        ),
        (
            "status_id, organization_id) REFERENCES procedures(id, procedure_template_id, status_id, organization_id)",
            "status_id) REFERENCES procedures(id, procedure_template_id, status_id)",
            "status-copy foreign key",
        ),
        (
            "REFERENCES weighing_procedure_templates(id)",
            "REFERENCES procedure_templates(id)",
            "typed template foreign key",
        ),
        ("guard_procedure_subtype()", "wrong_guard()", "guard_procedure_subtype"),
    ] {
        check(TypedProcedureSubtype::default(), &schema.replace(before, after), Some(violation));
    }
}

#[test]
fn quantities() {
    for root in ["procedures", "procedure_templates"] {
        for suffix in ["kg", "l", "k", "s", "pa", "hz", "rpm", "percentage"] {
            let schema = format!(
                "{ROOTS} CREATE TABLE typed (id UUID PRIMARY KEY REFERENCES {root}(id), amount_{suffix} DOUBLE PRECISION, bead_count SMALLINT);"
            );
            check(QuantityUnits::default(), &schema, None);
            check(
                QuantityUnits::default(),
                &schema.replace("DOUBLE PRECISION", "REAL"),
                Some("DOUBLE PRECISION"),
            );
            check(
                QuantityUnits::default(),
                &schema.replace(&format!("amount_{suffix}"), "amount"),
                Some("unit suffix"),
            );
        }
    }
    check(QuantityUnits::default(), "CREATE TABLE unrelated (value DOUBLE PRECISION);", None);
}

#[test]
fn model_names() {
    for (name, violation) in
        [("weighed_model_id", None), ("weighed_id", Some("must end with _model_id"))]
    {
        let schema = format!(
            "{ROOTS} CREATE TABLE asset_models (id UUID PRIMARY KEY); CREATE TABLE weighing_procedure_templates (id UUID PRIMARY KEY REFERENCES procedure_templates(id), {name} UUID REFERENCES asset_models(id));"
        );
        let database = ParserDB::parse::<PostgreSqlDialect>(&schema).unwrap();
        let mut constrainer = GenericConstrainer::default();
        constrainer.register_column_rule(Box::new(AssetModelColumnNaming::default()));
        if let Some(message) = violation {
            let error = constrainer.validate_schema(&database).unwrap_err().to_string();
            assert!(error.contains(message), "{error}");
        } else {
            constrainer.validate_schema(&database).unwrap();
        }
    }
}

#[test]
fn tenancy() {
    let schema = "CREATE TABLE asset_models (id UUID PRIMARY KEY, organization_id UUID); ALTER TABLE asset_models ENABLE ROW LEVEL SECURITY; ALTER TABLE asset_models FORCE ROW LEVEL SECURITY; CREATE POLICY asset_models_blocked ON asset_models AS RESTRICTIVE FOR ALL TO app USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations())))) WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));";
    check(OrganizationTenancy::default(), schema, None);
    for (before, after, violation) in [
        ("ALTER TABLE asset_models ENABLE ROW LEVEL SECURITY;", "", "requires row-level security"),
        ("ALTER TABLE asset_models FORCE ROW LEVEL SECURITY;", "", "without forcing"),
        ("AS RESTRICTIVE", "AS PERMISSIVE", "restrictive _blocked policy"),
        ("FOR ALL", "FOR SELECT", "restrictive _blocked policy"),
        ("TO app", "TO unrelated", "restrictive _blocked policy"),
        ("asset_models_blocked", "different_policy", "restrictive _blocked policy"),
        (
            "NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations())))",
            "true",
            "restrictive _blocked policy",
        ),
    ] {
        check(OrganizationTenancy::default(), &schema.replace(before, after), Some(violation));
    }
    check(
        OrganizationTenancy::default(),
        "CREATE TABLE asset_models (id UUID PRIMARY KEY); ALTER TABLE asset_models ENABLE ROW LEVEL SECURITY; ALTER TABLE asset_models FORCE ROW LEVEL SECURITY;",
        Some("requires organization_id"),
    );
    check(
        OrganizationTenancy::default(),
        "CREATE TABLE slot_roles (id TEXT PRIMARY KEY); CREATE TABLE slot_role_pairs (parent_id TEXT REFERENCES slot_roles(id)); CREATE TABLE ownables (id UUID PRIMARY KEY);",
        None,
    );
    check(
        OrganizationTenancy::default(),
        &format!(
            "{schema} CREATE TABLE model_details (model_id UUID REFERENCES asset_models(id));"
        ),
        Some("requires organization_id"),
    );
}

#[test]
fn metadata_examples() {
    let rules: Vec<Box<dyn TableRule<Database = ParserDB>>> = vec![
        Box::new(ProcedureDescendantNaming::default()),
        Box::new(ProcedureTemplateDescendantNaming::default()),
        Box::new(MatchingProcedureAndTemplateTables::default()),
        Box::new(TypedTemplateSubtype::default()),
        Box::new(TypedProcedureSubtype::default()),
        Box::new(OrganizationTenancy::default()),
        Box::new(QuantityUnits::default()),
    ];
    for rule in rules {
        let metadata = rule.metadata();
        assert_eq!(metadata.group.default_level(), Level::Deny);
        let good = ParserDB::parse::<PostgreSqlDialect>(metadata.example.good).unwrap();
        let bad = ParserDB::parse::<PostgreSqlDialect>(metadata.example.bad).unwrap();
        let mut constrainer = GenericConstrainer::default();
        constrainer.register_table_rule(rule);
        constrainer.validate_schema(&good).unwrap();
        let error = constrainer.validate_schema(&bad).unwrap_err().to_string();
        assert!(error.contains(metadata.name), "{error}");
    }
}

#[test]
fn roots_and_unrelated_tables_are_not_subtypes() {
    let schema = format!("{ROOTS} CREATE TABLE unrelated (id UUID PRIMARY KEY, amount REAL);");
    check(ProcedureDescendantNaming::default(), &schema, None);
    check(ProcedureTemplateDescendantNaming::default(), &schema, None);
    check(MatchingProcedureAndTemplateTables::default(), &schema, None);
    check(TypedTemplateSubtype::default(), &schema, None);
    check(TypedProcedureSubtype::default(), &schema, None);
    check(QuantityUnits::default(), &schema, None);
}

#[test]
fn unrelated_matching_name_is_not_a_subtype() {
    check(
        MatchingProcedureAndTemplateTables::default(),
        &format!(
            "{ROOTS} CREATE TABLE weighing_procedures (id UUID PRIMARY KEY REFERENCES procedures(id)); CREATE TABLE weighing_procedure_templates (id UUID PRIMARY KEY);"
        ),
        Some("matching subtype table"),
    );
}

#[test]
fn slot_key_target_columns_are_ordered() {
    check(
        TypedTemplateSubtype::default(),
        &format!("{ROOTS}{SLOTS}{TEMPLATE}").replace(
            "procedure_template_slots(id, procedure_template_id, asset_model_id, role_id, organization_id)",
            "procedure_template_slots(procedure_template_id, id, asset_model_id, role_id, organization_id)",
        ),
        Some("slot ownership foreign key"),
    );
}

#[test]
fn status_key_target_columns_are_ordered() {
    check(
        TypedProcedureSubtype::default(),
        &format!("{ROOTS}{SLOTS}{TEMPLATE}{PROCEDURE}").replace(
            "procedures(id, procedure_template_id, status_id, organization_id)",
            "procedures(procedure_template_id, id, status_id, organization_id)",
        ),
        Some("status-copy foreign key"),
    );
}

#[test]
fn policy_requires_both_clauses() {
    let schema = "CREATE TABLE assets (id UUID PRIMARY KEY, organization_id UUID); ALTER TABLE assets ENABLE ROW LEVEL SECURITY; ALTER TABLE assets FORCE ROW LEVEL SECURITY; CREATE POLICY assets_blocked ON assets AS RESTRICTIVE FOR ALL TO app USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));";
    check(OrganizationTenancy::default(), schema, Some("restrictive _blocked policy"));
    check(
        OrganizationTenancy::default(),
        &schema.replace("USING", "WITH CHECK"),
        Some("restrictive _blocked policy"),
    );
}

#[test]
fn tenant_scope_terminates_on_unrelated_cycles() {
    check(
        OrganizationTenancy::default(),
        "CREATE TABLE first (id UUID PRIMARY KEY, second_id UUID); CREATE TABLE second (id UUID PRIMARY KEY, first_id UUID REFERENCES first(id)); ALTER TABLE first ADD FOREIGN KEY (second_id) REFERENCES second(id);",
        None,
    );
}
