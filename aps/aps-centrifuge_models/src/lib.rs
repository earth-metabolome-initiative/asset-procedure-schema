#![allow(ambiguous_glob_reexports, reason = "TableModel trait names can collide across tables")]
extern crate self as aps_centrifuge_models;
pub mod centrifuge_models_model {
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
    /// Catalog of centrifuge model definitions.
    #[table_model(ancestors(
        aps_resources::resources,
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
    #[diesel(table_name = centrifuge_models)]
    pub struct CentrifugeModel {
        /// Stable model identifier inherited from physical_asset_models.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing this resource.
        #[same_as(aps_physical_asset_models::physical_asset_models::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Archive state inherited from this resource.
        #[same_as(aps_physical_asset_models::physical_asset_models::archived)]
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
    }
    ::diesel_builders::prelude::unique_index!(centrifuge_models::id, centrifuge_models::archived);
    impl ::diesel_builders::GetColumn<aps_resources::asset_models::id> for CentrifugeModel {
        fn get_column_ref(
            &self,
        ) -> &<centrifuge_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_physical_asset_models::physical_asset_models::id>
        for CentrifugeModel
    {
        fn get_column_ref(
            &self,
        ) -> &<centrifuge_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for CentrifugeModel {
        fn get_column_ref(
            &self,
        ) -> &<centrifuge_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    /// Shape of `centrifuge_models` the procedure runner renders.
    pub const CENTRIFUGE_MODELS_SHAPE: procedure_like::shape::AssetModelShape =
        procedure_like::shape::AssetModelShape {
            table_name: "centrifuge_models",
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
        centrifuge_models,
        ::aps_resources::asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        centrifuge_models,
        ::aps_commercial_products::commercial_product_lots
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        centrifuge_models,
        ::aps_commercial_products::commercial_products
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        centrifuge_models,
        ::aps_physical_asset_models::physical_asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(centrifuge_models, ::aps_resources::resources);
}
pub use centrifuge_models_model::*;
pub mod centrifuge_procedure_templates_model {
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
    /// Typed slots and parameters for centrifuge.
    #[table_model(ancestors(aps_resources::resources, aps_resources::procedure_templates))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_resources::ProcedureTemplate, foreign_key = id))]
    #[table_model(foreign_key((id), (::aps_resources::procedure_templates::id)))]
    #[table_model(
        foreign_key(
            (centrifuged_with_model_id),
            (::aps_centrifuge_models::centrifuge_models::id)
        )
    )]
    #[table_model(
        foreign_key(
            (centrifuged_container_model_id),
            (::aps_physical_asset_models::volumetric_container_models::id)
        )
    )]
    #[table_model(
        foreign_key(
            (
                centrifuged_with_slot_id,
                id,
                centrifuged_with_model_id,
                centrifuged_with_slot_role_id,
                organization_id
            ),
            (
                ::aps_resources::procedure_template_slots::id,
                ::aps_resources::procedure_template_slots::procedure_template_id,
                ::aps_resources::procedure_template_slots::asset_model_id,
                ::aps_resources::procedure_template_slots::role_id,
                ::aps_resources::procedure_template_slots::organization_id
            )
        )
    )]
    #[table_model(
        foreign_key(
            (
                centrifuged_container_slot_id,
                id,
                centrifuged_container_model_id,
                centrifuged_container_slot_role_id,
                organization_id
            ),
            (
                ::aps_resources::procedure_template_slots::id,
                ::aps_resources::procedure_template_slots::procedure_template_id,
                ::aps_resources::procedure_template_slots::asset_model_id,
                ::aps_resources::procedure_template_slots::role_id,
                ::aps_resources::procedure_template_slots::organization_id
            )
        )
    )]
    #[table_model(
        foreign_key(
            (centrifuged_with_model_id, centrifuged_with_model_archived),
            (
                ::aps_centrifuge_models::centrifuge_models::id,
                ::aps_centrifuge_models::centrifuge_models::archived
            )
        )
    )]
    #[table_model(
        foreign_key(
            (centrifuged_container_model_id, centrifuged_container_model_archived),
            (
                ::aps_physical_asset_models::volumetric_container_models::id,
                ::aps_physical_asset_models::volumetric_container_models::archived
            )
        )
    )]
    #[table_model(default(
        aps_resources::resources::kind,
        ::aps_authz_resource_kind::ResourceKind::Ownable
    ))]
    #[diesel(table_name = centrifuge_procedure_templates)]
    pub struct CentrifugeProcedureTemplate {
        /// Identity shared with the parent template.
        #[same_as(
            aps_resources::procedure_template_slots::procedure_template_id,
            (centrifuged_with_slot_id, centrifuged_container_slot_id)
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing the parent resource.
        #[same_as(aps_resources::procedure_templates::organization_id)]
        #[same_as(
            aps_resources::procedure_template_slots::organization_id,
            (centrifuged_with_slot_id, centrifuged_container_slot_id)
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Slot declaring the `centrifuged_with` assets.
        #[mandatory(aps_resources::procedure_template_slots)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        centrifuged_with_slot_id: ::rosetta_uuid::Uuid,
        /// Admissible model for the `centrifuged_with` slot.
        #[same_as(
            aps_resources::procedure_template_slots::asset_model_id,
            centrifuged_with_slot_id
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        centrifuged_with_model_id: ::rosetta_uuid::Uuid,
        /// Required role of the `centrifuged_with` slot.
        #[table_model(generated)]
        centrifuged_with_slot_role_id: String,
        /// Slot declaring the `centrifuged_container` assets.
        #[mandatory(aps_resources::procedure_template_slots)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        centrifuged_container_slot_id: ::rosetta_uuid::Uuid,
        /// Admissible model for the `centrifuged_container` slot.
        #[same_as(
            aps_resources::procedure_template_slots::asset_model_id,
            centrifuged_container_slot_id
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        centrifuged_container_model_id: ::rosetta_uuid::Uuid,
        /// Required role of the `centrifuged_container` slot.
        #[table_model(generated)]
        centrifuged_container_slot_role_id: String,
        /// Target temperature in kelvin.
        #[table_model(default = 293.15f64)]
        temperature_k: f64,
        /// Allowed temperature deviation as a percentage of the target.
        #[table_model(default = 1f64)]
        temperature_tolerance_percentage: f64,
        /// Treatment duration in seconds.
        #[table_model(default = 120f64)]
        duration_s: f64,
        /// Allowed duration deviation as a percentage of the target.
        #[table_model(default = 5f64)]
        duration_tolerance_percentage: f64,
        /// Rotational speed in revolutions per minute.
        #[table_model(default = 13000f64)]
        speed_rpm: f64,
        /// Allowed speed deviation as a percentage of the target.
        #[table_model(default = 5f64)]
        speed_rpm_tolerance_percentage: f64,
        /// Archive state inherited from this resource.
        #[same_as(aps_resources::procedure_templates::archived)]
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
        /// Archive state of the row named by `centrifuged_with_model_id`.
        #[table_model(default = false)]
        #[infallible]
        centrifuged_with_model_archived: bool,
        /// Archive state of the row named by `centrifuged_container_model_id`.
        #[table_model(default = false)]
        #[infallible]
        centrifuged_container_model_archived: bool,
    }
    ::diesel_builders::prelude::unique_index!(
        centrifuge_procedure_templates::id,
        centrifuge_procedure_templates::archived
    );
    impl ::diesel_builders::ValidateColumn<centrifuge_procedure_templates::temperature_k>
        for <centrifuge_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(temperature_k: &f64) -> Result<(), Self::Error> {
            use diesel::Column;
            if temperature_k <= &0f64 {
                return Err(
                    ::validation_errors::ValidationError::strictly_greater_than_value(
                        <crate::centrifuge_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::centrifuge_procedure_templates::temperature_k::NAME,
                        0f64,
                    ),
                );
            }
            if temperature_k.partial_cmp(&f64::INFINITY).is_none_or(::core::cmp::Ordering::is_ge) {
                return Err(
                    ::validation_errors::ValidationError::check_violated(
                        <crate::centrifuge_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        &[crate::centrifuge_procedure_templates::temperature_k::NAME],
                        "temperature_k < 'Infinity'::DOUBLE PRECISION",
                    ),
                );
            }
            Ok(())
        }
    }
    impl
        ::diesel_builders::ValidateColumn<
            centrifuge_procedure_templates::temperature_tolerance_percentage,
        > for <centrifuge_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(temperature_tolerance_percentage: &f64) -> Result<(), Self::Error> {
            use diesel::Column;
            if temperature_tolerance_percentage <= &0f64 {
                return Err(
                    ::validation_errors::ValidationError::strictly_greater_than_value(
                        <crate::centrifuge_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::centrifuge_procedure_templates::temperature_tolerance_percentage::NAME,
                        0f64,
                    ),
                );
            }
            if temperature_tolerance_percentage > &100f64 {
                return Err(
                    ::validation_errors::ValidationError::smaller_than_value(
                        <crate::centrifuge_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::centrifuge_procedure_templates::temperature_tolerance_percentage::NAME,
                        100f64,
                    ),
                );
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateColumn<centrifuge_procedure_templates::duration_s>
        for <centrifuge_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(duration_s: &f64) -> Result<(), Self::Error> {
            use diesel::Column;
            if duration_s < &30f64 {
                return Err(
                    ::validation_errors::ValidationError::greater_than_value(
                        <crate::centrifuge_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::centrifuge_procedure_templates::duration_s::NAME,
                        30f64,
                    ),
                );
            }
            if duration_s > &1800f64 {
                return Err(
                    ::validation_errors::ValidationError::smaller_than_value(
                        <crate::centrifuge_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::centrifuge_procedure_templates::duration_s::NAME,
                        1800f64,
                    ),
                );
            }
            Ok(())
        }
    }
    impl
        ::diesel_builders::ValidateColumn<
            centrifuge_procedure_templates::duration_tolerance_percentage,
        > for <centrifuge_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(duration_tolerance_percentage: &f64) -> Result<(), Self::Error> {
            use diesel::Column;
            if duration_tolerance_percentage <= &0f64 {
                return Err(
                    ::validation_errors::ValidationError::strictly_greater_than_value(
                        <crate::centrifuge_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::centrifuge_procedure_templates::duration_tolerance_percentage::NAME,
                        0f64,
                    ),
                );
            }
            if duration_tolerance_percentage > &100f64 {
                return Err(
                    ::validation_errors::ValidationError::smaller_than_value(
                        <crate::centrifuge_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::centrifuge_procedure_templates::duration_tolerance_percentage::NAME,
                        100f64,
                    ),
                );
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateColumn<centrifuge_procedure_templates::speed_rpm>
        for <centrifuge_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(speed_rpm: &f64) -> Result<(), Self::Error> {
            use diesel::Column;
            if speed_rpm < &5000f64 {
                return Err(
                    ::validation_errors::ValidationError::greater_than_value(
                        <crate::centrifuge_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::centrifuge_procedure_templates::speed_rpm::NAME,
                        5000f64,
                    ),
                );
            }
            if speed_rpm > &30000f64 {
                return Err(
                    ::validation_errors::ValidationError::smaller_than_value(
                        <crate::centrifuge_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::centrifuge_procedure_templates::speed_rpm::NAME,
                        30000f64,
                    ),
                );
            }
            Ok(())
        }
    }
    impl
        ::diesel_builders::ValidateColumn<
            centrifuge_procedure_templates::speed_rpm_tolerance_percentage,
        > for <centrifuge_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(speed_rpm_tolerance_percentage: &f64) -> Result<(), Self::Error> {
            use diesel::Column;
            if speed_rpm_tolerance_percentage <= &0f64 {
                return Err(
                    ::validation_errors::ValidationError::strictly_greater_than_value(
                        <crate::centrifuge_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::centrifuge_procedure_templates::speed_rpm_tolerance_percentage::NAME,
                        0f64,
                    ),
                );
            }
            if speed_rpm_tolerance_percentage > &100f64 {
                return Err(
                    ::validation_errors::ValidationError::smaller_than_value(
                        <crate::centrifuge_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::centrifuge_procedure_templates::speed_rpm_tolerance_percentage::NAME,
                        100f64,
                    ),
                );
            }
            Ok(())
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::procedure_templates::id>
        for CentrifugeProcedureTemplate
    {
        fn get_column_ref(
            &self,
        ) -> &<centrifuge_procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for CentrifugeProcedureTemplate {
        fn get_column_ref(
            &self,
        ) -> &<centrifuge_procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    /// Shape of `centrifuge_procedure_templates` the procedure runner renders.
    pub const CENTRIFUGE_PROCEDURE_TEMPLATES_SHAPE: procedure_like::shape::TemplateShape =
        procedure_like::shape::TemplateShape {
            table_name: "centrifuge_procedure_templates",
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
                procedure_like::shape::ColumnShape {
                    name: "speed_rpm",
                    label: "Rotational speed in revolutions per minute.",
                    sql_type: procedure_like::shape::SqlType::DoublePrecision,
                    unit: Some(procedure_like::shape::Unit::RevolutionsPerMinute),
                    required: true,
                },
                procedure_like::shape::ColumnShape {
                    name: "speed_rpm_tolerance_percentage",
                    label: "Allowed speed deviation as a percentage of the target.",
                    sql_type: procedure_like::shape::SqlType::DoublePrecision,
                    unit: Some(procedure_like::shape::Unit::Percentage),
                    required: true,
                },
            ],
            slots: &[
                procedure_like::shape::SlotShape {
                    name: "centrifuged_with",
                    role: procedure_like::shape::SlotRole::Instrument,
                    model_table_name: "centrifuge_models",
                    slot_column: "centrifuged_with_slot_id",
                    model_column: "centrifuged_with_model_id",
                    placed_into: None,
                    retrieved_from: None,
                },
                procedure_like::shape::SlotShape {
                    name: "centrifuged_container",
                    role: procedure_like::shape::SlotRole::Input,
                    model_table_name: "volumetric_container_models",
                    slot_column: "centrifuged_container_slot_id",
                    model_column: "centrifuged_container_model_id",
                    placed_into: None,
                    retrieved_from: None,
                },
            ],
        };
    ::diesel::allow_tables_to_appear_in_same_query!(
        centrifuge_procedure_templates,
        ::aps_centrifuge_models::centrifuge_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        centrifuge_procedure_templates,
        ::aps_resources::procedure_template_slots
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        centrifuge_procedure_templates,
        ::aps_resources::procedure_templates
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        centrifuge_procedure_templates,
        ::aps_resources::resources
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        centrifuge_procedure_templates,
        ::aps_physical_asset_models::volumetric_container_models
    );
}
pub use centrifuge_procedure_templates_model::*;
pub mod centrifuge_procedures_model {
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
        ::diesel_builders::prelude::TableModel,
    )]
    /// Recorded execution of a centrifuge template.
    #[table_model(ancestors(aps_resources::resources, aps_procedures::procedures))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[table_model(record_error = ::validation_errors::ValidationError)]
    #[table_model(foreign_key((id), (::aps_procedures::procedures::id)))]
    #[table_model(
        foreign_key(
            (centrifuge_procedure_template_id),
            (::aps_centrifuge_models::centrifuge_procedure_templates::id)
        )
    )]
    #[table_model(
        foreign_key(
            (centrifuge_procedure_template_id, centrifuge_procedure_template_archived),
            (
                ::aps_centrifuge_models::centrifuge_procedure_templates::id,
                ::aps_centrifuge_models::centrifuge_procedure_templates::archived
            )
        )
    )]
    #[table_model(default(
        aps_resources::resources::kind,
        ::aps_authz_resource_kind::ResourceKind::Ownable
    ))]
    #[diesel(table_name = centrifuge_procedures)]
    pub struct CentrifugeProcedure {
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
        centrifuge_procedure_template_id: ::rosetta_uuid::Uuid,
        /// Execution status copied from the parent procedure.
        #[same_as(aps_procedures::procedures::status_id)]
        #[table_model(default = "planned")]
        #[infallible]
        status_id: String,
        /// Measured temperature k.
        temperature_k: Option<f64>,
        /// Measured speed rpm.
        speed_rpm: Option<f64>,
        /// Archive state inherited from this resource.
        #[same_as(aps_procedures::procedures::archived)]
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
        /// Archive state of the row named by
        /// `centrifuge_procedure_template_id`.
        #[table_model(default = false)]
        #[infallible]
        centrifuge_procedure_template_archived: bool,
    }
    impl ::diesel_builders::ValidateColumn<centrifuge_procedures::temperature_k>
        for <centrifuge_procedures::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(temperature_k: &f64) -> Result<(), Self::Error> {
            use diesel::Column;
            if temperature_k <= &0f64 {
                return Err(
                    ::validation_errors::ValidationError::strictly_greater_than_value(
                        <crate::centrifuge_procedures::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::centrifuge_procedures::temperature_k::NAME,
                        0f64,
                    ),
                );
            }
            if temperature_k.partial_cmp(&f64::INFINITY).is_none_or(::core::cmp::Ordering::is_ge) {
                return Err(
                    ::validation_errors::ValidationError::check_violated(
                        <crate::centrifuge_procedures::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        &[crate::centrifuge_procedures::temperature_k::NAME],
                        "temperature_k < 'Infinity'::DOUBLE PRECISION",
                    ),
                );
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateColumn<centrifuge_procedures::speed_rpm>
        for <centrifuge_procedures::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(speed_rpm: &f64) -> Result<(), Self::Error> {
            use diesel::Column;
            if speed_rpm < &0f64 {
                return Err(
                    ::validation_errors::ValidationError::greater_than_value(
                        <crate::centrifuge_procedures::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::centrifuge_procedures::speed_rpm::NAME,
                        0f64,
                    ),
                );
            }
            if speed_rpm.partial_cmp(&f64::INFINITY).is_none_or(::core::cmp::Ordering::is_ge) {
                return Err(
                    ::validation_errors::ValidationError::check_violated(
                        <crate::centrifuge_procedures::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        &[crate::centrifuge_procedures::speed_rpm::NAME],
                        "speed_rpm < 'Infinity'::DOUBLE PRECISION",
                    ),
                );
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateRecord<centrifuge_procedures::table>
        for <centrifuge_procedures::table as ::diesel_builders::TableExt>::NewValues
    {
        #[inline]
        fn validate_record(&self) -> Result<(), ::validation_errors::ValidationError> {
            use diesel::Column;
            if let (Some(status_id), Some(temperature_k)) = (
                <Self as diesel_builders::MayGetColumn<
                    centrifuge_procedures::status_id,
                >>::may_get_column_ref(self),
                <Self as diesel_builders::MayGetColumn<
                    centrifuge_procedures::temperature_k,
                >>::may_get_column_ref(self),
            ) && !((status_id.as_str() != "completed") || (*temperature_k).is_some())
            {
                return Err(
                    ::validation_errors::ValidationError::check_violated(
                        <crate::centrifuge_procedures::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        &[
                            crate::centrifuge_procedures::status_id::NAME,
                            crate::centrifuge_procedures::temperature_k::NAME,
                        ],
                        "status_id <> 'completed' OR temperature_k IS NOT NULL",
                    ),
                );
            }
            if let (Some(status_id), Some(speed_rpm)) = (
                <Self as diesel_builders::MayGetColumn<
                    centrifuge_procedures::status_id,
                >>::may_get_column_ref(self),
                <Self as diesel_builders::MayGetColumn<
                    centrifuge_procedures::speed_rpm,
                >>::may_get_column_ref(self),
            ) && !((status_id.as_str() != "completed") || (*speed_rpm).is_some())
            {
                return Err(
                    ::validation_errors::ValidationError::check_violated(
                        <crate::centrifuge_procedures::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        &[
                            crate::centrifuge_procedures::status_id::NAME,
                            crate::centrifuge_procedures::speed_rpm::NAME,
                        ],
                        "status_id <> 'completed' OR speed_rpm IS NOT NULL",
                    ),
                );
            }
            Ok(())
        }
    }
    impl ::diesel_builders::GetColumn<aps_procedures::procedures::id> for CentrifugeProcedure {
        fn get_column_ref(
            &self,
        ) -> &<centrifuge_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for CentrifugeProcedure {
        fn get_column_ref(
            &self,
        ) -> &<centrifuge_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl procedure_like::ProcedureTableLike for centrifuge_procedures::table {
        type ProcedureTemplateTable = aps_centrifuge_models::centrifuge_procedure_templates::table;
    }
    /// Shape of `centrifuge_procedures` the procedure runner renders.
    pub const CENTRIFUGE_PROCEDURES_SHAPE: procedure_like::shape::ProcedureShape =
        procedure_like::shape::ProcedureShape {
            table_name: "centrifuge_procedures",
            template_table_name: "centrifuge_procedure_templates",
            measurements: &[
                procedure_like::shape::ColumnShape {
                    name: "temperature_k",
                    label: "Measured temperature k.",
                    sql_type: procedure_like::shape::SqlType::DoublePrecision,
                    unit: Some(procedure_like::shape::Unit::Kelvin),
                    required: true,
                },
                procedure_like::shape::ColumnShape {
                    name: "speed_rpm",
                    label: "Measured speed rpm.",
                    sql_type: procedure_like::shape::SqlType::DoublePrecision,
                    unit: Some(procedure_like::shape::Unit::RevolutionsPerMinute),
                    required: true,
                },
            ],
        };
    ::diesel::allow_tables_to_appear_in_same_query!(
        centrifuge_procedures,
        ::aps_centrifuge_models::centrifuge_procedure_templates
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        centrifuge_procedures,
        ::aps_procedures::procedures
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        centrifuge_procedures,
        ::aps_resources::resources
    );
}
pub use centrifuge_procedures_model::*;
pub mod centrifuges_model {
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
    /// Physical centrifuges tracked in APS inventory.
    #[table_model(ancestors(
        aps_resources::resources,
        aps_assets::assets,
        aps_assets::physical_assets
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_assets::PhysicalAsset, foreign_key = id))]
    #[table_model(foreign_key((id), (::aps_assets::physical_assets::id)))]
    #[table_model(
        foreign_key(
            (commercial_centrifuge_lot_id),
            (::aps_centrifuge_models::commercial_centrifuge_lots::id)
        )
    )]
    #[table_model(
        foreign_key(
            (commercial_centrifuge_lot_id, commercial_centrifuge_lot_archived),
            (
                ::aps_centrifuge_models::commercial_centrifuge_lots::id,
                ::aps_centrifuge_models::commercial_centrifuge_lots::archived
            )
        )
    )]
    #[table_model(default(
        aps_resources::resources::kind,
        ::aps_authz_resource_kind::ResourceKind::Ownable
    ))]
    #[diesel(table_name = centrifuges)]
    pub struct Centrifuge {
        /// Stable asset identifier inherited from physical_assets.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing this resource.
        #[same_as(aps_assets::physical_assets::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Commercial lot model instantiated by this physical asset.
        #[same_as(aps_assets::physical_assets::physical_asset_model_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        commercial_centrifuge_lot_id: ::rosetta_uuid::Uuid,
        /// Archive state inherited from this resource.
        #[same_as(aps_assets::physical_assets::archived)]
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
        /// Archive state of the row named by `commercial_centrifuge_lot_id`.
        #[table_model(default = false)]
        #[infallible]
        commercial_centrifuge_lot_archived: bool,
    }
    impl ::diesel_builders::GetColumn<aps_assets::assets::id> for Centrifuge {
        fn get_column_ref(
            &self,
        ) -> &<centrifuges::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_assets::physical_assets::id> for Centrifuge {
        fn get_column_ref(
            &self,
        ) -> &<centrifuges::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for Centrifuge {
        fn get_column_ref(
            &self,
        ) -> &<centrifuges::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    ::diesel::allow_tables_to_appear_in_same_query!(centrifuges, ::aps_assets::assets);
    ::diesel::allow_tables_to_appear_in_same_query!(
        centrifuges,
        ::aps_centrifuge_models::commercial_centrifuge_lots
    );
    ::diesel::allow_tables_to_appear_in_same_query!(centrifuges, ::aps_assets::physical_assets);
    ::diesel::allow_tables_to_appear_in_same_query!(centrifuges, ::aps_resources::resources);
}
pub use centrifuges_model::*;
pub mod commercial_centrifuge_lots_model {
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
    /// Catalog of lot-specific commercial centrifuge models.
    #[table_model(ancestors(
        aps_resources::resources,
        aps_resources::asset_models,
        aps_physical_asset_models::physical_asset_models,
        aps_centrifuge_models::centrifuge_models,
        aps_commercial_products::commercial_product_lots
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(
        belongs_to(aps_commercial_products::CommercialProductLot, foreign_key = id)
    )]
    #[diesel(belongs_to(aps_centrifuge_models::CentrifugeModel, foreign_key = id))]
    #[table_model(
        foreign_key((id), (::aps_commercial_products::commercial_product_lots::id))
    )]
    #[table_model(
        foreign_key(
            (commercial_centrifuge_model_id),
            (::aps_centrifuge_models::commercial_centrifuge_models::id)
        )
    )]
    #[table_model(foreign_key((id), (::aps_centrifuge_models::centrifuge_models::id)))]
    #[table_model(
        foreign_key(
            (commercial_centrifuge_model_id, commercial_centrifuge_model_archived),
            (
                ::aps_centrifuge_models::commercial_centrifuge_models::id,
                ::aps_centrifuge_models::commercial_centrifuge_models::archived
            )
        )
    )]
    #[table_model(default(
        aps_resources::resources::kind,
        ::aps_authz_resource_kind::ResourceKind::Ownable
    ))]
    #[diesel(table_name = commercial_centrifuge_lots)]
    pub struct CommercialCentrifugeLot {
        /// Stable lot-model identifier shared with parent lot/model tables.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing this resource.
        #[same_as(aps_centrifuge_models::centrifuge_models::organization_id)]
        #[same_as(aps_commercial_products::commercial_product_lots::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Commercial centrifuge model from which this lot derives.
        #[same_as(aps_commercial_products::commercial_product_lots::product_model_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        commercial_centrifuge_model_id: ::rosetta_uuid::Uuid,
        /// Archive state inherited from this resource.
        #[same_as(aps_centrifuge_models::centrifuge_models::archived)]
        #[same_as(aps_commercial_products::commercial_product_lots::archived)]
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
        /// Archive state of the row named by `commercial_centrifuge_model_id`.
        #[table_model(default = false)]
        #[infallible]
        commercial_centrifuge_model_archived: bool,
    }
    ::diesel_builders::prelude::unique_index!(
        commercial_centrifuge_lots::id,
        commercial_centrifuge_lots::archived
    );
    impl ::diesel_builders::GetColumn<aps_resources::asset_models::id> for CommercialCentrifugeLot {
        fn get_column_ref(
            &self,
        ) -> &<commercial_centrifuge_lots::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_centrifuge_models::centrifuge_models::id>
        for CommercialCentrifugeLot
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_centrifuge_lots::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_commercial_products::commercial_product_lots::id>
        for CommercialCentrifugeLot
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_centrifuge_lots::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_physical_asset_models::physical_asset_models::id>
        for CommercialCentrifugeLot
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_centrifuge_lots::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for CommercialCentrifugeLot {
        fn get_column_ref(
            &self,
        ) -> &<commercial_centrifuge_lots::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    /// Shape of `commercial_centrifuge_lots` the procedure runner renders.
    pub const COMMERCIAL_CENTRIFUGE_LOTS_SHAPE: procedure_like::shape::AssetModelShape =
        procedure_like::shape::AssetModelShape {
            table_name: "commercial_centrifuge_lots",
            asset_table_name: "centrifuges",
            model_column: "commercial_centrifuge_lot_id",
            required: &[
                procedure_like::shape::ColumnShape {
                    name: "organization_id",
                    label: "Organization containing this resource.",
                    sql_type: procedure_like::shape::SqlType::Uuid,
                    unit: None,
                    required: true,
                },
                procedure_like::shape::ColumnShape {
                    name: "commercial_centrifuge_lot_id",
                    label: "Commercial lot model instantiated by this physical asset.",
                    sql_type: procedure_like::shape::SqlType::Uuid,
                    unit: None,
                    required: true,
                },
            ],
        };
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_centrifuge_lots,
        ::aps_resources::asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_centrifuge_lots,
        ::aps_centrifuge_models::centrifuge_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_centrifuge_lots,
        ::aps_centrifuge_models::commercial_centrifuge_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_centrifuge_lots,
        ::aps_commercial_products::commercial_product_lots
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_centrifuge_lots,
        ::aps_physical_asset_models::physical_asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_centrifuge_lots,
        ::aps_resources::resources
    );
}
pub use commercial_centrifuge_lots_model::*;
pub mod commercial_centrifuge_models_model {
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
    /// Catalog of commercial centrifuge models.
    #[table_model(ancestors(
        aps_resources::resources,
        aps_resources::asset_models,
        aps_physical_asset_models::physical_asset_models,
        aps_commercial_products::commercial_products,
        aps_centrifuge_models::centrifuge_models
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_commercial_products::CommercialProduct, foreign_key = id))]
    #[table_model(foreign_key((id), (::aps_centrifuge_models::centrifuge_models::id)))]
    #[table_model(
        foreign_key(
            (centrifuge_model_id),
            (::aps_centrifuge_models::centrifuge_models::id)
        )
    )]
    #[table_model(
        foreign_key((id), (::aps_commercial_products::commercial_products::id))
    )]
    #[table_model(
        foreign_key(
            (centrifuge_model_id, centrifuge_model_archived),
            (
                ::aps_centrifuge_models::centrifuge_models::id,
                ::aps_centrifuge_models::centrifuge_models::archived
            )
        )
    )]
    #[table_model(default(
        aps_resources::resources::kind,
        ::aps_authz_resource_kind::ResourceKind::Ownable
    ))]
    #[diesel(table_name = commercial_centrifuge_models)]
    pub struct CommercialCentrifugeModel {
        /// Stable commercial model identifier shared with parent model tables.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing this resource.
        #[same_as(aps_centrifuge_models::centrifuge_models::organization_id)]
        #[same_as(aps_commercial_products::commercial_products::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Base centrifuge model represented by this commercial model.
        #[same_as(aps_resources::asset_models::parent_model_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        centrifuge_model_id: ::rosetta_uuid::Uuid,
        /// Archive state inherited from this resource.
        #[same_as(aps_centrifuge_models::centrifuge_models::archived)]
        #[same_as(aps_commercial_products::commercial_products::archived)]
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
        /// Archive state of the row named by `centrifuge_model_id`.
        #[table_model(default = false)]
        #[infallible]
        centrifuge_model_archived: bool,
    }
    ::diesel_builders::prelude::unique_index!(
        commercial_centrifuge_models::id,
        commercial_centrifuge_models::archived
    );
    impl ::diesel_builders::GetColumn<aps_resources::asset_models::id> for CommercialCentrifugeModel {
        fn get_column_ref(
            &self,
        ) -> &<commercial_centrifuge_models::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_centrifuge_models::centrifuge_models::id>
        for CommercialCentrifugeModel
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_centrifuge_models::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_commercial_products::commercial_products::id>
        for CommercialCentrifugeModel
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_centrifuge_models::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_physical_asset_models::physical_asset_models::id>
        for CommercialCentrifugeModel
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_centrifuge_models::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for CommercialCentrifugeModel {
        fn get_column_ref(
            &self,
        ) -> &<commercial_centrifuge_models::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    /// Shape of `commercial_centrifuge_models` the procedure runner renders.
    pub const COMMERCIAL_CENTRIFUGE_MODELS_SHAPE: procedure_like::shape::AssetModelShape =
        procedure_like::shape::AssetModelShape {
            table_name: "commercial_centrifuge_models",
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
        commercial_centrifuge_models,
        ::aps_resources::asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_centrifuge_models,
        ::aps_centrifuge_models::centrifuge_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_centrifuge_models,
        ::aps_commercial_products::commercial_products
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_centrifuge_models,
        ::aps_physical_asset_models::physical_asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_centrifuge_models,
        ::aps_resources::resources
    );
}
pub use commercial_centrifuge_models_model::*;
