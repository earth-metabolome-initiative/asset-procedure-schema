//! Crate defining the `ProcedureLike` trait, which is implemented by items that
//! behave like procedures.

use diesel_builders::{
    DescendantWithSelf, NestedTables, TableExt, TryGetDynamicColumn, TypedColumn,
};

/// Trait for items that behave like procedures.
pub trait ProcedureTableLike:
    TableExt<
        PrimaryKey: TypedColumn<
            Table = Self,
            ColumnType = rosetta_uuid::Uuid,
            ValueType = rosetta_uuid::Uuid,
        >,
    > + DescendantWithSelf<NestedAncestorsWithSelf: NestedTables<NestedModels: TryGetDynamicColumn>>
{
    /// The type defining a procedure template associated with this
    /// procedure-like item.
    type ProcedureTemplateTable: diesel::Table<
            PrimaryKey: TypedColumn<
                Table = Self::ProcedureTemplateTable,
                ColumnType = rosetta_uuid::Uuid,
                ValueType = rosetta_uuid::Uuid,
            >,
        > + TableExt
        + DescendantWithSelf
        + diesel_builders::ancestors::DescendantOfAll<(
            <<Self::ProcedureTemplateTable as diesel::Table>::PrimaryKey as diesel::Column>::Table,
        )>;
}
