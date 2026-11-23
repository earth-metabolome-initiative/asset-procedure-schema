//! The template graph, its pseudocode and guided runs of the pizza template on
//! the SQLite replica.

use core::convert::Infallible;

use aps_asset_model_ancestors::procedure_assets;
use aps_procedure_templates::{procedure_template_slots, procedure_templates};
use aps_procedures::{disposal_procedures, procedures};
use aps_test_utils::{Pizza, Replica, Tenant};
use aps_traits::validation::validated;
use diesel::{
    ExpressionMethods, NullableExpressionMethods, QueryDsl, RunQueryDsl, SqliteConnection,
};
use diesel_builders::{GetColumnExt, TableBuilder, TrySetColumnExt};
use procedure_template_visitor::{
    GuidedProcedure, GuidedProcedureError, GuidedProcedurePseudocode, ProcedureTemplateGraph,
};
use rosetta_utc::TimestampUTC;

/// A replica holding a tenant's pizza template with `toppings`, acting as the
/// tenant's user, and the graph of the template.
fn pizza(toppings: &[&str]) -> (Replica, Tenant, Pizza, ProcedureTemplateGraph) {
    let mut replica = Replica::new();
    let tenant = replica.deliver(|conn| Ok(Tenant::insert(conn))).unwrap();
    replica.act_as(Some(tenant.user()));
    let (pizza, graph) = replica
        .write(|conn| {
            let pizza = tenant.pizza(toppings, conn);
            let graph = ProcedureTemplateGraph::new(pizza.template(), conn)?;
            Ok((pizza, graph))
        })
        .unwrap();
    (replica, tenant, pizza, graph)
}

/// Sets the times of a procedure that started and ended at `at`.
fn finished(
    builder: TableBuilder<procedures::table>,
    at: TimestampUTC,
) -> Result<TableBuilder<procedures::table>, GuidedProcedureError> {
    let builder = validated(builder.try_set_column::<procedures::started_at>(Some(at)))?;
    Ok(validated(builder.try_set_column::<procedures::ended_at>(Some(at)))?)
}

/// Starts a run of `graph` for `tenant` and records its root, binding the
/// four slots of the pizza to their models.
fn root_recorded<'graph>(
    graph: &'graph ProcedureTemplateGraph,
    tenant: &Tenant,
    pizza: &Pizza,
    at: TimestampUTC,
    conn: &'graph mut SqliteConnection,
) -> GuidedProcedure<'graph, SqliteConnection> {
    GuidedProcedure::new(graph, tenant.organization(), tenant.project(), tenant.user(), conn)
        .and_then::<procedures::table, GuidedProcedureError>(|builder, slots, _| {
            slots
                .bind_model("flour", pizza.flour())?
                .bind_model("dough", pizza.dough())?
                .bind_model("pizza", pizza.pizza())?
                .bind_model("oven", pizza.oven())?;
            finished(builder, at)
        })
        .expect("root recorded")
}

#[test]
fn one_topping_makes_a_simple_path() {
    let (_, _, _, graph) = pizza(&["Add olives"]);
    assert_eq!(graph.number_of_procedure_templates(), 4);
    assert!(graph.is_simple_path());
}

#[test]
fn toppings_run_side_by_side() {
    let (_, _, _, graph) = pizza(&["Add olives", "Add basil", "Add cheese"]);
    assert_eq!(graph.number_of_procedure_templates(), 6);
    assert!(!graph.is_simple_path());
    let task_graph = graph.root().task_graph();
    assert_eq!(task_graph.roots().count(), 1);
    assert_eq!(task_graph.sinks().count(), 1);
    let prepare = &task_graph.steps()[0];
    assert_eq!(task_graph.successors(prepare).count(), 3);
}

#[test]
fn every_step_slot_belongs_to_the_root_slot_of_its_name() {
    let (_, _, _, graph) = pizza(&["Add olives"]);
    let root = graph.root().template().get_column::<procedure_templates::id>();
    for step in graph.root().task_graph().steps() {
        for slot in graph.child_of(step).slots() {
            let owner = graph.slot_owner(&[step], slot);
            assert_eq!(owner.depth(), 0);
            assert_eq!(
                owner.slot().get_column::<procedure_template_slots::procedure_template_id>(),
                root
            );
            assert_eq!(
                owner.slot().get_column_ref::<procedure_template_slots::name>(),
                slot.get_column_ref::<procedure_template_slots::name>()
            );
        }
    }
}

#[test]
fn pseudocode_records_the_root_then_each_step() {
    let (_, _, _, graph) = pizza(&["Add olives"]);
    let step = |comment: &str| {
        format!(
            "    // {comment}\n    .and_then::<procedures, core::convert::Infallible>(|builder, slots, conn| {{\n        // Set the times, the performer and the measurements on `builder`.\n        Ok(builder)\n    }})?\n"
        )
    };
    let expected = format!(
        "let make_a_pizza = GuidedProcedure::new(&graph, organization, project, author, conn)\n    // Make a pizza\n    .and_then::<procedures, core::convert::Infallible>(|builder, slots, conn| {{\n        // Set the times, the performer and the measurements on `builder`.\n        // Bind `dough`, `flour`, `oven`, `pizza` through `slots`.\n        Ok(builder)\n    }})?\n{}{}{}    .finish()?;\n",
        step("Prepare dough, step \"prepare the dough\""),
        step("Add olives, step \"Add olives\""),
        step("Bake pizza, step \"bake\""),
    );
    assert_eq!(GuidedProcedurePseudocode::new(&graph).pseudocode::<Infallible>(), expected);
}

#[test]
fn a_guided_run_records_every_procedure_completed() {
    let (mut replica, tenant, pizza, graph) = pizza(&["Add olives"]);
    let at = TimestampUTC::now();
    replica
        .write(|conn| {
            let mut run = root_recorded(&graph, &tenant, &pizza, at, conn);
            for _ in 0..3 {
                run = run
                    .and_then::<procedures::table, GuidedProcedureError>(|builder, _, _| {
                        finished(builder, at)
                    })
                    .expect("step recorded");
            }
            run.finish().expect("run finished");
            Ok(())
        })
        .unwrap();
    let conn = replica.connection();
    let statuses: Vec<String> = procedures::table.select(procedures::status_id).load(conn).unwrap();
    assert_eq!(statuses, vec!["completed"; 4]);
    let bindings: i64 = procedure_assets::table.count().get_result(conn).unwrap();
    assert_eq!(bindings, 10);
    let inherited: i64 = procedure_assets::table
        .filter(procedure_assets::parent_procedure_asset_id.is_not_null())
        .count()
        .get_result(conn)
        .unwrap();
    assert_eq!(inherited, 6);
}

#[test]
fn a_skipped_step_is_recorded_skipped_without_bindings() {
    let (mut replica, tenant, pizza, graph) = pizza(&["Add olives"]);
    let at = TimestampUTC::now();
    replica
        .write(|conn| {
            root_recorded(&graph, &tenant, &pizza, at, conn)
                .and_then::<procedures::table, GuidedProcedureError>(|builder, _, _| {
                    finished(builder, at)
                })
                .expect("dough prepared")
                .skip::<procedures::table>(at, Some("No olives left".to_owned()))
                .expect("topping skipped")
                .and_then::<procedures::table, GuidedProcedureError>(|builder, _, _| {
                    finished(builder, at)
                })
                .expect("pizza baked")
                .finish()
                .expect("run finished");
            Ok(())
        })
        .unwrap();
    let conn = replica.connection();
    let skipped: Vec<String> = procedures::table
        .filter(procedures::status_id.eq("skipped"))
        .select(procedures::note.assume_not_null())
        .load(conn)
        .unwrap();
    assert_eq!(skipped, ["No olives left"]);
    let bindings: i64 = procedure_assets::table.count().get_result(conn).unwrap();
    assert_eq!(bindings, 9);
}

#[test]
fn finishing_early_names_the_template_still_to_record() {
    let (mut replica, tenant, pizza, graph) = pizza(&["Add olives"]);
    let at = TimestampUTC::now();
    let error = replica
        .write(|conn| Ok(root_recorded(&graph, &tenant, &pizza, at, conn).finish()))
        .unwrap()
        .unwrap_err();
    assert!(
        matches!(&error, GuidedProcedureError::Unrecorded { template } if template == "Prepare dough"),
        "{error}"
    );
}

#[test]
fn a_procedure_table_of_another_template_is_refused() {
    let (mut replica, tenant, _, graph) = pizza(&["Add olives"]);
    let error = replica
        .write(|conn| {
            Ok(GuidedProcedure::new(
                &graph,
                tenant.organization(),
                tenant.project(),
                tenant.user(),
                conn,
            )
            .and_then::<disposal_procedures::table, GuidedProcedureError>(|builder, _, _| {
                Ok(builder)
            })
            .map(drop))
        })
        .unwrap()
        .unwrap_err();
    assert!(
        matches!(
            &error,
            GuidedProcedureError::UnexpectedTable { expected: "disposal_procedure_templates", found, .. }
                if found == "procedure_templates"
        ),
        "{error}"
    );
}
