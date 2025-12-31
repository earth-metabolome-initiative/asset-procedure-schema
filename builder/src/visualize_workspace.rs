//! Submodule for visualizing the workspace structure.

use mermaid::prelude::*;
use sql_traits::prelude::*;

/// Generates an Entity-Relationship Diagram (ERD) for the workspace
/// dependencies.
///
/// # Arguments
///
/// * `db` - A reference to the `ParserDB` database.
///
/// # Implementative details
///
/// * Only non-extending tables are considered for the ERD.
/// * Tables with extensions are renamed to `<original_name> DAG` for clarity.
pub fn workspace_dependencies(db: &ParserDB) -> Result<ERDiagram, Box<dyn std::error::Error>> {
    let mut builder = ERDiagramBuilder::default().configuration(
        ERDiagramConfigurationBuilder::default()
            .title("Workspace Dependencies")?
            .direction(Direction::TopToBottom)
            .renderer(Renderer::EclipseLayoutKernel),
    )?;

    // First, we inser all of the non-extending tables as nodes.
    for (number, table) in db.tables().enumerate() {
        if table.is_extension(db) {
            continue;
        }
        let table_name = table.table_name().replace("_", " ");
        let mut node_builder = ERNodeBuilder::default().id(number as u64).label(&table_name)?;

        if table.is_extended(db) {
            node_builder = node_builder
                .label(format!("{} DAG", table_name))?
                .style_property(StyleProperty::FontWeight(FontWeight::Bold))?;
        }

        builder.node(node_builder)?;
    }

    // Next, we insert the asset_models' parent-child relationships as edges.
    for (number, table) in db.tables().enumerate() {
        if table.is_extension(db) {
            continue;
        }

        let current_node = builder.get_node_by_id(number as u64).expect("Trackable node not found");

        for referenced_table in table.referenced_tables(db) {
            let root_table = referenced_table.extension_root_table(db).unwrap_or(referenced_table);

            // We do not illustrate self-references in the ERD
            if root_table == table {
                continue;
            }

            let position = db
                .tables()
                .position(|t| t == root_table)
                .expect("Referenced table not found in database");

            let parent_node =
                builder.get_node_by_id(position as u64).expect("Referenced node not found");
            builder.edge(
                EREdgeBuilder::one_to_one(current_node.clone(), parent_node).label("depends on")?,
            )?;
        }
    }

    Ok(builder.into())
}
