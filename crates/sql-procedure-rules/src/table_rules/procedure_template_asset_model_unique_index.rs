//! Submodule providing the `ProcedureTemplateAssetModelUniqueIndex` constraint,
//! which enforces that for each column `procedure_template_{am}` in a procedure
//! template table, there must exist a UNIQUE index on (id,
//! procedure_template_{am}).

use sql_rules::{error::RuleErrorInfo, prelude::*};
use sql_traits::traits::{DatabaseLike, TableLike};

use crate::{
    column_rules::PROCEDURE_TEMPLATE_ASSET_MODELS_TABLE_NAME,
    table_rules::PROCEDURE_TEMPLATES_TABLE_NAME,
};

/// Struct defining a constraint that enforces unique indexes for procedure
/// template asset model columns.
///
/// # Example
///
/// ```rust
/// use sql_procedure_rules::prelude::*;
///
/// let constrainer: GenericConstrainer<ParserDB> =
///     ProcedureTemplateAssetModelUniqueIndex::default().into();
///
/// // Invalid: has procedure_template_container_model but no UNIQUE (id, procedure_template_container_model)
/// let invalid_schema = ParserDB::parse::<sqlparser::dialect::GenericDialect>(
///     r#"
/// CREATE TABLE procedure_templates (id INT PRIMARY KEY);
/// CREATE TABLE procedure_template_asset_models (id INT PRIMARY KEY);
/// CREATE TABLE freezing_procedure_templates (
///     id INT PRIMARY KEY REFERENCES procedure_templates(id),
///     procedure_template_container_model INT REFERENCES procedure_template_asset_models(id)
/// );
/// "#,
/// )
/// .unwrap();
/// assert!(constrainer.validate_schema(&invalid_schema).is_err());
///
/// // Valid: has UNIQUE (id, procedure_template_container_model)
/// let valid_schema = ParserDB::parse::<sqlparser::dialect::GenericDialect>(
///     r#"
/// CREATE TABLE procedure_templates (id INT PRIMARY KEY);
/// CREATE TABLE procedure_template_asset_models (id INT PRIMARY KEY);
/// CREATE TABLE freezing_procedure_templates (
///     id INT PRIMARY KEY REFERENCES procedure_templates(id),
///     procedure_template_container_model INT REFERENCES procedure_template_asset_models(id),
///     UNIQUE (id, procedure_template_container_model)
/// );
/// "#,
/// )
/// .unwrap();
/// assert!(constrainer.validate_schema(&valid_schema).is_ok());
/// ```
pub struct ProcedureTemplateAssetModelUniqueIndex<DB>(std::marker::PhantomData<DB>);

impl<DB> Default for ProcedureTemplateAssetModelUniqueIndex<DB> {
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<DB> From<ProcedureTemplateAssetModelUniqueIndex<DB>> for Box<dyn TableRule<Database = DB>>
where
    DB: DatabaseLike + 'static,
{
    fn from(constraint: ProcedureTemplateAssetModelUniqueIndex<DB>) -> Self {
        Box::new(constraint)
    }
}

impl<DB: DatabaseLike + 'static> From<ProcedureTemplateAssetModelUniqueIndex<DB>>
    for GenericConstrainer<DB>
{
    fn from(constraint: ProcedureTemplateAssetModelUniqueIndex<DB>) -> Self {
        let mut constrainer = GenericConstrainer::default();
        constrainer.register_table_rule(Box::new(constraint));
        constrainer
    }
}

impl<DB: DatabaseLike> TableRule for ProcedureTemplateAssetModelUniqueIndex<DB> {
    type Database = DB;

    fn validate_table(
        &self,
        database: &Self::Database,
        table: &<Self::Database as DatabaseLike>::Table,
    ) -> Result<(), Error<Self::Database>> {
        // Skip validation if required tables don't exist
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

        // Find all columns that reference procedure_template_asset_models
        let procedure_template_asset_model_columns: Vec<_> = table
            .columns(database)
            .filter(|col| {
                col.column_name().starts_with("procedure_template_")
                    && table
                        .referenced_tables_via_column(database, col)
                        .contains(&procedure_template_asset_models_table)
            })
            .collect();

        // For each such column, verify there's a UNIQUE (id, procedure_template_{am})
        // index
        for column in procedure_template_asset_model_columns {
            let has_unique_index = table.unique_indices(database).any(|idx| {
                let idx_columns: Vec<_> = idx.columns(database).collect();
                idx_columns.len() == 2
                    && idx_columns.iter().any(|&c| c.column_name() == "id")
                    && idx_columns.iter().any(|&c| c.column_name() == column.column_name())
            });

            if !has_unique_index {
                let table_name = table.table_name();
                let column_name = column.column_name();

                let info: RuleErrorInfo = RuleErrorInfo::builder()
                    .rule("ProcedureTemplateAssetModelUniqueIndex")
                    .unwrap()
                    .object(table_name.to_owned())
                    .unwrap()
                    .message(format!(
                        "Procedure template table '{}' has column(s) referencing \
                        procedure_template_asset_models but missing required UNIQUE index on (id, {})",
                        table_name, column_name
                    ))
                    .unwrap()
                    .resolution(format!("Add unique index: UNIQUE (id, {})", column_name))
                    .unwrap()
                    .try_into()
                    .unwrap();
                return Err(Error::Table(Box::new(table.clone()), info.into()));
            }
        }

        Ok(())
    }
}
