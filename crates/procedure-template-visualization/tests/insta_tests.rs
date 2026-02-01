//! Tests for procedure template visualization.

use std::collections::BTreeMap;

use aps_procedure_templates::procedure_templates;
use aps_test_utils::{
    aps_conn, pizza_four_season_procedure_template, pizza_procedure_template, user,
};
use diesel_builders::prelude::*;
use procedure_template_visualization::MermaidDB;
use regex::Regex;

fn anonymize_mermaid(diagram: &str) -> String {
    let mut result = diagram.to_string();
    let mut uuid_map = BTreeMap::new();
    let mut node_map = BTreeMap::new();
    let mut uuid_counter = 0;
    let mut node_counter = 0;

    // Regex for UUIDs
    let uuid_regex =
        Regex::new(r"[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}").unwrap();

    // Regex for v<digits> nodes
    let node_regex = Regex::new(r"v\d+").unwrap();

    // Regex for e<digits> edges - wait, e<digits> are small integers usually in
    // mermaid-builder, but looking at snapshot: "e0@-.-", "v38772... e0@-.-"
    // "class e0 ptam_edge_..."
    // If e identifiers are sequential/deterministic we can leave them.
    // But v identifiers look like large hashes.

    // First pass: identify all unique UUIDs and assign replacements
    for cap in uuid_regex.find_iter(diagram) {
        let uuid = cap.as_str();
        if !uuid_map.contains_key(uuid) {
            uuid_map.insert(uuid.to_string(), format!("UUID_{}", uuid_counter));
            uuid_counter += 1;
        }
    }

    // First pass: identify all unique v-nodes and assign replacements
    for cap in node_regex.find_iter(diagram) {
        let node = cap.as_str();
        if !node_map.contains_key(node) {
            node_map.insert(node.to_string(), format!("v{}", node_counter));
            node_counter += 1;
        }
    }

    // Replace all occurrences
    // We should replace longer strings first or be careful about overlapping ...
    // but UUIDs and v-nodes are distinct.

    // Replace UUIDs
    for (uuid, replacement) in &uuid_map {
        result = result.replace(uuid, replacement);
    }

    // Replace v-nodes
    // Using simple replace might be dangerous if one node name is a substring of
    // another (e.g. v1 and v12) But here they seem to be v + many digits.
    // To be safe, we can use replace_all with a closure but rust regex doesn't
    // support closure replacement nicely with state easily in one pass on string
    // ownership. Instead, since the regex matches exact tokens, let's use the
    // regex to replace.

    result = uuid_regex
        .replace_all(&result, |caps: &regex::Captures| {
            uuid_map.get(&caps[0]).cloned().unwrap_or_else(|| caps[0].to_string())
        })
        .to_string();

    result = node_regex
        .replace_all(&result, |caps: &regex::Captures| {
            node_map.get(&caps[0]).cloned().unwrap_or_else(|| caps[0].to_string())
        })
        .to_string();

    result
}

#[test]
fn test_procedure_template_visualization_snapshot() {
    let mut conn = aps_conn();
    let author = user(&mut conn);
    let nested_procedure_template = pizza_procedure_template(&author, &mut conn);
    let procedure_template =
        nested_procedure_template.get_model_ref::<procedure_templates::table>();
    let diagram = procedure_template.to_mermaid(&mut conn).expect("Cannot make mermaid pizza!");

    insta::assert_snapshot!(anonymize_mermaid(&diagram.to_string()));
}

#[test]
fn test_procedure_template_visualization_snapshot_four_season() {
    let mut conn = aps_conn();
    let author = user(&mut conn);
    let nested_procedure_template = pizza_four_season_procedure_template(&author, &mut conn);
    let procedure_template =
        nested_procedure_template.get_model_ref::<procedure_templates::table>();
    let diagram = procedure_template.to_mermaid(&mut conn).expect("Cannot make mermaid pizza!");

    insta::assert_snapshot!(anonymize_mermaid(&diagram.to_string()));
}
