//! Submodule providing the `ReusedProcedureTemplateAssetModelsForeignKey`
//! constraint, which enforces that for each column `procedure_template_{am}` in
//! a procedure template table, there must exist a foreign key to
//! `reused_procedure_template_asset_models` on (id, procedure_template_{am}).

use sql_rules::{error::RuleErrorInfo, prelude::*};
use sql_traits::traits::{ColumnLike, DatabaseLike, ForeignKeyLike, TableLike};

use crate::{
    column_constraints::PROCEDURE_TEMPLATE_ASSET_MODELS_TABLE_NAME,
    table_constraints::PROCEDURE_TEMPLATES_TABLE_NAME,
};

pub const REUSED_PROCEDURE_TEMPLATE_ASSET_MODELS_TABLE_NAME: &str =
    "reused_procedure_template_asset_models";

/// Struct defining a constraint that enforces foreign keys to reused procedure
/// template asset models.
///
/// # Example
///
/// ```rust
/// use sql_procedure_rules::prelude::*;
///
/// let constrainer: GenericConstrainer<ParserDB> =
///     ReusedProcedureTemplateAssetModelsForeignKey::default().into();
///
/// // Invalid: has procedure_template_container_model but no FK to reused_procedure_template_asset_models
/// let invalid_schema = ParserDB::try_from(
///     r#"
/// CREATE TABLE procedure_templates (id INT PRIMARY KEY);
/// CREATE TABLE procedure_template_asset_models (id INT PRIMARY KEY);
/// CREATE TABLE reused_procedure_template_asset_models (
///     procedure_template_id INT REFERENCES procedure_templates(id),
///     procedure_template_asset_model_id INT REFERENCES procedure_template_asset_models(id),
///     PRIMARY KEY (procedure_template_id, procedure_template_asset_model_id)
/// );
/// CREATE TABLE freezing_procedure_templates (
///     id INT PRIMARY KEY REFERENCES procedure_templates(id),
///     procedure_template_container_model INT REFERENCES procedure_template_asset_models(id)
/// );
/// "#,
/// )
/// .unwrap();
/// assert!(constrainer.validate_schema(&invalid_schema).is_err());
///
/// // Valid: has FK to reused_procedure_template_asset_models
/// let valid_schema = ParserDB::try_from(
///     r#"
/// CREATE TABLE procedure_templates (id INT PRIMARY KEY);
/// CREATE TABLE procedure_template_asset_models (id INT PRIMARY KEY);
/// CREATE TABLE reused_procedure_template_asset_models (
///     procedure_template_id INT REFERENCES procedure_templates(id),
///     procedure_template_asset_model_id INT REFERENCES procedure_template_asset_models(id),
///     PRIMARY KEY (procedure_template_id, procedure_template_asset_model_id)
/// );
/// CREATE TABLE freezing_procedure_templates (
///     id INT PRIMARY KEY REFERENCES procedure_templates(id),
///     procedure_template_container_model INT REFERENCES procedure_template_asset_models(id),
///     FOREIGN KEY (id, procedure_template_container_model)
///         REFERENCES reused_procedure_template_asset_models(procedure_template_id, procedure_template_asset_model_id)
/// );
/// "#,
/// )
/// .unwrap();
/// assert!(constrainer.validate_schema(&valid_schema).is_ok());
///
/// // Invalid: has FK to reused_procedure_template_asset_models but no FK to procedure_template_asset_models
/// let invalid_schema_reverse = ParserDB::try_from(
///     r#"
/// CREATE TABLE procedure_templates (id INT PRIMARY KEY);
/// CREATE TABLE procedure_template_asset_models (id INT PRIMARY KEY);
/// CREATE TABLE reused_procedure_template_asset_models (
///     procedure_template_id INT REFERENCES procedure_templates(id),
///     procedure_template_asset_model_id INT REFERENCES procedure_template_asset_models(id),
///     PRIMARY KEY (procedure_template_id, procedure_template_asset_model_id)
/// );
/// CREATE TABLE freezing_procedure_templates (
///     id INT PRIMARY KEY REFERENCES procedure_templates(id),
///     procedure_template_container_model INT,
///     FOREIGN KEY (id, procedure_template_container_model)
///         REFERENCES reused_procedure_template_asset_models(procedure_template_id, procedure_template_asset_model_id)
/// );
/// "#,
/// )
/// .unwrap();
/// assert!(constrainer.validate_schema(&invalid_schema_reverse).is_err());
/// ```
pub struct ReusedProcedureTemplateAssetModelsForeignKey<DB>(std::marker::PhantomData<DB>);

impl<DB> Default for ReusedProcedureTemplateAssetModelsForeignKey<DB> {
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<DB> From<ReusedProcedureTemplateAssetModelsForeignKey<DB>>
    for Box<dyn TableRule<Database = DB>>
where
    DB: DatabaseLike + 'static,
{
    fn from(constraint: ReusedProcedureTemplateAssetModelsForeignKey<DB>) -> Self {
        Box::new(constraint)
    }
}

impl<DB: DatabaseLike + 'static> From<ReusedProcedureTemplateAssetModelsForeignKey<DB>>
    for GenericConstrainer<DB>
{
    fn from(constraint: ReusedProcedureTemplateAssetModelsForeignKey<DB>) -> Self {
        let mut constrainer = GenericConstrainer::default();
        constrainer.register_table_rule(Box::new(constraint));
        constrainer
    }
}

impl<DB: DatabaseLike> TableRule for ReusedProcedureTemplateAssetModelsForeignKey<DB> {
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

        let Some(reused_table) =
            database.table(None, REUSED_PROCEDURE_TEMPLATE_ASSET_MODELS_TABLE_NAME)
        else {
            return Err(Error::Unapplicable(format!(
                "Table '{}' does not exist",
                REUSED_PROCEDURE_TEMPLATE_ASSET_MODELS_TABLE_NAME
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
                table
                    .referenced_tables_via_column(database, col)
                    .contains(&procedure_template_asset_models_table)
            })
            .collect();

        // For each of these columns, we must find a composite foreign key to
        // reused_procedure_template_asset_models
        for col in procedure_template_asset_model_columns {
            let col_name = col.column_name();

            // Check if there is a foreign key that satisfies the condition
            let has_reused_fk = table.foreign_keys(database).any(|fk| {
                let referenced_table = fk.referenced_table(database);
                if referenced_table != reused_table {
                    return false;
                }

                let host_columns: Vec<_> = fk.host_columns(database).collect();
                let referenced_columns: Vec<_> = fk.referenced_columns(database).collect();

                if host_columns.len() != 2 || referenced_columns.len() != 2 {
                    return false;
                }

                // We expect mappings:
                // id -> procedure_template_id
                // col_name -> procedure_template_asset_model_id

                let mut maps_id = false;
                let mut maps_col = false;

                for (local_col, ref_col) in host_columns.iter().zip(referenced_columns.iter()) {
                    if local_col.column_name() == "id"
                        && ref_col.column_name() == "procedure_template_id"
                    {
                        maps_id = true;
                    } else if local_col.column_name() == col_name
                        && ref_col.column_name() == "procedure_template_asset_model_id"
                    {
                        maps_col = true;
                    }
                }

                maps_id && maps_col
            });

            if !has_reused_fk {
                let table_name = table.table_name();
                let info: RuleErrorInfo = RuleErrorInfo::builder()
                    .rule("ReusedProcedureTemplateAssetModelsForeignKey")
                    .unwrap()
                    .object(table_name.to_owned())
                    .unwrap()
                    .message(format!(
                        "Table '{}' has column '{}' referencing '{}' but is missing a foreign key to '{}' on (id, {}).",
                        table_name,
                        col_name,
                        PROCEDURE_TEMPLATE_ASSET_MODELS_TABLE_NAME,
                        REUSED_PROCEDURE_TEMPLATE_ASSET_MODELS_TABLE_NAME,
                        col_name
                    ))
                    .unwrap()
                    .resolution(format!(
                        "Add foreign key: FOREIGN KEY (id, {}) REFERENCES {}(procedure_template_id, procedure_template_asset_model_id)",
                        col_name,
                        REUSED_PROCEDURE_TEMPLATE_ASSET_MODELS_TABLE_NAME
                    ))
                    .unwrap()
                    .try_into()
                    .unwrap();
                return Err(Error::Table(Box::new(table.clone()), info.into()));
            }
        }

        // Check consistent usage: if we reference
        // reused_procedure_template_asset_models, the involved column must also
        // reference procedure_template_asset_models.
        for fk in table.foreign_keys(database) {
            if fk.referenced_table(database) != reused_table {
                continue;
            }

            let host_columns: Vec<_> = fk.host_columns(database).collect();
            let referenced_columns: Vec<_> = fk.referenced_columns(database).collect();

            if host_columns.len() != 2 || referenced_columns.len() != 2 {
                continue;
            }

            for (local_col, ref_col) in host_columns.iter().zip(referenced_columns.iter()) {
                if ref_col.column_name() == "procedure_template_asset_model_id"
                    && !table
                        .referenced_tables_via_column(database, local_col)
                        .contains(&procedure_template_asset_models_table)
                {
                    let table_name = table.table_name();
                    let col_name = local_col.column_name();
                    let info: RuleErrorInfo = RuleErrorInfo::builder()
                            .rule("ReusedProcedureTemplateAssetModelsForeignKey")
                            .unwrap()
                            .object(table_name.to_owned())
                            .unwrap()
                            .message(format!(
                                "Table '{}' has FK to '{}' using column '{}' but validation failed because '{}' does not reference '{}'.",
                                table_name,
                                REUSED_PROCEDURE_TEMPLATE_ASSET_MODELS_TABLE_NAME,
                                col_name,
                                col_name,
                                PROCEDURE_TEMPLATE_ASSET_MODELS_TABLE_NAME,
                            ))
                            .unwrap()
                            .resolution(format!(
                                "Add foreign key: FOREIGN KEY ({}) REFERENCES {}(id)",
                                col_name,
                                PROCEDURE_TEMPLATE_ASSET_MODELS_TABLE_NAME
                            ))
                            .unwrap()
                            .try_into()
                            .unwrap();
                    return Err(Error::Table(Box::new(table.clone()), info.into()));
                }
            }
        }

        Ok(())
    }
}
