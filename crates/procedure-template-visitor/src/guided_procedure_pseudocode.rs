//! Submodule defining the `GuidedProcedurePseudocodeBuilder` structure and its
//! associated methods.

mod error;
mod iterator;
mod listener;
use aps_procedure_templates::*;
use listener::GPPListener;
use quote::{format_ident, quote};

use crate::{
    guided_procedure_pseudocode::error::GuidedProcedurePseudocodeError,
    procedure_template_graph::ProcedureTemplateGraph, ptg_visitor::PTGVisitor,
};
#[derive(Debug, Clone)]
/// Struct providing functionalities to help the user concretely build a
/// procedure.
pub struct GuidedProcedurePseudocode<'graph> {
    visitor: PTGVisitor<'graph, ProcedureTemplateGraph, GPPListener>,
}

impl<'graph> GuidedProcedurePseudocode<'graph> {
    /// Creates a new `GuidedProcedurePseudocodeBuilder` instance.
    ///
    /// # Arguments
    ///
    /// * `graph` - A reference to the `ProcedureTemplateGraph` representing the
    ///   procedure template structure.
    /// * `skip_base_procedures` - A boolean indicating whether to skip base
    ///   procedures that do not require user guidance.
    ///
    /// # Returns
    ///
    /// A new instance of `GuidedProcedurePseudocode`.
    ///
    /// # Errors
    ///
    /// * `GuidedProcedurePseudocodeError::NotASimplePath` - If the provided
    ///   graph is not a simple path.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use aps_test_utils::{aps_conn, pizza_procedure_template, user};
    /// use procedure_template_visitor::{GuidedProcedurePseudocode, ProcedureTemplateGraph};
    /// let mut conn = aps_conn();
    /// let author = user(&mut conn);
    /// let procedure_template = pizza_procedure_template(&author, &mut conn);
    /// let graph = ProcedureTemplateGraph::new(&procedure_template, &mut conn)
    ///     .expect("Failed to create ProcedureTemplateGraph");
    /// let guided_pseudocode = GuidedProcedurePseudocode::new(&graph, false)
    ///     .expect("Failed to create GuidedProcedurePseudocode");
    /// ```
    pub fn new(
        graph: &'graph ProcedureTemplateGraph,
        skip_base_procedures: bool,
    ) -> Result<Self, GuidedProcedurePseudocodeError> {
        if !graph.is_simple_path() {
            return Err(GuidedProcedurePseudocodeError::NotASimplePath);
        }
        Ok(Self { visitor: PTGVisitor::new(graph, GPPListener::new(skip_base_procedures)) })
    }

    /// Writes out the guided procedure pseudocode.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::convert::Infallible;
    ///
    /// use aps_test_utils::{aps_conn, pizza_procedure_template, user};
    /// use procedure_template_visitor::{GuidedProcedurePseudocode, ProcedureTemplateGraph};
    /// let mut conn = aps_conn();
    /// let author = user(&mut conn);
    /// let procedure_template = pizza_procedure_template(&author, &mut conn);
    /// let graph = ProcedureTemplateGraph::new(&procedure_template, &mut conn)
    ///     .expect("Failed to create ProcedureTemplateGraph");
    /// let guided_pseudocode = GuidedProcedurePseudocode::new(&graph, false)
    ///     .expect("Failed to create GuidedProcedurePseudocode");
    /// let _pseudocode = guided_pseudocode.pseudocode::<Infallible>().unwrap();
    /// ```
    pub fn pseudocode<E>(mut self) -> Result<String, GuidedProcedurePseudocodeError> {
        let root_procedure_template = self.visitor.graph().root_procedure_template();
        let variable_name_str = root_procedure_template.name().to_lowercase().replace(' ', "_");
        let variable_name = format_ident!("{}", variable_name_str);

        let error_type_str = std::any::type_name::<E>();
        let error_type: syn::Type =
            syn::parse_str(error_type_str).expect("Failed to parse error type");

        let mut chain_calls = Vec::new();

        for procedure in &mut self {
            let procedure_name = procedure.name();
            let msg = format!("Implement the logic for \"{procedure_name}\"");

            let procedure_id_str = procedure.procedure_template_table_id();
            let procedure_type: syn::Type = syn::parse_str(procedure_id_str)
                .unwrap_or_else(|_| panic!("Failed to parse procedure type: {}", procedure_id_str));

            chain_calls.push(quote! {
                .and_then::<#procedure_type, #error_type>(|mut builder, conn| {
                    todo!(#msg);
                    Ok(builder)
                })?
            });
        }

        if chain_calls.is_empty() {
            return Err(GuidedProcedurePseudocodeError::NoUnskippedProcedures);
        }

        let code = quote! {
            fn dummy() -> anyhow::Result<()> {
                let #variable_name = GuidedProcedure::new()
                    .author(author)
                    .graph(&procedure_graph)
                    .connection(portal_conn)
                    .try_into()?;

                #variable_name
                    #(#chain_calls)*
                    .finish()?;
                Ok(())
            }
        };

        let syntax_tree = syn::parse2(code).expect("Failed to parse generated code");
        let formatted = prettyplease::unparse(&syntax_tree);

        // Strip the wrapper function
        let lines: Vec<&str> = formatted.lines().collect();

        let content = lines[1..lines.len() - 2].join("\n");
        // Dedent
        Ok(content
            .lines()
            .map(|l| l.strip_prefix("    ").unwrap_or(l))
            .collect::<Vec<_>>()
            .join("\n"))
    }
}

#[cfg(test)]
mod tests {

    #[test]
    /// Tests that an error is returned when trying to create a
    /// `GuidedProcedurePseudocode` from a non-simple path
    /// `ProcedureTemplateGraph`.
    fn test_error_case_on_tree_procedure_template_graph() {
        use aps_test_utils::{aps_conn, pizza_four_season_procedure_template, user};

        use crate::{GuidedProcedurePseudocode, ProcedureTemplateGraph};

        let mut conn = aps_conn();
        let author = user(&mut conn);
        let procedure_template = pizza_four_season_procedure_template(&author, &mut conn);
        let graph = ProcedureTemplateGraph::new(&procedure_template, &mut conn)
            .expect("Failed to create ProcedureTemplateGraph");
        let guided_pseudocode_result = GuidedProcedurePseudocode::new(&graph, false);
        assert!(matches!(
            guided_pseudocode_result,
            Err(crate::guided_procedure_pseudocode::error::GuidedProcedurePseudocodeError::NotASimplePath)
        ));
    }
}
