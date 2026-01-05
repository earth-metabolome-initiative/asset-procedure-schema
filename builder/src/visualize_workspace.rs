//! Submodule for visualizing the workspace structure.

use std::collections::HashSet;

use mermaid_builder::prelude::*;
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

    let mut edges = HashSet::new();

    // Next, we insert the asset_models' parent-child relationships as edges.
    for table in db.tables() {
        let source_root_table = table.extension_root_table(db).unwrap_or(table);

        let source_id =
            db.tables().position(|t| t == source_root_table).expect("Source root table not found")
                as u64;
        let current_node = builder.get_node_by_id(source_id).expect("Current node not found");

        for referenced_table in table.referenced_tables(db) {
            let destination_root_table =
                referenced_table.extension_root_table(db).unwrap_or(referenced_table);

            // We do not illustrate self-references in the ERD
            if destination_root_table == source_root_table {
                continue;
            }

            let destination_id =
                db.tables()
                    .position(|t| t == destination_root_table)
                    .expect("Referenced table not found in database") as u64;

            if edges.contains(&(source_id, destination_id))
                || edges.contains(&(destination_id, source_id))
            {
                continue;
            }
            edges.insert((source_id, destination_id));

            let parent_node =
                builder.get_node_by_id(destination_id).expect("Referenced node not found");
            builder.edge(
                EREdgeBuilder::one_to_one(current_node.clone(), parent_node).label("depends on")?,
            )?;
        }
    }

    Ok(builder.into())
}
