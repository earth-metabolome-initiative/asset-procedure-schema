//! Auto-generated crate for the `procedure_template_tables` table.
#[derive(
    Clone,
    Default,
    Debug,
    Hash,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    serde :: Serialize,
    serde :: Deserialize,
    diesel :: Queryable,
    diesel :: Selectable,
    diesel :: Identifiable,
    diesel_builders :: prelude :: TableModel,
)]
/// Undocumented table
# [diesel (table_name = procedure_template_tables)]
pub struct ProcedureTemplateTable {
    /// Undocumented column
    id: String,
}
