//! Submodule defining a function to recursively load the sub-procedure
//! templates of a given procedure template.

use std::rc::Rc;

use aps_parent_procedure_templates::*;
use aps_procedure_templates::*;
use diesel_builders::{GetColumnExt, LoadMany, NestedModel, TableModel, prelude::LoadNestedFirst};

#[allow(clippy::type_complexity)]
/// Recursively loads all sub-procedure templates of the given procedure
/// template, returning a vector of all loaded sub-procedure templates and a
/// vector of edges representing the parent-child relationships between them.
///
/// # Arguments
///
/// * `procedure_template` - The procedure template whose sub-procedure
///   templates are to be loaded.
/// * `conn` - The database connection to use for loading the sub-procedure
///   templates.
pub(super) fn load_subprocedure_templates<C>(
    procedure_template: &Rc<NestedModel<procedure_templates::table>>,
    conn: &mut C,
) -> Result<
    (
        Vec<Rc<NestedModel<procedure_templates::table>>>,
        Vec<(
            Rc<NestedModel<procedure_templates::table>>,
            Rc<NestedModel<procedure_templates::table>>,
        )>,
    ),
    diesel::result::Error,
>
where
    (procedure_templates::id,): LoadNestedFirst<procedure_templates::table, C>,
    (parent_procedure_templates::parent_id,): LoadMany<C>,
    ParentProcedureTemplate: FKParentProcedureTemplatesChildId<C>,
{
    let mut subprocedure_templates = Vec::new();
    let mut edges = Vec::new();
    let parent_child_relations = <(parent_procedure_templates::parent_id,)>::load_many(
        (*procedure_template.get_column::<procedure_templates::id>(),),
        conn,
    )?;

    for parent_child_relation in parent_child_relations {
        let child_procedure: Rc<NestedModel<procedure_templates::table>> =
            Rc::from(parent_child_relation.child(conn)?.nested(conn)?);
        let (child_subprocedure_templates, child_edges) =
            load_subprocedure_templates(&child_procedure, conn)?;
        subprocedure_templates.extend(child_subprocedure_templates);
        edges.push((procedure_template.clone(), child_procedure.clone()));
        subprocedure_templates.push(child_procedure);
        edges.extend(child_edges);
    }

    Ok((subprocedure_templates, edges))
}
