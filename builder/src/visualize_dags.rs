//! Submodule for visualizing the DAG structures present in the database.

use mermaid::prelude::*;
use sql_traits::prelude::*;

/// Visualizes the DAG structures present in the given database.
pub fn visualize_dags(db: &ParserDB) -> Result<(), Box<dyn std::error::Error>> {
    for root in db.root_tables() {
        visualize_dag(root, db)?;
    }
    Ok(())
}

fn visualize_dag(
    root: &<ParserDB as DatabaseLike>::Table,
    db: &ParserDB,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut builder: FlowchartBuilder = FlowchartBuilder::default().configuration(
        FlowchartConfigurationBuilder::default()
            .renderer(Renderer::EclipseLayoutKernel)
            .direction(Direction::TopToBottom)
            .title(root.table_name())?,
    )?;

    let tables = root.extending_tables(db).chain(std::iter::once(root)).collect::<Vec<_>>();

    for table in tables.iter() {
        builder.node(
            FlowchartNodeBuilder::default()
                .label(table.table_name())?
                .shape(FlowchartNodeShape::RoundEdges)
                .style_property(StyleProperty::BorderRadius(Unit::Pixel(3)))
                .unwrap(),
        )?;
    }

    for (table_id, table) in tables.iter().enumerate() {
        let source = builder.get_node_by_id(table_id as u64).expect("Node should exist");
        for referenced_table in table.referenced_tables(db) {
            let Some(referenced_table_id) = tables.iter().position(|t| t == &referenced_table)
            else {
                continue;
            };

            let destination =
                builder.get_node_by_id(referenced_table_id as u64).expect("Node should exist");

            builder.edge(
                FlowchartEdgeBuilder::default()
                    .source(source.clone())?
                    .destination(destination)?
                    .right_arrow_shape(ArrowShape::Normal)?,
            )?;
        }
    }

    let diagram: Flowchart = builder.into();
    std::fs::write(format!("{}.mmd", root.table_name()), diagram.to_string())?;

    Ok(())
}
