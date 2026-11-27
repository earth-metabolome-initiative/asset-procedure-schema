#![doc = include_str!("../README.md")]

use sqlippy::traits::Constrainer;

macro_rules! declare_rule {
    ($name:ident, $description:literal, $lint:literal, $bad:expr, $good:expr) => {
        #[doc = $description]
        pub struct $name<DB>(std::marker::PhantomData<DB>);

        impl<DB> Default for $name<DB> {
            fn default() -> Self {
                Self(std::marker::PhantomData)
            }
        }

        impl<DB> sqlippy::lint::Rule for $name<DB> {
            fn metadata(&self) -> &'static sqlippy::lint::RuleMetadata {
                static METADATA: sqlippy::lint::RuleMetadata = sqlippy::lint::RuleMetadata {
                    name: $lint,
                    group: sqlippy::lint::LintGroup::Correctness,
                    rationale: $description,
                    source: None,
                    basis: sqlippy::lint::Basis::Declared,
                    example: sqlippy::lint::RuleExample { bad: $bad, good: $good },
                };
                &METADATA
            }
        }
    };
}

pub mod column_rules;
pub mod table_rules;

/// Common rule and constrainer imports.
pub mod prelude {
    pub use sqlippy::prelude::*;

    pub use crate::{column_rules::*, register_procedure_constraints, table_rules::*};
}

/// Registers the APS subtype, naming, quantity and tenancy rules at their
/// default deny level.
pub fn register_procedure_constraints<DB: sql_traits::traits::DatabaseLike + 'static>(
    constrainer: &mut impl Constrainer<Database = DB>,
) {
    constrainer
        .register_table_rule(Box::new(table_rules::ProcedureDescendantNaming::<DB>::default()));
    constrainer.register_table_rule(Box::new(
        table_rules::ProcedureTemplateDescendantNaming::<DB>::default(),
    ));
    constrainer.register_table_rule(Box::new(
        table_rules::MatchingProcedureAndTemplateTables::<DB>::default(),
    ));
    constrainer.register_table_rule(Box::new(table_rules::TypedTemplateSubtype::<DB>::default()));
    constrainer.register_table_rule(Box::new(table_rules::TypedProcedureSubtype::<DB>::default()));
    constrainer.register_table_rule(Box::new(table_rules::OrganizationTenancy::<DB>::default()));
    constrainer.register_table_rule(Box::new(table_rules::QuantityUnits::<DB>::default()));
    constrainer
        .register_column_rule(Box::new(column_rules::AssetModelColumnNaming::<DB>::default()));
}
