//! The flowchart of the pizza template drawn from the SQLite replica.

use aps_test_utils::{Replica, Tenant};
use procedure_template_visitor::ProcedureTemplateGraph;
use procedure_template_visualization::flowchart;

/// The flowchart of a tenant's pizza template with `toppings`.
fn pizza_flowchart(toppings: &[&str]) -> String {
    let mut replica = Replica::new();
    let tenant = replica.deliver(|conn| Ok(Tenant::insert(conn))).unwrap();
    replica.act_as(Some(tenant.user()));
    let graph = replica
        .write(|conn| {
            let pizza = tenant.pizza(toppings, conn);
            ProcedureTemplateGraph::new(pizza.template(), conn)
        })
        .unwrap();
    flowchart(&graph).unwrap().to_string()
}

/// The number of edges of `mermaid` in a style class whose name starts with
/// `class_prefix`.
fn edges_in(mermaid: &str, class_prefix: &str) -> usize {
    mermaid
        .lines()
        .filter_map(|line| line.trim().strip_prefix("class e"))
        .filter(|rest| {
            rest.split_once(' ').is_some_and(|(_, class)| class.starts_with(class_prefix))
        })
        .count()
}

#[test]
fn every_slot_is_drawn_with_its_role() {
    let mermaid = pizza_flowchart(&["Add olives"]);
    for label in [
        "flour (consumed)",
        "dough (intermediate)",
        "pizza (produced)",
        "oven (instrument)",
        "prepare the dough: Prepare dough",
        "dough (input)",
        "bake: Bake pizza",
    ] {
        assert!(mermaid.contains(label), "no `{label}` in\n{mermaid}");
    }
}

#[test]
fn bindings_hand_the_root_slots_down_and_arrows_order_the_steps() {
    let mermaid = pizza_flowchart(&["Add olives", "Add basil"]);
    // Prepare binds flour and dough, each topping dough, bake dough, pizza and
    // oven.
    assert_eq!(edges_in(&mermaid, "binding_"), 7, "{mermaid}");
    // Prepare precedes each topping, and each topping precedes bake.
    assert_eq!(edges_in(&mermaid, "step_arrow"), 4, "{mermaid}");
}
