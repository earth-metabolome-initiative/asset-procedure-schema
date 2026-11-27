//! The `SQLippy` gate the APS schema passes before the builder generates
//! code from it.

use std::{collections::HashSet, path::Path, time::Duration};

use connetto_schema_lint::{
    rls2fga::types::TranslationNote,
    translation::{self, divergences, registry, translator},
};
use sql_traits::{
    errors::LookupError,
    prelude::{ForeignKeyLike, ParserDB},
    traits::{ColumnLike, DatabaseLike, FunctionLike, TableIndexLike, TableLike},
};
use sqlippy::prelude::{Constrainer, DefaultConstrainer, Error, TargetName};
use sqlparser::{
    ast::{CreateTable, FunctionSecurity, ReferentialAction},
    dialect::PostgreSqlDialect,
};

use crate::shape::named;

/// Cascading foreign keys that stay inside one logical row's family, grouped
/// by the reason each group stays.
const CASCADES: &[(&str, &[(&str, &str)])] = &[
    (
        "The status copy belongs to its procedure and leaves with it",
        &[
            (
                "aliquoting_procedures",
                "aliquoting_procedures_id_aliquoting_procedure_template_id__fkey",
            ),
            (
                "ball_mill_procedures",
                "ball_mill_procedures_id_ball_mill_procedure_template_id_st_fkey",
            ),
            (
                "centrifuge_procedures",
                "centrifuge_procedures_id_centrifuge_procedure_template_id__fkey",
            ),
            (
                "container_sealing_procedures",
                "container_sealing_procedures_id_container_sealing_procedur_fkey",
            ),
            (
                "disposal_procedures",
                "disposal_procedures_id_disposal_procedure_template_id_stat_fkey",
            ),
            (
                "fractioning_procedures",
                "fractioning_procedures_id_fractioning_procedure_template_i_fkey",
            ),
            (
                "freeze_drying_procedures",
                "freeze_drying_procedures_id_freeze_drying_procedure_templa_fkey",
            ),
            (
                "freezing_procedures",
                "freezing_procedures_id_freezing_procedure_template_id_stat_fkey",
            ),
            (
                "geopositioning_procedures",
                "geopositioning_procedures_id_geopositioning_procedure_temp_fkey",
            ),
            (
                "harvesting_procedures",
                "harvesting_procedures_id_harvesting_procedure_template_id__fkey",
            ),
            (
                "packaging_procedures",
                "packaging_procedures_id_packaging_procedure_template_id_st_fkey",
            ),
            (
                "photograph_procedures",
                "photograph_procedures_id_photograph_procedure_template_id__fkey",
            ),
            (
                "pouring_procedures",
                "pouring_procedures_id_pouring_procedure_template_id_status_fkey",
            ),
            ("procedure_assets", "procedure_assets_procedure_id_procedure_template_id_status_fkey"),
            (
                "procedure_parameter_values",
                "procedure_parameter_values_procedure_id_procedure_template_fkey",
            ),
            (
                "retrieval_procedures",
                "retrieval_procedures_id_retrieval_procedure_template_id_st_fkey",
            ),
            (
                "storage_procedures",
                "storage_procedures_id_storage_procedure_template_id_status_fkey",
            ),
            (
                "supernatant_procedures",
                "supernatant_procedures_id_supernatant_procedure_template_i_fkey",
            ),
            (
                "weighing_procedures",
                "weighing_procedures_id_weighing_procedure_template_id_stat_fkey",
            ),
        ],
    ),
    (
        "A subtype row is its parent row, so deleting the parent deletes all of it",
        &[
            ("aliquoting_procedure_templates", "aliquoting_procedure_templates_id_fkey"),
            ("aliquoting_procedures", "aliquoting_procedures_id_fkey"),
            ("ball_mill_machine_models", "ball_mill_machine_models_id_fkey"),
            ("ball_mill_machines", "ball_mill_machines_id_fkey"),
            ("ball_mill_procedure_templates", "ball_mill_procedure_templates_id_fkey"),
            ("ball_mill_procedures", "ball_mill_procedures_id_fkey"),
            ("bead_models", "bead_models_id_fkey"),
            ("camera_models", "camera_models_id_fkey"),
            ("cameras", "cameras_id_fkey"),
            ("centrifuge_models", "centrifuge_models_id_fkey"),
            ("centrifuge_procedure_templates", "centrifuge_procedure_templates_id_fkey"),
            ("centrifuge_procedures", "centrifuge_procedures_id_fkey"),
            ("centrifuges", "centrifuges_id_fkey"),
            ("commercial_ball_mill_machine_lots", "commercial_ball_mill_machine_lots_id_fkey"),
            ("commercial_ball_mill_machine_lots", "commercial_ball_mill_machine_lots_id_fkey1"),
            ("commercial_ball_mill_machine_models", "commercial_ball_mill_machine_models_id_fkey"),
            ("commercial_ball_mill_machine_models", "commercial_ball_mill_machine_models_id_fkey1"),
            ("commercial_camera_lots", "commercial_camera_lots_id_fkey"),
            ("commercial_camera_lots", "commercial_camera_lots_id_fkey1"),
            ("commercial_camera_models", "commercial_camera_models_id_fkey"),
            ("commercial_camera_models", "commercial_camera_models_id_fkey1"),
            ("commercial_centrifuge_lots", "commercial_centrifuge_lots_id_fkey"),
            ("commercial_centrifuge_lots", "commercial_centrifuge_lots_id_fkey1"),
            ("commercial_centrifuge_models", "commercial_centrifuge_models_id_fkey"),
            ("commercial_centrifuge_models", "commercial_centrifuge_models_id_fkey1"),
            ("commercial_freeze_dryer_lots", "commercial_freeze_dryer_lots_id_fkey"),
            ("commercial_freeze_dryer_lots", "commercial_freeze_dryer_lots_id_fkey1"),
            ("commercial_freeze_dryer_models", "commercial_freeze_dryer_models_id_fkey"),
            ("commercial_freeze_dryer_models", "commercial_freeze_dryer_models_id_fkey1"),
            ("commercial_freezer_lots", "commercial_freezer_lots_id_fkey"),
            ("commercial_freezer_lots", "commercial_freezer_lots_id_fkey1"),
            ("commercial_freezer_models", "commercial_freezer_models_id_fkey"),
            ("commercial_freezer_models", "commercial_freezer_models_id_fkey1"),
            (
                "commercial_geopositioning_device_lots",
                "commercial_geopositioning_device_lots_id_fkey",
            ),
            (
                "commercial_geopositioning_device_lots",
                "commercial_geopositioning_device_lots_id_fkey1",
            ),
            (
                "commercial_geopositioning_device_models",
                "commercial_geopositioning_device_models_id_fkey",
            ),
            (
                "commercial_geopositioning_device_models",
                "commercial_geopositioning_device_models_id_fkey1",
            ),
            ("commercial_marker_lots", "commercial_marker_lots_id_fkey"),
            ("commercial_marker_lots", "commercial_marker_lots_id_fkey1"),
            ("commercial_marker_models", "commercial_marker_models_id_fkey"),
            ("commercial_marker_models", "commercial_marker_models_id_fkey1"),
            ("commercial_mass_spectrometer_lots", "commercial_mass_spectrometer_lots_id_fkey"),
            ("commercial_mass_spectrometer_lots", "commercial_mass_spectrometer_lots_id_fkey1"),
            ("commercial_mass_spectrometer_models", "commercial_mass_spectrometer_models_id_fkey"),
            ("commercial_mass_spectrometer_models", "commercial_mass_spectrometer_models_id_fkey1"),
            (
                "commercial_personal_protective_equipment_lots",
                "commercial_personal_protective_equipment_lots_id_fkey",
            ),
            (
                "commercial_personal_protective_equipment_lots",
                "commercial_personal_protective_equipment_lots_id_fkey1",
            ),
            (
                "commercial_personal_protective_equipment_models",
                "commercial_personal_protective_equipment_models_id_fkey",
            ),
            (
                "commercial_personal_protective_equipment_models",
                "commercial_personal_protective_equipment_models_id_fkey1",
            ),
            ("commercial_phone_device_lots", "commercial_phone_device_lots_geopositioning"),
            ("commercial_phone_device_lots", "commercial_phone_device_lots_id_fkey"),
            ("commercial_phone_device_models", "commercial_phone_device_models_id_fkey"),
            ("commercial_phone_device_models", "commercial_phone_device_models_id_fkey1"),
            ("commercial_pipette_tip_lots", "commercial_pipette_tip_lots_id_fkey"),
            ("commercial_pipette_tip_lots", "commercial_pipette_tip_lots_id_fkey1"),
            ("commercial_pipette_tip_models", "commercial_pipette_tip_models_id_fkey"),
            ("commercial_pipette_tip_models", "commercial_pipette_tip_models_id_fkey1"),
            ("commercial_product_lots", "commercial_product_lots_id_fkey"),
            ("commercial_products", "commercial_products_id_fkey"),
            (
                "commercial_volume_measuring_device_lots",
                "commercial_volume_measuring_device_lots_id_fkey",
            ),
            (
                "commercial_volume_measuring_device_lots",
                "commercial_volume_measuring_device_lots_id_fkey1",
            ),
            (
                "commercial_volume_measuring_device_models",
                "commercial_volume_measuring_device_models_id_fkey",
            ),
            (
                "commercial_volume_measuring_device_models",
                "commercial_volume_measuring_device_models_id_fkey1",
            ),
            ("commercial_weighing_device_lots", "commercial_weighing_device_lots_id_fkey"),
            ("commercial_weighing_device_lots", "commercial_weighing_device_lots_id_fkey1"),
            ("commercial_weighing_device_models", "commercial_weighing_device_models_id_fkey"),
            ("commercial_weighing_device_models", "commercial_weighing_device_models_id_fkey1"),
            ("container_models", "container_models_id_fkey"),
            ("container_sealer_models", "container_sealer_models_id_fkey"),
            (
                "container_sealing_procedure_templates",
                "container_sealing_procedure_templates_id_fkey",
            ),
            ("container_sealing_procedures", "container_sealing_procedures_id_fkey"),
            ("containers", "containers_id_fkey"),
            ("digital_asset_models", "digital_asset_models_id_fkey"),
            ("digital_assets", "digital_assets_id_fkey"),
            ("disposal_procedure_templates", "disposal_procedure_templates_id_fkey"),
            ("disposal_procedures", "disposal_procedures_id_fkey"),
            ("fractioning_procedure_templates", "fractioning_procedure_templates_id_fkey"),
            ("fractioning_procedures", "fractioning_procedures_id_fkey"),
            ("freeze_dryer_models", "freeze_dryer_models_id_fkey"),
            ("freeze_dryers", "freeze_dryers_id_fkey"),
            ("freeze_drying_procedure_templates", "freeze_drying_procedure_templates_id_fkey"),
            ("freeze_drying_procedures", "freeze_drying_procedures_id_fkey"),
            ("freezer_models", "freezer_models_id_fkey"),
            ("freezers", "freezers_id_fkey"),
            ("freezing_procedure_templates", "freezing_procedure_templates_id_fkey"),
            ("freezing_procedures", "freezing_procedures_id_fkey"),
            ("geopositioning_device_models", "geopositioning_device_models_id_fkey"),
            ("geopositioning_devices", "geopositioning_devices_id_fkey"),
            ("geopositioning_procedure_templates", "geopositioning_procedure_templates_id_fkey"),
            ("geopositioning_procedures", "geopositioning_procedures_id_fkey"),
            ("harvesting_procedure_templates", "harvesting_procedure_templates_id_fkey"),
            ("harvesting_procedures", "harvesting_procedures_id_fkey"),
            ("marker_models", "marker_models_id_fkey"),
            ("markers", "markers_id_fkey"),
            ("mass_spectrometer_models", "mass_spectrometer_models_id_fkey"),
            ("mass_spectrometers", "mass_spectrometers_id_fkey"),
            ("packaging_models", "packaging_models_id_fkey"),
            ("packaging_procedure_templates", "packaging_procedure_templates_id_fkey"),
            ("packaging_procedures", "packaging_procedures_id_fkey"),
            ("panel_models", "panel_models_id_fkey"),
            ("panels", "panels_id_fkey"),
            (
                "personal_protective_equipment_models",
                "personal_protective_equipment_models_id_fkey",
            ),
            ("personal_protective_equipments", "personal_protective_equipments_id_fkey"),
            ("phone_device_models", "phone_device_models_geopositioning"),
            ("phone_device_models", "phone_device_models_id_fkey"),
            ("phone_devices", "phone_devices_geopositioning"),
            ("phone_devices", "phone_devices_id_fkey"),
            ("photograph_models", "photograph_models_id_fkey"),
            ("photograph_procedure_templates", "photograph_procedure_templates_id_fkey"),
            ("photograph_procedures", "photograph_procedures_id_fkey"),
            ("photographs", "photographs_id_fkey"),
            ("physical_asset_models", "physical_asset_models_id_fkey"),
            ("physical_assets", "physical_assets_id_fkey"),
            ("pipette_tip_models", "pipette_tip_models_id_fkey"),
            ("pipette_tips", "pipette_tips_id_fkey"),
            ("pouring_procedure_templates", "pouring_procedure_templates_id_fkey"),
            ("pouring_procedures", "pouring_procedures_id_fkey"),
            ("retrieval_procedure_templates", "retrieval_procedure_templates_id_fkey"),
            ("retrieval_procedures", "retrieval_procedures_id_fkey"),
            ("sample_models", "sample_models_id_fkey"),
            ("sample_source_models", "sample_source_models_id_fkey"),
            ("sample_sources", "sample_sources_id_fkey"),
            ("samples", "samples_id_fkey"),
            ("storage_procedure_templates", "storage_procedure_templates_id_fkey"),
            ("storage_procedures", "storage_procedures_id_fkey"),
            ("supernatant_procedure_templates", "supernatant_procedure_templates_id_fkey"),
            ("supernatant_procedures", "supernatant_procedures_id_fkey"),
            ("volume_measuring_device_models", "volume_measuring_device_models_id_fkey"),
            ("volume_measuring_devices", "volume_measuring_devices_id_fkey"),
            ("volumetric_container_models", "volumetric_container_models_id_fkey"),
            ("volumetric_containers", "volumetric_containers_id_fkey"),
            ("weighed_container_models", "weighed_container_models_id_fkey"),
            ("weighed_volumetric_container_models", "weighed_volumetric_container_models_id_fkey"),
            ("weighed_volumetric_container_models", "weighed_volumetric_container_models_id_fkey1"),
            ("weighing_device_models", "weighing_device_models_id_fkey"),
            ("weighing_devices", "weighing_devices_id_fkey"),
            ("weighing_procedure_templates", "weighing_procedure_templates_id_fkey"),
            ("weighing_procedures", "weighing_procedures_id_fkey"),
        ],
    ),
    (
        "An ownable row and its project copy are the same logical row",
        &[
            ("asset_models", "asset_models_id_fkey"),
            ("assets", "assets_id_fkey"),
            ("brands", "brands_id_fkey"),
            ("container_compatibility_rules", "container_compatibility_rules_id_fkey"),
            ("procedure_templates", "procedure_templates_id_fkey"),
            ("procedures", "procedures_id_fkey"),
        ],
    ),
    (
        "Slots, steps, bindings, edges, parameters, procedure assets, values and step runs belong to their template or procedure",
        &[
            (
                "procedure_template_parameters",
                "procedure_template_parameters_procedure_template_id_fkey",
            ),
            (
                "procedure_template_slot_bindings",
                "procedure_template_slot_bindi_parent_slot_id_parent_templa_fkey",
            ),
            (
                "procedure_template_slot_bindings",
                "procedure_template_slot_bindi_step_id_parent_template_id_c_fkey",
            ),
            ("procedure_template_slots", "procedure_template_slots_procedure_template_id_fkey"),
            (
                "procedure_template_step_edges",
                "procedure_template_step_edges_predecessor_step_id_parent_t_fkey",
            ),
            (
                "procedure_template_step_edges",
                "procedure_template_step_edges_successor_step_id_parent_tem_fkey",
            ),
            (
                "procedure_template_step_parameters",
                "procedure_template_step_param_step_id_parent_template_id_c_fkey",
            ),
            ("procedure_template_steps", "procedure_template_steps_parent_template_id_fkey"),
            ("procedures", "procedures_parent_procedure_id_parent_template_id_organiza_fkey"),
            ("procedures", "procedures_parent_procedure_id_root_procedure_id_organizat_fkey"),
        ],
    ),
    (
        "Only the deleted model's own closure rows go, since a model with children cannot be deleted",
        &[
            ("asset_model_ancestors", "asset_model_ancestors_ancestor_model_id_fkey"),
            ("asset_model_ancestors", "asset_model_ancestors_descendant_model_id_fkey"),
            (
                "asset_model_ancestors",
                "asset_model_ancestors_descendant_model_id_parent_model_id__fkey",
            ),
            (
                "asset_model_ancestors",
                "asset_model_ancestors_parent_model_id_ancestor_model_id_fkey",
            ),
        ],
    ),
    (
        "A key naming the model of a row follows the row's root, so deleting the root reaches a table two levels below it",
        &[
            (
                "ball_mill_machines",
                "ball_mill_machines_id_commercial_ball_mill_machine_lot_id_fkey",
            ),
            ("cameras", "cameras_id_commercial_camera_lot_id_fkey"),
            ("centrifuges", "centrifuges_id_commercial_centrifuge_lot_id_fkey"),
            (
                "commercial_ball_mill_machine_lots",
                "commercial_ball_mill_machine__id_commercial_ball_mill_mach_fkey",
            ),
            (
                "commercial_ball_mill_machine_models",
                "commercial_ball_mill_machine__id_ball_mill_machine_model_i_fkey",
            ),
            ("commercial_camera_lots", "commercial_camera_lots_id_commercial_camera_model_id_fkey"),
            ("commercial_camera_models", "commercial_camera_models_id_camera_model_id_fkey"),
            (
                "commercial_centrifuge_lots",
                "commercial_centrifuge_lots_id_commercial_centrifuge_model__fkey",
            ),
            (
                "commercial_centrifuge_models",
                "commercial_centrifuge_models_id_centrifuge_model_id_fkey",
            ),
            (
                "commercial_freeze_dryer_lots",
                "commercial_freeze_dryer_lots_id_commercial_freeze_dryer_mo_fkey",
            ),
            (
                "commercial_freeze_dryer_models",
                "commercial_freeze_dryer_models_id_freeze_dryer_model_id_fkey",
            ),
            (
                "commercial_freezer_lots",
                "commercial_freezer_lots_id_commercial_freezer_model_id_fkey",
            ),
            ("commercial_freezer_models", "commercial_freezer_models_id_freezer_model_id_fkey"),
            (
                "commercial_geopositioning_device_lots",
                "commercial_geopositioning_dev_id_commercial_geopositioning_fkey",
            ),
            (
                "commercial_geopositioning_device_models",
                "commercial_geopositioning_dev_id_geopositioning_device_mod_fkey",
            ),
            ("commercial_marker_lots", "commercial_marker_lots_id_commercial_marker_model_id_fkey"),
            ("commercial_marker_models", "commercial_marker_models_id_marker_model_id_fkey"),
            (
                "commercial_mass_spectrometer_lots",
                "commercial_mass_spectrometer__id_commercial_mass_spectrome_fkey",
            ),
            (
                "commercial_mass_spectrometer_models",
                "commercial_mass_spectrometer__id_mass_spectrometer_model_i_fkey",
            ),
            (
                "commercial_personal_protective_equipment_lots",
                "commercial_personal_protectiv_id_commercial_personal_prote_fkey",
            ),
            (
                "commercial_personal_protective_equipment_models",
                "commercial_personal_protectiv_id_personal_protective_equip_fkey",
            ),
            (
                "commercial_pipette_tip_lots",
                "commercial_pipette_tip_lots_id_commercial_pipette_tip_mode_fkey",
            ),
            (
                "commercial_pipette_tip_models",
                "commercial_pipette_tip_models_id_pipette_tip_model_id_fkey",
            ),
            ("commercial_product_lots", "commercial_product_lots_id_product_model_id_fkey"),
            (
                "commercial_volume_measuring_device_lots",
                "commercial_volume_measuring_d_id_commercial_volume_measuri_fkey",
            ),
            (
                "commercial_volume_measuring_device_models",
                "commercial_volume_measuring_d_id_volume_measuring_device_m_fkey",
            ),
            (
                "commercial_weighing_device_lots",
                "commercial_weighing_device_lo_id_commercial_weighing_devic_fkey",
            ),
            (
                "commercial_weighing_device_models",
                "commercial_weighing_device_mod_id_weighing_device_model_id_fkey",
            ),
            ("containers", "containers_id_container_model_id_fkey"),
            ("freeze_dryers", "freeze_dryers_id_commercial_freeze_dryer_lot_id_fkey"),
            ("freezers", "freezers_id_commercial_freezer_lot_id_fkey"),
            (
                "geopositioning_devices",
                "geopositioning_devices_id_commercial_geopositioning_device_fkey",
            ),
            ("markers", "markers_id_marker_model_id_fkey"),
            (
                "mass_spectrometers",
                "mass_spectrometers_id_commercial_mass_spectrometer_lot_id_fkey",
            ),
            ("panels", "panels_id_panel_model_id_fkey"),
            (
                "personal_protective_equipments",
                "personal_protective_equipment_id_personal_protective_equip_fkey",
            ),
            ("phone_devices", "phone_devices_id_commercial_phone_device_lot_id_fkey"),
            ("photographs", "photographs_id_photograph_model_id_fkey"),
            ("pipette_tips", "pipette_tips_id_pipette_tip_model_id_fkey"),
            ("sample_sources", "sample_sources_id_sample_source_model_id_fkey"),
            ("samples", "samples_id_sample_model_id_fkey"),
            (
                "volume_measuring_devices",
                "volume_measuring_devices_id_volume_measuring_device_model__fkey",
            ),
            (
                "volumetric_containers",
                "volumetric_containers_id_volumetric_container_model_id_fkey",
            ),
            ("weighing_devices", "weighing_devices_id_weighing_device_model_id_fkey"),
        ],
    ),
    (
        "A photograph model's media type copy follows its digital model, and the image check refuses a type that is not an image",
        &[("photograph_models", "photograph_models_id_mime_type_organization_id_fkey")],
    ),
    (
        "A binding's use, moves, occupancy and exits belong to it and leave with it, while the binding guard keeps the bindings of active and completed runs",
        &[
            ("asset_moves", "asset_moves_contained_binding_id_organization_id_fkey"),
            ("asset_moves", "asset_moves_holder_binding_id_organization_id_fkey"),
            ("asset_uses", "asset_uses_id_organization_id_asset_id_procedure_id_status_fkey"),
            (
                "container_occupancy",
                "container_occupancy_contained_binding_id_organization_id_fkey",
            ),
            ("container_occupancy", "container_occupancy_holder_binding_id_organization_id_fkey"),
            ("move_exits", "move_exits_contained_binding_id_organization_id_fkey"),
            ("move_exits", "move_exits_holder_binding_id_organization_id_fkey"),
        ],
    ),
];

/// The schema the gate checks and the builder generates from.
pub struct Schema {
    /// The cluster bootstraps connetto-schema and APS provision roles with.
    pub cluster: ParserDB,
    /// The cluster bootstraps, connetto-schema and every APS slice.
    pub database: ParserDB,
    /// Tables connetto-schema declares.
    connetto_tables: HashSet<String>,
}

impl Schema {
    /// Parses the cluster bootstraps, connetto-schema and the APS slices under
    /// the repository `root`, in the order `tests/run.sh` applies them.
    ///
    /// # Panics
    ///
    /// Panics if a slice cannot be read or parsed.
    #[must_use]
    pub fn load(root: &Path) -> Self {
        let connetto_cluster_path = root.join("connetto-schema/cluster");
        let aps_cluster_path = root.join("cluster");
        let connetto_path = root.join("connetto-schema/sql");
        let aps_path = root.join("sql");
        let cluster = ParserDB::from_paths::<PostgreSqlDialect>(&[
            connetto_cluster_path.as_path(),
            aps_cluster_path.as_path(),
        ])
        .expect("the cluster bootstraps parse");
        let connetto = ParserDB::from_paths::<PostgreSqlDialect>(&[
            connetto_cluster_path.as_path(),
            connetto_path.as_path(),
        ])
        .expect("connetto-schema parses");
        let database = ParserDB::from_paths::<PostgreSqlDialect>(&[
            connetto_cluster_path.as_path(),
            aps_cluster_path.as_path(),
            connetto_path.as_path(),
            aps_path.as_path(),
        ])
        .expect("the APS schema parses");
        let connetto_tables =
            connetto.tables().map(|table| table.table_name().to_owned()).collect();
        Self { cluster, database, connetto_tables }
    }

    /// The findings `SQLippy` and the procedure rules report on the schema
    /// under connetto-schema's deployment, extended with the APS tenant
    /// tables, after the allowances of both schemas.
    ///
    /// # Panics
    ///
    /// Panics if the columns of a table cannot be looked up.
    #[must_use]
    pub fn findings(&self) -> Vec<Error<ParserDB>> {
        let mut deployment = connetto_schema_lint::deployment(&self.cluster)
            .with_clock_tolerance(Duration::from_secs(5 * 60))
            .with_set_within("authz.resources_with('admin')", "authz.resources_with('writer')")
            .expect("a set expression")
            .with_set_within("authz.resources_with('writer')", "authz.resources_with('reader')")
            .expect("a set expression");
        for table in self.database.tables() {
            let holds_organization = table
                .columns(&self.database)
                .expect("the columns of an APS table")
                .any(|column| column.column_name() == "organization_id");
            if holds_organization && !self.connetto_tables.contains(table.table_name()) {
                deployment =
                    deployment.with_tenant_key("public", table.table_name(), ["organization_id"]);
            }
        }
        let mut constrainer = connetto_schema_lint::constrainer(deployment);
        sql_procedure_rules::register_procedure_constraints(&mut constrainer);
        allow(&mut constrainer);
        self.allow_references(&mut constrainer)
            .expect("the foreign keys of the APS tables resolve");
        for function in self.database.functions() {
            if let Some(table) =
                function.name().strip_prefix("require_").and_then(|rest| rest.strip_suffix("_row"))
                && self.database.tables().any(|candidate| candidate.table_name() == table)
            {
                constrainer.allow_function("catalog_name_as_text", function.target_name());
            }
        }
        constrainer.diagnostics(&self.database)
    }

    /// The policy clauses `rls2fga` translates below confidence A under
    /// connetto-schema's translation deployment, empty when the model it
    /// builds answers as `PostgreSQL` does. The set-returning `SECURITY
    /// DEFINER` helpers of the `public` schema join connetto-schema's
    /// `authz` helpers, so the policies calling them translate through their
    /// bodies.
    ///
    /// # Panics
    ///
    /// Panics if `rls2fga` cannot plan the model at all.
    #[must_use]
    pub fn divergences(&self) -> Vec<TranslationNote> {
        let mut registry = registry(&self.database);
        for (table, column) in OPTIONAL_REFERENCES {
            registry.register_attribute_column(*table, *column);
        }
        for table in self.database.tables() {
            if self.connetto_tables.contains(table.table_name()) {
                continue;
            }
            for column in table.columns(&self.database).expect("the APS columns resolve") {
                if column.column_name() == "archived" || column.column_name().ends_with("_archived")
                {
                    registry.register_attribute_column(table.table_name(), column.column_name());
                }
            }
        }
        registry.declare_helpers(
            self.database
                .functions()
                .filter(|function| {
                    function.target_name().schema().is_none_or(|schema| schema == "public")
                        && function.returns_set()
                        && function.security_mode() == FunctionSecurity::Definer
                })
                .map(|function| format!("public.{}", function.name())),
        );
        let translator = translator(translation::deployment(), registry);
        divergences(&translator, &self.database).expect("rls2fga plans the model")
    }

    /// Allows resource metadata copies and references to shared rows.
    fn allow_references(
        &self,
        constrainer: &mut DefaultConstrainer<ParserDB>,
    ) -> Result<(), LookupError> {
        let database = &self.database;
        let roots = SHARED
            .iter()
            .map(|(table, _reason)| named(database, table))
            .collect::<Result<Vec<_>, _>>()?;
        for host in database.tables() {
            if self.connetto_tables.contains(host.table_name()) {
                continue;
            }
            for foreign_key in host.foreign_keys(database)? {
                let referenced = foreign_key.referenced_table(database)?;
                let mut shared = roots.contains(&referenced)
                    || SHARED_REFERENCES.iter().any(|(table, target, _reason)| {
                        *table == host.table_name() && *target == referenced.table_name()
                    });
                for root in &roots {
                    shared |= referenced.is_descendant_of(database, root)?;
                }
                let resource_key = referenced.table_name() == "resources";
                let mut archive_key =
                    resource_key || foreign_key.on_delete(database) != ReferentialAction::Cascade;
                let mut archive_prefix = None;
                let mut identifier_prefix = None;
                let mut has_archive = false;
                let mut has_identifier = false;
                let mut needs_copy_allowance = false;
                let mut project_key = resource_key;
                let mut column_count = 0;
                let mut tenant_local = false;
                for (column, target) in foreign_key
                    .host_columns(database)?
                    .zip(foreign_key.referenced_columns(database)?)
                {
                    let stored_column = column;
                    let column = column.column_name();
                    let target = target.column_name();
                    column_count += 1;
                    let archive_pair = target == "archived"
                        && ((resource_key && column == "archived")
                            || column.ends_with("_archived"));
                    if archive_pair {
                        archive_prefix = column.strip_suffix("_archived");
                    }
                    if target == "id" {
                        identifier_prefix = column.strip_suffix("_id");
                    }
                    if archive_pair || (column == "organization_id" && target == "organization_id")
                    {
                        let mut independently_keyed =
                            stored_column.foreign_keys(database)?.any(|other| other != foreign_key);
                        if !independently_keyed {
                            for index in host.unique_indices(database)? {
                                if index.columns(database)?.any(|key| key == stored_column) {
                                    independently_keyed = true;
                                    break;
                                }
                            }
                        }
                        needs_copy_allowance |= !independently_keyed;
                    }
                    has_archive |= archive_pair;
                    has_identifier |= target == "id";
                    archive_key &= archive_pair
                        || target == "id"
                        || (column == target && matches!(target, "organization_id" | "kind"));
                    project_key &=
                        matches!((column, target), ("id", "id") | ("project_id", "parent_id"));
                    if shared {
                        tenant_local |= column == "organization_id" && target == "organization_id";
                        tenant_local |= pinned_to_resource(host, column, database)?
                            && pinned_to_resource(referenced, target, database)?;
                    }
                }
                archive_key &= has_archive
                    && has_identifier
                    && (resource_key
                        || (archive_prefix.is_some() && archive_prefix == identifier_prefix));
                project_key =
                    project_key && column_count == 2 && pinned_to_resource(host, "id", database)?;
                if !archive_key && !project_key && (!shared || tenant_local) {
                    continue;
                }
                let name = foreign_key
                    .constraint_name(database)?
                    .expect("PostgreSQL names every foreign key");
                if archive_key {
                    if needs_copy_allowance {
                        // Policies need archive state on the row being checked.
                        constrainer.allow_foreign_key(
                            "copied_column_through_superkey",
                            TargetName::new(host.table_name(), false),
                            &name,
                        );
                    }
                    // Archive copies must follow the referenced row despite its
                    // business-edit policies.
                    constrainer.allow_foreign_key(
                        "cascade_into_row_level_security_table",
                        TargetName::new(host.table_name(), false),
                        &name,
                    );
                }
                if project_key {
                    // The resource primary key and organization key identify
                    // the same tenant row.
                    constrainer.allow_foreign_key(
                        "tenant_local_reference",
                        TargetName::new(host.table_name(), false),
                        &name,
                    );
                    // The project copy follows the resource's parent.
                    constrainer.allow_foreign_key(
                        "cascade_into_row_level_security_table",
                        TargetName::new(host.table_name(), false),
                        &name,
                    );
                }
                if shared && !tenant_local {
                    constrainer.allow_foreign_key(
                        "tenant_local_reference",
                        TargetName::new(host.table_name(), false),
                        &name,
                    );
                }
            }
        }
        Ok(())
    }
}

/// Whether the resource key pins `column` to the table's organization.
fn pinned_to_resource(
    table: &CreateTable,
    column: &str,
    database: &ParserDB,
) -> Result<bool, LookupError> {
    for foreign_key in table.foreign_keys(database)? {
        if foreign_key.referenced_table(database)?.table_name() != "resources" {
            continue;
        }
        let mut pins_organization = false;
        let mut pins_resource = false;
        for (host, target) in
            foreign_key.host_columns(database)?.zip(foreign_key.referenced_columns(database)?)
        {
            pins_organization |= host.column_name() == "organization_id"
                && target.column_name() == "organization_id";
            pins_resource |= host.column_name() == column && target.column_name() == "id";
        }
        if pins_organization && pins_resource {
            return Ok(true);
        }
    }
    Ok(false)
}

/// Tables whose rows an organization may share with another, each with the
/// reason a key into it, or into a table extending it, may name a row of
/// another organization.
const SHARED: &[(&str, &str)] = &[
    (
        "asset_models",
        "An asset, a model or a template may name a model, commercial product or lot another organization shares with the writer",
    ),
    (
        "asset_model_ancestors",
        "A model may extend a model another organization shares, whose ancestry rows belong to that organization",
    ),
    ("brands", "A commercial product may name a brand another organization shares with the writer"),
    (
        "procedure_templates",
        "A template or a run may use a template another organization shares with the writer",
    ),
    (
        "procedure_template_slots",
        "A binding or a run may fill a slot of a template another organization shares",
    ),
    (
        "procedure_template_steps",
        "A run may execute a step of a template another organization shares",
    ),
    (
        "procedure_template_parameters",
        "A value or a step parameter may name a parameter of a template another organization shares",
    ),
];

/// Keys from a table into a table of rows an organization keeps to itself
/// that may still name another organization's row, with the reason.
const SHARED_REFERENCES: &[(&str, &str, &str)] = &[
    (
        "procedure_assets",
        "assets",
        "A run may bind an asset another organization shares with the writer",
    ),
    (
        "asset_moves",
        "assets",
        "A move copies the assets of its run's bindings, which may name an asset or container another organization shares",
    ),
    (
        "container_occupancy",
        "assets",
        "Occupancy copies the container of its placement, which another organization may share",
    ),
    (
        "container_occupancy",
        "container_compatibility_rules",
        "A placement counts against a rule of the container's model or its ancestry, which another organization may own and share",
    ),
    (
        "move_exits",
        "assets",
        "An exit keeps the placement's organization and names its container, which another organization may share",
    ),
    (
        "asset_uses",
        "assets",
        "A use copies the asset of its binding, which another organization may share",
    ),
];

/// Optional references whose policies admit a missing reference with
/// `IS NULL` and otherwise require access to the referenced row.
const OPTIONAL_REFERENCES: &[(&str, &str)] = &[
    ("asset_models", "parent_model_id"),
    ("asset_model_ancestors", "parent_model_id"),
    ("procedure_templates", "previous_version_id"),
    ("procedures", "parent_procedure_id"),
    ("procedures", "parent_template_id"),
    ("procedures", "performed_by_id"),
    ("procedure_assets", "asset_id"),
    ("procedure_assets", "parent_procedure_id"),
];

/// Keys whose existence-oracle findings the APS schema keeps on purpose,
/// grouped by the reason each group stays.
const ORACLES: &[(&str, &[(&str, &str)])] = &[
    (
        "The key names a row another organization shares with the writer, so it cannot carry `organization_id`, and the insert check confines it to rows the writer reads through `authz.resources_with`, which until connetto-schema leaves blocking organizations out of it still admits a row its organization's restrictive policy hides",
        &[
            (
                "asset_model_ancestors",
                "asset_model_ancestors_parent_model_id_ancestor_model_id_fkey",
            ),
            ("asset_models", "asset_models_parent_model_id_fkey"),
            ("asset_models", "asset_models_parent_model_id_parent_model_archived_fkey"),
            ("procedure_assets", "procedure_assets_asset_model_id_slot_asset_model_id_fkey"),
            ("procedure_assets", "procedure_assets_asset_model_id_asset_model_archived_fkey"),
            (
                "procedure_template_slot_bindings",
                "procedure_template_slot_bindi_parent_asset_model_id_child__fkey",
            ),
        ],
    ),
    (
        "The key names a descendant model of the writer's organization, whose required reflexive ancestor row makes it readable through `readable_models`",
        &[(
            "asset_model_ancestors",
            "asset_model_ancestors_descendant_model_id_parent_model_id__fkey",
        )],
    ),
    (
        "The key restates what the closure already proves, since a reflexive row's ancestor is its descendant and any other row's ancestor is the ancestor of the row its own key names",
        &[("asset_model_ancestors", "asset_model_ancestors_ancestor_model_id_fkey")],
    ),
];

/// Unique keys whose existence-oracle findings the APS schema keeps on
/// purpose, grouped by the reason each group stays.
const UNIQUES: &[(&str, &[(&str, &str)])] = &[
    (
        "The offline client mints the row's random UUID, so a collision only confirms an identifier the writer already holds and never discovers one",
        &[
            ("procedure_assets", "procedure_assets_pkey"),
            ("procedure_template_parameters", "procedure_template_parameters_pkey"),
            ("procedure_template_slots", "procedure_template_slots_pkey"),
            ("procedure_template_steps", "procedure_template_steps_pkey"),
            ("procedure_assets", "procedure_assets_id_organization_id_key"),
        ],
    ),
    (
        "An asset is produced once and consumed once across every organization, and the writer already holds `writer` on the asset whose history the collision reveals",
        &[
            ("procedure_assets", "procedure_assets_produced_once"),
            ("procedure_assets", "procedure_assets_consumed_once"),
        ],
    ),
    (
        "The collision names a model of the writer's organization, whose required reflexive ancestor row makes it readable through `readable_models`",
        &[
            ("asset_models", "asset_models_id_parent_model_id_organization_id_key"),
            ("asset_models", "asset_models_id_archived_key"),
        ],
    ),
];

/// Tables whose unique keys repeat NULL on purpose, with the reason.
const REPEATED_NULLS: &[(&str, &str)] =
    &[("procedures", "Every root procedure has no parent and runs no step")];

/// Functions whose findings of a rule the APS schema keeps on purpose, with
/// the reason.
const FUNCTIONS: &[(&str, &str, &str)] = &[
    (
        "cross_tenant_cap",
        "procedure_deviations",
        "A run may execute a template another organization shares with it, so a typed run and its typed template need not share an organization",
    ),
    (
        "cross_tenant_cap",
        "procedure_parameter_deviations",
        "A run may execute a template another organization shares with it, so a recorded value and its template parameter need not share an organization",
    ),
    (
        "cross_tenant_cap",
        "container_compatible",
        "A model may extend a model another organization shares with it, and a compatibility rule may name a shared model, so its ancestry rows and the rules they match need not share an organization",
    ),
    (
        "cross_tenant_cap",
        "require_publishable_procedure_template",
        "A step may run a template another organization shares with it, so a binding and the child slot it fills need not share an organization",
    ),
    (
        "cross_tenant_cap",
        "record_removal_exits",
        "A removal closes an asset's earlier placements in every organization, so the removal's run and the placement need not share an organization",
    ),
    (
        "cross_tenant_cap",
        "refresh_asset_moves",
        "A run may execute a template another organization shares with it, so the run and its template's slots need not share an organization",
    ),
    (
        "cross_tenant_cap",
        "use_compatibility_rules",
        "A model may extend a model another organization shares with it, so its ancestry rows and the compatibility rules they match need not share an organization",
    ),
    (
        "join_on_non_key_columns",
        "guard_procedure",
        "The check pairs each parent asset binding with every child step fulfilling the same slot, since a slot holds several assets and several steps may fill it",
    ),
    (
        "join_on_non_key_columns",
        "require_asset_lifetime",
        "The check pairs every two runs using the same asset to find overlapping uses",
    ),
    (
        "join_on_non_key_columns",
        "require_nested_times",
        "The check pairs every run of a successor step with every run of its predecessor steps under the same parent, since a step may run repeatedly",
    ),
    (
        "join_on_non_key_columns",
        "require_publishable_procedure_template",
        "The check pairs every two steps binding the same parent slot to verify their ordering",
    ),
];

/// Tables whose findings of a rule the APS schema keeps on purpose, with the
/// reason.
const TABLES: &[(&str, &str, &str)] = &[
    (
        "catalog_name_as_text",
        "brands",
        "The `table_names` catalog names tables by text, which the SQLite translation keeps, and the check pins the root row to its table",
    ),
    (
        "catalog_name_as_text",
        "container_compatibility_rules",
        "The `table_names` catalog names tables by text, which the SQLite translation keeps, and the check pins the root row to its table",
    ),
    (
        "singular_table_name",
        "container_occupancy",
        "Occupancy names a container's placement ledger, whose rows each pair a placement with the compatibility rule it counts against",
    ),
];

/// Excuses the findings the APS schema keeps on purpose.
fn allow(constrainer: &mut DefaultConstrainer<ParserDB>) {
    for (rule, groups) in [
        ("cascade_into_row_level_security_table", CASCADES),
        ("foreign_key_existence_oracle", ORACLES),
    ] {
        for (_reason, keys) in groups {
            for (table, foreign_key) in *keys {
                constrainer.allow_foreign_key(rule, TargetName::new(table, false), foreign_key);
            }
        }
    }
    for (_reason, keys) in UNIQUES {
        for (table, index) in *keys {
            constrainer.allow_index(
                "unique_key_existence_oracle",
                TargetName::new(table, false),
                index,
            );
        }
    }
    for (table, _reason) in REPEATED_NULLS {
        constrainer.allow_table("nullable_unique_column", TargetName::new(table, false));
    }
    for (rule, function, _reason) in FUNCTIONS {
        constrainer.allow_function(rule, TargetName::new(function, false));
    }
    for (rule, table, _reason) in TABLES {
        constrainer.allow_table(rule, TargetName::new(table, false));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn archive_keys_pin_resource_organizations() {
        let database = ParserDB::parse::<PostgreSqlDialect>(
            "CREATE TABLE resources (
                id UUID PRIMARY KEY,
                organization_id UUID NOT NULL,
                kind TEXT NOT NULL,
                archived BOOLEAN NOT NULL,
                UNIQUE (organization_id, id, archived),
                UNIQUE (organization_id, id, kind, archived)
            );
            CREATE TABLE subtype (
                id UUID PRIMARY KEY,
                organization_id UUID NOT NULL,
                archived BOOLEAN NOT NULL,
                FOREIGN KEY (organization_id, id, archived)
                    REFERENCES resources (organization_id, id, archived)
            );
            CREATE TABLE root (
                id UUID PRIMARY KEY,
                organization_id UUID NOT NULL,
                kind TEXT NOT NULL,
                archived BOOLEAN NOT NULL,
                FOREIGN KEY (organization_id, id, kind, archived)
                    REFERENCES resources (organization_id, id, kind, archived)
            );
            CREATE TABLE unpinned (
                id UUID PRIMARY KEY REFERENCES resources (id),
                organization_id UUID NOT NULL
            );",
        )
        .expect("the resource key schema parses");
        for table_name in ["subtype", "root"] {
            let table = named(&database, table_name).expect("the resource table exists");
            assert!(pinned_to_resource(table, "id", &database).expect("the key resolves"));
            assert!(
                !pinned_to_resource(table, "organization_id", &database).expect("the key resolves")
            );
        }
        let table = named(&database, "unpinned").expect("the unpinned table exists");
        assert!(!pinned_to_resource(table, "id", &database).expect("the key resolves"));
    }

    #[test]
    fn archive_key_allowances_match_reference_shapes() {
        let cases = [
            (
                "reference_copy",
                "FOREIGN KEY (model_id, model_archived) REFERENCES targets (id, archived) ON UPDATE CASCADE",
                true,
            ),
            (
                "resource_copy",
                "FOREIGN KEY (organization_id, id, archived) REFERENCES resources (organization_id, id, archived) ON UPDATE CASCADE ON DELETE CASCADE",
                true,
            ),
            (
                "resource_kind_copy",
                "FOREIGN KEY (organization_id, id, kind, archived) REFERENCES resources (organization_id, id, kind, archived) ON UPDATE CASCADE ON DELETE CASCADE",
                true,
            ),
            (
                "resource_reference_copy",
                "FOREIGN KEY (organization_id, model_id, model_archived) REFERENCES resources (organization_id, id, archived) ON UPDATE CASCADE",
                true,
            ),
            (
                "resource_parent_copy",
                "FOREIGN KEY (organization_id, procedure_id, archived) REFERENCES resources (organization_id, id, archived) ON UPDATE CASCADE",
                true,
            ),
            (
                "mismatched_prefix",
                "FOREIGN KEY (asset_id, model_archived) REFERENCES targets (id, archived) ON UPDATE CASCADE",
                false,
            ),
            (
                "bare_archive",
                "FOREIGN KEY (id, archived) REFERENCES targets (id, archived) ON UPDATE CASCADE",
                false,
            ),
            (
                "delete_cascade",
                "FOREIGN KEY (model_id, model_archived) REFERENCES targets (id, archived) ON UPDATE CASCADE ON DELETE CASCADE",
                false,
            ),
            (
                "missing_identifier",
                "FOREIGN KEY (model_archived) REFERENCES targets (archived) ON UPDATE CASCADE",
                false,
            ),
        ];
        let mut mismatches = Vec::new();
        for (case, foreign_key, allowed) in cases {
            let mut sql = String::from(
                "CREATE TABLE resources (
                    id UUID PRIMARY KEY, organization_id UUID NOT NULL,
                    kind TEXT NOT NULL, archived BOOLEAN NOT NULL,
                    UNIQUE (organization_id, id, archived),
                    UNIQUE (organization_id, id, kind, archived)
                );
                CREATE TABLE targets (
                    id UUID PRIMARY KEY, archived BOOLEAN NOT NULL UNIQUE,
                    UNIQUE (id, archived)
                );",
            );
            for (table, _) in SHARED {
                sql.push_str(&format!("CREATE TABLE {table} (id UUID PRIMARY KEY);"));
            }
            sql.push_str(&format!(
                "CREATE TABLE copies (
                    id UUID PRIMARY KEY, organization_id UUID NOT NULL,
                    kind TEXT NOT NULL, archived BOOLEAN NOT NULL,
                    model_id UUID, asset_id UUID, procedure_id UUID,
                    model_archived BOOLEAN NOT NULL,
                    CONSTRAINT archive_copy {foreign_key}
                );
                ALTER TABLE copies ENABLE ROW LEVEL SECURITY;
                CREATE POLICY copies_update ON copies FOR UPDATE USING (false);
                CREATE POLICY copies_delete ON copies FOR DELETE USING (false);"
            ));
            let schema = Schema {
                cluster: ParserDB::parse::<PostgreSqlDialect>("").expect("the cluster parses"),
                database: ParserDB::parse::<PostgreSqlDialect>(&sql)
                    .expect("the key schema parses"),
                connetto_tables: HashSet::new(),
            };
            let mut constrainer = DefaultConstrainer::default();
            constrainer.set_group_level(
                sqlippy::prelude::LintGroup::Pedantic,
                sqlippy::prelude::Level::Warn,
            );
            schema.allow_references(&mut constrainer).expect("the references resolve");
            let reports_cascade = constrainer.diagnostics(&schema.database).iter().any(|error| {
                matches!(error, Error::Violation(violation)
                    if violation.rule().name == "cascade_into_row_level_security_table")
            });
            if reports_cascade == allowed {
                mismatches.push(case);
            }
        }
        assert!(mismatches.is_empty(), "unexpected archive allowances for {mismatches:?}");
    }
}
