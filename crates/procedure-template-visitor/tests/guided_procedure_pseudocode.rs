//! Test suite for guided procedure pseudocode generation.

use std::convert::Infallible;

use aps_test_utils::{aps_conn, pizza_procedure_template, user};
use procedure_template_visitor::{GuidedProcedurePseudocode, ProcedureTemplateGraph};

#[test]
fn test_guided_procedure_pseudocode_output() {
    let mut conn = aps_conn();
    let author = user(&mut conn);
    let procedure_template = pizza_procedure_template(&author, &mut conn);
    let graph = ProcedureTemplateGraph::new(&procedure_template, &mut conn)
        .expect("Failed to create ProcedureTemplateGraph");
    let guided_pseudocode = GuidedProcedurePseudocode::new(&graph, false)
        .expect("Failed to create GuidedProcedurePseudocode");
    let pseudocode = guided_pseudocode.pseudocode::<Infallible>().unwrap();

    insta::assert_snapshot!(pseudocode);
}
