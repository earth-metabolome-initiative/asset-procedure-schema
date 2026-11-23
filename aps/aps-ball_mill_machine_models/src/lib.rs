#![allow(ambiguous_glob_reexports, reason = "TableModel trait names can collide across tables")]
extern crate self as aps_ball_mill_machine_models;
pub mod ball_mill_machine_models_model {
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
    /// Catalog of ball mill machine model definitions.
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
    #[diesel(table_name = ball_mill_machine_models)]
    pub struct BallMillMachineModel {
        /// Stable model identifier inherited from `physical_asset_models`.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing this resource.
        #[same_as(aps_physical_asset_models::physical_asset_models::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
    }
    impl ::diesel_builders::GetColumn<aps_resources::asset_models::id> for BallMillMachineModel {
        fn get_column_ref(
            &self,
        ) -> &<ball_mill_machine_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id> for BallMillMachineModel {
        fn get_column_ref(
            &self,
        ) -> &<ball_mill_machine_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_physical_asset_models::physical_asset_models::id>
        for BallMillMachineModel
    {
        fn get_column_ref(
            &self,
        ) -> &<ball_mill_machine_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for BallMillMachineModel {
        fn get_column_ref(
            &self,
        ) -> &<ball_mill_machine_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    /// Shape of `ball_mill_machine_models` the procedure runner renders.
    pub const BALL_MILL_MACHINE_MODELS_SHAPE: procedure_like::shape::AssetModelShape =
        procedure_like::shape::AssetModelShape {
            table_name: "ball_mill_machine_models",
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
        ball_mill_machine_models,
        ::aps_resources::asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        ball_mill_machine_models,
        ::aps_commercial_products::commercial_product_lots
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        ball_mill_machine_models,
        ::aps_commercial_products::commercial_products
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        ball_mill_machine_models,
        ::aps_resources::ownables
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        ball_mill_machine_models,
        ::aps_physical_asset_models::physical_asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        ball_mill_machine_models,
        ::aps_resources::resources
    );
}
pub use ball_mill_machine_models_model::*;
pub mod ball_mill_machines_model {
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
    /// Physical ball mill machines tracked in APS inventory.
    #[table_model(ancestors(
        aps_resources::resources,
        aps_resources::ownables,
        aps_assets::assets,
        aps_assets::physical_assets
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_assets::PhysicalAsset, foreign_key = id))]
    #[diesel(
        belongs_to(
            aps_ball_mill_machine_models::CommercialBallMillMachineLot,
            foreign_key = commercial_ball_mill_machine_lot_id
        )
    )]
    #[table_model(foreign_key((id), (::aps_assets::physical_assets::id)))]
    #[table_model(
        foreign_key(
            (commercial_ball_mill_machine_lot_id),
            (::aps_ball_mill_machine_models::commercial_ball_mill_machine_lots::id)
        )
    )]
    #[table_model(default(
        aps_resources::resources::kind,
        ::aps_authz_resource_kind::ResourceKind::Ownable
    ))]
    #[diesel(table_name = ball_mill_machines)]
    pub struct BallMillMachine {
        /// Stable asset identifier inherited from `physical_assets`.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing this resource.
        #[same_as(aps_assets::physical_assets::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Commercial lot model instantiated by this physical machine.
        #[same_as(aps_assets::physical_assets::physical_asset_model_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        commercial_ball_mill_machine_lot_id: ::rosetta_uuid::Uuid,
    }
    impl ::diesel_builders::GetColumn<aps_assets::assets::id> for BallMillMachine {
        fn get_column_ref(
            &self,
        ) -> &<ball_mill_machines::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id> for BallMillMachine {
        fn get_column_ref(
            &self,
        ) -> &<ball_mill_machines::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_assets::physical_assets::id> for BallMillMachine {
        fn get_column_ref(
            &self,
        ) -> &<ball_mill_machines::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for BallMillMachine {
        fn get_column_ref(
            &self,
        ) -> &<ball_mill_machines::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    ::diesel::allow_tables_to_appear_in_same_query!(ball_mill_machines, ::aps_assets::assets);
    ::diesel::allow_tables_to_appear_in_same_query!(
        ball_mill_machines,
        ::aps_ball_mill_machine_models::commercial_ball_mill_machine_lots
    );
    ::diesel::allow_tables_to_appear_in_same_query!(ball_mill_machines, ::aps_resources::ownables);
    ::diesel::allow_tables_to_appear_in_same_query!(
        ball_mill_machines,
        ::aps_assets::physical_assets
    );
    ::diesel::allow_tables_to_appear_in_same_query!(ball_mill_machines, ::aps_resources::resources);
}
pub use ball_mill_machines_model::*;
pub mod ball_mill_procedure_templates_model {
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
    /// Typed slots and parameters for ball mill.
    #[table_model(ancestors(
        aps_resources::resources,
        aps_resources::ownables,
        aps_procedure_templates::procedure_templates
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_procedure_templates::ProcedureTemplate, foreign_key = id))]
    #[diesel(
        belongs_to(aps_ball_mill_machine_models::BeadModel, foreign_key = bead_model_id)
    )]
    #[diesel(
        belongs_to(
            aps_ball_mill_machine_models::BallMillMachineModel,
            foreign_key = milled_with_model_id
        )
    )]
    #[diesel(
        belongs_to(
            aps_physical_asset_models::VolumetricContainerModel,
            foreign_key = milled_container_model_id
        )
    )]
    #[table_model(
        foreign_key((id), (::aps_procedure_templates::procedure_templates::id))
    )]
    #[table_model(
        foreign_key((bead_model_id), (::aps_ball_mill_machine_models::bead_models::id))
    )]
    #[table_model(
        foreign_key(
            (milled_with_model_id),
            (::aps_ball_mill_machine_models::ball_mill_machine_models::id)
        )
    )]
    #[table_model(
        foreign_key(
            (milled_container_model_id),
            (::aps_physical_asset_models::volumetric_container_models::id)
        )
    )]
    #[table_model(
        foreign_key(
            (bead_slot_id, id, bead_model_id, bead_slot_role_id, organization_id),
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
                milled_with_slot_id,
                id,
                milled_with_model_id,
                milled_with_slot_role_id,
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
                milled_container_slot_id,
                id,
                milled_container_model_id,
                milled_container_slot_role_id,
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
    #[diesel(table_name = ball_mill_procedure_templates)]
    pub struct BallMillProcedureTemplate {
        /// Identity shared with the parent template.
        #[same_as(
            aps_procedure_templates::procedure_template_slots::procedure_template_id,
            (bead_slot_id, milled_with_slot_id, milled_container_slot_id)
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing the parent resource.
        #[same_as(aps_procedure_templates::procedure_templates::organization_id)]
        #[same_as(
            aps_procedure_templates::procedure_template_slots::organization_id,
            (bead_slot_id, milled_with_slot_id, milled_container_slot_id)
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Slot declaring the `bead` assets.
        #[mandatory(aps_procedure_templates::procedure_template_slots)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        bead_slot_id: ::rosetta_uuid::Uuid,
        /// Admissible model for the `bead` slot.
        #[same_as(aps_procedure_templates::procedure_template_slots::asset_model_id, bead_slot_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        bead_model_id: ::rosetta_uuid::Uuid,
        /// Required role of the `bead` slot.
        #[table_model(generated)]
        bead_slot_role_id: String,
        /// Slot declaring the `milled_with` assets.
        #[mandatory(aps_procedure_templates::procedure_template_slots)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        milled_with_slot_id: ::rosetta_uuid::Uuid,
        /// Admissible model for the `milled_with` slot.
        #[same_as(
            aps_procedure_templates::procedure_template_slots::asset_model_id,
            milled_with_slot_id
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        milled_with_model_id: ::rosetta_uuid::Uuid,
        /// Required role of the `milled_with` slot.
        #[table_model(generated)]
        milled_with_slot_role_id: String,
        /// Slot declaring the `milled_container` assets.
        #[mandatory(aps_procedure_templates::procedure_template_slots)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        milled_container_slot_id: ::rosetta_uuid::Uuid,
        /// Admissible model for the `milled_container` slot.
        #[same_as(
            aps_procedure_templates::procedure_template_slots::asset_model_id,
            milled_container_slot_id
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        milled_container_model_id: ::rosetta_uuid::Uuid,
        /// Required role of the `milled_container` slot.
        #[table_model(generated)]
        milled_container_slot_role_id: String,
        /// Target temperature in kelvin.
        #[table_model(default = 293.15f64)]
        temperature_k: f64,
        /// Allowed temperature deviation as a percentage of the target.
        #[table_model(default = 1f64)]
        temperature_tolerance_percentage: f64,
        /// Treatment duration in seconds.
        #[table_model(default = 150f64)]
        duration_s: f64,
        /// Allowed duration deviation as a percentage of the target.
        #[table_model(default = 5f64)]
        duration_tolerance_percentage: f64,
        /// Oscillation frequency in hertz.
        #[table_model(default = 25f64)]
        frequency_hz: f64,
        /// Allowed frequency deviation as a percentage of the target.
        #[table_model(default = 5f64)]
        frequency_hz_tolerance_percentage: f64,
        /// Number of grinding beads.
        #[table_model(default = 3i16)]
        bead_count: i16,
    }
    impl ::diesel_builders::ValidateColumn<ball_mill_procedure_templates::temperature_k>
        for <ball_mill_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(temperature_k: &f64) -> Result<(), Self::Error> {
            use diesel::Column;
            if temperature_k <= &0f64 {
                return Err(
                    ::validation_errors::ValidationError::strictly_greater_than_value(
                        <crate::ball_mill_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::ball_mill_procedure_templates::temperature_k::NAME,
                        0f64,
                    ),
                );
            }
            if temperature_k.partial_cmp(&f64::INFINITY).is_none_or(::core::cmp::Ordering::is_ge) {
                return Err(
                    ::validation_errors::ValidationError::check_violated(
                        <crate::ball_mill_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        &[crate::ball_mill_procedure_templates::temperature_k::NAME],
                        "temperature_k < 'Infinity'::DOUBLE PRECISION",
                    ),
                );
            }
            Ok(())
        }
    }
    impl
        ::diesel_builders::ValidateColumn<
            ball_mill_procedure_templates::temperature_tolerance_percentage,
        > for <ball_mill_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(temperature_tolerance_percentage: &f64) -> Result<(), Self::Error> {
            use diesel::Column;
            if temperature_tolerance_percentage <= &0f64 {
                return Err(
                    ::validation_errors::ValidationError::strictly_greater_than_value(
                        <crate::ball_mill_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::ball_mill_procedure_templates::temperature_tolerance_percentage::NAME,
                        0f64,
                    ),
                );
            }
            if temperature_tolerance_percentage > &100f64 {
                return Err(
                    ::validation_errors::ValidationError::smaller_than_value(
                        <crate::ball_mill_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::ball_mill_procedure_templates::temperature_tolerance_percentage::NAME,
                        100f64,
                    ),
                );
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateColumn<ball_mill_procedure_templates::duration_s>
        for <ball_mill_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(duration_s: &f64) -> Result<(), Self::Error> {
            use diesel::Column;
            if duration_s > &900f64 {
                return Err(
                    ::validation_errors::ValidationError::smaller_than_value(
                        <crate::ball_mill_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::ball_mill_procedure_templates::duration_s::NAME,
                        900f64,
                    ),
                );
            }
            if duration_s < &30f64 {
                return Err(
                    ::validation_errors::ValidationError::greater_than_value(
                        <crate::ball_mill_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::ball_mill_procedure_templates::duration_s::NAME,
                        30f64,
                    ),
                );
            }
            Ok(())
        }
    }
    impl
        ::diesel_builders::ValidateColumn<
            ball_mill_procedure_templates::duration_tolerance_percentage,
        > for <ball_mill_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(duration_tolerance_percentage: &f64) -> Result<(), Self::Error> {
            use diesel::Column;
            if duration_tolerance_percentage <= &0f64 {
                return Err(
                    ::validation_errors::ValidationError::strictly_greater_than_value(
                        <crate::ball_mill_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::ball_mill_procedure_templates::duration_tolerance_percentage::NAME,
                        0f64,
                    ),
                );
            }
            if duration_tolerance_percentage > &100f64 {
                return Err(
                    ::validation_errors::ValidationError::smaller_than_value(
                        <crate::ball_mill_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::ball_mill_procedure_templates::duration_tolerance_percentage::NAME,
                        100f64,
                    ),
                );
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateColumn<ball_mill_procedure_templates::frequency_hz>
        for <ball_mill_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(frequency_hz: &f64) -> Result<(), Self::Error> {
            use diesel::Column;
            if frequency_hz > &50f64 {
                return Err(
                    ::validation_errors::ValidationError::smaller_than_value(
                        <crate::ball_mill_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::ball_mill_procedure_templates::frequency_hz::NAME,
                        50f64,
                    ),
                );
            }
            if frequency_hz < &15f64 {
                return Err(
                    ::validation_errors::ValidationError::greater_than_value(
                        <crate::ball_mill_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::ball_mill_procedure_templates::frequency_hz::NAME,
                        15f64,
                    ),
                );
            }
            Ok(())
        }
    }
    impl
        ::diesel_builders::ValidateColumn<
            ball_mill_procedure_templates::frequency_hz_tolerance_percentage,
        > for <ball_mill_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(frequency_hz_tolerance_percentage: &f64) -> Result<(), Self::Error> {
            use diesel::Column;
            if frequency_hz_tolerance_percentage <= &0f64 {
                return Err(
                    ::validation_errors::ValidationError::strictly_greater_than_value(
                        <crate::ball_mill_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::ball_mill_procedure_templates::frequency_hz_tolerance_percentage::NAME,
                        0f64,
                    ),
                );
            }
            if frequency_hz_tolerance_percentage > &100f64 {
                return Err(
                    ::validation_errors::ValidationError::smaller_than_value(
                        <crate::ball_mill_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::ball_mill_procedure_templates::frequency_hz_tolerance_percentage::NAME,
                        100f64,
                    ),
                );
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateColumn<ball_mill_procedure_templates::bead_count>
        for <ball_mill_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(bead_count: &i16) -> Result<(), Self::Error> {
            use diesel::Column;
            if bead_count <= &0i16 {
                return Err(
                    ::validation_errors::ValidationError::strictly_greater_than_value(
                        <crate::ball_mill_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::ball_mill_procedure_templates::bead_count::NAME,
                        0f64,
                    ),
                );
            }
            Ok(())
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id> for BallMillProcedureTemplate {
        fn get_column_ref(
            &self,
        ) -> &<ball_mill_procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_procedure_templates::procedure_templates::id>
        for BallMillProcedureTemplate
    {
        fn get_column_ref(
            &self,
        ) -> &<ball_mill_procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for BallMillProcedureTemplate {
        fn get_column_ref(
            &self,
        ) -> &<ball_mill_procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    /// Shape of `ball_mill_procedure_templates` the procedure runner renders.
    pub const BALL_MILL_PROCEDURE_TEMPLATES_SHAPE: procedure_like::shape::TemplateShape =
        procedure_like::shape::TemplateShape {
            table_name: "ball_mill_procedure_templates",
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
                    name: "frequency_hz",
                    label: "Oscillation frequency in hertz.",
                    sql_type: procedure_like::shape::SqlType::DoublePrecision,
                    unit: Some(procedure_like::shape::Unit::Hertz),
                    required: true,
                },
                procedure_like::shape::ColumnShape {
                    name: "frequency_hz_tolerance_percentage",
                    label: "Allowed frequency deviation as a percentage of the target.",
                    sql_type: procedure_like::shape::SqlType::DoublePrecision,
                    unit: Some(procedure_like::shape::Unit::Percentage),
                    required: true,
                },
                procedure_like::shape::ColumnShape {
                    name: "bead_count",
                    label: "Number of grinding beads.",
                    sql_type: procedure_like::shape::SqlType::SmallInt,
                    unit: Some(procedure_like::shape::Unit::Count),
                    required: true,
                },
            ],
            slots: &[
                procedure_like::shape::SlotShape {
                    name: "bead",
                    role: procedure_like::shape::SlotRole::Instrument,
                    model_table_name: "bead_models",
                    slot_column: "bead_slot_id",
                    model_column: "bead_model_id",
                    placed_into: None,
                    retrieved_from: None,
                },
                procedure_like::shape::SlotShape {
                    name: "milled_with",
                    role: procedure_like::shape::SlotRole::Instrument,
                    model_table_name: "ball_mill_machine_models",
                    slot_column: "milled_with_slot_id",
                    model_column: "milled_with_model_id",
                    placed_into: None,
                    retrieved_from: None,
                },
                procedure_like::shape::SlotShape {
                    name: "milled_container",
                    role: procedure_like::shape::SlotRole::Input,
                    model_table_name: "volumetric_container_models",
                    slot_column: "milled_container_slot_id",
                    model_column: "milled_container_model_id",
                    placed_into: None,
                    retrieved_from: None,
                },
            ],
        };
    ::diesel::allow_tables_to_appear_in_same_query!(
        ball_mill_procedure_templates,
        ::aps_ball_mill_machine_models::ball_mill_machine_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        ball_mill_procedure_templates,
        ::aps_ball_mill_machine_models::bead_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        ball_mill_procedure_templates,
        ::aps_resources::ownables
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        ball_mill_procedure_templates,
        ::aps_procedure_templates::procedure_template_slots
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        ball_mill_procedure_templates,
        ::aps_procedure_templates::procedure_templates
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        ball_mill_procedure_templates,
        ::aps_resources::resources
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        ball_mill_procedure_templates,
        ::aps_physical_asset_models::volumetric_container_models
    );
}
pub use ball_mill_procedure_templates_model::*;
pub mod ball_mill_procedures_model {
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
    /// Recorded execution of a ball mill template.
    #[table_model(ancestors(
        aps_resources::resources,
        aps_resources::ownables,
        aps_procedures::procedures
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[table_model(record_error = ::validation_errors::ValidationError)]
    #[diesel(
        belongs_to(
            aps_ball_mill_machine_models::BallMillProcedureTemplate,
            foreign_key = ball_mill_procedure_template_id
        )
    )]
    #[table_model(foreign_key((id), (::aps_procedures::procedures::id)))]
    #[table_model(
        foreign_key(
            (ball_mill_procedure_template_id),
            (::aps_ball_mill_machine_models::ball_mill_procedure_templates::id)
        )
    )]
    #[table_model(default(
        aps_resources::resources::kind,
        ::aps_authz_resource_kind::ResourceKind::Ownable
    ))]
    #[diesel(table_name = ball_mill_procedures)]
    pub struct BallMillProcedure {
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
        ball_mill_procedure_template_id: ::rosetta_uuid::Uuid,
        /// Execution status copied from the parent procedure.
        #[same_as(aps_procedures::procedures::status_id)]
        #[infallible]
        status_id: String,
        /// Measured temperature k.
        temperature_k: Option<f64>,
        /// Measured frequency hz.
        frequency_hz: Option<f64>,
    }
    impl ::diesel_builders::ValidateColumn<ball_mill_procedures::temperature_k>
        for <ball_mill_procedures::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(temperature_k: &f64) -> Result<(), Self::Error> {
            use diesel::Column;
            if temperature_k <= &0f64 {
                return Err(::validation_errors::ValidationError::strictly_greater_than_value(
                    <crate::ball_mill_procedures::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::ball_mill_procedures::temperature_k::NAME,
                    0f64,
                ));
            }
            if temperature_k.partial_cmp(&f64::INFINITY).is_none_or(::core::cmp::Ordering::is_ge) {
                return Err(::validation_errors::ValidationError::check_violated(
                    <crate::ball_mill_procedures::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    &[crate::ball_mill_procedures::temperature_k::NAME],
                    "temperature_k < 'Infinity'::DOUBLE PRECISION",
                ));
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateColumn<ball_mill_procedures::frequency_hz>
        for <ball_mill_procedures::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(frequency_hz: &f64) -> Result<(), Self::Error> {
            use diesel::Column;
            if frequency_hz < &0f64 {
                return Err(::validation_errors::ValidationError::greater_than_value(
                    <crate::ball_mill_procedures::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::ball_mill_procedures::frequency_hz::NAME,
                    0f64,
                ));
            }
            if frequency_hz.partial_cmp(&f64::INFINITY).is_none_or(::core::cmp::Ordering::is_ge) {
                return Err(::validation_errors::ValidationError::check_violated(
                    <crate::ball_mill_procedures::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    &[crate::ball_mill_procedures::frequency_hz::NAME],
                    "frequency_hz < 'Infinity'::DOUBLE PRECISION",
                ));
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateRecord<ball_mill_procedures::table>
        for <ball_mill_procedures::table as ::diesel_builders::TableExt>::NewValues
    {
        #[inline]
        fn validate_record(&self) -> Result<(), ::validation_errors::ValidationError> {
            use diesel::Column;
            if let (Some(status_id), Some(temperature_k)) = (
                <Self as diesel_builders::MayGetColumn<
                    ball_mill_procedures::status_id,
                >>::may_get_column_ref(self),
                <Self as diesel_builders::MayGetColumn<
                    ball_mill_procedures::temperature_k,
                >>::may_get_column_ref(self),
            ) && !((status_id.as_str() != "completed") || (*temperature_k).is_some())
            {
                return Err(
                    ::validation_errors::ValidationError::check_violated(
                        <crate::ball_mill_procedures::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        &[
                            crate::ball_mill_procedures::status_id::NAME,
                            crate::ball_mill_procedures::temperature_k::NAME,
                        ],
                        "status_id <> 'completed' OR temperature_k IS NOT NULL",
                    ),
                );
            }
            if let (Some(status_id), Some(frequency_hz)) = (
                <Self as diesel_builders::MayGetColumn<
                    ball_mill_procedures::status_id,
                >>::may_get_column_ref(self),
                <Self as diesel_builders::MayGetColumn<
                    ball_mill_procedures::frequency_hz,
                >>::may_get_column_ref(self),
            ) && !((status_id.as_str() != "completed") || (*frequency_hz).is_some())
            {
                return Err(
                    ::validation_errors::ValidationError::check_violated(
                        <crate::ball_mill_procedures::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        &[
                            crate::ball_mill_procedures::status_id::NAME,
                            crate::ball_mill_procedures::frequency_hz::NAME,
                        ],
                        "status_id <> 'completed' OR frequency_hz IS NOT NULL",
                    ),
                );
            }
            Ok(())
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id> for BallMillProcedure {
        fn get_column_ref(
            &self,
        ) -> &<ball_mill_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_procedures::procedures::id> for BallMillProcedure {
        fn get_column_ref(
            &self,
        ) -> &<ball_mill_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for BallMillProcedure {
        fn get_column_ref(
            &self,
        ) -> &<ball_mill_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl procedure_like::ProcedureTableLike for ball_mill_procedures::table {
        type ProcedureTemplateTable =
            aps_ball_mill_machine_models::ball_mill_procedure_templates::table;
    }
    /// Shape of `ball_mill_procedures` the procedure runner renders.
    pub const BALL_MILL_PROCEDURES_SHAPE: procedure_like::shape::ProcedureShape =
        procedure_like::shape::ProcedureShape {
            table_name: "ball_mill_procedures",
            template_table_name: "ball_mill_procedure_templates",
            measurements: &[
                procedure_like::shape::ColumnShape {
                    name: "temperature_k",
                    label: "Measured temperature k.",
                    sql_type: procedure_like::shape::SqlType::DoublePrecision,
                    unit: Some(procedure_like::shape::Unit::Kelvin),
                    required: true,
                },
                procedure_like::shape::ColumnShape {
                    name: "frequency_hz",
                    label: "Measured frequency hz.",
                    sql_type: procedure_like::shape::SqlType::DoublePrecision,
                    unit: Some(procedure_like::shape::Unit::Hertz),
                    required: true,
                },
            ],
        };
    ::diesel::allow_tables_to_appear_in_same_query!(
        ball_mill_procedures,
        ::aps_ball_mill_machine_models::ball_mill_procedure_templates
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        ball_mill_procedures,
        ::aps_resources::ownables
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        ball_mill_procedures,
        ::aps_procedures::procedures
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        ball_mill_procedures,
        ::aps_resources::resources
    );
}
pub use ball_mill_procedures_model::*;
pub mod bead_models_model {
    #[derive(
        Copy,
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
    /// Catalog of bead models with diameter metadata.
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
    #[diesel(table_name = bead_models)]
    pub struct BeadModel {
        /// Stable model identifier inherited from `physical_asset_models`.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing this resource.
        #[same_as(aps_physical_asset_models::physical_asset_models::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Bead diameter in meters.
        diameter_m: f64,
    }
    impl ::diesel_builders::ValidateColumn<bead_models::diameter_m>
        for <bead_models::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(diameter_m: &f64) -> Result<(), Self::Error> {
            use diesel::Column;
            if diameter_m <= &0f64 {
                return Err(::validation_errors::ValidationError::strictly_greater_than_value(
                    <crate::bead_models::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::bead_models::diameter_m::NAME,
                    0f64,
                ));
            }
            if diameter_m.partial_cmp(&f64::INFINITY).is_none_or(::core::cmp::Ordering::is_ge) {
                return Err(::validation_errors::ValidationError::check_violated(
                    <crate::bead_models::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    &[crate::bead_models::diameter_m::NAME],
                    "diameter_m < 'Infinity'::DOUBLE PRECISION",
                ));
            }
            Ok(())
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::asset_models::id> for BeadModel {
        fn get_column_ref(
            &self,
        ) -> &<bead_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id> for BeadModel {
        fn get_column_ref(
            &self,
        ) -> &<bead_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_physical_asset_models::physical_asset_models::id>
        for BeadModel
    {
        fn get_column_ref(
            &self,
        ) -> &<bead_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for BeadModel {
        fn get_column_ref(
            &self,
        ) -> &<bead_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    /// Shape of `bead_models` the procedure runner renders.
    pub const BEAD_MODELS_SHAPE: procedure_like::shape::AssetModelShape =
        procedure_like::shape::AssetModelShape {
            table_name: "bead_models",
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
    ::diesel::allow_tables_to_appear_in_same_query!(bead_models, ::aps_resources::asset_models);
    ::diesel::allow_tables_to_appear_in_same_query!(bead_models, ::aps_resources::ownables);
    ::diesel::allow_tables_to_appear_in_same_query!(
        bead_models,
        ::aps_physical_asset_models::physical_asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(bead_models, ::aps_resources::resources);
}
pub use bead_models_model::*;
pub mod commercial_ball_mill_machine_lots_model {
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
    /// Catalog of lot-specific commercial ball mill machine models.
    #[table_model(ancestors(
        aps_resources::resources,
        aps_resources::ownables,
        aps_resources::asset_models,
        aps_physical_asset_models::physical_asset_models,
        aps_ball_mill_machine_models::ball_mill_machine_models,
        aps_commercial_products::commercial_product_lots
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(
        belongs_to(aps_commercial_products::CommercialProductLot, foreign_key = id)
    )]
    #[diesel(
        belongs_to(
            aps_ball_mill_machine_models::CommercialBallMillMachineModel,
            foreign_key = commercial_ball_mill_machine_model_id
        )
    )]
    #[diesel(
        belongs_to(aps_ball_mill_machine_models::BallMillMachineModel, foreign_key = id)
    )]
    #[table_model(
        foreign_key((id), (::aps_commercial_products::commercial_product_lots::id))
    )]
    #[table_model(
        foreign_key(
            (commercial_ball_mill_machine_model_id),
            (::aps_ball_mill_machine_models::commercial_ball_mill_machine_models::id)
        )
    )]
    #[table_model(
        foreign_key((id), (::aps_ball_mill_machine_models::ball_mill_machine_models::id))
    )]
    #[table_model(default(
        aps_resources::resources::kind,
        ::aps_authz_resource_kind::ResourceKind::Ownable
    ))]
    #[diesel(table_name = commercial_ball_mill_machine_lots)]
    pub struct CommercialBallMillMachineLot {
        /// Stable lot-model identifier shared with parent lot/model tables.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing this resource.
        #[same_as(aps_ball_mill_machine_models::ball_mill_machine_models::organization_id)]
        #[same_as(aps_commercial_products::commercial_product_lots::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Commercial ball-mill model from which this lot derives.
        #[same_as(aps_commercial_products::commercial_product_lots::product_model_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        commercial_ball_mill_machine_model_id: ::rosetta_uuid::Uuid,
    }
    impl ::diesel_builders::GetColumn<aps_resources::asset_models::id>
        for CommercialBallMillMachineLot
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_ball_mill_machine_lots::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_ball_mill_machine_models::ball_mill_machine_models::id>
        for CommercialBallMillMachineLot
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_ball_mill_machine_lots::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_commercial_products::commercial_product_lots::id>
        for CommercialBallMillMachineLot
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_ball_mill_machine_lots::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id> for CommercialBallMillMachineLot {
        fn get_column_ref(
            &self,
        ) -> &<commercial_ball_mill_machine_lots::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_physical_asset_models::physical_asset_models::id>
        for CommercialBallMillMachineLot
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_ball_mill_machine_lots::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for CommercialBallMillMachineLot {
        fn get_column_ref(
            &self,
        ) -> &<commercial_ball_mill_machine_lots::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    /// Shape of `commercial_ball_mill_machine_lots` the procedure runner
    /// renders.
    pub const COMMERCIAL_BALL_MILL_MACHINE_LOTS_SHAPE: procedure_like::shape::AssetModelShape =
        procedure_like::shape::AssetModelShape {
            table_name: "commercial_ball_mill_machine_lots",
            asset_table_name: "ball_mill_machines",
            model_column: "commercial_ball_mill_machine_lot_id",
            required: &[
                procedure_like::shape::ColumnShape {
                    name: "organization_id",
                    label: "Organization containing this resource.",
                    sql_type: procedure_like::shape::SqlType::Uuid,
                    unit: None,
                    required: true,
                },
                procedure_like::shape::ColumnShape {
                    name: "commercial_ball_mill_machine_lot_id",
                    label: "Commercial lot model instantiated by this physical machine.",
                    sql_type: procedure_like::shape::SqlType::Uuid,
                    unit: None,
                    required: true,
                },
            ],
        };
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_ball_mill_machine_lots,
        ::aps_resources::asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_ball_mill_machine_lots,
        ::aps_ball_mill_machine_models::ball_mill_machine_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_ball_mill_machine_lots,
        ::aps_ball_mill_machine_models::commercial_ball_mill_machine_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_ball_mill_machine_lots,
        ::aps_commercial_products::commercial_product_lots
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_ball_mill_machine_lots,
        ::aps_resources::ownables
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_ball_mill_machine_lots,
        ::aps_physical_asset_models::physical_asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_ball_mill_machine_lots,
        ::aps_resources::resources
    );
}
pub use commercial_ball_mill_machine_lots_model::*;
pub mod commercial_ball_mill_machine_models_model {
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
    /// Catalog of commercial ball mill machine models.
    #[table_model(ancestors(
        aps_resources::resources,
        aps_resources::ownables,
        aps_resources::asset_models,
        aps_physical_asset_models::physical_asset_models,
        aps_commercial_products::commercial_products,
        aps_ball_mill_machine_models::ball_mill_machine_models
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_commercial_products::CommercialProduct, foreign_key = id))]
    #[table_model(
        foreign_key((id), (::aps_ball_mill_machine_models::ball_mill_machine_models::id))
    )]
    #[table_model(
        foreign_key(
            (ball_mill_machine_model_id),
            (::aps_ball_mill_machine_models::ball_mill_machine_models::id)
        )
    )]
    #[table_model(
        foreign_key((id), (::aps_commercial_products::commercial_products::id))
    )]
    #[table_model(default(
        aps_resources::resources::kind,
        ::aps_authz_resource_kind::ResourceKind::Ownable
    ))]
    #[diesel(table_name = commercial_ball_mill_machine_models)]
    pub struct CommercialBallMillMachineModel {
        /// Stable commercial model identifier shared with parent model tables.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing this resource.
        #[same_as(aps_ball_mill_machine_models::ball_mill_machine_models::organization_id)]
        #[same_as(aps_commercial_products::commercial_products::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Base ball-mill model represented by this commercial model.
        #[same_as(aps_resources::asset_models::parent_model_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        ball_mill_machine_model_id: ::rosetta_uuid::Uuid,
    }
    impl ::diesel_builders::GetColumn<aps_resources::asset_models::id>
        for CommercialBallMillMachineModel
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_ball_mill_machine_models::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_ball_mill_machine_models::ball_mill_machine_models::id>
        for CommercialBallMillMachineModel
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_ball_mill_machine_models::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_commercial_products::commercial_products::id>
        for CommercialBallMillMachineModel
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_ball_mill_machine_models::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id> for CommercialBallMillMachineModel {
        fn get_column_ref(
            &self,
        ) -> &<commercial_ball_mill_machine_models::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_physical_asset_models::physical_asset_models::id>
        for CommercialBallMillMachineModel
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_ball_mill_machine_models::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for CommercialBallMillMachineModel {
        fn get_column_ref(
            &self,
        ) -> &<commercial_ball_mill_machine_models::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    /// Shape of `commercial_ball_mill_machine_models` the procedure runner
    /// renders.
    pub const COMMERCIAL_BALL_MILL_MACHINE_MODELS_SHAPE: procedure_like::shape::AssetModelShape =
        procedure_like::shape::AssetModelShape {
            table_name: "commercial_ball_mill_machine_models",
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
        commercial_ball_mill_machine_models,
        ::aps_resources::asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_ball_mill_machine_models,
        ::aps_ball_mill_machine_models::ball_mill_machine_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_ball_mill_machine_models,
        ::aps_commercial_products::commercial_products
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_ball_mill_machine_models,
        ::aps_resources::ownables
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_ball_mill_machine_models,
        ::aps_physical_asset_models::physical_asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_ball_mill_machine_models,
        ::aps_resources::resources
    );
}
pub use commercial_ball_mill_machine_models_model::*;
