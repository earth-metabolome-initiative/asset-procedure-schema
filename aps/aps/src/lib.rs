pub use aps_assets::{
    self, Asset, AssetMove, AssetMoveKind, AssetUse, Container, ContainerOccupancy, DigitalAsset,
    MoveExit, Panel, PanelModel, Photograph, PhysicalAsset, ProcedureAsset, VolumetricContainer,
    asset_move_kinds, asset_moves, asset_uses, assets, container_occupancy, containers,
    digital_assets, move_exits, panel_models, panels, photographs, physical_assets,
    procedure_assets, volumetric_containers,
};
pub use aps_audit_events::{self, AuditEvent, audit_events};
pub use aps_ball_mill_machine_models::{
    self, BallMillMachine, BallMillMachineModel, BallMillProcedure, BallMillProcedureTemplate,
    BeadModel, CommercialBallMillMachineLot, CommercialBallMillMachineModel,
    ball_mill_machine_models, ball_mill_machines, ball_mill_procedure_templates,
    ball_mill_procedures, bead_models, commercial_ball_mill_machine_lots,
    commercial_ball_mill_machine_models,
};
pub use aps_centrifuge_models::{
    self, Centrifuge, CentrifugeModel, CentrifugeProcedure, CentrifugeProcedureTemplate,
    CommercialCentrifugeLot, CommercialCentrifugeModel, centrifuge_models,
    centrifuge_procedure_templates, centrifuge_procedures, centrifuges, commercial_centrifuge_lots,
    commercial_centrifuge_models,
};
pub use aps_commercial_freeze_dryer_models::{
    self, CommercialFreezeDryerLot, CommercialFreezeDryerModel, FreezeDryer,
    commercial_freeze_dryer_lots, commercial_freeze_dryer_models, freeze_dryers,
};
pub use aps_commercial_freezer_models::{
    self, CommercialFreezerLot, CommercialFreezerModel, Freezer, commercial_freezer_lots,
    commercial_freezer_models, freezers,
};
pub use aps_commercial_products::{
    self, Brand, CameraModel, CommercialCameraLot, CommercialCameraModel,
    CommercialGeopositioningDeviceLot, CommercialGeopositioningDeviceModel, CommercialMarkerLot,
    CommercialMarkerModel, CommercialMassSpectrometerLot, CommercialMassSpectrometerModel,
    CommercialPersonalProtectiveEquipmentLot, CommercialPersonalProtectiveEquipmentModel,
    CommercialPhoneDeviceLot, CommercialPhoneDeviceModel, CommercialPipetteTipLot,
    CommercialPipetteTipModel, CommercialProduct, CommercialProductLot,
    CommercialVolumeMeasuringDeviceLot, CommercialVolumeMeasuringDeviceModel,
    CommercialWeighingDeviceLot, CommercialWeighingDeviceModel, GeopositioningDeviceModel,
    GeopositioningProcedure, GeopositioningProcedureTemplate, Marker, MarkerModel,
    MassSpectrometer, MassSpectrometerModel, PersonalProtectiveEquipment,
    PersonalProtectiveEquipmentModel, PhoneDeviceModel, PipetteTip, PipetteTipModel, brands,
    camera_models, commercial_camera_lots, commercial_camera_models,
    commercial_geopositioning_device_lots, commercial_geopositioning_device_models,
    commercial_marker_lots, commercial_marker_models, commercial_mass_spectrometer_lots,
    commercial_mass_spectrometer_models, commercial_personal_protective_equipment_lots,
    commercial_personal_protective_equipment_models, commercial_phone_device_lots,
    commercial_phone_device_models, commercial_pipette_tip_lots, commercial_pipette_tip_models,
    commercial_product_lots, commercial_products, commercial_volume_measuring_device_lots,
    commercial_volume_measuring_device_models, commercial_weighing_device_lots,
    commercial_weighing_device_models, geopositioning_device_models,
    geopositioning_procedure_templates, geopositioning_procedures, marker_models, markers,
    mass_spectrometer_models, mass_spectrometers, personal_protective_equipment_models,
    personal_protective_equipments, phone_device_models, pipette_tip_models, pipette_tips,
};
pub use aps_geopositioning_devices::{
    self, Camera, GeopositioningDevice, PhoneDevice, cameras, geopositioning_devices, phone_devices,
};
pub use aps_grantees::{
    self, App, AppInstallation, Grantee, InstallationPermission, Invitation, Membership, Namespace,
    Organization, OrganizationBlock, PlatformOperator, ShareLink, Team, TeamMember, User,
    app_installations, apps, grantees, installation_permissions, invitations, memberships,
    namespaces, organization_blocks, organizations, platform_operators, share_links, team_members,
    teams, users,
};
pub use aps_photograph_procedure_templates::{
    self, PhotographProcedure, PhotographProcedureTemplate, photograph_procedure_templates,
    photograph_procedures,
};
pub use aps_physical_asset_models::{
    self, ContainerCompatibilityRule, ContainerModel, PhysicalAssetModel,
    RetrievalProcedureTemplate, StorageProcedureTemplate, VolumetricContainerModel,
    WeighedContainerModel, WeighedVolumetricContainerModel, container_compatibility_rules,
    container_models, physical_asset_models, retrieval_procedure_templates,
    storage_procedure_templates, volumetric_container_models, weighed_container_models,
    weighed_volumetric_container_models,
};
pub use aps_platform_audit_events::{self, PlatformAuditEvent, platform_audit_events};
pub use aps_procedures::{
    self, ContainerSealerModel, ContainerSealingProcedure, ContainerSealingProcedureTemplate,
    DisposalProcedure, DisposalProcedureTemplate, FreezeDryerModel, FreezeDryingProcedure,
    FreezeDryingProcedureTemplate, FreezerModel, FreezingProcedure, FreezingProcedureTemplate,
    ParameterValueKind, Procedure, ProcedureParameterValue, ProcedureStatus,
    ProcedureTemplateParameter, ProcedureTemplateStepParameter, QuantityKind, RetrievalProcedure,
    StorageProcedure, container_sealer_models, container_sealing_procedure_templates,
    container_sealing_procedures, disposal_procedure_templates, disposal_procedures,
    freeze_dryer_models, freeze_drying_procedure_templates, freeze_drying_procedures,
    freezer_models, freezing_procedure_templates, freezing_procedures, parameter_value_kinds,
    procedure_parameter_values, procedure_statuses, procedure_template_parameters,
    procedure_template_step_parameters, procedures, quantity_kinds, retrieval_procedures,
    storage_procedures,
};
pub use aps_resources::{
    self, AssetModel, AssetModelAncestor, DigitalAssetModel, Grant, PhotographModel,
    ProcedureTemplate, ProcedureTemplateSlot, ProcedureTemplateSlotBinding, ProcedureTemplateState,
    ProcedureTemplateStep, ProcedureTemplateStepEdge, Resource, SlotRole, SlotRolePair, TableName,
    TableNameAdmittedParent, TableNameAncestor, asset_model_ancestors, asset_models,
    digital_asset_models, grants, photograph_models, procedure_template_slot_bindings,
    procedure_template_slots, procedure_template_states, procedure_template_step_edges,
    procedure_template_steps, procedure_templates, resources, slot_role_pairs, slot_roles,
    table_name_admitted_parents, table_name_ancestors, table_names,
};
pub use aps_sample_models::{
    self, AliquotingProcedure, AliquotingProcedureTemplate, HarvestingProcedure,
    HarvestingProcedureTemplate, PackagingModel, PackagingProcedure, PackagingProcedureTemplate,
    PouringProcedure, PouringProcedureTemplate, Sample, SampleModel, SampleSource,
    SampleSourceModel, SupernatantProcedure, SupernatantProcedureTemplate, VolumeMeasuringDevice,
    VolumeMeasuringDeviceModel, aliquoting_procedure_templates, aliquoting_procedures,
    harvesting_procedure_templates, harvesting_procedures, packaging_models,
    packaging_procedure_templates, packaging_procedures, pouring_procedure_templates,
    pouring_procedures, sample_models, sample_source_models, sample_sources, samples,
    supernatant_procedure_templates, supernatant_procedures, volume_measuring_device_models,
    volume_measuring_devices,
};
pub use aps_weighing_device_models::{
    self, FractioningProcedure, FractioningProcedureTemplate, WeighingDevice, WeighingDeviceModel,
    WeighingProcedure, WeighingProcedureTemplate, fractioning_procedure_templates,
    fractioning_procedures, weighing_device_models, weighing_devices, weighing_procedure_templates,
    weighing_procedures,
};
/// The shape of the typed procedure table named `table_name_id`.
#[must_use]
pub fn procedure_shape(
    table_name_id: &str,
) -> Option<&'static procedure_like::shape::ProcedureShape> {
    match table_name_id {
        "aliquoting_procedures" => Some(&aps_sample_models::ALIQUOTING_PROCEDURES_SHAPE),
        "ball_mill_procedures" => Some(&aps_ball_mill_machine_models::BALL_MILL_PROCEDURES_SHAPE),
        "centrifuge_procedures" => Some(&aps_centrifuge_models::CENTRIFUGE_PROCEDURES_SHAPE),
        "container_sealing_procedures" => Some(&aps_procedures::CONTAINER_SEALING_PROCEDURES_SHAPE),
        "disposal_procedures" => Some(&aps_procedures::DISPOSAL_PROCEDURES_SHAPE),
        "fractioning_procedures" => Some(&aps_weighing_device_models::FRACTIONING_PROCEDURES_SHAPE),
        "freeze_drying_procedures" => Some(&aps_procedures::FREEZE_DRYING_PROCEDURES_SHAPE),
        "freezing_procedures" => Some(&aps_procedures::FREEZING_PROCEDURES_SHAPE),
        "geopositioning_procedures" => {
            Some(&aps_commercial_products::GEOPOSITIONING_PROCEDURES_SHAPE)
        }
        "harvesting_procedures" => Some(&aps_sample_models::HARVESTING_PROCEDURES_SHAPE),
        "packaging_procedures" => Some(&aps_sample_models::PACKAGING_PROCEDURES_SHAPE),
        "photograph_procedures" => {
            Some(&aps_photograph_procedure_templates::PHOTOGRAPH_PROCEDURES_SHAPE)
        }
        "pouring_procedures" => Some(&aps_sample_models::POURING_PROCEDURES_SHAPE),
        "retrieval_procedures" => Some(&aps_procedures::RETRIEVAL_PROCEDURES_SHAPE),
        "storage_procedures" => Some(&aps_procedures::STORAGE_PROCEDURES_SHAPE),
        "supernatant_procedures" => Some(&aps_sample_models::SUPERNATANT_PROCEDURES_SHAPE),
        "weighing_procedures" => Some(&aps_weighing_device_models::WEIGHING_PROCEDURES_SHAPE),
        _ => None,
    }
}
/// The shape of the typed template table named `table_name_id`.
#[must_use]
pub fn template_shape(
    table_name_id: &str,
) -> Option<&'static procedure_like::shape::TemplateShape> {
    match table_name_id {
        "aliquoting_procedure_templates" => {
            Some(&aps_sample_models::ALIQUOTING_PROCEDURE_TEMPLATES_SHAPE)
        }
        "ball_mill_procedure_templates" => {
            Some(&aps_ball_mill_machine_models::BALL_MILL_PROCEDURE_TEMPLATES_SHAPE)
        }
        "centrifuge_procedure_templates" => {
            Some(&aps_centrifuge_models::CENTRIFUGE_PROCEDURE_TEMPLATES_SHAPE)
        }
        "container_sealing_procedure_templates" => {
            Some(&aps_procedures::CONTAINER_SEALING_PROCEDURE_TEMPLATES_SHAPE)
        }
        "disposal_procedure_templates" => Some(&aps_procedures::DISPOSAL_PROCEDURE_TEMPLATES_SHAPE),
        "fractioning_procedure_templates" => {
            Some(&aps_weighing_device_models::FRACTIONING_PROCEDURE_TEMPLATES_SHAPE)
        }
        "freeze_drying_procedure_templates" => {
            Some(&aps_procedures::FREEZE_DRYING_PROCEDURE_TEMPLATES_SHAPE)
        }
        "freezing_procedure_templates" => Some(&aps_procedures::FREEZING_PROCEDURE_TEMPLATES_SHAPE),
        "geopositioning_procedure_templates" => {
            Some(&aps_commercial_products::GEOPOSITIONING_PROCEDURE_TEMPLATES_SHAPE)
        }
        "harvesting_procedure_templates" => {
            Some(&aps_sample_models::HARVESTING_PROCEDURE_TEMPLATES_SHAPE)
        }
        "packaging_procedure_templates" => {
            Some(&aps_sample_models::PACKAGING_PROCEDURE_TEMPLATES_SHAPE)
        }
        "photograph_procedure_templates" => {
            Some(&aps_photograph_procedure_templates::PHOTOGRAPH_PROCEDURE_TEMPLATES_SHAPE)
        }
        "pouring_procedure_templates" => {
            Some(&aps_sample_models::POURING_PROCEDURE_TEMPLATES_SHAPE)
        }
        "retrieval_procedure_templates" => {
            Some(&aps_physical_asset_models::RETRIEVAL_PROCEDURE_TEMPLATES_SHAPE)
        }
        "storage_procedure_templates" => {
            Some(&aps_physical_asset_models::STORAGE_PROCEDURE_TEMPLATES_SHAPE)
        }
        "supernatant_procedure_templates" => {
            Some(&aps_sample_models::SUPERNATANT_PROCEDURE_TEMPLATES_SHAPE)
        }
        "weighing_procedure_templates" => {
            Some(&aps_weighing_device_models::WEIGHING_PROCEDURE_TEMPLATES_SHAPE)
        }
        _ => None,
    }
}
/// The shape of the asset model table named `table_name_id`.
#[must_use]
pub fn asset_model_shape(
    table_name_id: &str,
) -> Option<&'static procedure_like::shape::AssetModelShape> {
    match table_name_id {
        "ball_mill_machine_models" => {
            Some(&aps_ball_mill_machine_models::BALL_MILL_MACHINE_MODELS_SHAPE)
        }
        "bead_models" => Some(&aps_ball_mill_machine_models::BEAD_MODELS_SHAPE),
        "camera_models" => Some(&aps_commercial_products::CAMERA_MODELS_SHAPE),
        "centrifuge_models" => Some(&aps_centrifuge_models::CENTRIFUGE_MODELS_SHAPE),
        "commercial_ball_mill_machine_lots" => {
            Some(&aps_ball_mill_machine_models::COMMERCIAL_BALL_MILL_MACHINE_LOTS_SHAPE)
        }
        "commercial_ball_mill_machine_models" => {
            Some(&aps_ball_mill_machine_models::COMMERCIAL_BALL_MILL_MACHINE_MODELS_SHAPE)
        }
        "commercial_camera_lots" => Some(&aps_commercial_products::COMMERCIAL_CAMERA_LOTS_SHAPE),
        "commercial_camera_models" => {
            Some(&aps_commercial_products::COMMERCIAL_CAMERA_MODELS_SHAPE)
        }
        "commercial_centrifuge_lots" => {
            Some(&aps_centrifuge_models::COMMERCIAL_CENTRIFUGE_LOTS_SHAPE)
        }
        "commercial_centrifuge_models" => {
            Some(&aps_centrifuge_models::COMMERCIAL_CENTRIFUGE_MODELS_SHAPE)
        }
        "commercial_freeze_dryer_lots" => {
            Some(&aps_commercial_freeze_dryer_models::COMMERCIAL_FREEZE_DRYER_LOTS_SHAPE)
        }
        "commercial_freeze_dryer_models" => {
            Some(&aps_commercial_freeze_dryer_models::COMMERCIAL_FREEZE_DRYER_MODELS_SHAPE)
        }
        "commercial_freezer_lots" => {
            Some(&aps_commercial_freezer_models::COMMERCIAL_FREEZER_LOTS_SHAPE)
        }
        "commercial_freezer_models" => {
            Some(&aps_commercial_freezer_models::COMMERCIAL_FREEZER_MODELS_SHAPE)
        }
        "commercial_geopositioning_device_lots" => {
            Some(&aps_commercial_products::COMMERCIAL_GEOPOSITIONING_DEVICE_LOTS_SHAPE)
        }
        "commercial_geopositioning_device_models" => {
            Some(&aps_commercial_products::COMMERCIAL_GEOPOSITIONING_DEVICE_MODELS_SHAPE)
        }
        "commercial_marker_lots" => Some(&aps_commercial_products::COMMERCIAL_MARKER_LOTS_SHAPE),
        "commercial_marker_models" => {
            Some(&aps_commercial_products::COMMERCIAL_MARKER_MODELS_SHAPE)
        }
        "commercial_mass_spectrometer_lots" => {
            Some(&aps_commercial_products::COMMERCIAL_MASS_SPECTROMETER_LOTS_SHAPE)
        }
        "commercial_mass_spectrometer_models" => {
            Some(&aps_commercial_products::COMMERCIAL_MASS_SPECTROMETER_MODELS_SHAPE)
        }
        "commercial_personal_protective_equipment_lots" => {
            Some(&aps_commercial_products::COMMERCIAL_PERSONAL_PROTECTIVE_EQUIPMENT_LOTS_SHAPE)
        }
        "commercial_personal_protective_equipment_models" => {
            Some(&aps_commercial_products::COMMERCIAL_PERSONAL_PROTECTIVE_EQUIPMENT_MODELS_SHAPE)
        }
        "commercial_phone_device_lots" => {
            Some(&aps_commercial_products::COMMERCIAL_PHONE_DEVICE_LOTS_SHAPE)
        }
        "commercial_phone_device_models" => {
            Some(&aps_commercial_products::COMMERCIAL_PHONE_DEVICE_MODELS_SHAPE)
        }
        "commercial_pipette_tip_lots" => {
            Some(&aps_commercial_products::COMMERCIAL_PIPETTE_TIP_LOTS_SHAPE)
        }
        "commercial_pipette_tip_models" => {
            Some(&aps_commercial_products::COMMERCIAL_PIPETTE_TIP_MODELS_SHAPE)
        }
        "commercial_product_lots" => Some(&aps_commercial_products::COMMERCIAL_PRODUCT_LOTS_SHAPE),
        "commercial_volume_measuring_device_lots" => {
            Some(&aps_commercial_products::COMMERCIAL_VOLUME_MEASURING_DEVICE_LOTS_SHAPE)
        }
        "commercial_volume_measuring_device_models" => {
            Some(&aps_commercial_products::COMMERCIAL_VOLUME_MEASURING_DEVICE_MODELS_SHAPE)
        }
        "commercial_weighing_device_lots" => {
            Some(&aps_commercial_products::COMMERCIAL_WEIGHING_DEVICE_LOTS_SHAPE)
        }
        "commercial_weighing_device_models" => {
            Some(&aps_commercial_products::COMMERCIAL_WEIGHING_DEVICE_MODELS_SHAPE)
        }
        "container_models" => Some(&aps_physical_asset_models::CONTAINER_MODELS_SHAPE),
        "container_sealer_models" => Some(&aps_procedures::CONTAINER_SEALER_MODELS_SHAPE),
        "digital_asset_models" => Some(&aps_resources::DIGITAL_ASSET_MODELS_SHAPE),
        "freeze_dryer_models" => Some(&aps_procedures::FREEZE_DRYER_MODELS_SHAPE),
        "freezer_models" => Some(&aps_procedures::FREEZER_MODELS_SHAPE),
        "geopositioning_device_models" => {
            Some(&aps_commercial_products::GEOPOSITIONING_DEVICE_MODELS_SHAPE)
        }
        "marker_models" => Some(&aps_commercial_products::MARKER_MODELS_SHAPE),
        "mass_spectrometer_models" => {
            Some(&aps_commercial_products::MASS_SPECTROMETER_MODELS_SHAPE)
        }
        "packaging_models" => Some(&aps_sample_models::PACKAGING_MODELS_SHAPE),
        "panel_models" => Some(&aps_assets::PANEL_MODELS_SHAPE),
        "personal_protective_equipment_models" => {
            Some(&aps_commercial_products::PERSONAL_PROTECTIVE_EQUIPMENT_MODELS_SHAPE)
        }
        "phone_device_models" => Some(&aps_commercial_products::PHONE_DEVICE_MODELS_SHAPE),
        "photograph_models" => Some(&aps_resources::PHOTOGRAPH_MODELS_SHAPE),
        "physical_asset_models" => Some(&aps_physical_asset_models::PHYSICAL_ASSET_MODELS_SHAPE),
        "pipette_tip_models" => Some(&aps_commercial_products::PIPETTE_TIP_MODELS_SHAPE),
        "sample_models" => Some(&aps_sample_models::SAMPLE_MODELS_SHAPE),
        "sample_source_models" => Some(&aps_sample_models::SAMPLE_SOURCE_MODELS_SHAPE),
        "volume_measuring_device_models" => {
            Some(&aps_sample_models::VOLUME_MEASURING_DEVICE_MODELS_SHAPE)
        }
        "volumetric_container_models" => {
            Some(&aps_physical_asset_models::VOLUMETRIC_CONTAINER_MODELS_SHAPE)
        }
        "weighed_container_models" => {
            Some(&aps_physical_asset_models::WEIGHED_CONTAINER_MODELS_SHAPE)
        }
        "weighed_volumetric_container_models" => {
            Some(&aps_physical_asset_models::WEIGHED_VOLUMETRIC_CONTAINER_MODELS_SHAPE)
        }
        "weighing_device_models" => Some(&aps_weighing_device_models::WEIGHING_DEVICE_MODELS_SHAPE),
        _ => None,
    }
}
/// Writes and reads the rows of the typed procedure, template and asset
/// tables a runner knows only by `table_name_id`.
pub trait TypedRows {
    /// Inserts a procedure of the typed procedure table `table_name_id`
    /// with its `resources` and `procedures` rows.
    ///
    /// # Errors
    ///
    /// Returns an error if no typed procedure table has that name, a
    /// builder refuses a value or the connection refuses a row.
    fn insert_typed_procedure(
        &mut self,
        table_name_id: &str,
        columns: &::procedure_like::rows::ProcedureColumns,
    ) -> Result<(), ::procedure_like::rows::TypedRowError>;
    /// Sets or clears the measurements of procedure `id` of the typed
    /// procedure table `table_name_id`, keyed by
    /// `ColumnShape::name`. A procedure takes them while
    /// `in_progress`, before it is set `completed`.
    ///
    /// # Errors
    ///
    /// Returns an error if no typed procedure table has that name, it
    /// has no measurement of a given name, a value does not fit its
    /// column or the connection refuses the update.
    fn set_measurements(
        &mut self,
        table_name_id: &str,
        id: ::rosetta_uuid::Uuid,
        values: &[(&str, Option<::procedure_like::rows::ColumnValue>)],
    ) -> Result<(), ::procedure_like::rows::TypedRowError>;
    /// The measurements of procedure `id` of the typed procedure table
    /// `table_name_id`, or the setpoints of template `id` of the typed
    /// template table `table_name_id`, keyed by `ColumnShape::name`.
    ///
    /// # Errors
    ///
    /// Returns an error if no typed procedure or template table has
    /// that name or the connection finds no row.
    fn typed_values(
        &mut self,
        table_name_id: &str,
        id: ::rosetta_uuid::Uuid,
    ) -> Result<
        Vec<(&'static str, Option<::procedure_like::rows::ColumnValue>)>,
        ::procedure_like::rows::TypedRowError,
    >;
    /// Inserts an asset of the asset table `asset_table_name_id` with
    /// its ancestor rows, filling the required columns `columns` does
    /// not cover from `values`, keyed by `ColumnShape::name`.
    ///
    /// # Errors
    ///
    /// Returns an error if no asset table has that name, a required
    /// column has no value, a value names no required column or does
    /// not fit it, a builder refuses a value or the connection refuses
    /// a row.
    fn insert_produced_asset(
        &mut self,
        asset_table_name_id: &str,
        columns: &::procedure_like::rows::AssetColumns,
        values: &[(&str, ::procedure_like::rows::ColumnValue)],
    ) -> Result<(), ::procedure_like::rows::TypedRowError>;
}
impl<C> TypedRows for C
where
    C: ::diesel::Connection,
    ::diesel_builders::TableBuilder<crate::aliquoting_procedures::table>:
        ::diesel_builders::Insert<C, ValidationError: Into<::procedure_like::rows::TypedRowError>>,
    ::diesel::dsl::Update<
        ::diesel::dsl::Filter<
            crate::aliquoting_procedures::table,
            ::diesel::dsl::Eq<crate::aliquoting_procedures::id, ::rosetta_uuid::Uuid>,
        >,
        ::diesel::dsl::Eq<crate::aliquoting_procedures::volume_l, Option<f64>>,
    >: ::diesel::query_dsl::methods::ExecuteDsl<C>,
    ::diesel_builders::TableBuilder<crate::ball_mill_procedures::table>:
        ::diesel_builders::Insert<C, ValidationError: Into<::procedure_like::rows::TypedRowError>>,
    ::diesel::dsl::Update<
        ::diesel::dsl::Filter<
            crate::ball_mill_procedures::table,
            ::diesel::dsl::Eq<crate::ball_mill_procedures::id, ::rosetta_uuid::Uuid>,
        >,
        ::diesel::dsl::Eq<crate::ball_mill_procedures::temperature_k, Option<f64>>,
    >: ::diesel::query_dsl::methods::ExecuteDsl<C>,
    ::diesel::dsl::Update<
        ::diesel::dsl::Filter<
            crate::ball_mill_procedures::table,
            ::diesel::dsl::Eq<crate::ball_mill_procedures::id, ::rosetta_uuid::Uuid>,
        >,
        ::diesel::dsl::Eq<crate::ball_mill_procedures::frequency_hz, Option<f64>>,
    >: ::diesel::query_dsl::methods::ExecuteDsl<C>,
    ::diesel_builders::TableBuilder<crate::centrifuge_procedures::table>:
        ::diesel_builders::Insert<C, ValidationError: Into<::procedure_like::rows::TypedRowError>>,
    ::diesel::dsl::Update<
        ::diesel::dsl::Filter<
            crate::centrifuge_procedures::table,
            ::diesel::dsl::Eq<crate::centrifuge_procedures::id, ::rosetta_uuid::Uuid>,
        >,
        ::diesel::dsl::Eq<crate::centrifuge_procedures::temperature_k, Option<f64>>,
    >: ::diesel::query_dsl::methods::ExecuteDsl<C>,
    ::diesel::dsl::Update<
        ::diesel::dsl::Filter<
            crate::centrifuge_procedures::table,
            ::diesel::dsl::Eq<crate::centrifuge_procedures::id, ::rosetta_uuid::Uuid>,
        >,
        ::diesel::dsl::Eq<crate::centrifuge_procedures::speed_rpm, Option<f64>>,
    >: ::diesel::query_dsl::methods::ExecuteDsl<C>,
    ::diesel_builders::TableBuilder<crate::container_sealing_procedures::table>:
        ::diesel_builders::Insert<C, ValidationError: Into<::procedure_like::rows::TypedRowError>>,
    ::diesel_builders::TableBuilder<crate::disposal_procedures::table>:
        ::diesel_builders::Insert<C, ValidationError: Into<::procedure_like::rows::TypedRowError>>,
    ::diesel_builders::TableBuilder<crate::fractioning_procedures::table>:
        ::diesel_builders::Insert<C, ValidationError: Into<::procedure_like::rows::TypedRowError>>,
    ::diesel::dsl::Update<
        ::diesel::dsl::Filter<
            crate::fractioning_procedures::table,
            ::diesel::dsl::Eq<crate::fractioning_procedures::id, ::rosetta_uuid::Uuid>,
        >,
        ::diesel::dsl::Eq<crate::fractioning_procedures::mass_kg, Option<f64>>,
    >: ::diesel::query_dsl::methods::ExecuteDsl<C>,
    ::diesel_builders::TableBuilder<crate::freeze_drying_procedures::table>:
        ::diesel_builders::Insert<C, ValidationError: Into<::procedure_like::rows::TypedRowError>>,
    ::diesel::dsl::Update<
        ::diesel::dsl::Filter<
            crate::freeze_drying_procedures::table,
            ::diesel::dsl::Eq<crate::freeze_drying_procedures::id, ::rosetta_uuid::Uuid>,
        >,
        ::diesel::dsl::Eq<crate::freeze_drying_procedures::temperature_k, Option<f64>>,
    >: ::diesel::query_dsl::methods::ExecuteDsl<C>,
    ::diesel::dsl::Update<
        ::diesel::dsl::Filter<
            crate::freeze_drying_procedures::table,
            ::diesel::dsl::Eq<crate::freeze_drying_procedures::id, ::rosetta_uuid::Uuid>,
        >,
        ::diesel::dsl::Eq<crate::freeze_drying_procedures::pressure_pa, Option<f64>>,
    >: ::diesel::query_dsl::methods::ExecuteDsl<C>,
    ::diesel_builders::TableBuilder<crate::freezing_procedures::table>:
        ::diesel_builders::Insert<C, ValidationError: Into<::procedure_like::rows::TypedRowError>>,
    ::diesel::dsl::Update<
        ::diesel::dsl::Filter<
            crate::freezing_procedures::table,
            ::diesel::dsl::Eq<crate::freezing_procedures::id, ::rosetta_uuid::Uuid>,
        >,
        ::diesel::dsl::Eq<crate::freezing_procedures::temperature_k, Option<f64>>,
    >: ::diesel::query_dsl::methods::ExecuteDsl<C>,
    ::diesel_builders::TableBuilder<crate::geopositioning_procedures::table>:
        ::diesel_builders::Insert<C, ValidationError: Into<::procedure_like::rows::TypedRowError>>,
    ::diesel::dsl::Update<
        ::diesel::dsl::Filter<
            crate::geopositioning_procedures::table,
            ::diesel::dsl::Eq<crate::geopositioning_procedures::id, ::rosetta_uuid::Uuid>,
        >,
        ::diesel::dsl::Eq<
            crate::geopositioning_procedures::location,
            Option<::sqlitegis::diesel::GeographyValue>,
        >,
    >: ::diesel::query_dsl::methods::ExecuteDsl<C>,
    ::diesel_builders::TableBuilder<crate::harvesting_procedures::table>:
        ::diesel_builders::Insert<C, ValidationError: Into<::procedure_like::rows::TypedRowError>>,
    ::diesel_builders::TableBuilder<crate::packaging_procedures::table>:
        ::diesel_builders::Insert<C, ValidationError: Into<::procedure_like::rows::TypedRowError>>,
    ::diesel_builders::TableBuilder<crate::photograph_procedures::table>:
        ::diesel_builders::Insert<C, ValidationError: Into<::procedure_like::rows::TypedRowError>>,
    ::diesel_builders::TableBuilder<crate::pouring_procedures::table>:
        ::diesel_builders::Insert<C, ValidationError: Into<::procedure_like::rows::TypedRowError>>,
    ::diesel::dsl::Update<
        ::diesel::dsl::Filter<
            crate::pouring_procedures::table,
            ::diesel::dsl::Eq<crate::pouring_procedures::id, ::rosetta_uuid::Uuid>,
        >,
        ::diesel::dsl::Eq<crate::pouring_procedures::volume_l, Option<f64>>,
    >: ::diesel::query_dsl::methods::ExecuteDsl<C>,
    ::diesel_builders::TableBuilder<crate::retrieval_procedures::table>:
        ::diesel_builders::Insert<C, ValidationError: Into<::procedure_like::rows::TypedRowError>>,
    ::diesel_builders::TableBuilder<crate::storage_procedures::table>:
        ::diesel_builders::Insert<C, ValidationError: Into<::procedure_like::rows::TypedRowError>>,
    ::diesel::dsl::Update<
        ::diesel::dsl::Filter<
            crate::storage_procedures::table,
            ::diesel::dsl::Eq<crate::storage_procedures::id, ::rosetta_uuid::Uuid>,
        >,
        ::diesel::dsl::Eq<crate::storage_procedures::temperature_k, Option<f64>>,
    >: ::diesel::query_dsl::methods::ExecuteDsl<C>,
    ::diesel_builders::TableBuilder<crate::supernatant_procedures::table>:
        ::diesel_builders::Insert<C, ValidationError: Into<::procedure_like::rows::TypedRowError>>,
    ::diesel::dsl::Update<
        ::diesel::dsl::Filter<
            crate::supernatant_procedures::table,
            ::diesel::dsl::Eq<crate::supernatant_procedures::id, ::rosetta_uuid::Uuid>,
        >,
        ::diesel::dsl::Eq<crate::supernatant_procedures::volume_l, Option<f64>>,
    >: ::diesel::query_dsl::methods::ExecuteDsl<C>,
    ::diesel_builders::TableBuilder<crate::weighing_procedures::table>:
        ::diesel_builders::Insert<C, ValidationError: Into<::procedure_like::rows::TypedRowError>>,
    ::diesel::dsl::Update<
        ::diesel::dsl::Filter<
            crate::weighing_procedures::table,
            ::diesel::dsl::Eq<crate::weighing_procedures::id, ::rosetta_uuid::Uuid>,
        >,
        ::diesel::dsl::Eq<crate::weighing_procedures::mass_kg, Option<f64>>,
    >: ::diesel::query_dsl::methods::ExecuteDsl<C>,
    (crate::aliquoting_procedures::id,): ::diesel_builders::LoadFirst<C>,
    (crate::ball_mill_procedures::id,): ::diesel_builders::LoadFirst<C>,
    (crate::centrifuge_procedures::id,): ::diesel_builders::LoadFirst<C>,
    (crate::container_sealing_procedures::id,): ::diesel_builders::LoadFirst<C>,
    (crate::disposal_procedures::id,): ::diesel_builders::LoadFirst<C>,
    (crate::fractioning_procedures::id,): ::diesel_builders::LoadFirst<C>,
    (crate::freeze_drying_procedures::id,): ::diesel_builders::LoadFirst<C>,
    (crate::freezing_procedures::id,): ::diesel_builders::LoadFirst<C>,
    (crate::geopositioning_procedures::id,): ::diesel_builders::LoadFirst<C>,
    (crate::harvesting_procedures::id,): ::diesel_builders::LoadFirst<C>,
    (crate::packaging_procedures::id,): ::diesel_builders::LoadFirst<C>,
    (crate::photograph_procedures::id,): ::diesel_builders::LoadFirst<C>,
    (crate::pouring_procedures::id,): ::diesel_builders::LoadFirst<C>,
    (crate::retrieval_procedures::id,): ::diesel_builders::LoadFirst<C>,
    (crate::storage_procedures::id,): ::diesel_builders::LoadFirst<C>,
    (crate::supernatant_procedures::id,): ::diesel_builders::LoadFirst<C>,
    (crate::weighing_procedures::id,): ::diesel_builders::LoadFirst<C>,
    (crate::aliquoting_procedure_templates::id,): ::diesel_builders::LoadFirst<C>,
    (crate::ball_mill_procedure_templates::id,): ::diesel_builders::LoadFirst<C>,
    (crate::centrifuge_procedure_templates::id,): ::diesel_builders::LoadFirst<C>,
    (crate::container_sealing_procedure_templates::id,): ::diesel_builders::LoadFirst<C>,
    (crate::disposal_procedure_templates::id,): ::diesel_builders::LoadFirst<C>,
    (crate::fractioning_procedure_templates::id,): ::diesel_builders::LoadFirst<C>,
    (crate::freeze_drying_procedure_templates::id,): ::diesel_builders::LoadFirst<C>,
    (crate::freezing_procedure_templates::id,): ::diesel_builders::LoadFirst<C>,
    (crate::geopositioning_procedure_templates::id,): ::diesel_builders::LoadFirst<C>,
    (crate::harvesting_procedure_templates::id,): ::diesel_builders::LoadFirst<C>,
    (crate::packaging_procedure_templates::id,): ::diesel_builders::LoadFirst<C>,
    (crate::photograph_procedure_templates::id,): ::diesel_builders::LoadFirst<C>,
    (crate::pouring_procedure_templates::id,): ::diesel_builders::LoadFirst<C>,
    (crate::retrieval_procedure_templates::id,): ::diesel_builders::LoadFirst<C>,
    (crate::storage_procedure_templates::id,): ::diesel_builders::LoadFirst<C>,
    (crate::supernatant_procedure_templates::id,): ::diesel_builders::LoadFirst<C>,
    (crate::weighing_procedure_templates::id,): ::diesel_builders::LoadFirst<C>,
    ::diesel_builders::TableBuilder<crate::physical_assets::table>:
        ::diesel_builders::Insert<C, ValidationError: Into<::procedure_like::rows::TypedRowError>>,
    ::diesel_builders::TableBuilder<crate::ball_mill_machines::table>:
        ::diesel_builders::Insert<C, ValidationError: Into<::procedure_like::rows::TypedRowError>>,
    ::diesel_builders::TableBuilder<crate::cameras::table>:
        ::diesel_builders::Insert<C, ValidationError: Into<::procedure_like::rows::TypedRowError>>,
    ::diesel_builders::TableBuilder<crate::centrifuges::table>:
        ::diesel_builders::Insert<C, ValidationError: Into<::procedure_like::rows::TypedRowError>>,
    ::diesel_builders::TableBuilder<crate::freeze_dryers::table>:
        ::diesel_builders::Insert<C, ValidationError: Into<::procedure_like::rows::TypedRowError>>,
    ::diesel_builders::TableBuilder<crate::freezers::table>:
        ::diesel_builders::Insert<C, ValidationError: Into<::procedure_like::rows::TypedRowError>>,
    ::diesel_builders::TableBuilder<crate::geopositioning_devices::table>:
        ::diesel_builders::Insert<C, ValidationError: Into<::procedure_like::rows::TypedRowError>>,
    ::diesel_builders::TableBuilder<crate::markers::table>:
        ::diesel_builders::Insert<C, ValidationError: Into<::procedure_like::rows::TypedRowError>>,
    ::diesel_builders::TableBuilder<crate::mass_spectrometers::table>:
        ::diesel_builders::Insert<C, ValidationError: Into<::procedure_like::rows::TypedRowError>>,
    ::diesel_builders::TableBuilder<crate::personal_protective_equipments::table>:
        ::diesel_builders::Insert<C, ValidationError: Into<::procedure_like::rows::TypedRowError>>,
    ::diesel_builders::TableBuilder<crate::phone_devices::table>:
        ::diesel_builders::Insert<C, ValidationError: Into<::procedure_like::rows::TypedRowError>>,
    ::diesel_builders::TableBuilder<crate::pipette_tips::table>:
        ::diesel_builders::Insert<C, ValidationError: Into<::procedure_like::rows::TypedRowError>>,
    ::diesel_builders::TableBuilder<crate::volume_measuring_devices::table>:
        ::diesel_builders::Insert<C, ValidationError: Into<::procedure_like::rows::TypedRowError>>,
    ::diesel_builders::TableBuilder<crate::weighing_devices::table>:
        ::diesel_builders::Insert<C, ValidationError: Into<::procedure_like::rows::TypedRowError>>,
    ::diesel_builders::TableBuilder<crate::containers::table>:
        ::diesel_builders::Insert<C, ValidationError: Into<::procedure_like::rows::TypedRowError>>,
    ::diesel_builders::TableBuilder<crate::digital_assets::table>:
        ::diesel_builders::Insert<C, ValidationError: Into<::procedure_like::rows::TypedRowError>>,
    ::diesel_builders::TableBuilder<crate::panels::table>:
        ::diesel_builders::Insert<C, ValidationError: Into<::procedure_like::rows::TypedRowError>>,
    ::diesel_builders::TableBuilder<crate::photographs::table>:
        ::diesel_builders::Insert<C, ValidationError: Into<::procedure_like::rows::TypedRowError>>,
    ::diesel_builders::TableBuilder<crate::samples::table>:
        ::diesel_builders::Insert<C, ValidationError: Into<::procedure_like::rows::TypedRowError>>,
    ::diesel_builders::TableBuilder<crate::sample_sources::table>:
        ::diesel_builders::Insert<C, ValidationError: Into<::procedure_like::rows::TypedRowError>>,
    ::diesel_builders::TableBuilder<crate::volumetric_containers::table>:
        ::diesel_builders::Insert<C, ValidationError: Into<::procedure_like::rows::TypedRowError>>,
{
    fn insert_typed_procedure(
        &mut self,
        table_name_id: &str,
        columns: &::procedure_like::rows::ProcedureColumns,
    ) -> Result<(), ::procedure_like::rows::TypedRowError> {
        use ::diesel_builders::prelude::*;
        match table_name_id {
            "aliquoting_procedures" => {
                let builder = <crate::aliquoting_procedures::table as ::diesel_builders::BuildableTable>::try_builder()?
                    .set_column::<crate::resources::id>(columns.id)
                    .set_column::<
                        crate::resources::organization_id,
                    >(columns.organization_id)
                    .set_column::<crate::resources::parent_id>(Some(columns.project_id))
                    .set_column::<
                        crate::resources::parent_kind,
                    >(Some(::aps_authz_resource_kind::ResourceKind::Project))
                    .set_column::<
                        crate::resources::created_by_id,
                    >(Some(columns.created_by_id))
                    .set_column::<
                        crate::procedures::organization_id,
                    >(columns.organization_id)
                    .set_column::<crate::procedures::project_id>(columns.project_id)
                    .set_column::<
                        crate::procedures::table_name_id,
                    >("aliquoting_procedures".to_owned())
                    .set_column::<
                        crate::procedures::procedure_template_id,
                    >(columns.procedure_template_id)
                    .set_column::<
                        crate::procedures::status_id,
                    >(columns.status_id.clone())
                    .set_column::<
                        crate::procedures::parent_procedure_id,
                    >(columns.parent_procedure_id)
                    .set_column::<
                        crate::procedures::parent_template_id,
                    >(columns.parent_template_id)
                    .set_column::<crate::procedures::step_id>(columns.step_id)
                    .set_column::<
                        crate::procedures::root_procedure_id,
                    >(columns.root_procedure_id)
                    .set_column::<
                        crate::procedures::repeat_procedure_asset_id,
                    >(columns.repeat_procedure_asset_id)
                    .set_column::<crate::procedures::planned_for>(columns.planned_for)
                    .set_column::<
                        crate::procedures::performed_by_id,
                    >(columns.performed_by_id)
                    .try_set_column::<crate::procedures::started_at>(columns.started_at)?
                    .try_set_column::<crate::procedures::ended_at>(columns.ended_at)?
                    .try_set_column::<crate::procedures::note>(columns.note.clone())?
                    .set_column::<
                        crate::aliquoting_procedures::organization_id,
                    >(columns.organization_id)
                    .set_column::<
                        crate::aliquoting_procedures::aliquoting_procedure_template_id,
                    >(columns.procedure_template_id);
                ::diesel_builders::Insert::insert(builder, self)?;
                Ok(())
            }
            "ball_mill_procedures" => {
                let builder = <crate::ball_mill_procedures::table as ::diesel_builders::BuildableTable>::try_builder()?
                    .set_column::<crate::resources::id>(columns.id)
                    .set_column::<
                        crate::resources::organization_id,
                    >(columns.organization_id)
                    .set_column::<crate::resources::parent_id>(Some(columns.project_id))
                    .set_column::<
                        crate::resources::parent_kind,
                    >(Some(::aps_authz_resource_kind::ResourceKind::Project))
                    .set_column::<
                        crate::resources::created_by_id,
                    >(Some(columns.created_by_id))
                    .set_column::<
                        crate::procedures::organization_id,
                    >(columns.organization_id)
                    .set_column::<crate::procedures::project_id>(columns.project_id)
                    .set_column::<
                        crate::procedures::table_name_id,
                    >("ball_mill_procedures".to_owned())
                    .set_column::<
                        crate::procedures::procedure_template_id,
                    >(columns.procedure_template_id)
                    .set_column::<
                        crate::procedures::status_id,
                    >(columns.status_id.clone())
                    .set_column::<
                        crate::procedures::parent_procedure_id,
                    >(columns.parent_procedure_id)
                    .set_column::<
                        crate::procedures::parent_template_id,
                    >(columns.parent_template_id)
                    .set_column::<crate::procedures::step_id>(columns.step_id)
                    .set_column::<
                        crate::procedures::root_procedure_id,
                    >(columns.root_procedure_id)
                    .set_column::<
                        crate::procedures::repeat_procedure_asset_id,
                    >(columns.repeat_procedure_asset_id)
                    .set_column::<crate::procedures::planned_for>(columns.planned_for)
                    .set_column::<
                        crate::procedures::performed_by_id,
                    >(columns.performed_by_id)
                    .try_set_column::<crate::procedures::started_at>(columns.started_at)?
                    .try_set_column::<crate::procedures::ended_at>(columns.ended_at)?
                    .try_set_column::<crate::procedures::note>(columns.note.clone())?
                    .set_column::<
                        crate::ball_mill_procedures::organization_id,
                    >(columns.organization_id)
                    .set_column::<
                        crate::ball_mill_procedures::ball_mill_procedure_template_id,
                    >(columns.procedure_template_id);
                ::diesel_builders::Insert::insert(builder, self)?;
                Ok(())
            }
            "centrifuge_procedures" => {
                let builder = <crate::centrifuge_procedures::table as ::diesel_builders::BuildableTable>::try_builder()?
                    .set_column::<crate::resources::id>(columns.id)
                    .set_column::<
                        crate::resources::organization_id,
                    >(columns.organization_id)
                    .set_column::<crate::resources::parent_id>(Some(columns.project_id))
                    .set_column::<
                        crate::resources::parent_kind,
                    >(Some(::aps_authz_resource_kind::ResourceKind::Project))
                    .set_column::<
                        crate::resources::created_by_id,
                    >(Some(columns.created_by_id))
                    .set_column::<
                        crate::procedures::organization_id,
                    >(columns.organization_id)
                    .set_column::<crate::procedures::project_id>(columns.project_id)
                    .set_column::<
                        crate::procedures::table_name_id,
                    >("centrifuge_procedures".to_owned())
                    .set_column::<
                        crate::procedures::procedure_template_id,
                    >(columns.procedure_template_id)
                    .set_column::<
                        crate::procedures::status_id,
                    >(columns.status_id.clone())
                    .set_column::<
                        crate::procedures::parent_procedure_id,
                    >(columns.parent_procedure_id)
                    .set_column::<
                        crate::procedures::parent_template_id,
                    >(columns.parent_template_id)
                    .set_column::<crate::procedures::step_id>(columns.step_id)
                    .set_column::<
                        crate::procedures::root_procedure_id,
                    >(columns.root_procedure_id)
                    .set_column::<
                        crate::procedures::repeat_procedure_asset_id,
                    >(columns.repeat_procedure_asset_id)
                    .set_column::<crate::procedures::planned_for>(columns.planned_for)
                    .set_column::<
                        crate::procedures::performed_by_id,
                    >(columns.performed_by_id)
                    .try_set_column::<crate::procedures::started_at>(columns.started_at)?
                    .try_set_column::<crate::procedures::ended_at>(columns.ended_at)?
                    .try_set_column::<crate::procedures::note>(columns.note.clone())?
                    .set_column::<
                        crate::centrifuge_procedures::organization_id,
                    >(columns.organization_id)
                    .set_column::<
                        crate::centrifuge_procedures::centrifuge_procedure_template_id,
                    >(columns.procedure_template_id);
                ::diesel_builders::Insert::insert(builder, self)?;
                Ok(())
            }
            "container_sealing_procedures" => {
                let builder = <crate::container_sealing_procedures::table as ::diesel_builders::BuildableTable>::try_builder()?
                    .set_column::<crate::resources::id>(columns.id)
                    .set_column::<
                        crate::resources::organization_id,
                    >(columns.organization_id)
                    .set_column::<crate::resources::parent_id>(Some(columns.project_id))
                    .set_column::<
                        crate::resources::parent_kind,
                    >(Some(::aps_authz_resource_kind::ResourceKind::Project))
                    .set_column::<
                        crate::resources::created_by_id,
                    >(Some(columns.created_by_id))
                    .set_column::<
                        crate::procedures::organization_id,
                    >(columns.organization_id)
                    .set_column::<crate::procedures::project_id>(columns.project_id)
                    .set_column::<
                        crate::procedures::table_name_id,
                    >("container_sealing_procedures".to_owned())
                    .set_column::<
                        crate::procedures::procedure_template_id,
                    >(columns.procedure_template_id)
                    .set_column::<
                        crate::procedures::status_id,
                    >(columns.status_id.clone())
                    .set_column::<
                        crate::procedures::parent_procedure_id,
                    >(columns.parent_procedure_id)
                    .set_column::<
                        crate::procedures::parent_template_id,
                    >(columns.parent_template_id)
                    .set_column::<crate::procedures::step_id>(columns.step_id)
                    .set_column::<
                        crate::procedures::root_procedure_id,
                    >(columns.root_procedure_id)
                    .set_column::<
                        crate::procedures::repeat_procedure_asset_id,
                    >(columns.repeat_procedure_asset_id)
                    .set_column::<crate::procedures::planned_for>(columns.planned_for)
                    .set_column::<
                        crate::procedures::performed_by_id,
                    >(columns.performed_by_id)
                    .try_set_column::<crate::procedures::started_at>(columns.started_at)?
                    .try_set_column::<crate::procedures::ended_at>(columns.ended_at)?
                    .try_set_column::<crate::procedures::note>(columns.note.clone())?
                    .set_column::<
                        crate::container_sealing_procedures::organization_id,
                    >(columns.organization_id)
                    .set_column::<
                        crate::container_sealing_procedures::container_sealing_procedure_template_id,
                    >(columns.procedure_template_id);
                ::diesel_builders::Insert::insert(builder, self)?;
                Ok(())
            }
            "disposal_procedures" => {
                let builder = <crate::disposal_procedures::table as ::diesel_builders::BuildableTable>::try_builder()?
                    .set_column::<crate::resources::id>(columns.id)
                    .set_column::<
                        crate::resources::organization_id,
                    >(columns.organization_id)
                    .set_column::<crate::resources::parent_id>(Some(columns.project_id))
                    .set_column::<
                        crate::resources::parent_kind,
                    >(Some(::aps_authz_resource_kind::ResourceKind::Project))
                    .set_column::<
                        crate::resources::created_by_id,
                    >(Some(columns.created_by_id))
                    .set_column::<
                        crate::procedures::organization_id,
                    >(columns.organization_id)
                    .set_column::<crate::procedures::project_id>(columns.project_id)
                    .set_column::<
                        crate::procedures::table_name_id,
                    >("disposal_procedures".to_owned())
                    .set_column::<
                        crate::procedures::procedure_template_id,
                    >(columns.procedure_template_id)
                    .set_column::<
                        crate::procedures::status_id,
                    >(columns.status_id.clone())
                    .set_column::<
                        crate::procedures::parent_procedure_id,
                    >(columns.parent_procedure_id)
                    .set_column::<
                        crate::procedures::parent_template_id,
                    >(columns.parent_template_id)
                    .set_column::<crate::procedures::step_id>(columns.step_id)
                    .set_column::<
                        crate::procedures::root_procedure_id,
                    >(columns.root_procedure_id)
                    .set_column::<
                        crate::procedures::repeat_procedure_asset_id,
                    >(columns.repeat_procedure_asset_id)
                    .set_column::<crate::procedures::planned_for>(columns.planned_for)
                    .set_column::<
                        crate::procedures::performed_by_id,
                    >(columns.performed_by_id)
                    .try_set_column::<crate::procedures::started_at>(columns.started_at)?
                    .try_set_column::<crate::procedures::ended_at>(columns.ended_at)?
                    .try_set_column::<crate::procedures::note>(columns.note.clone())?
                    .set_column::<
                        crate::disposal_procedures::organization_id,
                    >(columns.organization_id)
                    .set_column::<
                        crate::disposal_procedures::disposal_procedure_template_id,
                    >(columns.procedure_template_id);
                ::diesel_builders::Insert::insert(builder, self)?;
                Ok(())
            }
            "fractioning_procedures" => {
                let builder = <crate::fractioning_procedures::table as ::diesel_builders::BuildableTable>::try_builder()?
                    .set_column::<crate::resources::id>(columns.id)
                    .set_column::<
                        crate::resources::organization_id,
                    >(columns.organization_id)
                    .set_column::<crate::resources::parent_id>(Some(columns.project_id))
                    .set_column::<
                        crate::resources::parent_kind,
                    >(Some(::aps_authz_resource_kind::ResourceKind::Project))
                    .set_column::<
                        crate::resources::created_by_id,
                    >(Some(columns.created_by_id))
                    .set_column::<
                        crate::procedures::organization_id,
                    >(columns.organization_id)
                    .set_column::<crate::procedures::project_id>(columns.project_id)
                    .set_column::<
                        crate::procedures::table_name_id,
                    >("fractioning_procedures".to_owned())
                    .set_column::<
                        crate::procedures::procedure_template_id,
                    >(columns.procedure_template_id)
                    .set_column::<
                        crate::procedures::status_id,
                    >(columns.status_id.clone())
                    .set_column::<
                        crate::procedures::parent_procedure_id,
                    >(columns.parent_procedure_id)
                    .set_column::<
                        crate::procedures::parent_template_id,
                    >(columns.parent_template_id)
                    .set_column::<crate::procedures::step_id>(columns.step_id)
                    .set_column::<
                        crate::procedures::root_procedure_id,
                    >(columns.root_procedure_id)
                    .set_column::<
                        crate::procedures::repeat_procedure_asset_id,
                    >(columns.repeat_procedure_asset_id)
                    .set_column::<crate::procedures::planned_for>(columns.planned_for)
                    .set_column::<
                        crate::procedures::performed_by_id,
                    >(columns.performed_by_id)
                    .try_set_column::<crate::procedures::started_at>(columns.started_at)?
                    .try_set_column::<crate::procedures::ended_at>(columns.ended_at)?
                    .try_set_column::<crate::procedures::note>(columns.note.clone())?
                    .set_column::<
                        crate::fractioning_procedures::organization_id,
                    >(columns.organization_id)
                    .set_column::<
                        crate::fractioning_procedures::fractioning_procedure_template_id,
                    >(columns.procedure_template_id);
                ::diesel_builders::Insert::insert(builder, self)?;
                Ok(())
            }
            "freeze_drying_procedures" => {
                let builder = <crate::freeze_drying_procedures::table as ::diesel_builders::BuildableTable>::try_builder()?
                    .set_column::<crate::resources::id>(columns.id)
                    .set_column::<
                        crate::resources::organization_id,
                    >(columns.organization_id)
                    .set_column::<crate::resources::parent_id>(Some(columns.project_id))
                    .set_column::<
                        crate::resources::parent_kind,
                    >(Some(::aps_authz_resource_kind::ResourceKind::Project))
                    .set_column::<
                        crate::resources::created_by_id,
                    >(Some(columns.created_by_id))
                    .set_column::<
                        crate::procedures::organization_id,
                    >(columns.organization_id)
                    .set_column::<crate::procedures::project_id>(columns.project_id)
                    .set_column::<
                        crate::procedures::table_name_id,
                    >("freeze_drying_procedures".to_owned())
                    .set_column::<
                        crate::procedures::procedure_template_id,
                    >(columns.procedure_template_id)
                    .set_column::<
                        crate::procedures::status_id,
                    >(columns.status_id.clone())
                    .set_column::<
                        crate::procedures::parent_procedure_id,
                    >(columns.parent_procedure_id)
                    .set_column::<
                        crate::procedures::parent_template_id,
                    >(columns.parent_template_id)
                    .set_column::<crate::procedures::step_id>(columns.step_id)
                    .set_column::<
                        crate::procedures::root_procedure_id,
                    >(columns.root_procedure_id)
                    .set_column::<
                        crate::procedures::repeat_procedure_asset_id,
                    >(columns.repeat_procedure_asset_id)
                    .set_column::<crate::procedures::planned_for>(columns.planned_for)
                    .set_column::<
                        crate::procedures::performed_by_id,
                    >(columns.performed_by_id)
                    .try_set_column::<crate::procedures::started_at>(columns.started_at)?
                    .try_set_column::<crate::procedures::ended_at>(columns.ended_at)?
                    .try_set_column::<crate::procedures::note>(columns.note.clone())?
                    .set_column::<
                        crate::freeze_drying_procedures::organization_id,
                    >(columns.organization_id)
                    .set_column::<
                        crate::freeze_drying_procedures::freeze_drying_procedure_template_id,
                    >(columns.procedure_template_id);
                ::diesel_builders::Insert::insert(builder, self)?;
                Ok(())
            }
            "freezing_procedures" => {
                let builder = <crate::freezing_procedures::table as ::diesel_builders::BuildableTable>::try_builder()?
                    .set_column::<crate::resources::id>(columns.id)
                    .set_column::<
                        crate::resources::organization_id,
                    >(columns.organization_id)
                    .set_column::<crate::resources::parent_id>(Some(columns.project_id))
                    .set_column::<
                        crate::resources::parent_kind,
                    >(Some(::aps_authz_resource_kind::ResourceKind::Project))
                    .set_column::<
                        crate::resources::created_by_id,
                    >(Some(columns.created_by_id))
                    .set_column::<
                        crate::procedures::organization_id,
                    >(columns.organization_id)
                    .set_column::<crate::procedures::project_id>(columns.project_id)
                    .set_column::<
                        crate::procedures::table_name_id,
                    >("freezing_procedures".to_owned())
                    .set_column::<
                        crate::procedures::procedure_template_id,
                    >(columns.procedure_template_id)
                    .set_column::<
                        crate::procedures::status_id,
                    >(columns.status_id.clone())
                    .set_column::<
                        crate::procedures::parent_procedure_id,
                    >(columns.parent_procedure_id)
                    .set_column::<
                        crate::procedures::parent_template_id,
                    >(columns.parent_template_id)
                    .set_column::<crate::procedures::step_id>(columns.step_id)
                    .set_column::<
                        crate::procedures::root_procedure_id,
                    >(columns.root_procedure_id)
                    .set_column::<
                        crate::procedures::repeat_procedure_asset_id,
                    >(columns.repeat_procedure_asset_id)
                    .set_column::<crate::procedures::planned_for>(columns.planned_for)
                    .set_column::<
                        crate::procedures::performed_by_id,
                    >(columns.performed_by_id)
                    .try_set_column::<crate::procedures::started_at>(columns.started_at)?
                    .try_set_column::<crate::procedures::ended_at>(columns.ended_at)?
                    .try_set_column::<crate::procedures::note>(columns.note.clone())?
                    .set_column::<
                        crate::freezing_procedures::organization_id,
                    >(columns.organization_id)
                    .set_column::<
                        crate::freezing_procedures::freezing_procedure_template_id,
                    >(columns.procedure_template_id);
                ::diesel_builders::Insert::insert(builder, self)?;
                Ok(())
            }
            "geopositioning_procedures" => {
                let builder = <crate::geopositioning_procedures::table as ::diesel_builders::BuildableTable>::try_builder()?
                    .set_column::<crate::resources::id>(columns.id)
                    .set_column::<
                        crate::resources::organization_id,
                    >(columns.organization_id)
                    .set_column::<crate::resources::parent_id>(Some(columns.project_id))
                    .set_column::<
                        crate::resources::parent_kind,
                    >(Some(::aps_authz_resource_kind::ResourceKind::Project))
                    .set_column::<
                        crate::resources::created_by_id,
                    >(Some(columns.created_by_id))
                    .set_column::<
                        crate::procedures::organization_id,
                    >(columns.organization_id)
                    .set_column::<crate::procedures::project_id>(columns.project_id)
                    .set_column::<
                        crate::procedures::table_name_id,
                    >("geopositioning_procedures".to_owned())
                    .set_column::<
                        crate::procedures::procedure_template_id,
                    >(columns.procedure_template_id)
                    .set_column::<
                        crate::procedures::status_id,
                    >(columns.status_id.clone())
                    .set_column::<
                        crate::procedures::parent_procedure_id,
                    >(columns.parent_procedure_id)
                    .set_column::<
                        crate::procedures::parent_template_id,
                    >(columns.parent_template_id)
                    .set_column::<crate::procedures::step_id>(columns.step_id)
                    .set_column::<
                        crate::procedures::root_procedure_id,
                    >(columns.root_procedure_id)
                    .set_column::<
                        crate::procedures::repeat_procedure_asset_id,
                    >(columns.repeat_procedure_asset_id)
                    .set_column::<crate::procedures::planned_for>(columns.planned_for)
                    .set_column::<
                        crate::procedures::performed_by_id,
                    >(columns.performed_by_id)
                    .try_set_column::<crate::procedures::started_at>(columns.started_at)?
                    .try_set_column::<crate::procedures::ended_at>(columns.ended_at)?
                    .try_set_column::<crate::procedures::note>(columns.note.clone())?
                    .set_column::<
                        crate::geopositioning_procedures::organization_id,
                    >(columns.organization_id)
                    .set_column::<
                        crate::geopositioning_procedures::geopositioning_procedure_template_id,
                    >(columns.procedure_template_id);
                ::diesel_builders::Insert::insert(builder, self)?;
                Ok(())
            }
            "harvesting_procedures" => {
                let builder = <crate::harvesting_procedures::table as ::diesel_builders::BuildableTable>::try_builder()?
                    .set_column::<crate::resources::id>(columns.id)
                    .set_column::<
                        crate::resources::organization_id,
                    >(columns.organization_id)
                    .set_column::<crate::resources::parent_id>(Some(columns.project_id))
                    .set_column::<
                        crate::resources::parent_kind,
                    >(Some(::aps_authz_resource_kind::ResourceKind::Project))
                    .set_column::<
                        crate::resources::created_by_id,
                    >(Some(columns.created_by_id))
                    .set_column::<
                        crate::procedures::organization_id,
                    >(columns.organization_id)
                    .set_column::<crate::procedures::project_id>(columns.project_id)
                    .set_column::<
                        crate::procedures::table_name_id,
                    >("harvesting_procedures".to_owned())
                    .set_column::<
                        crate::procedures::procedure_template_id,
                    >(columns.procedure_template_id)
                    .set_column::<
                        crate::procedures::status_id,
                    >(columns.status_id.clone())
                    .set_column::<
                        crate::procedures::parent_procedure_id,
                    >(columns.parent_procedure_id)
                    .set_column::<
                        crate::procedures::parent_template_id,
                    >(columns.parent_template_id)
                    .set_column::<crate::procedures::step_id>(columns.step_id)
                    .set_column::<
                        crate::procedures::root_procedure_id,
                    >(columns.root_procedure_id)
                    .set_column::<
                        crate::procedures::repeat_procedure_asset_id,
                    >(columns.repeat_procedure_asset_id)
                    .set_column::<crate::procedures::planned_for>(columns.planned_for)
                    .set_column::<
                        crate::procedures::performed_by_id,
                    >(columns.performed_by_id)
                    .try_set_column::<crate::procedures::started_at>(columns.started_at)?
                    .try_set_column::<crate::procedures::ended_at>(columns.ended_at)?
                    .try_set_column::<crate::procedures::note>(columns.note.clone())?
                    .set_column::<
                        crate::harvesting_procedures::organization_id,
                    >(columns.organization_id)
                    .set_column::<
                        crate::harvesting_procedures::harvesting_procedure_template_id,
                    >(columns.procedure_template_id);
                ::diesel_builders::Insert::insert(builder, self)?;
                Ok(())
            }
            "packaging_procedures" => {
                let builder = <crate::packaging_procedures::table as ::diesel_builders::BuildableTable>::try_builder()?
                    .set_column::<crate::resources::id>(columns.id)
                    .set_column::<
                        crate::resources::organization_id,
                    >(columns.organization_id)
                    .set_column::<crate::resources::parent_id>(Some(columns.project_id))
                    .set_column::<
                        crate::resources::parent_kind,
                    >(Some(::aps_authz_resource_kind::ResourceKind::Project))
                    .set_column::<
                        crate::resources::created_by_id,
                    >(Some(columns.created_by_id))
                    .set_column::<
                        crate::procedures::organization_id,
                    >(columns.organization_id)
                    .set_column::<crate::procedures::project_id>(columns.project_id)
                    .set_column::<
                        crate::procedures::table_name_id,
                    >("packaging_procedures".to_owned())
                    .set_column::<
                        crate::procedures::procedure_template_id,
                    >(columns.procedure_template_id)
                    .set_column::<
                        crate::procedures::status_id,
                    >(columns.status_id.clone())
                    .set_column::<
                        crate::procedures::parent_procedure_id,
                    >(columns.parent_procedure_id)
                    .set_column::<
                        crate::procedures::parent_template_id,
                    >(columns.parent_template_id)
                    .set_column::<crate::procedures::step_id>(columns.step_id)
                    .set_column::<
                        crate::procedures::root_procedure_id,
                    >(columns.root_procedure_id)
                    .set_column::<
                        crate::procedures::repeat_procedure_asset_id,
                    >(columns.repeat_procedure_asset_id)
                    .set_column::<crate::procedures::planned_for>(columns.planned_for)
                    .set_column::<
                        crate::procedures::performed_by_id,
                    >(columns.performed_by_id)
                    .try_set_column::<crate::procedures::started_at>(columns.started_at)?
                    .try_set_column::<crate::procedures::ended_at>(columns.ended_at)?
                    .try_set_column::<crate::procedures::note>(columns.note.clone())?
                    .set_column::<
                        crate::packaging_procedures::organization_id,
                    >(columns.organization_id)
                    .set_column::<
                        crate::packaging_procedures::packaging_procedure_template_id,
                    >(columns.procedure_template_id);
                ::diesel_builders::Insert::insert(builder, self)?;
                Ok(())
            }
            "photograph_procedures" => {
                let builder = <crate::photograph_procedures::table as ::diesel_builders::BuildableTable>::try_builder()?
                    .set_column::<crate::resources::id>(columns.id)
                    .set_column::<
                        crate::resources::organization_id,
                    >(columns.organization_id)
                    .set_column::<crate::resources::parent_id>(Some(columns.project_id))
                    .set_column::<
                        crate::resources::parent_kind,
                    >(Some(::aps_authz_resource_kind::ResourceKind::Project))
                    .set_column::<
                        crate::resources::created_by_id,
                    >(Some(columns.created_by_id))
                    .set_column::<
                        crate::procedures::organization_id,
                    >(columns.organization_id)
                    .set_column::<crate::procedures::project_id>(columns.project_id)
                    .set_column::<
                        crate::procedures::table_name_id,
                    >("photograph_procedures".to_owned())
                    .set_column::<
                        crate::procedures::procedure_template_id,
                    >(columns.procedure_template_id)
                    .set_column::<
                        crate::procedures::status_id,
                    >(columns.status_id.clone())
                    .set_column::<
                        crate::procedures::parent_procedure_id,
                    >(columns.parent_procedure_id)
                    .set_column::<
                        crate::procedures::parent_template_id,
                    >(columns.parent_template_id)
                    .set_column::<crate::procedures::step_id>(columns.step_id)
                    .set_column::<
                        crate::procedures::root_procedure_id,
                    >(columns.root_procedure_id)
                    .set_column::<
                        crate::procedures::repeat_procedure_asset_id,
                    >(columns.repeat_procedure_asset_id)
                    .set_column::<crate::procedures::planned_for>(columns.planned_for)
                    .set_column::<
                        crate::procedures::performed_by_id,
                    >(columns.performed_by_id)
                    .try_set_column::<crate::procedures::started_at>(columns.started_at)?
                    .try_set_column::<crate::procedures::ended_at>(columns.ended_at)?
                    .try_set_column::<crate::procedures::note>(columns.note.clone())?
                    .set_column::<
                        crate::photograph_procedures::organization_id,
                    >(columns.organization_id)
                    .set_column::<
                        crate::photograph_procedures::photograph_procedure_template_id,
                    >(columns.procedure_template_id);
                ::diesel_builders::Insert::insert(builder, self)?;
                Ok(())
            }
            "pouring_procedures" => {
                let builder = <crate::pouring_procedures::table as ::diesel_builders::BuildableTable>::try_builder()?
                    .set_column::<crate::resources::id>(columns.id)
                    .set_column::<
                        crate::resources::organization_id,
                    >(columns.organization_id)
                    .set_column::<crate::resources::parent_id>(Some(columns.project_id))
                    .set_column::<
                        crate::resources::parent_kind,
                    >(Some(::aps_authz_resource_kind::ResourceKind::Project))
                    .set_column::<
                        crate::resources::created_by_id,
                    >(Some(columns.created_by_id))
                    .set_column::<
                        crate::procedures::organization_id,
                    >(columns.organization_id)
                    .set_column::<crate::procedures::project_id>(columns.project_id)
                    .set_column::<
                        crate::procedures::table_name_id,
                    >("pouring_procedures".to_owned())
                    .set_column::<
                        crate::procedures::procedure_template_id,
                    >(columns.procedure_template_id)
                    .set_column::<
                        crate::procedures::status_id,
                    >(columns.status_id.clone())
                    .set_column::<
                        crate::procedures::parent_procedure_id,
                    >(columns.parent_procedure_id)
                    .set_column::<
                        crate::procedures::parent_template_id,
                    >(columns.parent_template_id)
                    .set_column::<crate::procedures::step_id>(columns.step_id)
                    .set_column::<
                        crate::procedures::root_procedure_id,
                    >(columns.root_procedure_id)
                    .set_column::<
                        crate::procedures::repeat_procedure_asset_id,
                    >(columns.repeat_procedure_asset_id)
                    .set_column::<crate::procedures::planned_for>(columns.planned_for)
                    .set_column::<
                        crate::procedures::performed_by_id,
                    >(columns.performed_by_id)
                    .try_set_column::<crate::procedures::started_at>(columns.started_at)?
                    .try_set_column::<crate::procedures::ended_at>(columns.ended_at)?
                    .try_set_column::<crate::procedures::note>(columns.note.clone())?
                    .set_column::<
                        crate::pouring_procedures::organization_id,
                    >(columns.organization_id)
                    .set_column::<
                        crate::pouring_procedures::pouring_procedure_template_id,
                    >(columns.procedure_template_id);
                ::diesel_builders::Insert::insert(builder, self)?;
                Ok(())
            }
            "retrieval_procedures" => {
                let builder = <crate::retrieval_procedures::table as ::diesel_builders::BuildableTable>::try_builder()?
                    .set_column::<crate::resources::id>(columns.id)
                    .set_column::<
                        crate::resources::organization_id,
                    >(columns.organization_id)
                    .set_column::<crate::resources::parent_id>(Some(columns.project_id))
                    .set_column::<
                        crate::resources::parent_kind,
                    >(Some(::aps_authz_resource_kind::ResourceKind::Project))
                    .set_column::<
                        crate::resources::created_by_id,
                    >(Some(columns.created_by_id))
                    .set_column::<
                        crate::procedures::organization_id,
                    >(columns.organization_id)
                    .set_column::<crate::procedures::project_id>(columns.project_id)
                    .set_column::<
                        crate::procedures::table_name_id,
                    >("retrieval_procedures".to_owned())
                    .set_column::<
                        crate::procedures::procedure_template_id,
                    >(columns.procedure_template_id)
                    .set_column::<
                        crate::procedures::status_id,
                    >(columns.status_id.clone())
                    .set_column::<
                        crate::procedures::parent_procedure_id,
                    >(columns.parent_procedure_id)
                    .set_column::<
                        crate::procedures::parent_template_id,
                    >(columns.parent_template_id)
                    .set_column::<crate::procedures::step_id>(columns.step_id)
                    .set_column::<
                        crate::procedures::root_procedure_id,
                    >(columns.root_procedure_id)
                    .set_column::<
                        crate::procedures::repeat_procedure_asset_id,
                    >(columns.repeat_procedure_asset_id)
                    .set_column::<crate::procedures::planned_for>(columns.planned_for)
                    .set_column::<
                        crate::procedures::performed_by_id,
                    >(columns.performed_by_id)
                    .try_set_column::<crate::procedures::started_at>(columns.started_at)?
                    .try_set_column::<crate::procedures::ended_at>(columns.ended_at)?
                    .try_set_column::<crate::procedures::note>(columns.note.clone())?
                    .set_column::<
                        crate::retrieval_procedures::organization_id,
                    >(columns.organization_id)
                    .set_column::<
                        crate::retrieval_procedures::retrieval_procedure_template_id,
                    >(columns.procedure_template_id);
                ::diesel_builders::Insert::insert(builder, self)?;
                Ok(())
            }
            "storage_procedures" => {
                let builder = <crate::storage_procedures::table as ::diesel_builders::BuildableTable>::try_builder()?
                    .set_column::<crate::resources::id>(columns.id)
                    .set_column::<
                        crate::resources::organization_id,
                    >(columns.organization_id)
                    .set_column::<crate::resources::parent_id>(Some(columns.project_id))
                    .set_column::<
                        crate::resources::parent_kind,
                    >(Some(::aps_authz_resource_kind::ResourceKind::Project))
                    .set_column::<
                        crate::resources::created_by_id,
                    >(Some(columns.created_by_id))
                    .set_column::<
                        crate::procedures::organization_id,
                    >(columns.organization_id)
                    .set_column::<crate::procedures::project_id>(columns.project_id)
                    .set_column::<
                        crate::procedures::table_name_id,
                    >("storage_procedures".to_owned())
                    .set_column::<
                        crate::procedures::procedure_template_id,
                    >(columns.procedure_template_id)
                    .set_column::<
                        crate::procedures::status_id,
                    >(columns.status_id.clone())
                    .set_column::<
                        crate::procedures::parent_procedure_id,
                    >(columns.parent_procedure_id)
                    .set_column::<
                        crate::procedures::parent_template_id,
                    >(columns.parent_template_id)
                    .set_column::<crate::procedures::step_id>(columns.step_id)
                    .set_column::<
                        crate::procedures::root_procedure_id,
                    >(columns.root_procedure_id)
                    .set_column::<
                        crate::procedures::repeat_procedure_asset_id,
                    >(columns.repeat_procedure_asset_id)
                    .set_column::<crate::procedures::planned_for>(columns.planned_for)
                    .set_column::<
                        crate::procedures::performed_by_id,
                    >(columns.performed_by_id)
                    .try_set_column::<crate::procedures::started_at>(columns.started_at)?
                    .try_set_column::<crate::procedures::ended_at>(columns.ended_at)?
                    .try_set_column::<crate::procedures::note>(columns.note.clone())?
                    .set_column::<
                        crate::storage_procedures::organization_id,
                    >(columns.organization_id)
                    .set_column::<
                        crate::storage_procedures::storage_procedure_template_id,
                    >(columns.procedure_template_id);
                ::diesel_builders::Insert::insert(builder, self)?;
                Ok(())
            }
            "supernatant_procedures" => {
                let builder = <crate::supernatant_procedures::table as ::diesel_builders::BuildableTable>::try_builder()?
                    .set_column::<crate::resources::id>(columns.id)
                    .set_column::<
                        crate::resources::organization_id,
                    >(columns.organization_id)
                    .set_column::<crate::resources::parent_id>(Some(columns.project_id))
                    .set_column::<
                        crate::resources::parent_kind,
                    >(Some(::aps_authz_resource_kind::ResourceKind::Project))
                    .set_column::<
                        crate::resources::created_by_id,
                    >(Some(columns.created_by_id))
                    .set_column::<
                        crate::procedures::organization_id,
                    >(columns.organization_id)
                    .set_column::<crate::procedures::project_id>(columns.project_id)
                    .set_column::<
                        crate::procedures::table_name_id,
                    >("supernatant_procedures".to_owned())
                    .set_column::<
                        crate::procedures::procedure_template_id,
                    >(columns.procedure_template_id)
                    .set_column::<
                        crate::procedures::status_id,
                    >(columns.status_id.clone())
                    .set_column::<
                        crate::procedures::parent_procedure_id,
                    >(columns.parent_procedure_id)
                    .set_column::<
                        crate::procedures::parent_template_id,
                    >(columns.parent_template_id)
                    .set_column::<crate::procedures::step_id>(columns.step_id)
                    .set_column::<
                        crate::procedures::root_procedure_id,
                    >(columns.root_procedure_id)
                    .set_column::<
                        crate::procedures::repeat_procedure_asset_id,
                    >(columns.repeat_procedure_asset_id)
                    .set_column::<crate::procedures::planned_for>(columns.planned_for)
                    .set_column::<
                        crate::procedures::performed_by_id,
                    >(columns.performed_by_id)
                    .try_set_column::<crate::procedures::started_at>(columns.started_at)?
                    .try_set_column::<crate::procedures::ended_at>(columns.ended_at)?
                    .try_set_column::<crate::procedures::note>(columns.note.clone())?
                    .set_column::<
                        crate::supernatant_procedures::organization_id,
                    >(columns.organization_id)
                    .set_column::<
                        crate::supernatant_procedures::supernatant_procedure_template_id,
                    >(columns.procedure_template_id);
                ::diesel_builders::Insert::insert(builder, self)?;
                Ok(())
            }
            "weighing_procedures" => {
                let builder = <crate::weighing_procedures::table as ::diesel_builders::BuildableTable>::try_builder()?
                    .set_column::<crate::resources::id>(columns.id)
                    .set_column::<
                        crate::resources::organization_id,
                    >(columns.organization_id)
                    .set_column::<crate::resources::parent_id>(Some(columns.project_id))
                    .set_column::<
                        crate::resources::parent_kind,
                    >(Some(::aps_authz_resource_kind::ResourceKind::Project))
                    .set_column::<
                        crate::resources::created_by_id,
                    >(Some(columns.created_by_id))
                    .set_column::<
                        crate::procedures::organization_id,
                    >(columns.organization_id)
                    .set_column::<crate::procedures::project_id>(columns.project_id)
                    .set_column::<
                        crate::procedures::table_name_id,
                    >("weighing_procedures".to_owned())
                    .set_column::<
                        crate::procedures::procedure_template_id,
                    >(columns.procedure_template_id)
                    .set_column::<
                        crate::procedures::status_id,
                    >(columns.status_id.clone())
                    .set_column::<
                        crate::procedures::parent_procedure_id,
                    >(columns.parent_procedure_id)
                    .set_column::<
                        crate::procedures::parent_template_id,
                    >(columns.parent_template_id)
                    .set_column::<crate::procedures::step_id>(columns.step_id)
                    .set_column::<
                        crate::procedures::root_procedure_id,
                    >(columns.root_procedure_id)
                    .set_column::<
                        crate::procedures::repeat_procedure_asset_id,
                    >(columns.repeat_procedure_asset_id)
                    .set_column::<crate::procedures::planned_for>(columns.planned_for)
                    .set_column::<
                        crate::procedures::performed_by_id,
                    >(columns.performed_by_id)
                    .try_set_column::<crate::procedures::started_at>(columns.started_at)?
                    .try_set_column::<crate::procedures::ended_at>(columns.ended_at)?
                    .try_set_column::<crate::procedures::note>(columns.note.clone())?
                    .set_column::<
                        crate::weighing_procedures::organization_id,
                    >(columns.organization_id)
                    .set_column::<
                        crate::weighing_procedures::weighing_procedure_template_id,
                    >(columns.procedure_template_id);
                ::diesel_builders::Insert::insert(builder, self)?;
                Ok(())
            }
            other => Err(::procedure_like::rows::TypedRowError::UnknownTable(other.to_owned())),
        }
    }
    fn set_measurements(
        &mut self,
        table_name_id: &str,
        id: ::rosetta_uuid::Uuid,
        values: &[(&str, Option<::procedure_like::rows::ColumnValue>)],
    ) -> Result<(), ::procedure_like::rows::TypedRowError> {
        use ::diesel::{ExpressionMethods, QueryDsl};
        match table_name_id {
            "aliquoting_procedures" => {
                for (column, value) in values {
                    match *column {
                        "volume_l" => {
                            let value = value.clone();
                            let value = value
                                .map(|value| {
                                    ::procedure_like::rows::column_value::<f64>(
                                        "aliquoting_procedures",
                                        "volume_l",
                                        value,
                                    )
                                })
                                .transpose()?;
                            ::diesel::RunQueryDsl::execute(
                                ::diesel::update(
                                    crate::aliquoting_procedures::table
                                        .filter(crate::aliquoting_procedures::id.eq(id)),
                                )
                                .set(crate::aliquoting_procedures::volume_l.eq(value)),
                                self,
                            )?;
                        }
                        other => {
                            return Err(::procedure_like::rows::TypedRowError::UnknownColumn {
                                table: "aliquoting_procedures",
                                column: other.to_owned(),
                            });
                        }
                    }
                }
                Ok(())
            }
            "ball_mill_procedures" => {
                for (column, value) in values {
                    match *column {
                        "temperature_k" => {
                            let value = value.clone();
                            let value = value
                                .map(|value| {
                                    ::procedure_like::rows::column_value::<f64>(
                                        "ball_mill_procedures",
                                        "temperature_k",
                                        value,
                                    )
                                })
                                .transpose()?;
                            ::diesel::RunQueryDsl::execute(
                                ::diesel::update(
                                    crate::ball_mill_procedures::table
                                        .filter(crate::ball_mill_procedures::id.eq(id)),
                                )
                                .set(crate::ball_mill_procedures::temperature_k.eq(value)),
                                self,
                            )?;
                        }
                        "frequency_hz" => {
                            let value = value.clone();
                            let value = value
                                .map(|value| {
                                    ::procedure_like::rows::column_value::<f64>(
                                        "ball_mill_procedures",
                                        "frequency_hz",
                                        value,
                                    )
                                })
                                .transpose()?;
                            ::diesel::RunQueryDsl::execute(
                                ::diesel::update(
                                    crate::ball_mill_procedures::table
                                        .filter(crate::ball_mill_procedures::id.eq(id)),
                                )
                                .set(crate::ball_mill_procedures::frequency_hz.eq(value)),
                                self,
                            )?;
                        }
                        other => {
                            return Err(::procedure_like::rows::TypedRowError::UnknownColumn {
                                table: "ball_mill_procedures",
                                column: other.to_owned(),
                            });
                        }
                    }
                }
                Ok(())
            }
            "centrifuge_procedures" => {
                for (column, value) in values {
                    match *column {
                        "temperature_k" => {
                            let value = value.clone();
                            let value = value
                                .map(|value| {
                                    ::procedure_like::rows::column_value::<f64>(
                                        "centrifuge_procedures",
                                        "temperature_k",
                                        value,
                                    )
                                })
                                .transpose()?;
                            ::diesel::RunQueryDsl::execute(
                                ::diesel::update(
                                    crate::centrifuge_procedures::table
                                        .filter(crate::centrifuge_procedures::id.eq(id)),
                                )
                                .set(crate::centrifuge_procedures::temperature_k.eq(value)),
                                self,
                            )?;
                        }
                        "speed_rpm" => {
                            let value = value.clone();
                            let value = value
                                .map(|value| {
                                    ::procedure_like::rows::column_value::<f64>(
                                        "centrifuge_procedures",
                                        "speed_rpm",
                                        value,
                                    )
                                })
                                .transpose()?;
                            ::diesel::RunQueryDsl::execute(
                                ::diesel::update(
                                    crate::centrifuge_procedures::table
                                        .filter(crate::centrifuge_procedures::id.eq(id)),
                                )
                                .set(crate::centrifuge_procedures::speed_rpm.eq(value)),
                                self,
                            )?;
                        }
                        other => {
                            return Err(::procedure_like::rows::TypedRowError::UnknownColumn {
                                table: "centrifuge_procedures",
                                column: other.to_owned(),
                            });
                        }
                    }
                }
                Ok(())
            }
            "container_sealing_procedures" => {
                if let Some((column, _)) = values.first() {
                    return Err(::procedure_like::rows::TypedRowError::UnknownColumn {
                        table: "container_sealing_procedures",
                        column: (*column).to_owned(),
                    });
                }
                Ok(())
            }
            "disposal_procedures" => {
                if let Some((column, _)) = values.first() {
                    return Err(::procedure_like::rows::TypedRowError::UnknownColumn {
                        table: "disposal_procedures",
                        column: (*column).to_owned(),
                    });
                }
                Ok(())
            }
            "fractioning_procedures" => {
                for (column, value) in values {
                    match *column {
                        "mass_kg" => {
                            let value = value.clone();
                            let value = value
                                .map(|value| {
                                    ::procedure_like::rows::column_value::<f64>(
                                        "fractioning_procedures",
                                        "mass_kg",
                                        value,
                                    )
                                })
                                .transpose()?;
                            ::diesel::RunQueryDsl::execute(
                                ::diesel::update(
                                    crate::fractioning_procedures::table
                                        .filter(crate::fractioning_procedures::id.eq(id)),
                                )
                                .set(crate::fractioning_procedures::mass_kg.eq(value)),
                                self,
                            )?;
                        }
                        other => {
                            return Err(::procedure_like::rows::TypedRowError::UnknownColumn {
                                table: "fractioning_procedures",
                                column: other.to_owned(),
                            });
                        }
                    }
                }
                Ok(())
            }
            "freeze_drying_procedures" => {
                for (column, value) in values {
                    match *column {
                        "temperature_k" => {
                            let value = value.clone();
                            let value = value
                                .map(|value| {
                                    ::procedure_like::rows::column_value::<f64>(
                                        "freeze_drying_procedures",
                                        "temperature_k",
                                        value,
                                    )
                                })
                                .transpose()?;
                            ::diesel::RunQueryDsl::execute(
                                ::diesel::update(
                                    crate::freeze_drying_procedures::table
                                        .filter(crate::freeze_drying_procedures::id.eq(id)),
                                )
                                .set(crate::freeze_drying_procedures::temperature_k.eq(value)),
                                self,
                            )?;
                        }
                        "pressure_pa" => {
                            let value = value.clone();
                            let value = value
                                .map(|value| {
                                    ::procedure_like::rows::column_value::<f64>(
                                        "freeze_drying_procedures",
                                        "pressure_pa",
                                        value,
                                    )
                                })
                                .transpose()?;
                            ::diesel::RunQueryDsl::execute(
                                ::diesel::update(
                                    crate::freeze_drying_procedures::table
                                        .filter(crate::freeze_drying_procedures::id.eq(id)),
                                )
                                .set(crate::freeze_drying_procedures::pressure_pa.eq(value)),
                                self,
                            )?;
                        }
                        other => {
                            return Err(::procedure_like::rows::TypedRowError::UnknownColumn {
                                table: "freeze_drying_procedures",
                                column: other.to_owned(),
                            });
                        }
                    }
                }
                Ok(())
            }
            "freezing_procedures" => {
                for (column, value) in values {
                    match *column {
                        "temperature_k" => {
                            let value = value.clone();
                            let value = value
                                .map(|value| {
                                    ::procedure_like::rows::column_value::<f64>(
                                        "freezing_procedures",
                                        "temperature_k",
                                        value,
                                    )
                                })
                                .transpose()?;
                            ::diesel::RunQueryDsl::execute(
                                ::diesel::update(
                                    crate::freezing_procedures::table
                                        .filter(crate::freezing_procedures::id.eq(id)),
                                )
                                .set(crate::freezing_procedures::temperature_k.eq(value)),
                                self,
                            )?;
                        }
                        other => {
                            return Err(::procedure_like::rows::TypedRowError::UnknownColumn {
                                table: "freezing_procedures",
                                column: other.to_owned(),
                            });
                        }
                    }
                }
                Ok(())
            }
            "geopositioning_procedures" => {
                for (column, value) in values {
                    match *column {
                        "location" => {
                            let value = value.clone();
                            let value = value
                                .map(|value| {
                                    ::procedure_like::rows::column_value::<
                                        ::sqlitegis::diesel::GeographyValue,
                                    >(
                                        "geopositioning_procedures", "location", value
                                    )
                                })
                                .transpose()?;
                            ::diesel::RunQueryDsl::execute(
                                ::diesel::update(
                                    crate::geopositioning_procedures::table
                                        .filter(crate::geopositioning_procedures::id.eq(id)),
                                )
                                .set(crate::geopositioning_procedures::location.eq(value)),
                                self,
                            )?;
                        }
                        other => {
                            return Err(::procedure_like::rows::TypedRowError::UnknownColumn {
                                table: "geopositioning_procedures",
                                column: other.to_owned(),
                            });
                        }
                    }
                }
                Ok(())
            }
            "harvesting_procedures" => {
                if let Some((column, _)) = values.first() {
                    return Err(::procedure_like::rows::TypedRowError::UnknownColumn {
                        table: "harvesting_procedures",
                        column: (*column).to_owned(),
                    });
                }
                Ok(())
            }
            "packaging_procedures" => {
                if let Some((column, _)) = values.first() {
                    return Err(::procedure_like::rows::TypedRowError::UnknownColumn {
                        table: "packaging_procedures",
                        column: (*column).to_owned(),
                    });
                }
                Ok(())
            }
            "photograph_procedures" => {
                if let Some((column, _)) = values.first() {
                    return Err(::procedure_like::rows::TypedRowError::UnknownColumn {
                        table: "photograph_procedures",
                        column: (*column).to_owned(),
                    });
                }
                Ok(())
            }
            "pouring_procedures" => {
                for (column, value) in values {
                    match *column {
                        "volume_l" => {
                            let value = value.clone();
                            let value = value
                                .map(|value| {
                                    ::procedure_like::rows::column_value::<f64>(
                                        "pouring_procedures",
                                        "volume_l",
                                        value,
                                    )
                                })
                                .transpose()?;
                            ::diesel::RunQueryDsl::execute(
                                ::diesel::update(
                                    crate::pouring_procedures::table
                                        .filter(crate::pouring_procedures::id.eq(id)),
                                )
                                .set(crate::pouring_procedures::volume_l.eq(value)),
                                self,
                            )?;
                        }
                        other => {
                            return Err(::procedure_like::rows::TypedRowError::UnknownColumn {
                                table: "pouring_procedures",
                                column: other.to_owned(),
                            });
                        }
                    }
                }
                Ok(())
            }
            "retrieval_procedures" => {
                if let Some((column, _)) = values.first() {
                    return Err(::procedure_like::rows::TypedRowError::UnknownColumn {
                        table: "retrieval_procedures",
                        column: (*column).to_owned(),
                    });
                }
                Ok(())
            }
            "storage_procedures" => {
                for (column, value) in values {
                    match *column {
                        "temperature_k" => {
                            let value = value.clone();
                            let value = value
                                .map(|value| {
                                    ::procedure_like::rows::column_value::<f64>(
                                        "storage_procedures",
                                        "temperature_k",
                                        value,
                                    )
                                })
                                .transpose()?;
                            ::diesel::RunQueryDsl::execute(
                                ::diesel::update(
                                    crate::storage_procedures::table
                                        .filter(crate::storage_procedures::id.eq(id)),
                                )
                                .set(crate::storage_procedures::temperature_k.eq(value)),
                                self,
                            )?;
                        }
                        other => {
                            return Err(::procedure_like::rows::TypedRowError::UnknownColumn {
                                table: "storage_procedures",
                                column: other.to_owned(),
                            });
                        }
                    }
                }
                Ok(())
            }
            "supernatant_procedures" => {
                for (column, value) in values {
                    match *column {
                        "volume_l" => {
                            let value = value.clone();
                            let value = value
                                .map(|value| {
                                    ::procedure_like::rows::column_value::<f64>(
                                        "supernatant_procedures",
                                        "volume_l",
                                        value,
                                    )
                                })
                                .transpose()?;
                            ::diesel::RunQueryDsl::execute(
                                ::diesel::update(
                                    crate::supernatant_procedures::table
                                        .filter(crate::supernatant_procedures::id.eq(id)),
                                )
                                .set(crate::supernatant_procedures::volume_l.eq(value)),
                                self,
                            )?;
                        }
                        other => {
                            return Err(::procedure_like::rows::TypedRowError::UnknownColumn {
                                table: "supernatant_procedures",
                                column: other.to_owned(),
                            });
                        }
                    }
                }
                Ok(())
            }
            "weighing_procedures" => {
                for (column, value) in values {
                    match *column {
                        "mass_kg" => {
                            let value = value.clone();
                            let value = value
                                .map(|value| {
                                    ::procedure_like::rows::column_value::<f64>(
                                        "weighing_procedures",
                                        "mass_kg",
                                        value,
                                    )
                                })
                                .transpose()?;
                            ::diesel::RunQueryDsl::execute(
                                ::diesel::update(
                                    crate::weighing_procedures::table
                                        .filter(crate::weighing_procedures::id.eq(id)),
                                )
                                .set(crate::weighing_procedures::mass_kg.eq(value)),
                                self,
                            )?;
                        }
                        other => {
                            return Err(::procedure_like::rows::TypedRowError::UnknownColumn {
                                table: "weighing_procedures",
                                column: other.to_owned(),
                            });
                        }
                    }
                }
                Ok(())
            }
            other => Err(::procedure_like::rows::TypedRowError::UnknownTable(other.to_owned())),
        }
    }
    fn typed_values(
        &mut self,
        table_name_id: &str,
        id: ::rosetta_uuid::Uuid,
    ) -> Result<
        Vec<(&'static str, Option<::procedure_like::rows::ColumnValue>)>,
        ::procedure_like::rows::TypedRowError,
    > {
        match table_name_id {
            "aliquoting_procedures" => {
                let row = <(crate::aliquoting_procedures::id,) as ::diesel_builders::LoadFirst<
                    C,
                >>::load_first((id,), self)?;
                Ok(vec![(
                    "volume_l",
                    ::diesel_builders::GetColumnExt::get_column::<
                        crate::aliquoting_procedures::volume_l,
                    >(&row)
                    .map(::procedure_like::rows::ColumnValue::from),
                )])
            }
            "ball_mill_procedures" => {
                let row = <(crate::ball_mill_procedures::id,) as ::diesel_builders::LoadFirst<
                    C,
                >>::load_first((id,), self)?;
                Ok(vec![
                    (
                        "temperature_k",
                        ::diesel_builders::GetColumnExt::get_column::<
                            crate::ball_mill_procedures::temperature_k,
                        >(&row)
                        .map(::procedure_like::rows::ColumnValue::from),
                    ),
                    (
                        "frequency_hz",
                        ::diesel_builders::GetColumnExt::get_column::<
                            crate::ball_mill_procedures::frequency_hz,
                        >(&row)
                        .map(::procedure_like::rows::ColumnValue::from),
                    ),
                ])
            }
            "centrifuge_procedures" => {
                let row = <(crate::centrifuge_procedures::id,) as ::diesel_builders::LoadFirst<
                    C,
                >>::load_first((id,), self)?;
                Ok(vec![
                    (
                        "temperature_k",
                        ::diesel_builders::GetColumnExt::get_column::<
                            crate::centrifuge_procedures::temperature_k,
                        >(&row)
                        .map(::procedure_like::rows::ColumnValue::from),
                    ),
                    (
                        "speed_rpm",
                        ::diesel_builders::GetColumnExt::get_column::<
                            crate::centrifuge_procedures::speed_rpm,
                        >(&row)
                        .map(::procedure_like::rows::ColumnValue::from),
                    ),
                ])
            }
            "container_sealing_procedures" => {
                <(
                    crate::container_sealing_procedures::id,
                ) as ::diesel_builders::LoadFirst<C>>::load_first((id,), self)?;
                Ok(Vec::new())
            }
            "disposal_procedures" => {
                <(crate::disposal_procedures::id,) as ::diesel_builders::LoadFirst<C>>::load_first(
                    (id,),
                    self,
                )?;
                Ok(Vec::new())
            }
            "fractioning_procedures" => {
                let row = <(crate::fractioning_procedures::id,) as ::diesel_builders::LoadFirst<
                    C,
                >>::load_first((id,), self)?;
                Ok(vec![(
                    "mass_kg",
                    ::diesel_builders::GetColumnExt::get_column::<
                        crate::fractioning_procedures::mass_kg,
                    >(&row)
                    .map(::procedure_like::rows::ColumnValue::from),
                )])
            }
            "freeze_drying_procedures" => {
                let row = <(
                    crate::freeze_drying_procedures::id,
                ) as ::diesel_builders::LoadFirst<C>>::load_first((id,), self)?;
                Ok(vec![
                    (
                        "temperature_k",
                        ::diesel_builders::GetColumnExt::get_column::<
                            crate::freeze_drying_procedures::temperature_k,
                        >(&row)
                        .map(::procedure_like::rows::ColumnValue::from),
                    ),
                    (
                        "pressure_pa",
                        ::diesel_builders::GetColumnExt::get_column::<
                            crate::freeze_drying_procedures::pressure_pa,
                        >(&row)
                        .map(::procedure_like::rows::ColumnValue::from),
                    ),
                ])
            }
            "freezing_procedures" => {
                let row = <(
                    crate::freezing_procedures::id,
                ) as ::diesel_builders::LoadFirst<C>>::load_first((id,), self)?;
                Ok(vec![(
                    "temperature_k",
                    ::diesel_builders::GetColumnExt::get_column::<
                        crate::freezing_procedures::temperature_k,
                    >(&row)
                    .map(::procedure_like::rows::ColumnValue::from),
                )])
            }
            "geopositioning_procedures" => {
                let row = <(
                    crate::geopositioning_procedures::id,
                ) as ::diesel_builders::LoadFirst<C>>::load_first((id,), self)?;
                Ok(vec![(
                    "location",
                    ::diesel_builders::GetColumnExt::get_column::<
                        crate::geopositioning_procedures::location,
                    >(&row)
                    .map(::procedure_like::rows::ColumnValue::from),
                )])
            }
            "harvesting_procedures" => {
                <(
                    crate::harvesting_procedures::id,
                ) as ::diesel_builders::LoadFirst<C>>::load_first((id,), self)?;
                Ok(Vec::new())
            }
            "packaging_procedures" => {
                <(
                    crate::packaging_procedures::id,
                ) as ::diesel_builders::LoadFirst<C>>::load_first((id,), self)?;
                Ok(Vec::new())
            }
            "photograph_procedures" => {
                <(
                    crate::photograph_procedures::id,
                ) as ::diesel_builders::LoadFirst<C>>::load_first((id,), self)?;
                Ok(Vec::new())
            }
            "pouring_procedures" => {
                let row = <(
                    crate::pouring_procedures::id,
                ) as ::diesel_builders::LoadFirst<C>>::load_first((id,), self)?;
                Ok(vec![(
                    "volume_l",
                    ::diesel_builders::GetColumnExt::get_column::<
                        crate::pouring_procedures::volume_l,
                    >(&row)
                    .map(::procedure_like::rows::ColumnValue::from),
                )])
            }
            "retrieval_procedures" => {
                <(
                    crate::retrieval_procedures::id,
                ) as ::diesel_builders::LoadFirst<C>>::load_first((id,), self)?;
                Ok(Vec::new())
            }
            "storage_procedures" => {
                let row = <(
                    crate::storage_procedures::id,
                ) as ::diesel_builders::LoadFirst<C>>::load_first((id,), self)?;
                Ok(vec![(
                    "temperature_k",
                    ::diesel_builders::GetColumnExt::get_column::<
                        crate::storage_procedures::temperature_k,
                    >(&row)
                    .map(::procedure_like::rows::ColumnValue::from),
                )])
            }
            "supernatant_procedures" => {
                let row = <(crate::supernatant_procedures::id,) as ::diesel_builders::LoadFirst<
                    C,
                >>::load_first((id,), self)?;
                Ok(vec![(
                    "volume_l",
                    ::diesel_builders::GetColumnExt::get_column::<
                        crate::supernatant_procedures::volume_l,
                    >(&row)
                    .map(::procedure_like::rows::ColumnValue::from),
                )])
            }
            "weighing_procedures" => {
                let row = <(
                    crate::weighing_procedures::id,
                ) as ::diesel_builders::LoadFirst<C>>::load_first((id,), self)?;
                Ok(vec![(
                    "mass_kg",
                    ::diesel_builders::GetColumnExt::get_column::<
                        crate::weighing_procedures::mass_kg,
                    >(&row)
                    .map(::procedure_like::rows::ColumnValue::from),
                )])
            }
            "aliquoting_procedure_templates" => {
                let row = <(
                    crate::aliquoting_procedure_templates::id,
                ) as ::diesel_builders::LoadFirst<C>>::load_first((id,), self)?;
                Ok(
                    vec![
                        ("volume_l",
                        Some(::procedure_like::rows::ColumnValue::from(::diesel_builders::GetColumnExt::get_column::
                        < crate ::aliquoting_procedure_templates::volume_l > (& row)))),
                        ("volume_l_tolerance_percentage",
                        Some(::procedure_like::rows::ColumnValue::from(::diesel_builders::GetColumnExt::get_column::
                        < crate
                        ::aliquoting_procedure_templates::volume_l_tolerance_percentage >
                        (& row))))
                    ],
                )
            }
            "ball_mill_procedure_templates" => {
                let row = <(
                    crate::ball_mill_procedure_templates::id,
                ) as ::diesel_builders::LoadFirst<C>>::load_first((id,), self)?;
                Ok(
                    vec![
                        ("temperature_k",
                        Some(::procedure_like::rows::ColumnValue::from(::diesel_builders::GetColumnExt::get_column::
                        < crate ::ball_mill_procedure_templates::temperature_k > (&
                        row)))), ("temperature_tolerance_percentage",
                        Some(::procedure_like::rows::ColumnValue::from(::diesel_builders::GetColumnExt::get_column::
                        < crate
                        ::ball_mill_procedure_templates::temperature_tolerance_percentage
                        > (& row)))), ("duration_s",
                        Some(::procedure_like::rows::ColumnValue::from(::diesel_builders::GetColumnExt::get_column::
                        < crate ::ball_mill_procedure_templates::duration_s > (& row)))),
                        ("duration_tolerance_percentage",
                        Some(::procedure_like::rows::ColumnValue::from(::diesel_builders::GetColumnExt::get_column::
                        < crate
                        ::ball_mill_procedure_templates::duration_tolerance_percentage >
                        (& row)))), ("frequency_hz",
                        Some(::procedure_like::rows::ColumnValue::from(::diesel_builders::GetColumnExt::get_column::
                        < crate ::ball_mill_procedure_templates::frequency_hz > (&
                        row)))), ("frequency_hz_tolerance_percentage",
                        Some(::procedure_like::rows::ColumnValue::from(::diesel_builders::GetColumnExt::get_column::
                        < crate
                        ::ball_mill_procedure_templates::frequency_hz_tolerance_percentage
                        > (& row)))), ("bead_count",
                        Some(::procedure_like::rows::ColumnValue::from(::diesel_builders::GetColumnExt::get_column::
                        < crate ::ball_mill_procedure_templates::bead_count > (& row))))
                    ],
                )
            }
            "centrifuge_procedure_templates" => {
                let row = <(
                    crate::centrifuge_procedure_templates::id,
                ) as ::diesel_builders::LoadFirst<C>>::load_first((id,), self)?;
                Ok(
                    vec![
                        ("temperature_k",
                        Some(::procedure_like::rows::ColumnValue::from(::diesel_builders::GetColumnExt::get_column::
                        < crate ::centrifuge_procedure_templates::temperature_k > (&
                        row)))), ("temperature_tolerance_percentage",
                        Some(::procedure_like::rows::ColumnValue::from(::diesel_builders::GetColumnExt::get_column::
                        < crate
                        ::centrifuge_procedure_templates::temperature_tolerance_percentage
                        > (& row)))), ("duration_s",
                        Some(::procedure_like::rows::ColumnValue::from(::diesel_builders::GetColumnExt::get_column::
                        < crate ::centrifuge_procedure_templates::duration_s > (&
                        row)))), ("duration_tolerance_percentage",
                        Some(::procedure_like::rows::ColumnValue::from(::diesel_builders::GetColumnExt::get_column::
                        < crate
                        ::centrifuge_procedure_templates::duration_tolerance_percentage >
                        (& row)))), ("speed_rpm",
                        Some(::procedure_like::rows::ColumnValue::from(::diesel_builders::GetColumnExt::get_column::
                        < crate ::centrifuge_procedure_templates::speed_rpm > (& row)))),
                        ("speed_rpm_tolerance_percentage",
                        Some(::procedure_like::rows::ColumnValue::from(::diesel_builders::GetColumnExt::get_column::
                        < crate
                        ::centrifuge_procedure_templates::speed_rpm_tolerance_percentage
                        > (& row))))
                    ],
                )
            }
            "container_sealing_procedure_templates" => {
                <(
                    crate::container_sealing_procedure_templates::id,
                ) as ::diesel_builders::LoadFirst<C>>::load_first((id,), self)?;
                Ok(Vec::new())
            }
            "disposal_procedure_templates" => {
                <(
                    crate::disposal_procedure_templates::id,
                ) as ::diesel_builders::LoadFirst<C>>::load_first((id,), self)?;
                Ok(Vec::new())
            }
            "fractioning_procedure_templates" => {
                let row = <(
                    crate::fractioning_procedure_templates::id,
                ) as ::diesel_builders::LoadFirst<C>>::load_first((id,), self)?;
                Ok(vec![
                    (
                        "mass_kg",
                        Some(::procedure_like::rows::ColumnValue::from(
                            ::diesel_builders::GetColumnExt::get_column::<
                                crate::fractioning_procedure_templates::mass_kg,
                            >(&row),
                        )),
                    ),
                    (
                        "tolerance_percentage",
                        Some(::procedure_like::rows::ColumnValue::from(
                            ::diesel_builders::GetColumnExt::get_column::<
                                crate::fractioning_procedure_templates::tolerance_percentage,
                            >(&row),
                        )),
                    ),
                ])
            }
            "freeze_drying_procedure_templates" => {
                let row = <(
                    crate::freeze_drying_procedure_templates::id,
                ) as ::diesel_builders::LoadFirst<C>>::load_first((id,), self)?;
                Ok(
                    vec![
                        ("temperature_k",
                        Some(::procedure_like::rows::ColumnValue::from(::diesel_builders::GetColumnExt::get_column::
                        < crate ::freeze_drying_procedure_templates::temperature_k > (&
                        row)))), ("temperature_tolerance_percentage",
                        Some(::procedure_like::rows::ColumnValue::from(::diesel_builders::GetColumnExt::get_column::
                        < crate
                        ::freeze_drying_procedure_templates::temperature_tolerance_percentage
                        > (& row)))), ("pressure_pa",
                        Some(::procedure_like::rows::ColumnValue::from(::diesel_builders::GetColumnExt::get_column::
                        < crate ::freeze_drying_procedure_templates::pressure_pa > (&
                        row)))), ("pressure_pa_tolerance_percentage",
                        Some(::procedure_like::rows::ColumnValue::from(::diesel_builders::GetColumnExt::get_column::
                        < crate
                        ::freeze_drying_procedure_templates::pressure_pa_tolerance_percentage
                        > (& row)))), ("duration_s",
                        Some(::procedure_like::rows::ColumnValue::from(::diesel_builders::GetColumnExt::get_column::
                        < crate ::freeze_drying_procedure_templates::duration_s > (&
                        row)))), ("duration_tolerance_percentage",
                        Some(::procedure_like::rows::ColumnValue::from(::diesel_builders::GetColumnExt::get_column::
                        < crate
                        ::freeze_drying_procedure_templates::duration_tolerance_percentage
                        > (& row))))
                    ],
                )
            }
            "freezing_procedure_templates" => {
                let row =
                    <(crate::freezing_procedure_templates::id,) as ::diesel_builders::LoadFirst<
                        C,
                    >>::load_first((id,), self)?;
                Ok(
                    vec![
                        ("temperature_k",
                        Some(::procedure_like::rows::ColumnValue::from(::diesel_builders::GetColumnExt::get_column::
                        < crate ::freezing_procedure_templates::temperature_k > (&
                        row)))), ("temperature_tolerance_percentage",
                        Some(::procedure_like::rows::ColumnValue::from(::diesel_builders::GetColumnExt::get_column::
                        < crate
                        ::freezing_procedure_templates::temperature_tolerance_percentage
                        > (& row)))), ("duration_s",
                        Some(::procedure_like::rows::ColumnValue::from(::diesel_builders::GetColumnExt::get_column::
                        < crate ::freezing_procedure_templates::duration_s > (& row)))),
                        ("duration_tolerance_percentage",
                        Some(::procedure_like::rows::ColumnValue::from(::diesel_builders::GetColumnExt::get_column::
                        < crate
                        ::freezing_procedure_templates::duration_tolerance_percentage >
                        (& row))))
                    ],
                )
            }
            "geopositioning_procedure_templates" => {
                <(
                    crate::geopositioning_procedure_templates::id,
                ) as ::diesel_builders::LoadFirst<C>>::load_first((id,), self)?;
                Ok(Vec::new())
            }
            "harvesting_procedure_templates" => {
                <(crate::harvesting_procedure_templates::id,) as ::diesel_builders::LoadFirst<
                    C,
                >>::load_first((id,), self)?;
                Ok(Vec::new())
            }
            "packaging_procedure_templates" => {
                <(
                    crate::packaging_procedure_templates::id,
                ) as ::diesel_builders::LoadFirst<C>>::load_first((id,), self)?;
                Ok(Vec::new())
            }
            "photograph_procedure_templates" => {
                <(crate::photograph_procedure_templates::id,) as ::diesel_builders::LoadFirst<
                    C,
                >>::load_first((id,), self)?;
                Ok(Vec::new())
            }
            "pouring_procedure_templates" => {
                let row =
                    <(crate::pouring_procedure_templates::id,) as ::diesel_builders::LoadFirst<
                        C,
                    >>::load_first((id,), self)?;
                Ok(vec![
                    (
                        "volume_l",
                        Some(::procedure_like::rows::ColumnValue::from(
                            ::diesel_builders::GetColumnExt::get_column::<
                                crate::pouring_procedure_templates::volume_l,
                            >(&row),
                        )),
                    ),
                    (
                        "volume_l_tolerance_percentage",
                        Some(::procedure_like::rows::ColumnValue::from(
                            ::diesel_builders::GetColumnExt::get_column::<
                                crate::pouring_procedure_templates::volume_l_tolerance_percentage,
                            >(&row),
                        )),
                    ),
                ])
            }
            "retrieval_procedure_templates" => {
                <(
                    crate::retrieval_procedure_templates::id,
                ) as ::diesel_builders::LoadFirst<C>>::load_first((id,), self)?;
                Ok(Vec::new())
            }
            "storage_procedure_templates" => {
                let row =
                    <(crate::storage_procedure_templates::id,) as ::diesel_builders::LoadFirst<
                        C,
                    >>::load_first((id,), self)?;
                Ok(
                    vec![
                        ("temperature_k",
                        Some(::procedure_like::rows::ColumnValue::from(::diesel_builders::GetColumnExt::get_column::
                        < crate ::storage_procedure_templates::temperature_k > (&
                        row)))), ("temperature_tolerance_percentage",
                        Some(::procedure_like::rows::ColumnValue::from(::diesel_builders::GetColumnExt::get_column::
                        < crate
                        ::storage_procedure_templates::temperature_tolerance_percentage >
                        (& row))))
                    ],
                )
            }
            "supernatant_procedure_templates" => {
                let row = <(
                    crate::supernatant_procedure_templates::id,
                ) as ::diesel_builders::LoadFirst<C>>::load_first((id,), self)?;
                Ok(
                    vec![
                        ("volume_l",
                        Some(::procedure_like::rows::ColumnValue::from(::diesel_builders::GetColumnExt::get_column::
                        < crate ::supernatant_procedure_templates::volume_l > (& row)))),
                        ("volume_l_tolerance_percentage",
                        Some(::procedure_like::rows::ColumnValue::from(::diesel_builders::GetColumnExt::get_column::
                        < crate
                        ::supernatant_procedure_templates::volume_l_tolerance_percentage
                        > (& row))))
                    ],
                )
            }
            "weighing_procedure_templates" => {
                <(
                    crate::weighing_procedure_templates::id,
                ) as ::diesel_builders::LoadFirst<C>>::load_first((id,), self)?;
                Ok(Vec::new())
            }
            other => Err(::procedure_like::rows::TypedRowError::UnknownTable(other.to_owned())),
        }
    }
    fn insert_produced_asset(
        &mut self,
        asset_table_name_id: &str,
        columns: &::procedure_like::rows::AssetColumns,
        values: &[(&str, ::procedure_like::rows::ColumnValue)],
    ) -> Result<(), ::procedure_like::rows::TypedRowError> {
        use ::diesel_builders::prelude::*;
        match asset_table_name_id {
            "physical_assets" => {
                if let Some((column, _)) = values.first() {
                    return Err(::procedure_like::rows::TypedRowError::UnknownColumn {
                        table: "physical_assets",
                        column: (*column).to_owned(),
                    });
                }
                let builder = <crate::physical_assets::table as ::diesel_builders::BuildableTable>::try_builder()?
                    .set_column::<crate::resources::id>(columns.id)
                    .set_column::<
                        crate::resources::organization_id,
                    >(columns.organization_id)
                    .set_column::<crate::resources::parent_id>(Some(columns.project_id))
                    .set_column::<
                        crate::resources::parent_kind,
                    >(Some(::aps_authz_resource_kind::ResourceKind::Project))
                    .set_column::<
                        crate::resources::created_by_id,
                    >(Some(columns.created_by_id))
                    .set_column::<
                        crate::assets::organization_id,
                    >(columns.organization_id)
                    .set_column::<crate::assets::project_id>(columns.project_id)
                    .set_column::<
                        crate::assets::table_name_id,
                    >("physical_assets".to_owned())
                    .set_column::<crate::assets::model_id>(columns.model_id)
                    .try_set_column::<crate::assets::name>(columns.name.clone())?
                    .try_set_column::<
                        crate::assets::description,
                    >(columns.description.clone())?
                    .set_column::<
                        crate::physical_assets::organization_id,
                    >(columns.organization_id)
                    .set_column::<
                        crate::physical_assets::physical_asset_model_id,
                    >(columns.model_id);
                ::diesel_builders::Insert::insert(builder, self)?;
                Ok(())
            }
            "ball_mill_machines" => {
                if let Some((column, _)) = values.first() {
                    return Err(::procedure_like::rows::TypedRowError::UnknownColumn {
                        table: "ball_mill_machines",
                        column: (*column).to_owned(),
                    });
                }
                let builder = <crate::ball_mill_machines::table as ::diesel_builders::BuildableTable>::try_builder()?
                    .set_column::<crate::resources::id>(columns.id)
                    .set_column::<
                        crate::resources::organization_id,
                    >(columns.organization_id)
                    .set_column::<crate::resources::parent_id>(Some(columns.project_id))
                    .set_column::<
                        crate::resources::parent_kind,
                    >(Some(::aps_authz_resource_kind::ResourceKind::Project))
                    .set_column::<
                        crate::resources::created_by_id,
                    >(Some(columns.created_by_id))
                    .set_column::<
                        crate::assets::organization_id,
                    >(columns.organization_id)
                    .set_column::<crate::assets::project_id>(columns.project_id)
                    .set_column::<
                        crate::assets::table_name_id,
                    >("ball_mill_machines".to_owned())
                    .set_column::<crate::assets::model_id>(columns.model_id)
                    .try_set_column::<crate::assets::name>(columns.name.clone())?
                    .try_set_column::<
                        crate::assets::description,
                    >(columns.description.clone())?
                    .set_column::<
                        crate::physical_assets::organization_id,
                    >(columns.organization_id)
                    .set_column::<
                        crate::physical_assets::physical_asset_model_id,
                    >(columns.model_id)
                    .set_column::<
                        crate::ball_mill_machines::organization_id,
                    >(columns.organization_id)
                    .set_column::<
                        crate::ball_mill_machines::commercial_ball_mill_machine_lot_id,
                    >(columns.model_id);
                ::diesel_builders::Insert::insert(builder, self)?;
                Ok(())
            }
            "cameras" => {
                if let Some((column, _)) = values.first() {
                    return Err(::procedure_like::rows::TypedRowError::UnknownColumn {
                        table: "cameras",
                        column: (*column).to_owned(),
                    });
                }
                let builder =
                    <crate::cameras::table as ::diesel_builders::BuildableTable>::try_builder()?
                        .set_column::<crate::resources::id>(columns.id)
                        .set_column::<crate::resources::organization_id>(columns.organization_id)
                        .set_column::<crate::resources::parent_id>(Some(columns.project_id))
                        .set_column::<crate::resources::parent_kind>(Some(
                            ::aps_authz_resource_kind::ResourceKind::Project,
                        ))
                        .set_column::<crate::resources::created_by_id>(Some(columns.created_by_id))
                        .set_column::<crate::assets::organization_id>(columns.organization_id)
                        .set_column::<crate::assets::project_id>(columns.project_id)
                        .set_column::<crate::assets::table_name_id>("cameras".to_owned())
                        .set_column::<crate::assets::model_id>(columns.model_id)
                        .try_set_column::<crate::assets::name>(columns.name.clone())?
                        .try_set_column::<crate::assets::description>(columns.description.clone())?
                        .set_column::<crate::physical_assets::organization_id>(
                            columns.organization_id,
                        )
                        .set_column::<crate::physical_assets::physical_asset_model_id>(
                            columns.model_id,
                        )
                        .set_column::<crate::cameras::organization_id>(columns.organization_id)
                        .set_column::<crate::cameras::commercial_camera_lot_id>(columns.model_id);
                ::diesel_builders::Insert::insert(builder, self)?;
                Ok(())
            }
            "centrifuges" => {
                if let Some((column, _)) = values.first() {
                    return Err(::procedure_like::rows::TypedRowError::UnknownColumn {
                        table: "centrifuges",
                        column: (*column).to_owned(),
                    });
                }
                let builder =
                    <crate::centrifuges::table as ::diesel_builders::BuildableTable>::try_builder(
                    )?
                    .set_column::<crate::resources::id>(columns.id)
                    .set_column::<crate::resources::organization_id>(columns.organization_id)
                    .set_column::<crate::resources::parent_id>(Some(columns.project_id))
                    .set_column::<crate::resources::parent_kind>(Some(
                        ::aps_authz_resource_kind::ResourceKind::Project,
                    ))
                    .set_column::<crate::resources::created_by_id>(Some(columns.created_by_id))
                    .set_column::<crate::assets::organization_id>(columns.organization_id)
                    .set_column::<crate::assets::project_id>(columns.project_id)
                    .set_column::<crate::assets::table_name_id>("centrifuges".to_owned())
                    .set_column::<crate::assets::model_id>(columns.model_id)
                    .try_set_column::<crate::assets::name>(columns.name.clone())?
                    .try_set_column::<crate::assets::description>(columns.description.clone())?
                    .set_column::<crate::physical_assets::organization_id>(columns.organization_id)
                    .set_column::<crate::physical_assets::physical_asset_model_id>(columns.model_id)
                    .set_column::<crate::centrifuges::organization_id>(columns.organization_id)
                    .set_column::<crate::centrifuges::commercial_centrifuge_lot_id>(
                        columns.model_id,
                    );
                ::diesel_builders::Insert::insert(builder, self)?;
                Ok(())
            }
            "freeze_dryers" => {
                if let Some((column, _)) = values.first() {
                    return Err(::procedure_like::rows::TypedRowError::UnknownColumn {
                        table: "freeze_dryers",
                        column: (*column).to_owned(),
                    });
                }
                let builder = <crate::freeze_dryers::table as ::diesel_builders::BuildableTable>::try_builder()?
                    .set_column::<crate::resources::id>(columns.id)
                    .set_column::<
                        crate::resources::organization_id,
                    >(columns.organization_id)
                    .set_column::<crate::resources::parent_id>(Some(columns.project_id))
                    .set_column::<
                        crate::resources::parent_kind,
                    >(Some(::aps_authz_resource_kind::ResourceKind::Project))
                    .set_column::<
                        crate::resources::created_by_id,
                    >(Some(columns.created_by_id))
                    .set_column::<
                        crate::assets::organization_id,
                    >(columns.organization_id)
                    .set_column::<crate::assets::project_id>(columns.project_id)
                    .set_column::<
                        crate::assets::table_name_id,
                    >("freeze_dryers".to_owned())
                    .set_column::<crate::assets::model_id>(columns.model_id)
                    .try_set_column::<crate::assets::name>(columns.name.clone())?
                    .try_set_column::<
                        crate::assets::description,
                    >(columns.description.clone())?
                    .set_column::<
                        crate::physical_assets::organization_id,
                    >(columns.organization_id)
                    .set_column::<
                        crate::physical_assets::physical_asset_model_id,
                    >(columns.model_id)
                    .set_column::<
                        crate::freeze_dryers::organization_id,
                    >(columns.organization_id)
                    .set_column::<
                        crate::freeze_dryers::commercial_freeze_dryer_lot_id,
                    >(columns.model_id);
                ::diesel_builders::Insert::insert(builder, self)?;
                Ok(())
            }
            "freezers" => {
                if let Some((column, _)) = values.first() {
                    return Err(::procedure_like::rows::TypedRowError::UnknownColumn {
                        table: "freezers",
                        column: (*column).to_owned(),
                    });
                }
                let builder =
                    <crate::freezers::table as ::diesel_builders::BuildableTable>::try_builder()?
                        .set_column::<crate::resources::id>(columns.id)
                        .set_column::<crate::resources::organization_id>(columns.organization_id)
                        .set_column::<crate::resources::parent_id>(Some(columns.project_id))
                        .set_column::<crate::resources::parent_kind>(Some(
                            ::aps_authz_resource_kind::ResourceKind::Project,
                        ))
                        .set_column::<crate::resources::created_by_id>(Some(columns.created_by_id))
                        .set_column::<crate::assets::organization_id>(columns.organization_id)
                        .set_column::<crate::assets::project_id>(columns.project_id)
                        .set_column::<crate::assets::table_name_id>("freezers".to_owned())
                        .set_column::<crate::assets::model_id>(columns.model_id)
                        .try_set_column::<crate::assets::name>(columns.name.clone())?
                        .try_set_column::<crate::assets::description>(columns.description.clone())?
                        .set_column::<crate::physical_assets::organization_id>(
                            columns.organization_id,
                        )
                        .set_column::<crate::physical_assets::physical_asset_model_id>(
                            columns.model_id,
                        )
                        .set_column::<crate::freezers::organization_id>(columns.organization_id)
                        .set_column::<crate::freezers::commercial_freezer_lot_id>(columns.model_id);
                ::diesel_builders::Insert::insert(builder, self)?;
                Ok(())
            }
            "geopositioning_devices" => {
                if let Some((column, _)) = values.first() {
                    return Err(::procedure_like::rows::TypedRowError::UnknownColumn {
                        table: "geopositioning_devices",
                        column: (*column).to_owned(),
                    });
                }
                let builder = <crate::geopositioning_devices::table as ::diesel_builders::BuildableTable>::try_builder()?
                    .set_column::<crate::resources::id>(columns.id)
                    .set_column::<
                        crate::resources::organization_id,
                    >(columns.organization_id)
                    .set_column::<crate::resources::parent_id>(Some(columns.project_id))
                    .set_column::<
                        crate::resources::parent_kind,
                    >(Some(::aps_authz_resource_kind::ResourceKind::Project))
                    .set_column::<
                        crate::resources::created_by_id,
                    >(Some(columns.created_by_id))
                    .set_column::<
                        crate::assets::organization_id,
                    >(columns.organization_id)
                    .set_column::<crate::assets::project_id>(columns.project_id)
                    .set_column::<
                        crate::assets::table_name_id,
                    >("geopositioning_devices".to_owned())
                    .set_column::<crate::assets::model_id>(columns.model_id)
                    .try_set_column::<crate::assets::name>(columns.name.clone())?
                    .try_set_column::<
                        crate::assets::description,
                    >(columns.description.clone())?
                    .set_column::<
                        crate::physical_assets::organization_id,
                    >(columns.organization_id)
                    .set_column::<
                        crate::physical_assets::physical_asset_model_id,
                    >(columns.model_id)
                    .set_column::<
                        crate::geopositioning_devices::organization_id,
                    >(columns.organization_id)
                    .set_column::<
                        crate::geopositioning_devices::commercial_geopositioning_device_lot_id,
                    >(columns.model_id);
                ::diesel_builders::Insert::insert(builder, self)?;
                Ok(())
            }
            "markers" => {
                if let Some((column, _)) = values.first() {
                    return Err(::procedure_like::rows::TypedRowError::UnknownColumn {
                        table: "markers",
                        column: (*column).to_owned(),
                    });
                }
                let builder =
                    <crate::markers::table as ::diesel_builders::BuildableTable>::try_builder()?
                        .set_column::<crate::resources::id>(columns.id)
                        .set_column::<crate::resources::organization_id>(columns.organization_id)
                        .set_column::<crate::resources::parent_id>(Some(columns.project_id))
                        .set_column::<crate::resources::parent_kind>(Some(
                            ::aps_authz_resource_kind::ResourceKind::Project,
                        ))
                        .set_column::<crate::resources::created_by_id>(Some(columns.created_by_id))
                        .set_column::<crate::assets::organization_id>(columns.organization_id)
                        .set_column::<crate::assets::project_id>(columns.project_id)
                        .set_column::<crate::assets::table_name_id>("markers".to_owned())
                        .set_column::<crate::assets::model_id>(columns.model_id)
                        .try_set_column::<crate::assets::name>(columns.name.clone())?
                        .try_set_column::<crate::assets::description>(columns.description.clone())?
                        .set_column::<crate::physical_assets::organization_id>(
                            columns.organization_id,
                        )
                        .set_column::<crate::physical_assets::physical_asset_model_id>(
                            columns.model_id,
                        )
                        .set_column::<crate::markers::organization_id>(columns.organization_id)
                        .set_column::<crate::markers::marker_model_id>(columns.model_id);
                ::diesel_builders::Insert::insert(builder, self)?;
                Ok(())
            }
            "mass_spectrometers" => {
                if let Some((column, _)) = values.first() {
                    return Err(::procedure_like::rows::TypedRowError::UnknownColumn {
                        table: "mass_spectrometers",
                        column: (*column).to_owned(),
                    });
                }
                let builder = <crate::mass_spectrometers::table as ::diesel_builders::BuildableTable>::try_builder()?
                    .set_column::<crate::resources::id>(columns.id)
                    .set_column::<
                        crate::resources::organization_id,
                    >(columns.organization_id)
                    .set_column::<crate::resources::parent_id>(Some(columns.project_id))
                    .set_column::<
                        crate::resources::parent_kind,
                    >(Some(::aps_authz_resource_kind::ResourceKind::Project))
                    .set_column::<
                        crate::resources::created_by_id,
                    >(Some(columns.created_by_id))
                    .set_column::<
                        crate::assets::organization_id,
                    >(columns.organization_id)
                    .set_column::<crate::assets::project_id>(columns.project_id)
                    .set_column::<
                        crate::assets::table_name_id,
                    >("mass_spectrometers".to_owned())
                    .set_column::<crate::assets::model_id>(columns.model_id)
                    .try_set_column::<crate::assets::name>(columns.name.clone())?
                    .try_set_column::<
                        crate::assets::description,
                    >(columns.description.clone())?
                    .set_column::<
                        crate::physical_assets::organization_id,
                    >(columns.organization_id)
                    .set_column::<
                        crate::physical_assets::physical_asset_model_id,
                    >(columns.model_id)
                    .set_column::<
                        crate::mass_spectrometers::organization_id,
                    >(columns.organization_id)
                    .set_column::<
                        crate::mass_spectrometers::commercial_mass_spectrometer_lot_id,
                    >(columns.model_id);
                ::diesel_builders::Insert::insert(builder, self)?;
                Ok(())
            }
            "personal_protective_equipments" => {
                if let Some((column, _)) = values.first() {
                    return Err(::procedure_like::rows::TypedRowError::UnknownColumn {
                        table: "personal_protective_equipments",
                        column: (*column).to_owned(),
                    });
                }
                let builder = <crate::personal_protective_equipments::table as ::diesel_builders::BuildableTable>::try_builder()?
                    .set_column::<crate::resources::id>(columns.id)
                    .set_column::<
                        crate::resources::organization_id,
                    >(columns.organization_id)
                    .set_column::<crate::resources::parent_id>(Some(columns.project_id))
                    .set_column::<
                        crate::resources::parent_kind,
                    >(Some(::aps_authz_resource_kind::ResourceKind::Project))
                    .set_column::<
                        crate::resources::created_by_id,
                    >(Some(columns.created_by_id))
                    .set_column::<
                        crate::assets::organization_id,
                    >(columns.organization_id)
                    .set_column::<crate::assets::project_id>(columns.project_id)
                    .set_column::<
                        crate::assets::table_name_id,
                    >("personal_protective_equipments".to_owned())
                    .set_column::<crate::assets::model_id>(columns.model_id)
                    .try_set_column::<crate::assets::name>(columns.name.clone())?
                    .try_set_column::<
                        crate::assets::description,
                    >(columns.description.clone())?
                    .set_column::<
                        crate::physical_assets::organization_id,
                    >(columns.organization_id)
                    .set_column::<
                        crate::physical_assets::physical_asset_model_id,
                    >(columns.model_id)
                    .set_column::<
                        crate::personal_protective_equipments::organization_id,
                    >(columns.organization_id)
                    .set_column::<
                        crate::personal_protective_equipments::personal_protective_equipment_model_id,
                    >(columns.model_id);
                ::diesel_builders::Insert::insert(builder, self)?;
                Ok(())
            }
            "phone_devices" => {
                if let Some((column, _)) = values.first() {
                    return Err(::procedure_like::rows::TypedRowError::UnknownColumn {
                        table: "phone_devices",
                        column: (*column).to_owned(),
                    });
                }
                let builder = <crate::phone_devices::table as ::diesel_builders::BuildableTable>::try_builder()?
                    .set_column::<crate::resources::id>(columns.id)
                    .set_column::<
                        crate::resources::organization_id,
                    >(columns.organization_id)
                    .set_column::<crate::resources::parent_id>(Some(columns.project_id))
                    .set_column::<
                        crate::resources::parent_kind,
                    >(Some(::aps_authz_resource_kind::ResourceKind::Project))
                    .set_column::<
                        crate::resources::created_by_id,
                    >(Some(columns.created_by_id))
                    .set_column::<
                        crate::assets::organization_id,
                    >(columns.organization_id)
                    .set_column::<crate::assets::project_id>(columns.project_id)
                    .set_column::<
                        crate::assets::table_name_id,
                    >("phone_devices".to_owned())
                    .set_column::<crate::assets::model_id>(columns.model_id)
                    .try_set_column::<crate::assets::name>(columns.name.clone())?
                    .try_set_column::<
                        crate::assets::description,
                    >(columns.description.clone())?
                    .set_column::<
                        crate::physical_assets::organization_id,
                    >(columns.organization_id)
                    .set_column::<
                        crate::physical_assets::physical_asset_model_id,
                    >(columns.model_id)
                    .set_column::<
                        crate::cameras::organization_id,
                    >(columns.organization_id)
                    .set_column::<
                        crate::cameras::commercial_camera_lot_id,
                    >(columns.model_id)
                    .set_column::<
                        crate::geopositioning_devices::organization_id,
                    >(columns.organization_id)
                    .set_column::<
                        crate::geopositioning_devices::commercial_geopositioning_device_lot_id,
                    >(columns.model_id)
                    .set_column::<
                        crate::phone_devices::organization_id,
                    >(columns.organization_id)
                    .set_column::<
                        crate::phone_devices::commercial_phone_device_lot_id,
                    >(columns.model_id);
                ::diesel_builders::Insert::insert(builder, self)?;
                Ok(())
            }
            "pipette_tips" => {
                if let Some((column, _)) = values.first() {
                    return Err(::procedure_like::rows::TypedRowError::UnknownColumn {
                        table: "pipette_tips",
                        column: (*column).to_owned(),
                    });
                }
                let builder =
                    <crate::pipette_tips::table as ::diesel_builders::BuildableTable>::try_builder(
                    )?
                    .set_column::<crate::resources::id>(columns.id)
                    .set_column::<crate::resources::organization_id>(columns.organization_id)
                    .set_column::<crate::resources::parent_id>(Some(columns.project_id))
                    .set_column::<crate::resources::parent_kind>(Some(
                        ::aps_authz_resource_kind::ResourceKind::Project,
                    ))
                    .set_column::<crate::resources::created_by_id>(Some(columns.created_by_id))
                    .set_column::<crate::assets::organization_id>(columns.organization_id)
                    .set_column::<crate::assets::project_id>(columns.project_id)
                    .set_column::<crate::assets::table_name_id>("pipette_tips".to_owned())
                    .set_column::<crate::assets::model_id>(columns.model_id)
                    .try_set_column::<crate::assets::name>(columns.name.clone())?
                    .try_set_column::<crate::assets::description>(columns.description.clone())?
                    .set_column::<crate::physical_assets::organization_id>(columns.organization_id)
                    .set_column::<crate::physical_assets::physical_asset_model_id>(columns.model_id)
                    .set_column::<crate::pipette_tips::organization_id>(columns.organization_id)
                    .set_column::<crate::pipette_tips::pipette_tip_model_id>(columns.model_id);
                ::diesel_builders::Insert::insert(builder, self)?;
                Ok(())
            }
            "volume_measuring_devices" => {
                if let Some((column, _)) = values.first() {
                    return Err(::procedure_like::rows::TypedRowError::UnknownColumn {
                        table: "volume_measuring_devices",
                        column: (*column).to_owned(),
                    });
                }
                let builder = <crate::volume_measuring_devices::table as ::diesel_builders::BuildableTable>::try_builder()?
                    .set_column::<crate::resources::id>(columns.id)
                    .set_column::<
                        crate::resources::organization_id,
                    >(columns.organization_id)
                    .set_column::<crate::resources::parent_id>(Some(columns.project_id))
                    .set_column::<
                        crate::resources::parent_kind,
                    >(Some(::aps_authz_resource_kind::ResourceKind::Project))
                    .set_column::<
                        crate::resources::created_by_id,
                    >(Some(columns.created_by_id))
                    .set_column::<
                        crate::assets::organization_id,
                    >(columns.organization_id)
                    .set_column::<crate::assets::project_id>(columns.project_id)
                    .set_column::<
                        crate::assets::table_name_id,
                    >("volume_measuring_devices".to_owned())
                    .set_column::<crate::assets::model_id>(columns.model_id)
                    .try_set_column::<crate::assets::name>(columns.name.clone())?
                    .try_set_column::<
                        crate::assets::description,
                    >(columns.description.clone())?
                    .set_column::<
                        crate::physical_assets::organization_id,
                    >(columns.organization_id)
                    .set_column::<
                        crate::physical_assets::physical_asset_model_id,
                    >(columns.model_id)
                    .set_column::<
                        crate::volume_measuring_devices::organization_id,
                    >(columns.organization_id)
                    .set_column::<
                        crate::volume_measuring_devices::volume_measuring_device_model_id,
                    >(columns.model_id);
                ::diesel_builders::Insert::insert(builder, self)?;
                Ok(())
            }
            "weighing_devices" => {
                if let Some((column, _)) = values.first() {
                    return Err(::procedure_like::rows::TypedRowError::UnknownColumn {
                        table: "weighing_devices",
                        column: (*column).to_owned(),
                    });
                }
                let builder = <crate::weighing_devices::table as ::diesel_builders::BuildableTable>::try_builder()?
                    .set_column::<crate::resources::id>(columns.id)
                    .set_column::<
                        crate::resources::organization_id,
                    >(columns.organization_id)
                    .set_column::<crate::resources::parent_id>(Some(columns.project_id))
                    .set_column::<
                        crate::resources::parent_kind,
                    >(Some(::aps_authz_resource_kind::ResourceKind::Project))
                    .set_column::<
                        crate::resources::created_by_id,
                    >(Some(columns.created_by_id))
                    .set_column::<
                        crate::assets::organization_id,
                    >(columns.organization_id)
                    .set_column::<crate::assets::project_id>(columns.project_id)
                    .set_column::<
                        crate::assets::table_name_id,
                    >("weighing_devices".to_owned())
                    .set_column::<crate::assets::model_id>(columns.model_id)
                    .try_set_column::<crate::assets::name>(columns.name.clone())?
                    .try_set_column::<
                        crate::assets::description,
                    >(columns.description.clone())?
                    .set_column::<
                        crate::physical_assets::organization_id,
                    >(columns.organization_id)
                    .set_column::<
                        crate::physical_assets::physical_asset_model_id,
                    >(columns.model_id)
                    .set_column::<
                        crate::weighing_devices::organization_id,
                    >(columns.organization_id)
                    .set_column::<
                        crate::weighing_devices::weighing_device_model_id,
                    >(columns.model_id);
                ::diesel_builders::Insert::insert(builder, self)?;
                Ok(())
            }
            "containers" => {
                if let Some((column, _)) = values.first() {
                    return Err(::procedure_like::rows::TypedRowError::UnknownColumn {
                        table: "containers",
                        column: (*column).to_owned(),
                    });
                }
                let builder =
                    <crate::containers::table as ::diesel_builders::BuildableTable>::try_builder()?
                        .set_column::<crate::resources::id>(columns.id)
                        .set_column::<crate::resources::organization_id>(columns.organization_id)
                        .set_column::<crate::resources::parent_id>(Some(columns.project_id))
                        .set_column::<crate::resources::parent_kind>(Some(
                            ::aps_authz_resource_kind::ResourceKind::Project,
                        ))
                        .set_column::<crate::resources::created_by_id>(Some(columns.created_by_id))
                        .set_column::<crate::assets::organization_id>(columns.organization_id)
                        .set_column::<crate::assets::project_id>(columns.project_id)
                        .set_column::<crate::assets::table_name_id>("containers".to_owned())
                        .set_column::<crate::assets::model_id>(columns.model_id)
                        .try_set_column::<crate::assets::name>(columns.name.clone())?
                        .try_set_column::<crate::assets::description>(columns.description.clone())?
                        .set_column::<crate::physical_assets::organization_id>(
                            columns.organization_id,
                        )
                        .set_column::<crate::physical_assets::physical_asset_model_id>(
                            columns.model_id,
                        )
                        .set_column::<crate::containers::organization_id>(columns.organization_id)
                        .set_column::<crate::containers::container_model_id>(columns.model_id);
                ::diesel_builders::Insert::insert(builder, self)?;
                Ok(())
            }
            "digital_assets" => {
                if let Some((column, _)) = values.first() {
                    return Err(::procedure_like::rows::TypedRowError::UnknownColumn {
                        table: "digital_assets",
                        column: (*column).to_owned(),
                    });
                }
                let builder = <crate::digital_assets::table as ::diesel_builders::BuildableTable>::try_builder()?
                    .set_column::<crate::resources::id>(columns.id)
                    .set_column::<
                        crate::resources::organization_id,
                    >(columns.organization_id)
                    .set_column::<crate::resources::parent_id>(Some(columns.project_id))
                    .set_column::<
                        crate::resources::parent_kind,
                    >(Some(::aps_authz_resource_kind::ResourceKind::Project))
                    .set_column::<
                        crate::resources::created_by_id,
                    >(Some(columns.created_by_id))
                    .set_column::<
                        crate::assets::organization_id,
                    >(columns.organization_id)
                    .set_column::<crate::assets::project_id>(columns.project_id)
                    .set_column::<
                        crate::assets::table_name_id,
                    >("digital_assets".to_owned())
                    .set_column::<crate::assets::model_id>(columns.model_id)
                    .try_set_column::<crate::assets::name>(columns.name.clone())?
                    .try_set_column::<
                        crate::assets::description,
                    >(columns.description.clone())?
                    .set_column::<
                        crate::digital_assets::organization_id,
                    >(columns.organization_id)
                    .set_column::<
                        crate::digital_assets::digital_asset_model_id,
                    >(columns.model_id);
                ::diesel_builders::Insert::insert(builder, self)?;
                Ok(())
            }
            "panels" => {
                if let Some((column, _)) = values.first() {
                    return Err(::procedure_like::rows::TypedRowError::UnknownColumn {
                        table: "panels",
                        column: (*column).to_owned(),
                    });
                }
                let builder =
                    <crate::panels::table as ::diesel_builders::BuildableTable>::try_builder()?
                        .set_column::<crate::resources::id>(columns.id)
                        .set_column::<crate::resources::organization_id>(columns.organization_id)
                        .set_column::<crate::resources::parent_id>(Some(columns.project_id))
                        .set_column::<crate::resources::parent_kind>(Some(
                            ::aps_authz_resource_kind::ResourceKind::Project,
                        ))
                        .set_column::<crate::resources::created_by_id>(Some(columns.created_by_id))
                        .set_column::<crate::assets::organization_id>(columns.organization_id)
                        .set_column::<crate::assets::project_id>(columns.project_id)
                        .set_column::<crate::assets::table_name_id>("panels".to_owned())
                        .set_column::<crate::assets::model_id>(columns.model_id)
                        .try_set_column::<crate::assets::name>(columns.name.clone())?
                        .try_set_column::<crate::assets::description>(columns.description.clone())?
                        .set_column::<crate::physical_assets::organization_id>(
                            columns.organization_id,
                        )
                        .set_column::<crate::physical_assets::physical_asset_model_id>(
                            columns.model_id,
                        )
                        .set_column::<crate::panels::organization_id>(columns.organization_id)
                        .set_column::<crate::panels::panel_model_id>(columns.model_id);
                ::diesel_builders::Insert::insert(builder, self)?;
                Ok(())
            }
            "photographs" => {
                if let Some((column, _)) = values.first() {
                    return Err(::procedure_like::rows::TypedRowError::UnknownColumn {
                        table: "photographs",
                        column: (*column).to_owned(),
                    });
                }
                let builder =
                    <crate::photographs::table as ::diesel_builders::BuildableTable>::try_builder(
                    )?
                    .set_column::<crate::resources::id>(columns.id)
                    .set_column::<crate::resources::organization_id>(columns.organization_id)
                    .set_column::<crate::resources::parent_id>(Some(columns.project_id))
                    .set_column::<crate::resources::parent_kind>(Some(
                        ::aps_authz_resource_kind::ResourceKind::Project,
                    ))
                    .set_column::<crate::resources::created_by_id>(Some(columns.created_by_id))
                    .set_column::<crate::assets::organization_id>(columns.organization_id)
                    .set_column::<crate::assets::project_id>(columns.project_id)
                    .set_column::<crate::assets::table_name_id>("photographs".to_owned())
                    .set_column::<crate::assets::model_id>(columns.model_id)
                    .try_set_column::<crate::assets::name>(columns.name.clone())?
                    .try_set_column::<crate::assets::description>(columns.description.clone())?
                    .set_column::<crate::digital_assets::organization_id>(columns.organization_id)
                    .set_column::<crate::digital_assets::digital_asset_model_id>(columns.model_id)
                    .set_column::<crate::photographs::organization_id>(columns.organization_id)
                    .set_column::<crate::photographs::photograph_model_id>(columns.model_id);
                ::diesel_builders::Insert::insert(builder, self)?;
                Ok(())
            }
            "samples" => {
                let mut sample_source_model_id_value = None;
                for (column, value) in values {
                    match *column {
                        "sample_source_model_id" => {
                            sample_source_model_id_value =
                                Some(::procedure_like::rows::column_value::<::rosetta_uuid::Uuid>(
                                    "samples",
                                    "sample_source_model_id",
                                    value.clone(),
                                )?);
                        }
                        other => {
                            return Err(::procedure_like::rows::TypedRowError::UnknownColumn {
                                table: "samples",
                                column: other.to_owned(),
                            });
                        }
                    }
                }
                let builder =
                    <crate::samples::table as ::diesel_builders::BuildableTable>::try_builder()?
                        .set_column::<crate::resources::id>(columns.id)
                        .set_column::<crate::resources::organization_id>(columns.organization_id)
                        .set_column::<crate::resources::parent_id>(Some(columns.project_id))
                        .set_column::<crate::resources::parent_kind>(Some(
                            ::aps_authz_resource_kind::ResourceKind::Project,
                        ))
                        .set_column::<crate::resources::created_by_id>(Some(columns.created_by_id))
                        .set_column::<crate::assets::organization_id>(columns.organization_id)
                        .set_column::<crate::assets::project_id>(columns.project_id)
                        .set_column::<crate::assets::table_name_id>("samples".to_owned())
                        .set_column::<crate::assets::model_id>(columns.model_id)
                        .try_set_column::<crate::assets::name>(columns.name.clone())?
                        .try_set_column::<crate::assets::description>(columns.description.clone())?
                        .set_column::<crate::physical_assets::organization_id>(
                            columns.organization_id,
                        )
                        .set_column::<crate::physical_assets::physical_asset_model_id>(
                            columns.model_id,
                        )
                        .set_column::<crate::samples::organization_id>(columns.organization_id)
                        .set_column::<crate::samples::sample_model_id>(columns.model_id)
                        .try_set_column::<crate::samples::sample_source_model_id>(
                            sample_source_model_id_value.ok_or(
                                ::procedure_like::rows::TypedRowError::MissingValue {
                                    table: "samples",
                                    column: "sample_source_model_id",
                                },
                            )?,
                        )?;
                ::diesel_builders::Insert::insert(builder, self)?;
                Ok(())
            }
            "sample_sources" => {
                if let Some((column, _)) = values.first() {
                    return Err(::procedure_like::rows::TypedRowError::UnknownColumn {
                        table: "sample_sources",
                        column: (*column).to_owned(),
                    });
                }
                let builder = <crate::sample_sources::table as ::diesel_builders::BuildableTable>::try_builder()?
                    .set_column::<crate::resources::id>(columns.id)
                    .set_column::<
                        crate::resources::organization_id,
                    >(columns.organization_id)
                    .set_column::<crate::resources::parent_id>(Some(columns.project_id))
                    .set_column::<
                        crate::resources::parent_kind,
                    >(Some(::aps_authz_resource_kind::ResourceKind::Project))
                    .set_column::<
                        crate::resources::created_by_id,
                    >(Some(columns.created_by_id))
                    .set_column::<
                        crate::assets::organization_id,
                    >(columns.organization_id)
                    .set_column::<crate::assets::project_id>(columns.project_id)
                    .set_column::<
                        crate::assets::table_name_id,
                    >("sample_sources".to_owned())
                    .set_column::<crate::assets::model_id>(columns.model_id)
                    .try_set_column::<crate::assets::name>(columns.name.clone())?
                    .try_set_column::<
                        crate::assets::description,
                    >(columns.description.clone())?
                    .set_column::<
                        crate::physical_assets::organization_id,
                    >(columns.organization_id)
                    .set_column::<
                        crate::physical_assets::physical_asset_model_id,
                    >(columns.model_id)
                    .set_column::<
                        crate::sample_sources::organization_id,
                    >(columns.organization_id)
                    .set_column::<
                        crate::sample_sources::sample_source_model_id,
                    >(columns.model_id);
                ::diesel_builders::Insert::insert(builder, self)?;
                Ok(())
            }
            "volumetric_containers" => {
                if let Some((column, _)) = values.first() {
                    return Err(::procedure_like::rows::TypedRowError::UnknownColumn {
                        table: "volumetric_containers",
                        column: (*column).to_owned(),
                    });
                }
                let builder = <crate::volumetric_containers::table as ::diesel_builders::BuildableTable>::try_builder()?
                    .set_column::<crate::resources::id>(columns.id)
                    .set_column::<
                        crate::resources::organization_id,
                    >(columns.organization_id)
                    .set_column::<crate::resources::parent_id>(Some(columns.project_id))
                    .set_column::<
                        crate::resources::parent_kind,
                    >(Some(::aps_authz_resource_kind::ResourceKind::Project))
                    .set_column::<
                        crate::resources::created_by_id,
                    >(Some(columns.created_by_id))
                    .set_column::<
                        crate::assets::organization_id,
                    >(columns.organization_id)
                    .set_column::<crate::assets::project_id>(columns.project_id)
                    .set_column::<
                        crate::assets::table_name_id,
                    >("volumetric_containers".to_owned())
                    .set_column::<crate::assets::model_id>(columns.model_id)
                    .try_set_column::<crate::assets::name>(columns.name.clone())?
                    .try_set_column::<
                        crate::assets::description,
                    >(columns.description.clone())?
                    .set_column::<
                        crate::physical_assets::organization_id,
                    >(columns.organization_id)
                    .set_column::<
                        crate::physical_assets::physical_asset_model_id,
                    >(columns.model_id)
                    .set_column::<
                        crate::containers::organization_id,
                    >(columns.organization_id)
                    .set_column::<
                        crate::containers::container_model_id,
                    >(columns.model_id)
                    .set_column::<
                        crate::volumetric_containers::organization_id,
                    >(columns.organization_id)
                    .set_column::<
                        crate::volumetric_containers::volumetric_container_model_id,
                    >(columns.model_id);
                ::diesel_builders::Insert::insert(builder, self)?;
                Ok(())
            }
            other => Err(::procedure_like::rows::TypedRowError::UnknownTable(other.to_owned())),
        }
    }
}
