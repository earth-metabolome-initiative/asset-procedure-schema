pub use aps_assets::{
    self, Asset, Container, DigitalAsset, Marker, MarkerModel, Panel, PanelModel,
    PersonalProtectiveEquipment, PersonalProtectiveEquipmentModel, Photograph, PhysicalAsset,
    PipetteTip, PipetteTipModel, SampleSource, SampleSourceModel, VolumeMeasuringDevice,
    VolumetricContainer, WeighingDevice, assets, containers, digital_assets, marker_models,
    markers, panel_models, panels, personal_protective_equipment_models,
    personal_protective_equipments, photographs, physical_assets, pipette_tip_models, pipette_tips,
    sample_source_models, sample_sources, volume_measuring_devices, volumetric_containers,
    weighing_devices,
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
    self, Camera, CameraModel, CommercialCameraLot, CommercialCameraModel,
    CommercialGeopositioningDeviceLot, CommercialGeopositioningDeviceModel, CommercialMarkerLot,
    CommercialMarkerModel, CommercialMassSpectrometerLot, CommercialMassSpectrometerModel,
    CommercialPersonalProtectiveEquipmentLot, CommercialPersonalProtectiveEquipmentModel,
    CommercialPhoneDeviceLot, CommercialPhoneDeviceModel, CommercialPipetteTipLot,
    CommercialPipetteTipModel, CommercialProduct, CommercialProductLot,
    CommercialVolumeMeasuringDeviceLot, CommercialVolumeMeasuringDeviceModel,
    CommercialWeighingDeviceLot, CommercialWeighingDeviceModel, GeopositioningDevice,
    GeopositioningDeviceModel, GeopositioningProcedure, GeopositioningProcedureTemplate,
    MassSpectrometer, MassSpectrometerModel, PhoneDevice, PhoneDeviceModel, camera_models, cameras,
    commercial_camera_lots, commercial_camera_models, commercial_geopositioning_device_lots,
    commercial_geopositioning_device_models, commercial_marker_lots, commercial_marker_models,
    commercial_mass_spectrometer_lots, commercial_mass_spectrometer_models,
    commercial_personal_protective_equipment_lots, commercial_personal_protective_equipment_models,
    commercial_phone_device_lots, commercial_phone_device_models, commercial_pipette_tip_lots,
    commercial_pipette_tip_models, commercial_product_lots, commercial_products,
    commercial_volume_measuring_device_lots, commercial_volume_measuring_device_models,
    commercial_weighing_device_lots, commercial_weighing_device_models,
    geopositioning_device_models, geopositioning_devices, geopositioning_procedure_templates,
    geopositioning_procedures, mass_spectrometer_models, mass_spectrometers, phone_device_models,
    phone_devices,
};
pub use aps_freeze_dryer_models::{
    self, FreezeDryerModel, FreezeDryingProcedure, FreezeDryingProcedureTemplate,
    freeze_dryer_models, freeze_drying_procedure_templates, freeze_drying_procedures,
};
pub use aps_grantees::{
    self, App, AppInstallation, Grantee, InstallationPermission, Invitation, Membership,
    Organization, OrganizationBlock, PlatformOperator, ShareLink, Team, TeamMember, TeamParent,
    User, app_installations, apps, grantees, installation_permissions, invitations, memberships,
    organization_blocks, organizations, platform_operators, share_links, team_members,
    team_parents, teams, users,
};
pub use aps_photograph_procedure_templates::{
    self, PhotographProcedure, PhotographProcedureTemplate, photograph_procedure_templates,
    photograph_procedures,
};
pub use aps_physical_asset_models::{
    self, ContainerCompatibilityRule, ContainerModel, PhysicalAssetModel,
    RetrievalProcedureTemplate, StorageProcedureTemplate, VolumeMeasuringDeviceModel,
    VolumetricContainerModel, WeighedContainerModel, WeighedVolumetricContainerModel,
    container_compatibility_rules, container_models, physical_asset_models,
    retrieval_procedure_templates, storage_procedure_templates, volume_measuring_device_models,
    volumetric_container_models, weighed_container_models, weighed_volumetric_container_models,
};
pub use aps_platform_audit_events::{self, PlatformAuditEvent, platform_audit_events};
pub use aps_procedure_templates::{
    self, ProcedureTemplate, ProcedureTemplateSlot, ProcedureTemplateState, ProcedureTemplateStep,
    ProcedureTemplateStepEdge, SlotRole, procedure_template_slots, procedure_template_states,
    procedure_template_step_edges, procedure_template_steps, procedure_templates, slot_roles,
};
pub use aps_procedures::{
    self, ContainerSealerModel, ContainerSealingProcedure, ContainerSealingProcedureTemplate,
    DisposalProcedure, DisposalProcedureTemplate, FreezerModel, FreezingProcedure,
    FreezingProcedureTemplate, ParameterValueKind, Procedure, ProcedureParameterValue,
    ProcedureStatus, ProcedureTemplateParameter, ProcedureTemplateStepParameter, QuantityKind,
    RetrievalProcedure, StorageProcedure, container_sealer_models,
    container_sealing_procedure_templates, container_sealing_procedures,
    disposal_procedure_templates, disposal_procedures, freezer_models,
    freezing_procedure_templates, freezing_procedures, parameter_value_kinds,
    procedure_parameter_values, procedure_statuses, procedure_template_parameters,
    procedure_template_step_parameters, procedures, quantity_kinds, retrieval_procedures,
    storage_procedures,
};
pub use aps_resources::{
    self, AssetModel, Brand, DigitalAssetModel, Grant, Ownable, PhotographModel, Resource,
    ResourceParent, TableName, TableNameAdmittedParent, TableNameAncestor, TableNameInstance,
    asset_models, brands, digital_asset_models, grants, ownables, photograph_models,
    resource_parents, resources, table_name_admitted_parents, table_name_ancestors,
    table_name_instances, table_names,
};
pub use aps_sample_models::{
    self, AliquotingProcedure, AliquotingProcedureTemplate, AssetModelAncestor,
    FractioningProcedure, FractioningProcedureTemplate, HarvestingProcedure,
    HarvestingProcedureTemplate, PackagingModel, PackagingProcedure, PackagingProcedureTemplate,
    PouringProcedure, PouringProcedureTemplate, ProcedureAsset, ProcedureTemplateSlotBinding,
    Sample, SampleModel, SlotRolePair, SupernatantProcedure, SupernatantProcedureTemplate,
    aliquoting_procedure_templates, aliquoting_procedures, asset_model_ancestors,
    fractioning_procedure_templates, fractioning_procedures, harvesting_procedure_templates,
    harvesting_procedures, packaging_models, packaging_procedure_templates, packaging_procedures,
    pouring_procedure_templates, pouring_procedures, procedure_assets,
    procedure_template_slot_bindings, sample_models, samples, slot_role_pairs,
    supernatant_procedure_templates, supernatant_procedures,
};
pub use aps_weighing_device_models::{
    self, WeighingDeviceModel, WeighingProcedure, WeighingProcedureTemplate,
    weighing_device_models, weighing_procedure_templates, weighing_procedures,
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
        "fractioning_procedures" => Some(&aps_sample_models::FRACTIONING_PROCEDURES_SHAPE),
        "freeze_drying_procedures" => {
            Some(&aps_freeze_dryer_models::FREEZE_DRYING_PROCEDURES_SHAPE)
        }
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
            Some(&aps_sample_models::FRACTIONING_PROCEDURE_TEMPLATES_SHAPE)
        }
        "freeze_drying_procedure_templates" => {
            Some(&aps_freeze_dryer_models::FREEZE_DRYING_PROCEDURE_TEMPLATES_SHAPE)
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
        "freeze_dryer_models" => Some(&aps_freeze_dryer_models::FREEZE_DRYER_MODELS_SHAPE),
        "freezer_models" => Some(&aps_procedures::FREEZER_MODELS_SHAPE),
        "geopositioning_device_models" => {
            Some(&aps_commercial_products::GEOPOSITIONING_DEVICE_MODELS_SHAPE)
        }
        "marker_models" => Some(&aps_assets::MARKER_MODELS_SHAPE),
        "mass_spectrometer_models" => {
            Some(&aps_commercial_products::MASS_SPECTROMETER_MODELS_SHAPE)
        }
        "packaging_models" => Some(&aps_sample_models::PACKAGING_MODELS_SHAPE),
        "panel_models" => Some(&aps_assets::PANEL_MODELS_SHAPE),
        "personal_protective_equipment_models" => {
            Some(&aps_assets::PERSONAL_PROTECTIVE_EQUIPMENT_MODELS_SHAPE)
        }
        "phone_device_models" => Some(&aps_commercial_products::PHONE_DEVICE_MODELS_SHAPE),
        "photograph_models" => Some(&aps_resources::PHOTOGRAPH_MODELS_SHAPE),
        "physical_asset_models" => Some(&aps_physical_asset_models::PHYSICAL_ASSET_MODELS_SHAPE),
        "pipette_tip_models" => Some(&aps_assets::PIPETTE_TIP_MODELS_SHAPE),
        "sample_models" => Some(&aps_sample_models::SAMPLE_MODELS_SHAPE),
        "sample_source_models" => Some(&aps_assets::SAMPLE_SOURCE_MODELS_SHAPE),
        "volume_measuring_device_models" => {
            Some(&aps_physical_asset_models::VOLUME_MEASURING_DEVICE_MODELS_SHAPE)
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
