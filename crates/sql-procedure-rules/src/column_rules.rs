//! Asset model column naming in typed procedure and template tables.

use sql_traits::traits::{ColumnLike, DatabaseLike, ForeignKeyLike};
use sqlippy::{
    error::Interruption,
    traits::{ColumnRule, Reporter},
};

use crate::table_rules::{descendant, named_root, typed};

declare_rule!(
    AssetModelColumnNaming,
    "Columns referencing asset models in typed subtypes end in `_model_id`, except archive companions.",
    "asset_model_column_naming",
    "CREATE TABLE asset_models (id INT PRIMARY KEY); CREATE TABLE procedure_templates (id INT PRIMARY KEY); CREATE TABLE typed (id INT PRIMARY KEY REFERENCES procedure_templates(id), tool_id INT REFERENCES asset_models(id));",
    "CREATE TABLE asset_models (id INT PRIMARY KEY); CREATE TABLE procedure_templates (id INT PRIMARY KEY); CREATE TABLE typed (id INT PRIMARY KEY REFERENCES procedure_templates(id), tool_model_id INT REFERENCES asset_models(id));"
);

impl<DB: DatabaseLike> ColumnRule for AssetModelColumnNaming<DB> {
    type Database = DB;
    fn check_column(
        &self,
        database: &DB,
        column: &DB::Column,
        reporter: &mut Reporter<'_, DB>,
    ) -> Result<(), Interruption> {
        if column.column_name().ends_with("_model_id") || !typed(database, column.table(database))?
        {
            return Ok(());
        }
        for foreign_key in column.foreign_keys(database)? {
            let referenced = foreign_key.referenced_table(database)?;
            if named_root(referenced, "asset_models")
                || descendant(database, referenced, "asset_models")?
            {
                if foreign_key
                    .referenced_column_for_host_column(database, column)?
                    .is_some_and(|referenced_column| referenced_column.column_name() == "archived")
                {
                    continue;
                }
                reporter.column(
                    column,
                    "Asset model column must end with _model_id.".into(),
                    "Rename the column with the _model_id suffix.".into(),
                )?;
                break;
            }
        }
        Ok(())
    }
}
