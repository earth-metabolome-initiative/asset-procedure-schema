//! Auto-generated crate for the `procedure_tables` table.
#[derive(
    Clone,
    Default,
    Debug,
    Hash,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    :: serde :: Serialize,
    :: serde :: Deserialize,
    :: diesel :: Queryable,
    :: diesel :: Selectable,
    :: diesel :: Identifiable,
    :: diesel_builders :: prelude :: TableModel,
)]
/// and facilitate DAG traversal.
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (table_name = procedure_tables)]
pub struct ProcedureTable {
    /// and facilitate DAG traversal.
    id: String,
}
impl ::diesel_builders::ValidateColumn<procedure_tables::id>
    for <procedure_tables::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column(id: &String) -> Result<(), Self::Error> {
        use diesel::Column;
        if id.is_empty() {
            return Err(::validation_errors::ValidationError::empty(
                "procedure_tables",
                crate::procedure_tables::id::NAME,
            ));
        }
        Ok(())
    }
}
