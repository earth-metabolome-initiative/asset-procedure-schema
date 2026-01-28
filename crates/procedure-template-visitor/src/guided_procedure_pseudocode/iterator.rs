//! Submodule implementing the iterator for the `GuidedProcedureBuilder`.

use aps_procedure_templates::*;

use crate::GuidedProcedurePseudocode;

impl<'graph> Iterator for GuidedProcedurePseudocode<'graph> {
    type Item = &'graph ProcedureTemplate;

    fn next(&mut self) -> Option<Self::Item> {
        match self.visitor.next()? {
            Ok(output) => {
                match output {
                    super::listener::GPPListenerOutput::NoOp => self.next(),
                    super::listener::GPPListenerOutput::Template(template) => Some(template),
                }
            }
            Err(_) => unreachable!("The PTGVisitor with GPPListener is infallible"),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::convert::Infallible;

    use crate::guided_procedure_pseudocode::error::GuidedProcedurePseudocodeError;

    #[test]
    /// Test to ensure that the iterator yields the expected procedure
    /// templates.
    fn test_iterator() {
        use aps_procedure_templates::ProcedureTemplate;
        use aps_test_utils::{aps_conn, pizza_procedure_template, user};

        use crate::{GuidedProcedurePseudocode, ProcedureTemplateGraph};

        let mut conn = aps_conn();
        let author = user(&mut conn);
        let procedure_template = pizza_procedure_template(&author, &mut conn);
        let graph = ProcedureTemplateGraph::new(&procedure_template, &mut conn)
            .expect("Failed to create ProcedureTemplateGraph");
        assert_eq!(
            graph.number_of_procedure_templates(),
            4,
            "Expected 4 procedure templates in the graph"
        );

        let visitor = GuidedProcedurePseudocode::new(&graph, true)
            .expect("Failed to create GuidedProcedurePseudocode");

        assert_eq!(
            visitor.pseudocode::<Infallible>().unwrap_err(),
            GuidedProcedurePseudocodeError::NoUnskippedProcedures,
            "Expected error when all procedures are skipped"
        );

        let guided_pseudocode = GuidedProcedurePseudocode::new(&graph, false)
            .expect("Failed to create GuidedProcedurePseudocode");
        let iterator = guided_pseudocode.into_iter();

        let visited_pts = iterator.collect::<Vec<&ProcedureTemplate>>();
        assert_eq!(visited_pts.len(), 4, "Expected 4 procedure templates to be visited");
    }
}
