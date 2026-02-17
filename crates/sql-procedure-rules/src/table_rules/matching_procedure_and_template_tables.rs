//! Submodule providing the `MatchingProcedureAndTemplateTables` constraint,
//! which enforces that for every `{t}_procedures` table, there must exist a
//! corresponding `{t}_procedure_templates` table.

use sql_rules::{error::RuleErrorInfo, prelude::*};
use sql_traits::traits::{DatabaseLike, TableLike};

use super::{
    procedure_descendant_naming::PROCEDURES_TABLE_NAME,
    procedure_template_descendant_naming::PROCEDURE_TEMPLATES_TABLE_NAME,
};

/// Struct defining a constraint that enforces that for every `{t}_procedures`
/// table, there must exist a corresponding `{t}_procedure_templates` table.
///
/// # Example
///
/// Here follows an example of validating an invalid SQL statement with the
/// `MatchingProcedureAndTemplateTables` constraint.
///
/// ```rust
/// use sql_procedure_rules::prelude::*;
///
/// let constrainer: GenericConstrainer<ParserDB> =
///     MatchingProcedureAndTemplateTables::default().into();
///
/// // Invalid: freezing_procedures exists but freezing_procedure_templates doesn't
/// let invalid_schema = ParserDB::parse::<sqlparser::dialect::GenericDialect>(
///     r#"
/// CREATE TABLE procedures (id INT PRIMARY KEY);
/// CREATE TABLE procedure_templates (id INT PRIMARY KEY);
/// CREATE TABLE freezing_procedures (id INT PRIMARY KEY REFERENCES procedures(id));
/// "#,
/// )
/// .unwrap();
/// assert!(constrainer.validate_schema(&invalid_schema).is_err());
///
/// // Valid: both freezing_procedures and freezing_procedure_templates exist
/// let valid_schema = ParserDB::parse::<sqlparser::dialect::GenericDialect>(
///     r#"
/// CREATE TABLE procedures (id INT PRIMARY KEY);
/// CREATE TABLE procedure_templates (id INT PRIMARY KEY);
/// CREATE TABLE freezing_procedures (id INT PRIMARY KEY REFERENCES procedures(id));
/// CREATE TABLE freezing_procedure_templates (id INT PRIMARY KEY REFERENCES procedure_templates(id));
/// "#,
/// )
/// .unwrap();
/// assert!(constrainer.validate_schema(&valid_schema).is_ok());
///
/// // Valid: the base procedures and procedure_templates tables don't need matches
/// let valid_base_schema = ParserDB::parse::<sqlparser::dialect::GenericDialect>(
///     r#"
/// CREATE TABLE procedures (id INT PRIMARY KEY);
/// CREATE TABLE procedure_templates (id INT PRIMARY KEY);
/// "#,
/// )
/// .unwrap();
/// assert!(constrainer.validate_schema(&valid_base_schema).is_ok());
/// ```
pub struct MatchingProcedureAndTemplateTables<DB>(std::marker::PhantomData<DB>);

impl<DB> Default for MatchingProcedureAndTemplateTables<DB> {
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<DB> From<MatchingProcedureAndTemplateTables<DB>> for Box<dyn TableRule<Database = DB>>
where
    DB: DatabaseLike + 'static,
{
    fn from(constraint: MatchingProcedureAndTemplateTables<DB>) -> Self {
        Box::new(constraint)
    }
}

impl<DB: DatabaseLike + 'static> From<MatchingProcedureAndTemplateTables<DB>>
    for GenericConstrainer<DB>
{
    fn from(constraint: MatchingProcedureAndTemplateTables<DB>) -> Self {
        let mut constrainer = GenericConstrainer::default();
        constrainer.register_table_rule(Box::new(constraint));
        constrainer
    }
}

impl<DB: DatabaseLike> TableRule for MatchingProcedureAndTemplateTables<DB> {
    type Database = DB;

    fn validate_table(
        &self,
        database: &Self::Database,
        table: &<Self::Database as DatabaseLike>::Table,
    ) -> Result<(), Error<Self::Database>> {
        let Some(procedure_templates) = database.table(None, PROCEDURE_TEMPLATES_TABLE_NAME) else {
            return Err(Error::Unapplicable(format!(
                "Table '{}' does not exist",
                PROCEDURE_TEMPLATES_TABLE_NAME
            )));
        };
        let Some(procedures) = database.table(None, PROCEDURES_TABLE_NAME) else {
            return Err(Error::Unapplicable(format!(
                "Table '{}' does not exist",
                PROCEDURES_TABLE_NAME
            )));
        };

        if table == procedures || table == procedure_templates {
            return Ok(());
        }

        if !table.is_descendant_of(database, procedure_templates)
            && !table.is_descendant_of(database, procedures)
        {
            return Ok(());
        }

        let table_name = table.table_name();

        if let Some(prefix) = table_name.strip_suffix("_procedures") {
            let expected_template_table_name = format!("{}_procedure_templates", prefix);
            if database.table(None, &expected_template_table_name).is_none() {
                let info: RuleErrorInfo = RuleErrorInfo::builder()
                    .rule("MatchingProcedureAndTemplateTables")
                    .unwrap()
                    .object(table.table_name().to_owned())
                    .unwrap()
                    .message(format!(
                        "Table '{}' is a procedures table without corresponding procedure_templates table - expected table '{}' to exist",
                        table_name, expected_template_table_name
                    ))
                    .unwrap()
                    .resolution(format!(
                        "Create the matching table '{}' that corresponds to '{}'",
                        expected_template_table_name, table_name
                    ))
                    .unwrap()
                    .try_into()
                    .unwrap();
                return Err(Error::Table(Box::new(table.clone()), info.into()));
            }
        } else if let Some(prefix) = table_name.strip_suffix("_procedure_templates") {
            let expected_procedure_table_name = format!("{}_procedures", prefix);
            if database.table(None, &expected_procedure_table_name).is_none() {
                let info: RuleErrorInfo = RuleErrorInfo::builder()
                    .rule("MatchingProcedureAndTemplateTables")
                    .unwrap()
                    .object(table.table_name().to_owned())
                    .unwrap()
                    .message(format!(
                        "Table '{}' is a procedure_templates table without corresponding procedures table - expected table '{}' to exist",
                        table_name, expected_procedure_table_name
                    ))
                    .unwrap()
                    .resolution(format!(
                        "Create the matching table '{}' that corresponds to '{}'",
                        expected_procedure_table_name, table_name
                    ))
                    .unwrap()
                    .try_into()
                    .unwrap();
                return Err(Error::Table(Box::new(table.clone()), info.into()));
            }
        }

        Ok(())
    }
}
