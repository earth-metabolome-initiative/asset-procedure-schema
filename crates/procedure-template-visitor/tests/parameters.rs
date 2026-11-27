//! The parameters a template declares and the values its parent's steps fix
//! for them, as the template graph loads them.

use aps_procedures::{procedure_template_parameters, procedure_template_step_parameters};
use aps_resources::{procedure_template_steps, procedure_templates};
use aps_test_utils::{Replica, Tenant};
use aps_traits::{ProcedureTemplateNode, ProcedureTemplateStepNode, SlotRole};
use diesel::{QueryResult, SqliteConnection, connection::SimpleConnection};
use diesel_builders::GetColumnExt;
use procedure_template_visitor::ProcedureTemplateGraph;
use rosetta_uuid::Uuid;

/// `id` as a SQLite blob literal.
fn blob(id: Uuid) -> String {
    format!("x'{}'", id.to_string().replace('-', ""))
}

/// Inserts into template `template` of `tenant` the parameter `id` named
/// `name`, with its kind columns given as SQL literals.
fn parameter(
    tenant: &Tenant,
    template: Uuid,
    id: Uuid,
    name: &str,
    kind: &str,
    conn: &mut SqliteConnection,
) -> QueryResult<()> {
    let (organization, template, id) = (blob(tenant.organization()), blob(template), blob(id));
    conn.batch_execute(&format!(
        "INSERT INTO procedure_template_parameters
            (id, organization_id, procedure_template_id, name, value_kind_id, quantity_kind_id, default_number,
            minimum_number, maximum_number)
            VALUES ({id}, {organization}, {template}, '{name}', {kind});"
    ))
}

#[test]
fn the_graph_loads_template_parameters_and_the_values_steps_fix() {
    let mut replica = Replica::new();
    let tenant = replica.deliver(|conn| Ok(Tenant::insert(conn))).unwrap();
    replica.act_as(Some(tenant.user()));
    let (temperature, minutes) = (Uuid::new_v4(), Uuid::new_v4());
    let graph = replica
        .write(|conn| {
            let pizza = tenant.asset_model("Pizza", None, conn);
            let bake = tenant.procedure_template("Bake", conn);
            let baked = bake.slot("pizza", &pizza, SlotRole::Input, 1, Some(1), conn).unwrap();
            let bake_id = bake.get_column::<procedure_templates::id>();
            parameter(
                &tenant,
                bake_id,
                temperature,
                "temperature",
                "'number', 'temperature', 480.0, 300.0, 600.0",
                conn,
            )?;
            parameter(&tenant, bake_id, minutes, "minutes", "'number', 'duration', NULL, 0.0, NULL", conn)?;
            bake.publish(conn)?;
            let make = tenant.procedure_template("Make a pizza", conn);
            let made = make.slot("pizza", &pizza, SlotRole::Input, 1, Some(1), conn).unwrap();
            let step = make.step(&bake, "bake", 0, conn).unwrap();
            step.bind(&baked, &made, conn).unwrap();
            let (organization, make_id) =
                (blob(tenant.organization()), blob(make.get_column::<procedure_templates::id>()));
            conn.batch_execute(&format!(
                "INSERT INTO procedure_template_step_parameters
                    (step_id, parameter_id, parent_template_id, child_template_id, value_kind_id, organization_id,
                    number_value)
                    VALUES ({}, {}, {make_id}, {}, 'number', {organization}, 500.0);",
                blob(step.get_column::<procedure_template_steps::id>()),
                blob(temperature),
                blob(bake_id),
            ))?;
            ProcedureTemplateGraph::new(&make, conn)
        })
        .unwrap();

    let step = &graph.root().task_graph().steps()[0];
    let bake = graph.child_of(step);
    let parameters: Vec<_> = bake
        .parameters()
        .iter()
        .map(|parameter| {
            (
                parameter.get_column::<procedure_template_parameters::id>(),
                parameter.get_column::<procedure_template_parameters::default_number>(),
            )
        })
        .collect();
    assert_eq!(parameters, vec![(minutes, None), (temperature, Some(480.0))]);
    assert!(graph.root().parameters().is_empty());
    let fixed: Vec<_> = step
        .parameters()
        .iter()
        .map(|fixed| {
            (
                fixed.get_column::<procedure_template_step_parameters::parameter_id>(),
                fixed.get_column::<procedure_template_step_parameters::number_value>(),
            )
        })
        .collect();
    assert_eq!(fixed, vec![(temperature, Some(500.0))]);
}
