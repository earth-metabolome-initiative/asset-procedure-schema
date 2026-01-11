//! Submodule defining the `ProcedureTemplateNode` trait, which helps construct
//! the edges between procedure templates, including parent-child relationships.

use aps_next_procedure_templates::*;
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
    /// Creates a new parent-child relationship for procedure templates.
    ///
    /// # Arguments
    ///
    /// * `child_procedure_template`: The child procedure template to be added.
    /// * `user`: The user who is creating the procedure template relationship.
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

    /// Creates a sequential relationship between procedure templates.
    ///
    /// # Arguments
    ///
    /// * `predecessor`: The predecessor procedure template.
    /// * `successor`: The successor procedure template.
    /// * `user`: The user who is creating the relationship.
    /// * `conn`: The database connection to use for the insertion.
    ///
    /// # Errors
    ///
    /// * If the insertion fails, a `diesel::result::Error` is returned.
    ///
    /// # Example
    ///
    /// ```rust
    /// use aps_next_procedure_templates::*;
    /// use diesel::associations::Identifiable;
    /// use procedure_traits::ProcedureTemplateNode;
    /// let mut conn = aps_test_utils::aps_conn();
    /// let user = aps_test_utils::user(&mut conn);
    /// let parent_template = aps_test_utils::procedure_template("Parent Procedure", &user, &mut conn);
    /// let predecessor_template =
    ///     aps_test_utils::procedure_template("Predecessor Procedure", &user, &mut conn);
    /// let successor_template =
    ///     aps_test_utils::procedure_template("Successor Procedure", &user, &mut conn);
    /// let next_relation = parent_template
    ///     .append(&predecessor_template, &successor_template, &user, &mut conn)
    ///     .expect("Failed to create next procedure relationship");
    /// assert_eq!(next_relation.parent_id(), parent_template.id());
    /// assert_eq!(next_relation.predecessor_id(), predecessor_template.id());
    /// assert_eq!(next_relation.successor_id(), successor_template.id());
    /// ```
    fn append<C, L, R>(
        &self,
        predecessor: &L,
        successor: &R,
        user: impl HasTable<Table: DescendantOf<users::table>> + GetColumn<users::id>,
        conn: &mut C,
    ) -> Result<NextProcedureTemplate, diesel::result::Error>
    where
        L: GetColumn<procedure_templates::id> + ?Sized,
        R: GetColumn<procedure_templates::id> + ?Sized,
        TableBuilder<next_procedure_templates::table>: Insert<C>,
    {
        Ok(next_procedure_templates::table::builder()
            .try_parent_id(self.get_column())?
            .try_predecessor_id(predecessor.get_column())?
            .try_successor_id(successor.get_column())?
            .creator_id(user.get_column())
            .insert(conn)?)
    }

    /// Creates sequential relationships between a series of procedure
    /// templates.
    ///
    /// # Arguments
    ///
    /// * `children`: The procedure templates to be chained sequentially.
    /// * `user`: The user who is creating the relationships.
    /// * `conn`: The database connection to use for the insertion.
    ///
    /// # Errors
    ///
    /// * If the insertion fails, a `diesel::result::Error` is returned.
    ///
    /// # Example
    ///
    /// ```rust
    /// use aps_next_procedure_templates::*;
    /// use diesel::associations::Identifiable;
    /// use diesel_builders::GetColumn;
    /// use procedure_traits::ProcedureTemplateNode;
    /// let mut conn = aps_test_utils::aps_conn();
    /// let user = aps_test_utils::user(&mut conn);
    /// let parent_template = aps_test_utils::procedure_template("Parent Procedure", &user, &mut conn);
    /// let child1 = aps_test_utils::procedure_template("Child 1", &user, &mut conn);
    /// let child2 = aps_test_utils::procedure_template("Child 2", &user, &mut conn);
    /// let child3 = aps_test_utils::procedure_template("Child 3", &user, &mut conn);
    /// let relations = parent_template
    ///     .extend([&child1, &child2, &child3], &user, &mut conn)
    ///     .expect("Failed to create sequential relationships");
    /// assert_eq!(relations.len(), 2);
    /// assert_eq!(relations[0].predecessor_id(), child1.id());
    /// assert_eq!(relations[0].successor_id(), child2.id());
    /// assert_eq!(relations[0].parent_id(), parent_template.id());
    /// assert_eq!(relations[1].predecessor_id(), child2.id());
    /// assert_eq!(relations[1].successor_id(), child3.id());
    /// assert_eq!(relations[1].parent_id(), parent_template.id());
    /// ```
    fn extend<I, C>(
        &self,
        children: I,
        user: impl HasTable<Table: DescendantOf<users::table>> + GetColumn<users::id>,
        conn: &mut C,
    ) -> Result<Vec<NextProcedureTemplate>, diesel::result::Error>
    where
        TableBuilder<next_procedure_templates::table>: Insert<C>,
        I: IntoIterator<Item: GetColumn<procedure_templates::id>>,
    {
        let mut previous: Option<Box<dyn GetColumn<procedure_templates::id>>> = None;
        let mut relations = Vec::new();
        for child in children {
            if let Some(prev) = previous {
                relations.push(self.append(prev.as_ref(), &child, &user, conn)?);
            }
            previous = Some(Box::new(child));
        }
        Ok(relations)
    }
}

impl<T> ProcedureTemplateNode for T where
    T: HasTable<Table: DescendantOf<procedure_templates::table>>
        + GetColumn<procedure_templates::id>
{
}
