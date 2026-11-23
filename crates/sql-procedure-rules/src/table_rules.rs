//! APS subtype, quantity and organization contracts.

use sql_traits::{
    errors::LookupError,
    traits::{
        ColumnLike, DataTypeLike, DatabaseLike, ForeignKeyLike, PolicyLike, TableLike, TriggerLike,
    },
};
use sqlippy::{
    error::Interruption,
    traits::{Reporter, TableRule},
};
use sqlparser::ast::{CreatePolicyCommand, CreatePolicyType, Expr, Owner, ReferentialAction};

/// The root procedure table.
pub const PROCEDURES_TABLE_NAME: &str = "procedures";
/// The root procedure template table.
pub const PROCEDURE_TEMPLATES_TABLE_NAME: &str = "procedure_templates";

mod naming;
pub use naming::{
    MatchingProcedureAndTemplateTables, ProcedureDescendantNaming,
    ProcedureTemplateDescendantNaming,
};
mod subtypes;
pub use subtypes::{TypedProcedureSubtype, TypedTemplateSubtype};
mod tenancy;
pub use tenancy::OrganizationTenancy;
mod quantities;
pub use quantities::QuantityUnits;

pub(crate) fn named_root<T: TableLike>(table: &T, name: &str) -> bool {
    table.table_name() == name && matches!(table.table_schema(), None | Some("public"))
}

pub(crate) fn descendant<DB: DatabaseLike>(
    database: &DB,
    table: &DB::Table,
    root: &str,
) -> Result<bool, LookupError> {
    let Some(parent) = database.tables().find(|candidate| named_root(*candidate, root)) else {
        return Ok(false);
    };
    Ok(table != parent && table.is_descendant_of(database, parent)?)
}

pub(crate) fn typed<DB: DatabaseLike>(
    database: &DB,
    table: &DB::Table,
) -> Result<bool, LookupError> {
    Ok(descendant(database, table, PROCEDURES_TABLE_NAME)?
        || descendant(database, table, PROCEDURE_TEMPLATES_TABLE_NAME)?)
}

fn has_column<DB: DatabaseLike>(
    database: &DB,
    table: &DB::Table,
    name: &str,
) -> Result<bool, LookupError> {
    Ok(table.columns(database)?.any(|column| column.column_name() == name))
}

fn key_matches<DB: DatabaseLike>(
    database: &DB,
    foreign_key: &DB::ForeignKey,
    target: &str,
    host_columns: &[&str],
    target_columns: &[&str],
) -> Result<bool, LookupError> {
    Ok(named_root(foreign_key.referenced_table(database)?, target)
        && foreign_key
            .host_columns(database)?
            .map(ColumnLike::column_name)
            .eq(host_columns.iter().copied())
        && foreign_key
            .referenced_columns(database)?
            .map(ColumnLike::column_name)
            .eq(target_columns.iter().copied()))
}

fn guard<DB: DatabaseLike>(
    database: &DB,
    table: &DB::Table,
    function: &str,
    reporter: &mut Reporter<'_, DB>,
) -> Result<(), Interruption> {
    if !table.triggers(database)?.any(|trigger| trigger.function_name() == Some(function)) {
        reporter.table(
            table,
            format!("Table requires a trigger executing {function}()."),
            "Add the subtype guard trigger.".into(),
        )?;
    }
    Ok(())
}
