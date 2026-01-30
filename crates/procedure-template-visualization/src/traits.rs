//! Submodule defining visualization traits for core structures.

use mermaid_builder::traits::Diagram;

/// A trait for DB types that can be converted into a Mermaid diagram.
pub trait MermaidDB<C> {
    /// The type of the diagram.
    type Diagram: Diagram;
    /// The error type that can be returned.
    type Error: From<diesel::result::Error>;

    /// Converts the implementing type into a Mermaid diagram string.
    ///
    /// # Arguments
    ///
    /// * `conn`: The database connection to use for fetching data.
    ///
    /// # Errors
    ///
    /// * If the conversion to a Mermaid diagram fails.
    /// * If the database connection fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::convert::Infallible;
    ///
    /// use aps_test_utils::{aps_conn, pizza_procedure_template, user};
    /// use procedure_template_visualization::MermaidDB;
    ///
    /// let mut conn = aps_conn();
    /// let author = user(&mut conn);
    /// let procedure_template = pizza_procedure_template(&author, &mut conn);
    /// let _diagram = procedure_template.to_mermaid(&mut conn).expect("Cannot make mermaid pizza!");
    /// ```
    fn to_mermaid(&self, conn: &mut C) -> Result<Self::Diagram, Self::Error>;
}
