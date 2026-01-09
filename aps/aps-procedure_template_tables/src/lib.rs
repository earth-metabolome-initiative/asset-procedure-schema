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
/// Struct representing a row in the `procedure_template_tables` table.
# [diesel (table_name = procedure_template_tables)]
pub struct ProcedureTemplateTable {
    /// Field representing the `id` column in table `procedure_template_tables`.
    id: String,
}
