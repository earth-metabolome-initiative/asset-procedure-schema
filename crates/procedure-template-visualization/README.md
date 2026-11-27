# Procedure template visualization

Draws a procedure template graph as a Mermaid flowchart. Each step is a node nested in the procedure running it, solid arrows order the steps of a template, and each slot is a node shaped by its role (input rectangle, consumed reversed trapezoid, produced trapezoid, intermediate hexagon, instrument subprocess) and coloured by the slot its assets come from. A dashed arrow runs from a parent's slot to the slot a step binding hands it to.

The graph is loaded through any Diesel connection the `aps/` crates support, and drawing it reads no database.

```rust
use aps_test_utils::{Replica, Tenant};
use procedure_template_visitor::ProcedureTemplateGraph;
use procedure_template_visualization::flowchart;

let mut replica = Replica::new();
let tenant = replica.deliver(|conn| Ok(Tenant::insert(conn))).unwrap();
replica.act_as(Some(tenant.user()));
let graph = replica
    .write(|conn| {
        let pizza = tenant.pizza(&["Add olives", "Add basil"], conn);
        ProcedureTemplateGraph::new(pizza.template(), conn)
    })
    .unwrap();
let mermaid = flowchart(&graph).unwrap().to_string();
assert!(mermaid.contains("flour (consumed)"));
```
