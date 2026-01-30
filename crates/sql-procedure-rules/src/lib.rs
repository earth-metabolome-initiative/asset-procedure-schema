//! Crate providing rules to validate tables in the `procedures` and
//! `procedure_templates` DAGs.

use sql_rules::traits::Constrainer;

pub mod column_rules;
pub mod table_rules;

/// Prelude module re-exporting commonly used items from the crate.
pub mod prelude {
    pub use sql_rules::prelude::*;

    pub use crate::{column_rules::*, table_rules::*};
}

/// Function to register all procedure-related constraints to a given
/// constrainer.
pub fn register_procedure_constraints<DB: sql_traits::traits::DatabaseLike + 'static>(
    constrainer: &mut impl Constrainer<Database = DB>,
) {
    constrainer.register_table_rule(Box::new(
        table_rules::MatchingProcedureAndTemplateTables::<DB>::default(),
    ));
    constrainer
        .register_table_rule(Box::new(table_rules::ProcedureDescendantNaming::<DB>::default()));
    constrainer.register_table_rule(Box::new(
        table_rules::ProcedureTemplateAssetModelUniqueIndex::<DB>::default(),
    ));
    constrainer.register_table_rule(Box::new(
        table_rules::ReusedProcedureTemplateAssetModelsForeignKey::<DB>::default(),
    ));
    constrainer.register_table_rule(Box::new(
        table_rules::ReusedProcedureTemplateAssetModelsTrigger::<DB>::default(),
    ));
    constrainer.register_table_rule(Box::new(
        table_rules::ProcedureTemplateDescendantNaming::<DB>::default(),
    ));
    constrainer.register_table_rule(Box::new(table_rules::UpdateTimestampTrigger::<DB>::default()));

    constrainer
        .register_column_rule(Box::new(column_rules::AssetColumnNaming::<DB::Column>::default()));
    constrainer.register_column_rule(Box::new(
        column_rules::AssetModelColumnNaming::<DB::Column>::default(),
    ));
    constrainer.register_column_rule(Box::new(column_rules::HorizontalAssetModelForeignKey::<
        DB::Column,
    >::default()));
    constrainer.register_column_rule(Box::new(
        column_rules::MatchingAssetModelColumns::<DB::Column>::default(),
    ));
    constrainer.register_column_rule(Box::new(
        column_rules::ProcedureTemplateAssetModelColumnNaming::<DB::Column>::default(),
    ));
}
