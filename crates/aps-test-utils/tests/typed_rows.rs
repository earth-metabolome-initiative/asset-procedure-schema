//! A runner writes and reads the rows of typed tables it knows only by
//! `table_name_id`, on the SQLite replica.

use aps::{TypedRows, asset_models, containers, procedures, resources, storage_procedures};
use aps_authz_resource_kind::ResourceKind;
use aps_test_utils::{Replica, Tenant};
use aps_traits::CanContain;
use diesel::{
    ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl, SqliteConnection,
    connection::SimpleConnection,
};
use diesel_builders::{BuildableTable, Insert, prelude::*};
use procedure_like::{
    rows::{AssetColumns, ColumnValue, ProcedureColumns, TypedRowError},
    shape::SqlType,
};
use rosetta_utc::TimestampUTC;
use rosetta_uuid::Uuid;

/// `id` as a SQLite blob literal.
fn blob(id: Uuid) -> String {
    format!("x'{}'", id.to_string().replace('-', ""))
}

/// `error` as the error a replica write rolls back on.
fn rolled_back(error: impl Into<TypedRowError>) -> diesel::result::Error {
    diesel::result::Error::QueryBuilderError(error.into().into())
}

/// A tenant's published storage template, storing a vial into a fridge at
/// 277.15 K, and a published root template running it as its one step.
struct Storage {
    /// The storage template.
    template: Uuid,
    /// The fridge model.
    fridge: Uuid,
    /// The vial model.
    vial: Uuid,
    /// The fridge slot of the storage template.
    fridge_slot: Uuid,
    /// The vial slot of the storage template.
    vial_slot: Uuid,
    /// The root template.
    root: Uuid,
    /// The step of the root template running the storage template.
    step: Uuid,
}

/// Inserts template `id` named `name` into the typed template table `table`,
/// with a fridge slot and a vial slot placed into it, both input slots of
/// one asset.
fn template(
    tenant: &Tenant,
    table: &str,
    name: &str,
    [id, fridge_slot, vial_slot]: [Uuid; 3],
    [fridge, vial]: [Uuid; 2],
    conn: &mut SqliteConnection,
) -> QueryResult<()> {
    let (organization, project, user) =
        (blob(tenant.organization()), blob(tenant.project()), blob(tenant.user()));
    let (id, fridge_slot, vial_slot) = (blob(id), blob(fridge_slot), blob(vial_slot));
    let (fridge, vial) = (blob(fridge), blob(vial));
    conn.batch_execute(&format!(
        "INSERT INTO resources (id, kind, organization_id, parent_id, parent_kind, created_by_id)
            VALUES ({id}, 'ownable', {organization}, {project}, 'project', {user});
        INSERT INTO procedure_templates (id, organization_id, project_id, table_name_id, name)
            VALUES ({id}, {organization}, {project}, '{table}', '{name}');
        INSERT INTO procedure_template_slots
            (id, organization_id, procedure_template_id, name, asset_model_id, role_id, minimum_count, maximum_count)
            VALUES ({fridge_slot}, {organization}, {id}, 'fridge', {fridge}, 'input', 1, 1);
        INSERT INTO procedure_template_slots
            (id, organization_id, procedure_template_id, name, asset_model_id, role_id, minimum_count, maximum_count,
            placed_into_slot_id)
            VALUES ({vial_slot}, {organization}, {id}, 'vial', {vial}, 'input', 1, 1, {fridge_slot});"
    ))
}

/// Publishes template `id`.
fn publish(id: Uuid, conn: &mut SqliteConnection) -> QueryResult<()> {
    conn.batch_execute(&format!(
        "UPDATE procedure_templates SET state_id = 'published' WHERE id = {}",
        blob(id)
    ))
}

/// A replica acting as a tenant holding a [`Storage`].
fn storage() -> (Replica, Tenant, Storage) {
    let mut replica = Replica::new();
    let tenant = replica.deliver(|conn| Ok(Tenant::insert(conn))).unwrap();
    replica.act_as(Some(tenant.user()));
    let [template_id, fridge_slot, vial_slot, root, root_fridge, root_vial, step] =
        [(); 7].map(|()| Uuid::new_v4());
    let (fridge, vial) = replica
        .write(|conn| {
            let fridge = tenant.container_model("Fridge", conn);
            let vial = tenant.physical_asset_model("Vial", conn);
            fridge.can_contain(&vial, 1, tenant.user(), conn).expect("fridge holds a vial");
            let fridge = fridge.get_column::<asset_models::id>();
            let vial = vial.get_column::<asset_models::id>();
            let storage = [template_id, fridge_slot, vial_slot];
            template(&tenant, "storage_procedure_templates", "Store", storage, [fridge, vial], conn)?;
            let (organization, template_id) = (blob(tenant.organization()), blob(template_id));
            let (fridge_slot, vial_slot) = (blob(fridge_slot), blob(vial_slot));
            let (fridge_id, vial_id) = (blob(fridge), blob(vial));
            conn.batch_execute(&format!(
                "INSERT INTO storage_procedure_templates
                    (id, organization_id, stored_into_slot_id, stored_into_model_id, stored_asset_slot_id,
                    stored_asset_model_id, temperature_k)
                    VALUES ({template_id}, {organization}, {fridge_slot}, {fridge_id}, {vial_slot}, {vial_id},
                    277.15);"
            ))?;
            Ok((fridge, vial))
        })
        .unwrap();
    replica.write(|conn| publish(template_id, conn)).unwrap();
    replica
        .write(|conn| {
            let slots = [root, root_fridge, root_vial];
            template(&tenant, "procedure_templates", "Store samples", slots, [fridge, vial], conn)?;
            let organization = blob(tenant.organization());
            let (root, step, child) = (blob(root), blob(step), blob(template_id));
            let mut statements = format!(
                "INSERT INTO procedure_template_steps
                    (id, organization_id, parent_template_id, child_template_id, name, position)
                    VALUES ({step}, {organization}, {root}, {child}, 'store', 0);"
            );
            for (child_slot, parent_slot, model) in
                [(fridge_slot, root_fridge, fridge), (vial_slot, root_vial, vial)]
            {
                let (child_slot, parent_slot, model) =
                    (blob(child_slot), blob(parent_slot), blob(model));
                statements.push_str(&format!(
                    "INSERT INTO procedure_template_slot_bindings
                        (step_id, child_slot_id, parent_template_id, child_template_id, parent_slot_id,
                        child_asset_model_id, parent_asset_model_id, child_role_id, parent_role_id,
                        organization_id)
                        VALUES ({step}, {child_slot}, {root}, {child}, {parent_slot}, {model}, {model}, 'input',
                        'input', {organization});"
                ));
            }
            conn.batch_execute(&statements)
        })
        .unwrap();
    replica.write(|conn| publish(root, conn)).unwrap();
    let storage =
        Storage { template: template_id, fridge, vial, fridge_slot, vial_slot, root, step };
    (replica, tenant, storage)
}

/// The columns of a run of `template` by `tenant` as the root of its tree,
/// in progress since `at`.
fn in_progress(tenant: &Tenant, template: Uuid, at: TimestampUTC) -> ProcedureColumns {
    let id = Uuid::new_v4();
    ProcedureColumns {
        id,
        organization_id: tenant.organization(),
        project_id: tenant.project(),
        created_by_id: tenant.user(),
        procedure_template_id: template,
        status_id: "in_progress".to_owned(),
        parent_procedure_id: None,
        parent_template_id: None,
        step_id: None,
        root_procedure_id: id,
        repeat_procedure_asset_id: None,
        planned_for: None,
        started_at: Some(at),
        ended_at: None,
        performed_by_id: Some(tenant.user()),
        note: None,
    }
}

/// The columns of a planned run of the storage step of `root`, a run of the
/// root template.
fn planned_step(tenant: &Tenant, storage: &Storage, root: &ProcedureColumns) -> ProcedureColumns {
    ProcedureColumns {
        id: Uuid::new_v4(),
        procedure_template_id: storage.template,
        status_id: "planned".to_owned(),
        parent_procedure_id: Some(root.id),
        parent_template_id: Some(storage.root),
        step_id: Some(storage.step),
        root_procedure_id: root.id,
        started_at: None,
        performed_by_id: None,
        ..in_progress(tenant, storage.template, TimestampUTC::now())
    }
}

/// Inserts the untyped run `columns` of the root template.
fn insert_root(columns: &ProcedureColumns, conn: &mut SqliteConnection) -> QueryResult<()> {
    let builder = procedures::table::try_builder()
        .map_err(rolled_back)?
        .set_column::<resources::id>(columns.id)
        .set_column::<resources::organization_id>(columns.organization_id)
        .set_column::<resources::parent_id>(Some(columns.project_id))
        .set_column::<resources::parent_kind>(Some(ResourceKind::Project))
        .set_column::<resources::created_by_id>(Some(columns.created_by_id))
        .set_column::<procedures::organization_id>(columns.organization_id)
        .set_column::<procedures::project_id>(columns.project_id)
        .set_column::<procedures::table_name_id>("procedures".to_owned())
        .set_column::<procedures::procedure_template_id>(columns.procedure_template_id)
        .set_column::<procedures::status_id>(columns.status_id.clone())
        .set_column::<procedures::parent_procedure_id>(None)
        .set_column::<procedures::parent_template_id>(None)
        .set_column::<procedures::step_id>(None)
        .set_column::<procedures::root_procedure_id>(columns.id)
        .set_column::<procedures::repeat_procedure_asset_id>(None)
        .set_column::<procedures::planned_for>(None)
        .set_column::<procedures::performed_by_id>(columns.performed_by_id)
        .try_set_column::<procedures::started_at>(columns.started_at)
        .map_err(rolled_back)?
        .try_set_column::<procedures::ended_at>(None)
        .map_err(rolled_back)?
        .try_set_column::<procedures::note>(None)
        .map_err(rolled_back)?;
    builder.insert(conn).map_err(rolled_back)?;
    Ok(())
}

/// Binds the slots of the storage run `run` to the fridge and vial models,
/// with the assets left open.
fn bind_models(
    storage: &Storage,
    run: &ProcedureColumns,
    conn: &mut SqliteConnection,
) -> QueryResult<()> {
    let (organization, procedure, template) =
        (blob(run.organization_id), blob(run.id), blob(storage.template));
    for (slot, model) in [(storage.fridge_slot, storage.fridge), (storage.vial_slot, storage.vial)]
    {
        let (id, slot, model) = (blob(Uuid::new_v4()), blob(slot), blob(model));
        conn.batch_execute(&format!(
            "INSERT INTO procedure_assets
                (id, organization_id, procedure_id, procedure_template_id, slot_id, slot_asset_model_id,
                slot_role_id, asset_model_id)
                VALUES ({id}, {organization}, {procedure}, {template}, {slot}, {model}, 'input', {model});"
        ))?;
    }
    Ok(())
}

/// Moves procedure `id` to `status`, ending it at `at`.
fn end(
    id: Uuid,
    status: &str,
    at: TimestampUTC,
    conn: &mut SqliteConnection,
) -> QueryResult<usize> {
    diesel::update(procedures::table.filter(procedures::id.eq(id)))
        .set((procedures::status_id.eq(status), procedures::ended_at.eq(Some(at))))
        .execute(conn)
}

/// The status the typed copy of storage run `id` holds.
fn typed_status(id: Uuid, conn: &mut SqliteConnection) -> String {
    storage_procedures::table
        .filter(storage_procedures::id.eq(id))
        .select(storage_procedures::status_id)
        .first(conn)
        .unwrap()
}

#[test]
fn a_typed_run_takes_corrects_and_keeps_its_measurement_through_completion() {
    let (mut replica, tenant, storage) = storage();
    let at = TimestampUTC::now();
    let run = in_progress(&tenant, storage.template, at);
    let id = run.id;
    replica
        .write(|conn| {
            conn.insert_typed_procedure("storage_procedures", &run).map_err(rolled_back)?;
            bind_models(&storage, &run, conn)
        })
        .unwrap();
    let conn = replica.connection();
    assert_eq!(conn.typed_values("storage_procedures", id).unwrap(), vec![("temperature_k", None)]);
    let measured = Some(ColumnValue::DoublePrecision(278.0));
    replica
        .write(|conn| {
            conn.set_measurements("storage_procedures", id, &[("temperature_k", measured.clone())])
                .map_err(rolled_back)
        })
        .unwrap();
    let conn = replica.connection();
    assert_eq!(
        conn.typed_values("storage_procedures", id).unwrap(),
        vec![("temperature_k", measured.clone())]
    );
    replica
        .write(|conn| {
            conn.set_measurements("storage_procedures", id, &[("temperature_k", None)])
                .map_err(rolled_back)
        })
        .unwrap();
    let conn = replica.connection();
    assert_eq!(conn.typed_values("storage_procedures", id).unwrap(), vec![("temperature_k", None)]);
    assert!(replica.write(|conn| end(id, "completed", at, conn)).is_err());
    replica
        .write(|conn| {
            conn.set_measurements("storage_procedures", id, &[("temperature_k", measured.clone())])
                .map_err(rolled_back)?;
            end(id, "completed", at, conn)
        })
        .unwrap();
    let conn = replica.connection();
    assert_eq!(typed_status(id, conn), "completed");
    assert_eq!(
        conn.typed_values("storage_procedures", id).unwrap(),
        vec![("temperature_k", measured)]
    );
}

/// A replica holding a [`Storage`], a run of its root template in progress
/// and a planned run of the storage step under it, with the planned run's
/// columns.
fn planned() -> (Replica, ProcedureColumns) {
    let (mut replica, tenant, storage) = storage();
    let root = in_progress(&tenant, storage.root, TimestampUTC::now());
    let step = planned_step(&tenant, &storage, &root);
    replica
        .write(|conn| {
            insert_root(&root, conn)?;
            conn.insert_typed_procedure("storage_procedures", &step).map_err(rolled_back)
        })
        .unwrap();
    (replica, step)
}

#[test]
fn a_planned_step_run_holds_no_measurement_until_skipped() {
    let (mut replica, step) = planned();
    let conn = replica.connection();
    assert_eq!(typed_status(step.id, conn), "planned");
    assert_eq!(
        conn.typed_values("storage_procedures", step.id).unwrap(),
        vec![("temperature_k", None)]
    );
    replica
        .write(|conn| {
            diesel::update(procedures::table.filter(procedures::id.eq(step.id)))
                .set((
                    procedures::status_id.eq("skipped"),
                    procedures::ended_at.eq(Some(TimestampUTC::now())),
                    procedures::note.eq(Some("the fridge is broken")),
                ))
                .execute(conn)
        })
        .unwrap();
    let conn = replica.connection();
    assert_eq!(typed_status(step.id, conn), "skipped");
    assert_eq!(
        conn.typed_values("storage_procedures", step.id).unwrap(),
        vec![("temperature_k", None)]
    );
}

#[test]
fn an_aborted_step_run_keeps_no_measurement() {
    let (mut replica, step) = planned();
    replica.write(|conn| end(step.id, "aborted", TimestampUTC::now(), conn)).unwrap();
    let conn = replica.connection();
    assert_eq!(typed_status(step.id, conn), "aborted");
    assert_eq!(
        conn.typed_values("storage_procedures", step.id).unwrap(),
        vec![("temperature_k", None)]
    );
}

#[test]
fn a_typed_template_shows_its_setpoints() {
    let (mut replica, _, storage) = storage();
    let conn = replica.connection();
    assert_eq!(
        conn.typed_values("storage_procedure_templates", storage.template).unwrap(),
        vec![
            ("temperature_k", Some(ColumnValue::DoublePrecision(277.15))),
            ("temperature_tolerance_percentage", Some(ColumnValue::DoublePrecision(1.0))),
        ]
    );
}

#[test]
fn a_measurement_refuses_an_unknown_table_an_unknown_column_and_a_wrong_type() {
    let (mut replica, ..) = storage();
    let conn = replica.connection();
    let id = Uuid::new_v4();
    let celsius = Some(ColumnValue::DoublePrecision(4.0));
    assert!(matches!(
        conn.set_measurements("procedures", id, &[("temperature_k", celsius.clone())]),
        Err(TypedRowError::UnknownTable(table)) if table == "procedures"
    ));
    assert!(matches!(
        conn.set_measurements("storage_procedures", id, &[("temperature_c", celsius)]),
        Err(TypedRowError::UnknownColumn { table: "storage_procedures", column }) if column == "temperature_c"
    ));
    assert!(matches!(
        conn.set_measurements(
            "storage_procedures",
            id,
            &[("temperature_k", Some(ColumnValue::SmallInt(4)))]
        ),
        Err(TypedRowError::ColumnType {
            table: "storage_procedures",
            column: "temperature_k",
            expected: SqlType::DoublePrecision,
            given: SqlType::SmallInt,
        })
    ));
}

/// The columns of an asset named `name` of `model` created by `tenant`.
fn asset(tenant: &Tenant, model: Uuid, name: &str) -> AssetColumns {
    AssetColumns {
        id: Uuid::new_v4(),
        organization_id: tenant.organization(),
        project_id: tenant.project(),
        created_by_id: tenant.user(),
        model_id: model,
        name: name.to_owned(),
        description: None,
    }
}

#[test]
fn a_produced_container_is_inserted_as_a_row_of_its_model() {
    let (mut replica, tenant, storage) = storage();
    let fridge = asset(&tenant, storage.fridge, "Fridge 1");
    replica
        .write(|conn| conn.insert_produced_asset("containers", &fridge, &[]).map_err(rolled_back))
        .unwrap();
    let model: Uuid = containers::table
        .filter(containers::id.eq(fridge.id))
        .select(containers::container_model_id)
        .first(replica.connection())
        .unwrap();
    assert_eq!(model, storage.fridge);
}

#[test]
fn a_produced_asset_refuses_a_missing_or_unknown_value() {
    let (mut replica, tenant, storage) = storage();
    let conn = replica.connection();
    let fridge = asset(&tenant, storage.fridge, "Fridge 1");
    assert!(matches!(
        conn.insert_produced_asset("containers", &fridge, &[("volume_l", ColumnValue::DoublePrecision(1.0))]),
        Err(TypedRowError::UnknownColumn { table: "containers", column }) if column == "volume_l"
    ));
    assert!(matches!(
        conn.insert_produced_asset("samples", &fridge, &[]),
        Err(TypedRowError::MissingValue { table: "samples", column: "sample_source_model_id" })
    ));
}
