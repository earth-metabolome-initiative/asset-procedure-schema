#![allow(ambiguous_glob_reexports, reason = "TableModel trait names can collide across tables")]
extern crate self as aps_supernatant_procedure_templates;
pub mod supernatant_procedure_templates_model {
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
    /// Typed slots and parameters for supernatant.
    #[table_model(ancestors(
        aps_resources::resources,
        aps_resources::ownables,
        aps_procedure_templates::procedure_templates
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[table_model(record_error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_procedure_templates::ProcedureTemplate, foreign_key = id))]
    #[diesel(
        belongs_to(
            aps_physical_asset_models::VolumeMeasuringDeviceModel,
            foreign_key = transferred_with_model_id
        )
    )]
    #[table_model(
        foreign_key((id), (::aps_procedure_templates::procedure_templates::id))
    )]
    #[table_model(
        foreign_key((source_sample_model_id), (::aps_assets::sample_models::id))
    )]
    #[table_model(
        foreign_key(
            (stratified_source_model_id),
            (::aps_physical_asset_models::volumetric_container_models::id)
        )
    )]
    #[table_model(
        foreign_key(
            (supernatant_destination_model_id),
            (::aps_physical_asset_models::volumetric_container_models::id)
        )
    )]
    #[table_model(
        foreign_key((supernatant_model_id), (::aps_assets::sample_models::id))
    )]
    #[table_model(
        foreign_key(
            (transferred_with_model_id),
            (::aps_physical_asset_models::volume_measuring_device_models::id)
        )
    )]
    #[table_model(
        foreign_key(
            (
                source_sample_slot_id,
                id,
                source_sample_model_id,
                source_sample_slot_role_id,
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
                stratified_source_slot_id,
                id,
                stratified_source_model_id,
                stratified_source_slot_role_id,
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
                supernatant_destination_slot_id,
                id,
                supernatant_destination_model_id,
                supernatant_destination_slot_role_id,
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
                transferred_with_slot_id,
                id,
                transferred_with_model_id,
                transferred_with_slot_role_id,
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
                supernatant_slot_id,
                id,
                supernatant_model_id,
                supernatant_slot_role_id,
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
            (supernatant_slot_id, id, supernatant_destination_slot_id, organization_id),
            (
                ::aps_procedure_templates::procedure_template_slots::id,
                ::aps_procedure_templates::procedure_template_slots::procedure_template_id,
                ::aps_procedure_templates::procedure_template_slots::placed_into_slot_id,
                ::aps_procedure_templates::procedure_template_slots::organization_id
            )
        )
    )]
    #[table_model(default(
        aps_resources::resources::kind,
        ::aps_authz_resource_kind::ResourceKind::Ownable
    ))]
    #[diesel(table_name = supernatant_procedure_templates)]
    pub struct SupernatantProcedureTemplate {
        /// Identity shared with the parent template.
        #[same_as(
            aps_procedure_templates::procedure_template_slots::procedure_template_id,
            (
                source_sample_slot_id,
                stratified_source_slot_id,
                supernatant_destination_slot_id,
                transferred_with_slot_id,
                supernatant_slot_id
            )
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing the parent resource.
        #[same_as(aps_procedure_templates::procedure_templates::organization_id)]
        #[same_as(
            aps_procedure_templates::procedure_template_slots::organization_id,
            (
                source_sample_slot_id,
                stratified_source_slot_id,
                supernatant_destination_slot_id,
                transferred_with_slot_id,
                supernatant_slot_id
            )
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Slot declaring the source sample.
        #[mandatory(aps_procedure_templates::procedure_template_slots)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        source_sample_slot_id: ::rosetta_uuid::Uuid,
        /// Admissible model for the source sample.
        #[same_as(
            aps_procedure_templates::procedure_template_slots::asset_model_id,
            source_sample_slot_id
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        source_sample_model_id: ::rosetta_uuid::Uuid,
        /// Required role of the source sample.
        #[table_model(generated)]
        source_sample_slot_role_id: String,
        /// Slot declaring the `stratified_source` assets.
        #[mandatory(aps_procedure_templates::procedure_template_slots)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        stratified_source_slot_id: ::rosetta_uuid::Uuid,
        /// Admissible model for the `stratified_source` slot.
        #[same_as(
            aps_procedure_templates::procedure_template_slots::asset_model_id,
            stratified_source_slot_id
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        stratified_source_model_id: ::rosetta_uuid::Uuid,
        /// Required role of the `stratified_source` slot.
        #[table_model(generated)]
        stratified_source_slot_role_id: String,
        /// Slot declaring the `supernatant_destination` assets.
        #[same_as(
            aps_procedure_templates::procedure_template_slots::placed_into_slot_id,
            supernatant_slot_id
        )]
        #[mandatory(aps_procedure_templates::procedure_template_slots)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        supernatant_destination_slot_id: ::rosetta_uuid::Uuid,
        /// Admissible model for the `supernatant_destination` slot.
        #[same_as(
            aps_procedure_templates::procedure_template_slots::asset_model_id,
            supernatant_destination_slot_id
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        supernatant_destination_model_id: ::rosetta_uuid::Uuid,
        /// Required role of the `supernatant_destination` slot.
        #[table_model(generated)]
        supernatant_destination_slot_role_id: String,
        /// Slot declaring the produced `supernatant` sample.
        #[mandatory(aps_procedure_templates::procedure_template_slots)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        supernatant_slot_id: ::rosetta_uuid::Uuid,
        /// Admissible model for the `supernatant` sample.
        #[same_as(
            aps_procedure_templates::procedure_template_slots::asset_model_id,
            supernatant_slot_id
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        supernatant_model_id: ::rosetta_uuid::Uuid,
        /// Required role of the `supernatant` slot.
        #[table_model(generated)]
        supernatant_slot_role_id: String,
        /// Slot declaring the `transferred_with` assets.
        #[mandatory(aps_procedure_templates::procedure_template_slots)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        transferred_with_slot_id: ::rosetta_uuid::Uuid,
        /// Admissible model for the `transferred_with` slot.
        #[same_as(
            aps_procedure_templates::procedure_template_slots::asset_model_id,
            transferred_with_slot_id
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        transferred_with_model_id: ::rosetta_uuid::Uuid,
        /// Required role of the `transferred_with` slot.
        #[table_model(generated)]
        transferred_with_slot_role_id: String,
        /// Volume in liters.
        volume_l: f64,
        /// Allowed volume deviation as a percentage of the target.
        #[table_model(default = 5f64)]
        volume_l_tolerance_percentage: f64,
    }
    impl ::diesel_builders::ValidateColumn<supernatant_procedure_templates::volume_l>
        for <supernatant_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(volume_l: &f64) -> Result<(), Self::Error> {
            use diesel::Column;
            if volume_l <= &0f64 {
                return Err(
                    ::validation_errors::ValidationError::strictly_greater_than_value(
                        <crate::supernatant_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::supernatant_procedure_templates::volume_l::NAME,
                        0f64,
                    ),
                );
            }
            if volume_l.partial_cmp(&f64::INFINITY).is_none_or(::core::cmp::Ordering::is_ge) {
                return Err(
                    ::validation_errors::ValidationError::check_violated(
                        <crate::supernatant_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        &[crate::supernatant_procedure_templates::volume_l::NAME],
                        "volume_l < 'Infinity'::DOUBLE PRECISION",
                    ),
                );
            }
            Ok(())
        }
    }
    impl
        ::diesel_builders::ValidateColumn<
            supernatant_procedure_templates::volume_l_tolerance_percentage,
        > for <supernatant_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(volume_l_tolerance_percentage: &f64) -> Result<(), Self::Error> {
            use diesel::Column;
            if volume_l_tolerance_percentage <= &0f64 {
                return Err(
                    ::validation_errors::ValidationError::strictly_greater_than_value(
                        <crate::supernatant_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::supernatant_procedure_templates::volume_l_tolerance_percentage::NAME,
                        0f64,
                    ),
                );
            }
            if volume_l_tolerance_percentage > &100f64 {
                return Err(
                    ::validation_errors::ValidationError::smaller_than_value(
                        <crate::supernatant_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::supernatant_procedure_templates::volume_l_tolerance_percentage::NAME,
                        100f64,
                    ),
                );
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateRecord<supernatant_procedure_templates::table>
        for <supernatant_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
    {
        #[inline]
        fn validate_record(&self) -> Result<(), ::validation_errors::ValidationError> {
            use diesel::Column;
            if let (Some(stratified_source_slot_id), Some(supernatant_destination_slot_id)) = (
                <Self as diesel_builders::MayGetColumn<
                    supernatant_procedure_templates::stratified_source_slot_id,
                >>::may_get_column_ref(self),
                <Self as diesel_builders::MayGetColumn<
                    supernatant_procedure_templates::supernatant_destination_slot_id,
                >>::may_get_column_ref(self),
            ) && stratified_source_slot_id == supernatant_destination_slot_id
            {
                return Err(
                    ::validation_errors::ValidationError::equal(
                        <crate::supernatant_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::supernatant_procedure_templates::stratified_source_slot_id::NAME,
                        crate::supernatant_procedure_templates::supernatant_destination_slot_id::NAME,
                    ),
                );
            }
            Ok(())
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id> for SupernatantProcedureTemplate {
        fn get_column_ref(
            &self,
        ) -> &<supernatant_procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_procedure_templates::procedure_templates::id>
        for SupernatantProcedureTemplate
    {
        fn get_column_ref(
            &self,
        ) -> &<supernatant_procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for SupernatantProcedureTemplate {
        fn get_column_ref(
            &self,
        ) -> &<supernatant_procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    /// Shape of `supernatant_procedure_templates` the procedure runner renders.
    pub const SUPERNATANT_PROCEDURE_TEMPLATES_SHAPE: procedure_like::shape::TemplateShape =
        procedure_like::shape::TemplateShape {
            table_name: "supernatant_procedure_templates",
            setpoints: &[
                procedure_like::shape::ColumnShape {
                    name: "volume_l",
                    label: "Volume in liters.",
                    sql_type: procedure_like::shape::SqlType::DoublePrecision,
                    unit: Some(procedure_like::shape::Unit::Liter),
                    required: true,
                },
                procedure_like::shape::ColumnShape {
                    name: "volume_l_tolerance_percentage",
                    label: "Allowed volume deviation as a percentage of the target.",
                    sql_type: procedure_like::shape::SqlType::DoublePrecision,
                    unit: Some(procedure_like::shape::Unit::Percentage),
                    required: true,
                },
            ],
            slots: &[
                procedure_like::shape::SlotShape {
                    name: "source_sample",
                    role: procedure_like::shape::SlotRole::Input,
                    model_table_name: "sample_models",
                    slot_column: "source_sample_slot_id",
                    model_column: "source_sample_model_id",
                    placed_into: None,
                    retrieved_from: None,
                },
                procedure_like::shape::SlotShape {
                    name: "stratified_source",
                    role: procedure_like::shape::SlotRole::Input,
                    model_table_name: "volumetric_container_models",
                    slot_column: "stratified_source_slot_id",
                    model_column: "stratified_source_model_id",
                    placed_into: None,
                    retrieved_from: None,
                },
                procedure_like::shape::SlotShape {
                    name: "supernatant_destination",
                    role: procedure_like::shape::SlotRole::Input,
                    model_table_name: "volumetric_container_models",
                    slot_column: "supernatant_destination_slot_id",
                    model_column: "supernatant_destination_model_id",
                    placed_into: None,
                    retrieved_from: None,
                },
                procedure_like::shape::SlotShape {
                    name: "transferred_with",
                    role: procedure_like::shape::SlotRole::Instrument,
                    model_table_name: "volume_measuring_device_models",
                    slot_column: "transferred_with_slot_id",
                    model_column: "transferred_with_model_id",
                    placed_into: None,
                    retrieved_from: None,
                },
                procedure_like::shape::SlotShape {
                    name: "supernatant",
                    role: procedure_like::shape::SlotRole::Produced,
                    model_table_name: "sample_models",
                    slot_column: "supernatant_slot_id",
                    model_column: "supernatant_model_id",
                    placed_into: Some("supernatant_destination"),
                    retrieved_from: None,
                },
            ],
        };
    ::diesel::allow_tables_to_appear_in_same_query!(
        supernatant_procedure_templates,
        ::aps_resources::ownables
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        supernatant_procedure_templates,
        ::aps_procedure_templates::procedure_template_slots
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        supernatant_procedure_templates,
        ::aps_procedure_templates::procedure_templates
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        supernatant_procedure_templates,
        ::aps_resources::resources
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        supernatant_procedure_templates,
        ::aps_assets::sample_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        supernatant_procedure_templates,
        ::aps_physical_asset_models::volume_measuring_device_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        supernatant_procedure_templates,
        ::aps_physical_asset_models::volumetric_container_models
    );
}
pub use supernatant_procedure_templates_model::*;
pub mod supernatant_procedures_model {
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
    /// Recorded execution of a supernatant template.
    #[table_model(ancestors(
        aps_resources::resources,
        aps_resources::ownables,
        aps_procedures::procedures
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[table_model(record_error = ::validation_errors::ValidationError)]
    #[diesel(
        belongs_to(
            aps_supernatant_procedure_templates::SupernatantProcedureTemplate,
            foreign_key = supernatant_procedure_template_id
        )
    )]
    #[table_model(foreign_key((id), (::aps_procedures::procedures::id)))]
    #[table_model(
        foreign_key(
            (supernatant_procedure_template_id),
            (::aps_supernatant_procedure_templates::supernatant_procedure_templates::id)
        )
    )]
    #[table_model(default(
        aps_resources::resources::kind,
        ::aps_authz_resource_kind::ResourceKind::Ownable
    ))]
    #[diesel(table_name = supernatant_procedures)]
    pub struct SupernatantProcedure {
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
        supernatant_procedure_template_id: ::rosetta_uuid::Uuid,
        /// Execution status copied from the parent procedure.
        #[same_as(aps_procedures::procedures::status_id)]
        #[infallible]
        status_id: String,
        /// Measured volume l.
        volume_l: Option<f64>,
    }
    impl ::diesel_builders::ValidateColumn<supernatant_procedures::volume_l>
        for <supernatant_procedures::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(volume_l: &f64) -> Result<(), Self::Error> {
            use diesel::Column;
            if volume_l <= &0f64 {
                return Err(
                    ::validation_errors::ValidationError::strictly_greater_than_value(
                        <crate::supernatant_procedures::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::supernatant_procedures::volume_l::NAME,
                        0f64,
                    ),
                );
            }
            if volume_l.partial_cmp(&f64::INFINITY).is_none_or(::core::cmp::Ordering::is_ge) {
                return Err(
                    ::validation_errors::ValidationError::check_violated(
                        <crate::supernatant_procedures::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        &[crate::supernatant_procedures::volume_l::NAME],
                        "volume_l < 'Infinity'::DOUBLE PRECISION",
                    ),
                );
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateRecord<supernatant_procedures::table>
        for <supernatant_procedures::table as ::diesel_builders::TableExt>::NewValues
    {
        #[inline]
        fn validate_record(&self) -> Result<(), ::validation_errors::ValidationError> {
            use diesel::Column;
            if let (Some(status_id), Some(volume_l)) = (
                <Self as diesel_builders::MayGetColumn<
                    supernatant_procedures::status_id,
                >>::may_get_column_ref(self),
                <Self as diesel_builders::MayGetColumn<
                    supernatant_procedures::volume_l,
                >>::may_get_column_ref(self),
            ) && !((status_id.as_str() != "completed") || (*volume_l).is_some())
            {
                return Err(
                    ::validation_errors::ValidationError::check_violated(
                        <crate::supernatant_procedures::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        &[
                            crate::supernatant_procedures::status_id::NAME,
                            crate::supernatant_procedures::volume_l::NAME,
                        ],
                        "status_id <> 'completed' OR volume_l IS NOT NULL",
                    ),
                );
            }
            Ok(())
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id> for SupernatantProcedure {
        fn get_column_ref(
            &self,
        ) -> &<supernatant_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_procedures::procedures::id> for SupernatantProcedure {
        fn get_column_ref(
            &self,
        ) -> &<supernatant_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for SupernatantProcedure {
        fn get_column_ref(
            &self,
        ) -> &<supernatant_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl procedure_like::ProcedureTableLike for supernatant_procedures::table {
        type ProcedureTemplateTable =
            aps_supernatant_procedure_templates::supernatant_procedure_templates::table;
    }
    /// Shape of `supernatant_procedures` the procedure runner renders.
    pub const SUPERNATANT_PROCEDURES_SHAPE: procedure_like::shape::ProcedureShape =
        procedure_like::shape::ProcedureShape {
            table_name: "supernatant_procedures",
            template_table_name: "supernatant_procedure_templates",
            measurements: &[procedure_like::shape::ColumnShape {
                name: "volume_l",
                label: "Measured volume l.",
                sql_type: procedure_like::shape::SqlType::DoublePrecision,
                unit: Some(procedure_like::shape::Unit::Liter),
                required: true,
            }],
        };
    ::diesel::allow_tables_to_appear_in_same_query!(
        supernatant_procedures,
        ::aps_resources::ownables
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        supernatant_procedures,
        ::aps_procedures::procedures
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        supernatant_procedures,
        ::aps_resources::resources
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        supernatant_procedures,
        ::aps_supernatant_procedure_templates::supernatant_procedure_templates
    );
}
pub use supernatant_procedures_model::*;
