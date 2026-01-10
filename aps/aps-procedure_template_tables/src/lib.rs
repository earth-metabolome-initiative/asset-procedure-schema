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
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (table_name = procedure_template_tables)]
pub struct ProcedureTemplateTable {
    /// Field representing the `id` column in table `procedure_template_tables`.
    id: String,
}
impl ::diesel_builders::ValidateColumn<procedure_template_tables::id>
    for <procedure_template_tables::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column(id: &String) -> Result<(), Self::Error> {
        use diesel::Column;
        if id.is_empty() {
            return Err(::validation_errors::ValidationError::empty(
                "procedure_template_tables",
                crate::procedure_template_tables::id::NAME,
            ));
        }
        Ok(())
    }
}
