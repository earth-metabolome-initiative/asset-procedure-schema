//! Auto-generated crate for the `procedure_templates` table.
#[derive(
    Copy,
    Clone,
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
    :: diesel :: Associations,
    :: diesel_builders :: prelude :: TableModel,
)]
/// Struct representing a row in the `procedure_templates` table.
#[table_model(ancestors(
    aps_entities::entities,
    aps_ownables::ownables,
    aps_namespaced_ownables::namespaced_ownables
))]
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (belongs_to (aps_namespaced_ownables :: NamespacedOwnable , foreign_key = id))]
# [table_model (foreign_key ((id ,) , (:: aps_namespaced_ownables :: namespaced_ownables :: id)))]
#[table_model(default(aps_entities::entities::table_name_id, "procedure_templates"))]
# [diesel (table_name = procedure_templates)]
pub struct ProcedureTemplate {
    /// Identifier of the procedure_id template
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Version of the procedure_id template.
    #[table_model(default = 1i32)]
    #[infallible]
    version: i32,
    /// Whether this procedure_id template is deprecated and should not be used
    /// for new procedures
    #[table_model(default = false)]
    #[infallible]
    deprecated: bool,
}
impl ::diesel_builders::GetColumn<aps_entities::entities::id> for ProcedureTemplate {
    fn get_column_ref(
        &self,
    ) -> &<procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_namespaced_ownables::namespaced_ownables::id>
    for ProcedureTemplate
{
    fn get_column_ref(
        &self,
    ) -> &<procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_ownables::ownables::id> for ProcedureTemplate {
    fn get_column_ref(
        &self,
    ) -> &<procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
