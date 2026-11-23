use std::sync::LazyLock;

use sqlparser::{dialect::PostgreSqlDialect, parser::Parser};

use super::*;

declare_rule!(
    OrganizationTenancy,
    "APS roots and tables referencing them transitively carry `organization_id`, enabled and forced RLS and the restrictive `_blocked` policy for `app`.",
    "organization_tenancy",
    "CREATE TABLE asset_models (id INT PRIMARY KEY);",
    "CREATE ROLE app; CREATE TABLE asset_models (id INT PRIMARY KEY, organization_id UUID); ALTER TABLE asset_models ENABLE ROW LEVEL SECURITY; ALTER TABLE asset_models FORCE ROW LEVEL SECURITY; CREATE POLICY asset_models_blocked ON asset_models AS RESTRICTIVE FOR ALL TO app USING (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations())))) WITH CHECK (NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations()))));"
);

fn tenant_table<DB: DatabaseLike>(database: &DB, table: &DB::Table) -> Result<bool, LookupError> {
    let mut visited = vec![table];
    let mut index = 0;
    while let Some(current) = visited.get(index).copied() {
        if ["asset_models", "assets", PROCEDURES_TABLE_NAME, PROCEDURE_TEMPLATES_TABLE_NAME]
            .iter()
            .any(|name| named_root(current, name))
        {
            return Ok(true);
        }
        for foreign_key in current.foreign_keys(database)? {
            let parent = foreign_key.referenced_table(database)?;
            if !visited.contains(&parent) {
                visited.push(parent);
            }
        }
        index += 1;
    }
    Ok(false)
}

static BLOCKED_EXPRESSION: LazyLock<Expr> = LazyLock::new(|| {
    Parser::new(&PostgreSqlDialect {})
        .try_with_sql("NOT (organization_id = ANY (ARRAY(SELECT authz.blocking_organizations())))")
        .expect("Valid blocked policy expression")
        .parse_expr()
        .expect("Valid blocked policy expression")
});

impl<DB: DatabaseLike> TableRule for OrganizationTenancy<DB> {
    type Database = DB;
    fn check_table(
        &self,
        database: &DB,
        table: &DB::Table,
        reporter: &mut Reporter<'_, DB>,
    ) -> Result<(), Interruption> {
        if !tenant_table(database, table)? {
            return Ok(());
        }
        if !has_column(database, table, "organization_id")? {
            reporter.table(
                table,
                "Tenant table requires organization_id.".into(),
                "Add the organization identity column.".into(),
            )?;
        }
        if !table.has_row_level_security(database)? {
            reporter.table(
                table,
                "Tenant table requires row-level security.".into(),
                "Enable row-level security on the table.".into(),
            )?;
        }
        sqlippy::rules::UnforcedRowLevelSecurity::<DB>::default()
            .check_table(database, table, reporter)?;
        let policy_name = format!("{}_blocked", table.table_name());
        let mut blocked = false;
        for policy in table.policies(database)? {
            if policy.name() == policy_name
                && policy.policy_type() == CreatePolicyType::Restrictive
                && policy.command() == CreatePolicyCommand::All
                && policy
                    .roles(database)
                    .any(|role| matches!(role, Owner::Ident(name) if name.value == "app"))
                && policy.written_using(database)? == Some(&*BLOCKED_EXPRESSION)
                && policy.written_check(database)? == Some(&*BLOCKED_EXPRESSION)
            {
                blocked = true;
                break;
            }
        }
        if !blocked {
            reporter.table(table, "Tenant table requires the restrictive _blocked policy for all app commands.".into(), "Use organization_id and authz.blocking_organizations() in both USING and WITH CHECK.".into())?;
        }
        Ok(())
    }
}
