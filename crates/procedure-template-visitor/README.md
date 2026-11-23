# Procedure template visitor

Loads a procedure template with every template its steps run into a `ProcedureTemplateGraph`, walks it with a `Visitor`, and records finished runs of it with a `GuidedProcedure`. Every operation takes any Diesel connection the `aps/` crates support.

A `GuidedProcedure` records the procedures of a run in the order its steps run, one `and_then` per procedure, each closure setting the times and measurements and binding through `Slots` the slots no step binding fills. `GuidedProcedurePseudocode` writes the chain of calls a graph needs.

```rust
use core::convert::Infallible;

use aps_test_utils::{Replica, Tenant};
use procedure_template_visitor::{GuidedProcedurePseudocode, ProcedureTemplateGraph};

let mut replica = Replica::new();
let tenant = replica.deliver(|conn| Ok(Tenant::insert(conn))).unwrap();
replica.act_as(Some(tenant.user()));
let graph = replica
    .write(|conn| {
        let pizza = tenant.pizza(&["Add olives", "Add basil"], conn);
        ProcedureTemplateGraph::new(pizza.template(), conn)
    })
    .unwrap();
assert_eq!(graph.number_of_procedure_templates(), 5);
let pseudocode = GuidedProcedurePseudocode::new(&graph).pseudocode::<Infallible>();
assert!(pseudocode.contains("// Bind `dough`, `flour`, `oven`, `pizza` through `slots`."));
```
