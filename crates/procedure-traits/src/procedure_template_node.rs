//! Submodule defining the `ProcedureTemplateNode` trait, which helps construct
//! the edges between procedure templates, including parent-child relationships.

use aps_parent_procedure_templates::*;
use aps_procedure_templates::procedure_templates;
use aps_users::users;
use diesel::associations::HasTable;
use diesel_builders::{BuildableTable, DescendantOf, GetColumn, Insert, TableBuilder};

/// Trait defining the methods for managing parent-child relationships in
/// procedure templates.
pub trait ProcedureTemplateNode:
    HasTable<Table: DescendantOf<procedure_templates::table>> + GetColumn<procedure_templates::id>
{
    /// Creates a new parent-child relationship for a procedure.
    ///
    /// # Arguments
    ///
    /// * `child_procedure`: The child procedure template to be added.
    /// * `user`: The user who is creating the tool category.
    /// * `conn`: The database connection to use for the insertion.
    ///
    /// # Errors
    ///
    /// * If the insertion fails, a `diesel::result::Error` is returned.
    ///
    /// # Example
    ///
    /// ```rust
    /// use aps_parent_procedure_templates::*;
    /// use diesel::associations::Identifiable;
    /// use procedure_traits::ProcedureTemplateNode;
    /// let mut conn = aps_test_utils::aps_conn();
    /// let user = aps_test_utils::user(&mut conn);
    /// let parent_template = aps_test_utils::procedure_template("Parent Procedure", &user, &mut conn);
    /// let child_template = aps_test_utils::procedure_template("Child Procedure", &user, &mut conn);
    /// let parent_child_relation = parent_template
    ///     .child(&child_template, &user, &mut conn)
    ///     .expect("Failed to create parent-child relationship");
    /// assert_eq!(parent_child_relation.parent_id(), parent_template.id());
    /// assert_eq!(parent_child_relation.child_id(), child_template.id());
    /// ```
    fn child<C>(
        &self,
        child_procedure_template: impl HasTable<Table: DescendantOf<procedure_templates::table>>
        + GetColumn<procedure_templates::id>,
        user: impl HasTable<Table: DescendantOf<users::table>> + GetColumn<users::id>,
        conn: &mut C,
    ) -> Result<ParentProcedureTemplate, diesel::result::Error>
    where
        TableBuilder<parent_procedure_templates::table>: Insert<C>,
    {
        Ok(parent_procedure_templates::table::builder()
            .try_parent_id(self.get_column())?
            .try_child_id(child_procedure_template.get_column())?
            .creator_id(user.get_column())
            .insert(conn)?)
    }
}

impl<T> ProcedureTemplateNode for T where
    T: HasTable<Table: DescendantOf<procedure_templates::table>>
        + GetColumn<procedure_templates::id>
{
}
