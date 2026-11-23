#![allow(ambiguous_glob_reexports, reason = "TableModel trait names can collide across tables")]
extern crate self as aps_freeze_dryer_models;
pub mod freeze_dryer_models_model {
    #[derive(
        Copy,
        Clone,
        Debug,
        Hash,
        Ord,
        PartialOrd,
        Eq,
        PartialEq,
        ::serde::Serialize,
        ::serde::Deserialize,
        ::diesel::Queryable,
        ::diesel::Selectable,
        ::diesel::Identifiable,
        ::diesel::Associations,
        ::diesel_builders::prelude::TableModel,
    )]
    /// Catalog of freeze dryer model definitions.
    #[table_model(ancestors(
        aps_resources::resources,
        aps_resources::ownables,
        aps_resources::asset_models,
        aps_physical_asset_models::physical_asset_models
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(
        belongs_to(aps_physical_asset_models::PhysicalAssetModel, foreign_key = id)
    )]
    #[table_model(
        foreign_key((id), (::aps_physical_asset_models::physical_asset_models::id))
    )]
    #[table_model(default(
        aps_resources::resources::kind,
        ::aps_authz_resource_kind::ResourceKind::Ownable
    ))]
    #[diesel(table_name = freeze_dryer_models)]
    pub struct FreezeDryerModel {
        /// Stable model identifier inherited from physical_asset_models.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing this resource.
        #[same_as(aps_physical_asset_models::physical_asset_models::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
    }
    impl ::diesel_builders::GetColumn<aps_resources::asset_models::id> for FreezeDryerModel {
        fn get_column_ref(
            &self,
        ) -> &<freeze_dryer_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id> for FreezeDryerModel {
        fn get_column_ref(
            &self,
        ) -> &<freeze_dryer_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_physical_asset_models::physical_asset_models::id>
        for FreezeDryerModel
    {
        fn get_column_ref(
            &self,
        ) -> &<freeze_dryer_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for FreezeDryerModel {
        fn get_column_ref(
            &self,
        ) -> &<freeze_dryer_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    /// Shape of `freeze_dryer_models` the procedure runner renders.
    pub const FREEZE_DRYER_MODELS_SHAPE: procedure_like::shape::AssetModelShape =
        procedure_like::shape::AssetModelShape {
            table_name: "freeze_dryer_models",
            asset_table_name: "physical_assets",
            model_column: "physical_asset_model_id",
            required: &[
                procedure_like::shape::ColumnShape {
                    name: "organization_id",
                    label: "connetto organization owning the row.",
                    sql_type: procedure_like::shape::SqlType::Uuid,
                    unit: None,
                    required: true,
                },
                procedure_like::shape::ColumnShape {
                    name: "physical_asset_model_id",
                    label: "Model of the asset, which must be a physical asset model.",
                    sql_type: procedure_like::shape::SqlType::Uuid,
                    unit: None,
                    required: true,
                },
            ],
        };
    ::diesel::allow_tables_to_appear_in_same_query!(
        freeze_dryer_models,
        ::aps_resources::asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(freeze_dryer_models, ::aps_resources::ownables);
    ::diesel::allow_tables_to_appear_in_same_query!(
        freeze_dryer_models,
        ::aps_physical_asset_models::physical_asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        freeze_dryer_models,
        ::aps_resources::resources
    );
}
pub use freeze_dryer_models_model::*;
pub mod freeze_drying_procedure_templates_model {
    #[derive(
        Clone,
        Debug,
        PartialOrd,
        PartialEq,
        ::serde::Serialize,
        ::serde::Deserialize,
        ::diesel::Queryable,
        ::diesel::Selectable,
        ::diesel::Identifiable,
        ::diesel::Associations,
        ::diesel_builders::prelude::TableModel,
    )]
    /// Typed slots and parameters for freeze drying.
    #[table_model(ancestors(
        aps_resources::resources,
        aps_resources::ownables,
        aps_procedure_templates::procedure_templates
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_procedure_templates::ProcedureTemplate, foreign_key = id))]
    #[diesel(
        belongs_to(
            aps_freeze_dryer_models::FreezeDryerModel,
            foreign_key = freeze_dried_with_model_id
        )
    )]
    #[diesel(
        belongs_to(
            aps_physical_asset_models::VolumetricContainerModel,
            foreign_key = freeze_dried_container_model_id
        )
    )]
    #[table_model(
        foreign_key((id), (::aps_procedure_templates::procedure_templates::id))
    )]
    #[table_model(
        foreign_key(
            (freeze_dried_with_model_id),
            (::aps_freeze_dryer_models::freeze_dryer_models::id)
        )
    )]
    #[table_model(
        foreign_key(
            (freeze_dried_container_model_id),
            (::aps_physical_asset_models::volumetric_container_models::id)
        )
    )]
    #[table_model(
        foreign_key(
            (
                freeze_dried_with_slot_id,
                id,
                freeze_dried_with_model_id,
                freeze_dried_with_slot_role_id,
                organization_id
            ),
            (
                ::aps_procedure_templates::procedure_template_slots::id,
                ::aps_procedure_templates::procedure_template_slots::procedure_template_id,
                ::aps_procedure_templates::procedure_template_slots::asset_model_id,
                ::aps_procedure_templates::procedure_template_slots::role_id,
                ::aps_procedure_templates::procedure_template_slots::organization_id
            )
        )
    )]
    #[table_model(
        foreign_key(
            (
                freeze_dried_container_slot_id,
                id,
                freeze_dried_container_model_id,
                freeze_dried_container_slot_role_id,
                organization_id
            ),
            (
                ::aps_procedure_templates::procedure_template_slots::id,
                ::aps_procedure_templates::procedure_template_slots::procedure_template_id,
                ::aps_procedure_templates::procedure_template_slots::asset_model_id,
                ::aps_procedure_templates::procedure_template_slots::role_id,
                ::aps_procedure_templates::procedure_template_slots::organization_id
            )
        )
    )]
    #[table_model(default(
        aps_resources::resources::kind,
        ::aps_authz_resource_kind::ResourceKind::Ownable
    ))]
    #[diesel(table_name = freeze_drying_procedure_templates)]
    pub struct FreezeDryingProcedureTemplate {
        /// Identity shared with the parent template.
        #[same_as(
            aps_procedure_templates::procedure_template_slots::procedure_template_id,
            (freeze_dried_with_slot_id, freeze_dried_container_slot_id)
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing the parent resource.
        #[same_as(aps_procedure_templates::procedure_templates::organization_id)]
        #[same_as(
            aps_procedure_templates::procedure_template_slots::organization_id,
            (freeze_dried_with_slot_id, freeze_dried_container_slot_id)
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Slot declaring the `freeze_dried_with` assets.
        #[mandatory(aps_procedure_templates::procedure_template_slots)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        freeze_dried_with_slot_id: ::rosetta_uuid::Uuid,
        /// Admissible model for the `freeze_dried_with` slot.
        #[same_as(
            aps_procedure_templates::procedure_template_slots::asset_model_id,
            freeze_dried_with_slot_id
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        freeze_dried_with_model_id: ::rosetta_uuid::Uuid,
        /// Required role of the `freeze_dried_with` slot.
        #[table_model(generated)]
        freeze_dried_with_slot_role_id: String,
        /// Slot declaring the `freeze_dried_container` assets.
        #[mandatory(aps_procedure_templates::procedure_template_slots)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        freeze_dried_container_slot_id: ::rosetta_uuid::Uuid,
        /// Admissible model for the `freeze_dried_container` slot.
        #[same_as(
            aps_procedure_templates::procedure_template_slots::asset_model_id,
            freeze_dried_container_slot_id
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        freeze_dried_container_model_id: ::rosetta_uuid::Uuid,
        /// Required role of the `freeze_dried_container` slot.
        #[table_model(generated)]
        freeze_dried_container_slot_role_id: String,
        /// Target temperature in kelvin.
        #[table_model(default = 203.15f64)]
        temperature_k: f64,
        /// Allowed temperature deviation as a percentage of the target.
        #[table_model(default = 5f64)]
        temperature_tolerance_percentage: f64,
        /// Target pressure in pascals.
        #[table_model(default = 4f64)]
        pressure_pa: f64,
        /// Allowed pressure deviation as a percentage of the target.
        #[table_model(default = 5f64)]
        pressure_pa_tolerance_percentage: f64,
        /// Treatment duration in seconds.
        #[table_model(default = 259200f64)]
        duration_s: f64,
        /// Allowed duration deviation as a percentage of the target.
        #[table_model(default = 5f64)]
        duration_tolerance_percentage: f64,
    }
    impl ::diesel_builders::ValidateColumn<freeze_drying_procedure_templates::temperature_k>
        for <freeze_drying_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(temperature_k: &f64) -> Result<(), Self::Error> {
            use diesel::Column;
            if temperature_k <= &0f64 {
                return Err(
                    ::validation_errors::ValidationError::strictly_greater_than_value(
                        <crate::freeze_drying_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::freeze_drying_procedure_templates::temperature_k::NAME,
                        0f64,
                    ),
                );
            }
            if temperature_k.partial_cmp(&f64::INFINITY).is_none_or(::core::cmp::Ordering::is_ge) {
                return Err(
                    ::validation_errors::ValidationError::check_violated(
                        <crate::freeze_drying_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        &[crate::freeze_drying_procedure_templates::temperature_k::NAME],
                        "temperature_k < 'Infinity'::DOUBLE PRECISION",
                    ),
                );
            }
            Ok(())
        }
    }
    impl
        ::diesel_builders::ValidateColumn<
            freeze_drying_procedure_templates::temperature_tolerance_percentage,
        > for <freeze_drying_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(temperature_tolerance_percentage: &f64) -> Result<(), Self::Error> {
            use diesel::Column;
            if temperature_tolerance_percentage <= &0f64 {
                return Err(
                    ::validation_errors::ValidationError::strictly_greater_than_value(
                        <crate::freeze_drying_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::freeze_drying_procedure_templates::temperature_tolerance_percentage::NAME,
                        0f64,
                    ),
                );
            }
            if temperature_tolerance_percentage > &100f64 {
                return Err(
                    ::validation_errors::ValidationError::smaller_than_value(
                        <crate::freeze_drying_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::freeze_drying_procedure_templates::temperature_tolerance_percentage::NAME,
                        100f64,
                    ),
                );
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateColumn<freeze_drying_procedure_templates::pressure_pa>
        for <freeze_drying_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(pressure_pa: &f64) -> Result<(), Self::Error> {
            use diesel::Column;
            if pressure_pa < &0f64 {
                return Err(
                    ::validation_errors::ValidationError::greater_than_value(
                        <crate::freeze_drying_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::freeze_drying_procedure_templates::pressure_pa::NAME,
                        0f64,
                    ),
                );
            }
            if pressure_pa > &500f64 {
                return Err(
                    ::validation_errors::ValidationError::smaller_than_value(
                        <crate::freeze_drying_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::freeze_drying_procedure_templates::pressure_pa::NAME,
                        500f64,
                    ),
                );
            }
            Ok(())
        }
    }
    impl
        ::diesel_builders::ValidateColumn<
            freeze_drying_procedure_templates::pressure_pa_tolerance_percentage,
        > for <freeze_drying_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(pressure_pa_tolerance_percentage: &f64) -> Result<(), Self::Error> {
            use diesel::Column;
            if pressure_pa_tolerance_percentage <= &0f64 {
                return Err(
                    ::validation_errors::ValidationError::strictly_greater_than_value(
                        <crate::freeze_drying_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::freeze_drying_procedure_templates::pressure_pa_tolerance_percentage::NAME,
                        0f64,
                    ),
                );
            }
            if pressure_pa_tolerance_percentage > &100f64 {
                return Err(
                    ::validation_errors::ValidationError::smaller_than_value(
                        <crate::freeze_drying_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::freeze_drying_procedure_templates::pressure_pa_tolerance_percentage::NAME,
                        100f64,
                    ),
                );
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateColumn<freeze_drying_procedure_templates::duration_s>
        for <freeze_drying_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(duration_s: &f64) -> Result<(), Self::Error> {
            use diesel::Column;
            if duration_s < &7200f64 {
                return Err(
                    ::validation_errors::ValidationError::greater_than_value(
                        <crate::freeze_drying_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::freeze_drying_procedure_templates::duration_s::NAME,
                        7200f64,
                    ),
                );
            }
            if duration_s > &604800f64 {
                return Err(
                    ::validation_errors::ValidationError::smaller_than_value(
                        <crate::freeze_drying_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::freeze_drying_procedure_templates::duration_s::NAME,
                        604800f64,
                    ),
                );
            }
            Ok(())
        }
    }
    impl
        ::diesel_builders::ValidateColumn<
            freeze_drying_procedure_templates::duration_tolerance_percentage,
        > for <freeze_drying_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(duration_tolerance_percentage: &f64) -> Result<(), Self::Error> {
            use diesel::Column;
            if duration_tolerance_percentage <= &0f64 {
                return Err(
                    ::validation_errors::ValidationError::strictly_greater_than_value(
                        <crate::freeze_drying_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::freeze_drying_procedure_templates::duration_tolerance_percentage::NAME,
                        0f64,
                    ),
                );
            }
            if duration_tolerance_percentage > &100f64 {
                return Err(
                    ::validation_errors::ValidationError::smaller_than_value(
                        <crate::freeze_drying_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::freeze_drying_procedure_templates::duration_tolerance_percentage::NAME,
                        100f64,
                    ),
                );
            }
            Ok(())
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id> for FreezeDryingProcedureTemplate {
        fn get_column_ref(
            &self,
        ) -> &<freeze_drying_procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_procedure_templates::procedure_templates::id>
        for FreezeDryingProcedureTemplate
    {
        fn get_column_ref(
            &self,
        ) -> &<freeze_drying_procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for FreezeDryingProcedureTemplate {
        fn get_column_ref(
            &self,
        ) -> &<freeze_drying_procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    /// Shape of `freeze_drying_procedure_templates` the procedure runner
    /// renders.
    pub const FREEZE_DRYING_PROCEDURE_TEMPLATES_SHAPE: procedure_like::shape::TemplateShape =
        procedure_like::shape::TemplateShape {
            table_name: "freeze_drying_procedure_templates",
            setpoints: &[
                procedure_like::shape::ColumnShape {
                    name: "temperature_k",
                    label: "Target temperature in kelvin.",
                    sql_type: procedure_like::shape::SqlType::DoublePrecision,
                    unit: Some(procedure_like::shape::Unit::Kelvin),
                    required: true,
                },
                procedure_like::shape::ColumnShape {
                    name: "temperature_tolerance_percentage",
                    label: "Allowed temperature deviation as a percentage of the target.",
                    sql_type: procedure_like::shape::SqlType::DoublePrecision,
                    unit: Some(procedure_like::shape::Unit::Percentage),
                    required: true,
                },
                procedure_like::shape::ColumnShape {
                    name: "pressure_pa",
                    label: "Target pressure in pascals.",
                    sql_type: procedure_like::shape::SqlType::DoublePrecision,
                    unit: Some(procedure_like::shape::Unit::Pascal),
                    required: true,
                },
                procedure_like::shape::ColumnShape {
                    name: "pressure_pa_tolerance_percentage",
                    label: "Allowed pressure deviation as a percentage of the target.",
                    sql_type: procedure_like::shape::SqlType::DoublePrecision,
                    unit: Some(procedure_like::shape::Unit::Percentage),
                    required: true,
                },
                procedure_like::shape::ColumnShape {
                    name: "duration_s",
                    label: "Treatment duration in seconds.",
                    sql_type: procedure_like::shape::SqlType::DoublePrecision,
                    unit: Some(procedure_like::shape::Unit::Second),
                    required: true,
                },
                procedure_like::shape::ColumnShape {
                    name: "duration_tolerance_percentage",
                    label: "Allowed duration deviation as a percentage of the target.",
                    sql_type: procedure_like::shape::SqlType::DoublePrecision,
                    unit: Some(procedure_like::shape::Unit::Percentage),
                    required: true,
                },
            ],
            slots: &[
                procedure_like::shape::SlotShape {
                    name: "freeze_dried_with",
                    role: procedure_like::shape::SlotRole::Instrument,
                    model_table_name: "freeze_dryer_models",
                    slot_column: "freeze_dried_with_slot_id",
                    model_column: "freeze_dried_with_model_id",
                    placed_into: None,
                    retrieved_from: None,
                },
                procedure_like::shape::SlotShape {
                    name: "freeze_dried_container",
                    role: procedure_like::shape::SlotRole::Input,
                    model_table_name: "volumetric_container_models",
                    slot_column: "freeze_dried_container_slot_id",
                    model_column: "freeze_dried_container_model_id",
                    placed_into: None,
                    retrieved_from: None,
                },
            ],
        };
    ::diesel::allow_tables_to_appear_in_same_query!(
        freeze_drying_procedure_templates,
        ::aps_freeze_dryer_models::freeze_dryer_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        freeze_drying_procedure_templates,
        ::aps_resources::ownables
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        freeze_drying_procedure_templates,
        ::aps_procedure_templates::procedure_template_slots
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        freeze_drying_procedure_templates,
        ::aps_procedure_templates::procedure_templates
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        freeze_drying_procedure_templates,
        ::aps_resources::resources
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        freeze_drying_procedure_templates,
        ::aps_physical_asset_models::volumetric_container_models
    );
}
pub use freeze_drying_procedure_templates_model::*;
pub mod freeze_drying_procedures_model {
    #[derive(
        Clone,
        Debug,
        PartialOrd,
        PartialEq,
        ::serde::Serialize,
        ::serde::Deserialize,
        ::diesel::Queryable,
        ::diesel::Selectable,
        ::diesel::Identifiable,
        ::diesel::Associations,
        ::diesel_builders::prelude::TableModel,
    )]
    /// Recorded execution of a freeze drying template.
    #[table_model(ancestors(
        aps_resources::resources,
        aps_resources::ownables,
        aps_procedures::procedures
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[table_model(record_error = ::validation_errors::ValidationError)]
    #[diesel(
        belongs_to(
            aps_freeze_dryer_models::FreezeDryingProcedureTemplate,
            foreign_key = freeze_drying_procedure_template_id
        )
    )]
    #[table_model(foreign_key((id), (::aps_procedures::procedures::id)))]
    #[table_model(
        foreign_key(
            (freeze_drying_procedure_template_id),
            (::aps_freeze_dryer_models::freeze_drying_procedure_templates::id)
        )
    )]
    #[table_model(default(
        aps_resources::resources::kind,
        ::aps_authz_resource_kind::ResourceKind::Ownable
    ))]
    #[diesel(table_name = freeze_drying_procedures)]
    pub struct FreezeDryingProcedure {
        /// Identity shared with the parent procedure.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing the parent resource.
        #[same_as(aps_procedures::procedures::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Typed template executed by this procedure.
        #[same_as(aps_procedures::procedures::procedure_template_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        freeze_drying_procedure_template_id: ::rosetta_uuid::Uuid,
        /// Execution status copied from the parent procedure.
        #[same_as(aps_procedures::procedures::status_id)]
        #[infallible]
        status_id: String,
        /// Measured temperature k.
        temperature_k: Option<f64>,
        /// Measured pressure pa.
        pressure_pa: Option<f64>,
    }
    impl ::diesel_builders::ValidateColumn<freeze_drying_procedures::temperature_k>
        for <freeze_drying_procedures::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(temperature_k: &f64) -> Result<(), Self::Error> {
            use diesel::Column;
            if temperature_k <= &0f64 {
                return Err(
                    ::validation_errors::ValidationError::strictly_greater_than_value(
                        <crate::freeze_drying_procedures::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::freeze_drying_procedures::temperature_k::NAME,
                        0f64,
                    ),
                );
            }
            if temperature_k.partial_cmp(&f64::INFINITY).is_none_or(::core::cmp::Ordering::is_ge) {
                return Err(
                    ::validation_errors::ValidationError::check_violated(
                        <crate::freeze_drying_procedures::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        &[crate::freeze_drying_procedures::temperature_k::NAME],
                        "temperature_k < 'Infinity'::DOUBLE PRECISION",
                    ),
                );
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateColumn<freeze_drying_procedures::pressure_pa>
        for <freeze_drying_procedures::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(pressure_pa: &f64) -> Result<(), Self::Error> {
            use diesel::Column;
            if pressure_pa < &0f64 {
                return Err(
                    ::validation_errors::ValidationError::greater_than_value(
                        <crate::freeze_drying_procedures::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::freeze_drying_procedures::pressure_pa::NAME,
                        0f64,
                    ),
                );
            }
            if pressure_pa.partial_cmp(&f64::INFINITY).is_none_or(::core::cmp::Ordering::is_ge) {
                return Err(
                    ::validation_errors::ValidationError::check_violated(
                        <crate::freeze_drying_procedures::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        &[crate::freeze_drying_procedures::pressure_pa::NAME],
                        "pressure_pa < 'Infinity'::DOUBLE PRECISION",
                    ),
                );
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateRecord<freeze_drying_procedures::table>
        for <freeze_drying_procedures::table as ::diesel_builders::TableExt>::NewValues
    {
        #[inline]
        fn validate_record(&self) -> Result<(), ::validation_errors::ValidationError> {
            use diesel::Column;
            if let (Some(status_id), Some(temperature_k)) = (
                <Self as diesel_builders::MayGetColumn<
                    freeze_drying_procedures::status_id,
                >>::may_get_column_ref(self),
                <Self as diesel_builders::MayGetColumn<
                    freeze_drying_procedures::temperature_k,
                >>::may_get_column_ref(self),
            ) && !((status_id.as_str() != "completed") || (*temperature_k).is_some())
            {
                return Err(
                    ::validation_errors::ValidationError::check_violated(
                        <crate::freeze_drying_procedures::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        &[
                            crate::freeze_drying_procedures::status_id::NAME,
                            crate::freeze_drying_procedures::temperature_k::NAME,
                        ],
                        "status_id <> 'completed' OR temperature_k IS NOT NULL",
                    ),
                );
            }
            if let (Some(status_id), Some(pressure_pa)) = (
                <Self as diesel_builders::MayGetColumn<
                    freeze_drying_procedures::status_id,
                >>::may_get_column_ref(self),
                <Self as diesel_builders::MayGetColumn<
                    freeze_drying_procedures::pressure_pa,
                >>::may_get_column_ref(self),
            ) && !((status_id.as_str() != "completed") || (*pressure_pa).is_some())
            {
                return Err(
                    ::validation_errors::ValidationError::check_violated(
                        <crate::freeze_drying_procedures::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        &[
                            crate::freeze_drying_procedures::status_id::NAME,
                            crate::freeze_drying_procedures::pressure_pa::NAME,
                        ],
                        "status_id <> 'completed' OR pressure_pa IS NOT NULL",
                    ),
                );
            }
            Ok(())
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id> for FreezeDryingProcedure {
        fn get_column_ref(
            &self,
        ) -> &<freeze_drying_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_procedures::procedures::id> for FreezeDryingProcedure {
        fn get_column_ref(
            &self,
        ) -> &<freeze_drying_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for FreezeDryingProcedure {
        fn get_column_ref(
            &self,
        ) -> &<freeze_drying_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl procedure_like::ProcedureTableLike for freeze_drying_procedures::table {
        type ProcedureTemplateTable =
            aps_freeze_dryer_models::freeze_drying_procedure_templates::table;
    }
    /// Shape of `freeze_drying_procedures` the procedure runner renders.
    pub const FREEZE_DRYING_PROCEDURES_SHAPE: procedure_like::shape::ProcedureShape =
        procedure_like::shape::ProcedureShape {
            table_name: "freeze_drying_procedures",
            template_table_name: "freeze_drying_procedure_templates",
            measurements: &[
                procedure_like::shape::ColumnShape {
                    name: "temperature_k",
                    label: "Measured temperature k.",
                    sql_type: procedure_like::shape::SqlType::DoublePrecision,
                    unit: Some(procedure_like::shape::Unit::Kelvin),
                    required: true,
                },
                procedure_like::shape::ColumnShape {
                    name: "pressure_pa",
                    label: "Measured pressure pa.",
                    sql_type: procedure_like::shape::SqlType::DoublePrecision,
                    unit: Some(procedure_like::shape::Unit::Pascal),
                    required: true,
                },
            ],
        };
    ::diesel::allow_tables_to_appear_in_same_query!(
        freeze_drying_procedures,
        ::aps_freeze_dryer_models::freeze_drying_procedure_templates
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        freeze_drying_procedures,
        ::aps_resources::ownables
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        freeze_drying_procedures,
        ::aps_procedures::procedures
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        freeze_drying_procedures,
        ::aps_resources::resources
    );
}
pub use freeze_drying_procedures_model::*;
