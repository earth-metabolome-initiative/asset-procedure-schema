#![allow(ambiguous_glob_reexports, reason = "TableModel trait names can collide across tables")]
extern crate self as aps_fractioning_procedure_templates;
pub mod fractioning_procedure_templates_model {
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
    /// Typed slots and parameters for fractioning.
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
            aps_procedures::WeighingDeviceModel,
            foreign_key = weighed_with_model_id
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
            (weighed_with_model_id),
            (::aps_procedures::weighing_device_models::id)
        )
    )]
    #[table_model(
        foreign_key(
            (fragment_container_model_id),
            (::aps_physical_asset_models::volumetric_container_models::id)
        )
    )]
    #[table_model(
        foreign_key(
            (fragment_placed_into_model_id),
            (::aps_physical_asset_models::volumetric_container_models::id)
        )
    )]
    #[table_model(foreign_key((fraction_model_id), (::aps_assets::sample_models::id)))]
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
                weighed_with_slot_id,
                id,
                weighed_with_model_id,
                weighed_with_slot_role_id,
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
                fragment_container_slot_id,
                id,
                fragment_container_model_id,
                fragment_container_slot_role_id,
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
                fragment_placed_into_slot_id,
                id,
                fragment_placed_into_model_id,
                fragment_placed_into_slot_role_id,
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
                fraction_slot_id,
                id,
                fraction_model_id,
                fraction_slot_role_id,
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
            (fraction_slot_id, id, fragment_placed_into_slot_id, organization_id),
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
    #[diesel(table_name = fractioning_procedure_templates)]
    pub struct FractioningProcedureTemplate {
        /// Identity shared with the parent template.
        #[same_as(
            aps_procedure_templates::procedure_template_slots::procedure_template_id,
            (
                source_sample_slot_id,
                weighed_with_slot_id,
                fragment_container_slot_id,
                fragment_placed_into_slot_id,
                fraction_slot_id
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
                weighed_with_slot_id,
                fragment_container_slot_id,
                fragment_placed_into_slot_id,
                fraction_slot_id
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
        /// Slot declaring the `weighed_with` assets.
        #[mandatory(aps_procedure_templates::procedure_template_slots)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        weighed_with_slot_id: ::rosetta_uuid::Uuid,
        /// Admissible model for the `weighed_with` slot.
        #[same_as(
            aps_procedure_templates::procedure_template_slots::asset_model_id,
            weighed_with_slot_id
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        weighed_with_model_id: ::rosetta_uuid::Uuid,
        /// Required role of the `weighed_with` slot.
        #[table_model(generated)]
        weighed_with_slot_role_id: String,
        /// Slot declaring the `fragment_container` assets.
        #[mandatory(aps_procedure_templates::procedure_template_slots)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        fragment_container_slot_id: ::rosetta_uuid::Uuid,
        /// Admissible model for the `fragment_container` slot.
        #[same_as(
            aps_procedure_templates::procedure_template_slots::asset_model_id,
            fragment_container_slot_id
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        fragment_container_model_id: ::rosetta_uuid::Uuid,
        /// Required role of the `fragment_container` slot.
        #[table_model(generated)]
        fragment_container_slot_role_id: String,
        /// Slot declaring the `fragment_placed_into` assets.
        #[same_as(
            aps_procedure_templates::procedure_template_slots::placed_into_slot_id,
            fraction_slot_id
        )]
        #[mandatory(aps_procedure_templates::procedure_template_slots)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        fragment_placed_into_slot_id: ::rosetta_uuid::Uuid,
        /// Admissible model for the `fragment_placed_into` slot.
        #[same_as(
            aps_procedure_templates::procedure_template_slots::asset_model_id,
            fragment_placed_into_slot_id
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        fragment_placed_into_model_id: ::rosetta_uuid::Uuid,
        /// Required role of the `fragment_placed_into` slot.
        #[table_model(generated)]
        fragment_placed_into_slot_role_id: String,
        /// Slot declaring the produced `fraction` sample.
        #[mandatory(aps_procedure_templates::procedure_template_slots)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        fraction_slot_id: ::rosetta_uuid::Uuid,
        /// Admissible model for the `fraction` sample.
        #[same_as(
            aps_procedure_templates::procedure_template_slots::asset_model_id,
            fraction_slot_id
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        fraction_model_id: ::rosetta_uuid::Uuid,
        /// Required role of the `fraction` slot.
        #[table_model(generated)]
        fraction_slot_role_id: String,
        /// Mass in kilograms.
        mass_kg: f64,
        /// Allowed mass deviation as a percentage of the target.
        tolerance_percentage: f64,
    }
    impl ::diesel_builders::ValidateColumn<fractioning_procedure_templates::mass_kg>
        for <fractioning_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(mass_kg: &f64) -> Result<(), Self::Error> {
            use diesel::Column;
            if mass_kg <= &0f64 {
                return Err(
                    ::validation_errors::ValidationError::strictly_greater_than_value(
                        <crate::fractioning_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::fractioning_procedure_templates::mass_kg::NAME,
                        0f64,
                    ),
                );
            }
            if mass_kg.partial_cmp(&f64::INFINITY).is_none_or(::core::cmp::Ordering::is_ge) {
                return Err(
                    ::validation_errors::ValidationError::check_violated(
                        <crate::fractioning_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        &[crate::fractioning_procedure_templates::mass_kg::NAME],
                        "mass_kg < 'Infinity'::DOUBLE PRECISION",
                    ),
                );
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateColumn<fractioning_procedure_templates::tolerance_percentage>
        for <fractioning_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(tolerance_percentage: &f64) -> Result<(), Self::Error> {
            use diesel::Column;
            if tolerance_percentage <= &0f64 {
                return Err(
                    ::validation_errors::ValidationError::strictly_greater_than_value(
                        <crate::fractioning_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::fractioning_procedure_templates::tolerance_percentage::NAME,
                        0f64,
                    ),
                );
            }
            if tolerance_percentage > &100f64 {
                return Err(
                    ::validation_errors::ValidationError::smaller_than_value(
                        <crate::fractioning_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::fractioning_procedure_templates::tolerance_percentage::NAME,
                        100f64,
                    ),
                );
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateRecord<fractioning_procedure_templates::table>
        for <fractioning_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
    {
        #[inline]
        fn validate_record(&self) -> Result<(), ::validation_errors::ValidationError> {
            use diesel::Column;
            if let (Some(fragment_container_slot_id), Some(fragment_placed_into_slot_id)) = (
                <Self as diesel_builders::MayGetColumn<
                    fractioning_procedure_templates::fragment_container_slot_id,
                >>::may_get_column_ref(self),
                <Self as diesel_builders::MayGetColumn<
                    fractioning_procedure_templates::fragment_placed_into_slot_id,
                >>::may_get_column_ref(self),
            ) && fragment_container_slot_id == fragment_placed_into_slot_id
            {
                return Err(
                    ::validation_errors::ValidationError::equal(
                        <crate::fractioning_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::fractioning_procedure_templates::fragment_container_slot_id::NAME,
                        crate::fractioning_procedure_templates::fragment_placed_into_slot_id::NAME,
                    ),
                );
            }
            Ok(())
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id> for FractioningProcedureTemplate {
        fn get_column_ref(
            &self,
        ) -> &<fractioning_procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_procedure_templates::procedure_templates::id>
        for FractioningProcedureTemplate
    {
        fn get_column_ref(
            &self,
        ) -> &<fractioning_procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for FractioningProcedureTemplate {
        fn get_column_ref(
            &self,
        ) -> &<fractioning_procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    /// Shape of `fractioning_procedure_templates` the procedure runner renders.
    pub const FRACTIONING_PROCEDURE_TEMPLATES_SHAPE: procedure_like::shape::TemplateShape =
        procedure_like::shape::TemplateShape {
            table_name: "fractioning_procedure_templates",
            setpoints: &[
                procedure_like::shape::ColumnShape {
                    name: "mass_kg",
                    label: "Mass in kilograms.",
                    sql_type: procedure_like::shape::SqlType::DoublePrecision,
                    unit: Some(procedure_like::shape::Unit::Kilogram),
                    required: true,
                },
                procedure_like::shape::ColumnShape {
                    name: "tolerance_percentage",
                    label: "Allowed mass deviation as a percentage of the target.",
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
                    name: "weighed_with",
                    role: procedure_like::shape::SlotRole::Instrument,
                    model_table_name: "weighing_device_models",
                    slot_column: "weighed_with_slot_id",
                    model_column: "weighed_with_model_id",
                    placed_into: None,
                    retrieved_from: None,
                },
                procedure_like::shape::SlotShape {
                    name: "fragment_container",
                    role: procedure_like::shape::SlotRole::Input,
                    model_table_name: "volumetric_container_models",
                    slot_column: "fragment_container_slot_id",
                    model_column: "fragment_container_model_id",
                    placed_into: None,
                    retrieved_from: None,
                },
                procedure_like::shape::SlotShape {
                    name: "fragment_placed_into",
                    role: procedure_like::shape::SlotRole::Input,
                    model_table_name: "volumetric_container_models",
                    slot_column: "fragment_placed_into_slot_id",
                    model_column: "fragment_placed_into_model_id",
                    placed_into: None,
                    retrieved_from: None,
                },
                procedure_like::shape::SlotShape {
                    name: "fraction",
                    role: procedure_like::shape::SlotRole::Produced,
                    model_table_name: "sample_models",
                    slot_column: "fraction_slot_id",
                    model_column: "fraction_model_id",
                    placed_into: Some("fragment_placed_into"),
                    retrieved_from: None,
                },
            ],
        };
    ::diesel::allow_tables_to_appear_in_same_query!(
        fractioning_procedure_templates,
        ::aps_resources::ownables
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        fractioning_procedure_templates,
        ::aps_procedure_templates::procedure_template_slots
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        fractioning_procedure_templates,
        ::aps_procedure_templates::procedure_templates
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        fractioning_procedure_templates,
        ::aps_resources::resources
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        fractioning_procedure_templates,
        ::aps_assets::sample_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        fractioning_procedure_templates,
        ::aps_physical_asset_models::volumetric_container_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        fractioning_procedure_templates,
        ::aps_procedures::weighing_device_models
    );
}
pub use fractioning_procedure_templates_model::*;
pub mod fractioning_procedures_model {
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
    /// Recorded execution of a fractioning template.
    #[table_model(ancestors(
        aps_resources::resources,
        aps_resources::ownables,
        aps_procedures::procedures
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[table_model(record_error = ::validation_errors::ValidationError)]
    #[diesel(
        belongs_to(
            aps_fractioning_procedure_templates::FractioningProcedureTemplate,
            foreign_key = fractioning_procedure_template_id
        )
    )]
    #[table_model(foreign_key((id), (::aps_procedures::procedures::id)))]
    #[table_model(
        foreign_key(
            (fractioning_procedure_template_id),
            (::aps_fractioning_procedure_templates::fractioning_procedure_templates::id)
        )
    )]
    #[table_model(default(
        aps_resources::resources::kind,
        ::aps_authz_resource_kind::ResourceKind::Ownable
    ))]
    #[diesel(table_name = fractioning_procedures)]
    pub struct FractioningProcedure {
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
        fractioning_procedure_template_id: ::rosetta_uuid::Uuid,
        /// Execution status copied from the parent procedure.
        #[same_as(aps_procedures::procedures::status_id)]
        #[infallible]
        status_id: String,
        /// Mass in kilograms.
        mass_kg: Option<f64>,
    }
    impl ::diesel_builders::ValidateColumn<fractioning_procedures::mass_kg>
        for <fractioning_procedures::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(mass_kg: &f64) -> Result<(), Self::Error> {
            use diesel::Column;
            if mass_kg <= &0f64 {
                return Err(
                    ::validation_errors::ValidationError::strictly_greater_than_value(
                        <crate::fractioning_procedures::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::fractioning_procedures::mass_kg::NAME,
                        0f64,
                    ),
                );
            }
            if mass_kg.partial_cmp(&f64::INFINITY).is_none_or(::core::cmp::Ordering::is_ge) {
                return Err(
                    ::validation_errors::ValidationError::check_violated(
                        <crate::fractioning_procedures::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        &[crate::fractioning_procedures::mass_kg::NAME],
                        "mass_kg < 'Infinity'::DOUBLE PRECISION",
                    ),
                );
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateRecord<fractioning_procedures::table>
        for <fractioning_procedures::table as ::diesel_builders::TableExt>::NewValues
    {
        #[inline]
        fn validate_record(&self) -> Result<(), ::validation_errors::ValidationError> {
            use diesel::Column;
            if let (Some(status_id), Some(mass_kg)) = (
                <Self as diesel_builders::MayGetColumn<
                    fractioning_procedures::status_id,
                >>::may_get_column_ref(self),
                <Self as diesel_builders::MayGetColumn<
                    fractioning_procedures::mass_kg,
                >>::may_get_column_ref(self),
            ) && !((status_id.as_str() != "completed") || (*mass_kg).is_some())
            {
                return Err(
                    ::validation_errors::ValidationError::check_violated(
                        <crate::fractioning_procedures::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        &[
                            crate::fractioning_procedures::status_id::NAME,
                            crate::fractioning_procedures::mass_kg::NAME,
                        ],
                        "status_id <> 'completed' OR mass_kg IS NOT NULL",
                    ),
                );
            }
            Ok(())
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id> for FractioningProcedure {
        fn get_column_ref(
            &self,
        ) -> &<fractioning_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_procedures::procedures::id> for FractioningProcedure {
        fn get_column_ref(
            &self,
        ) -> &<fractioning_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for FractioningProcedure {
        fn get_column_ref(
            &self,
        ) -> &<fractioning_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl procedure_like::ProcedureTableLike for fractioning_procedures::table {
        type ProcedureTemplateTable =
            aps_fractioning_procedure_templates::fractioning_procedure_templates::table;
    }
    /// Shape of `fractioning_procedures` the procedure runner renders.
    pub const FRACTIONING_PROCEDURES_SHAPE: procedure_like::shape::ProcedureShape =
        procedure_like::shape::ProcedureShape {
            table_name: "fractioning_procedures",
            template_table_name: "fractioning_procedure_templates",
            measurements: &[procedure_like::shape::ColumnShape {
                name: "mass_kg",
                label: "Mass in kilograms.",
                sql_type: procedure_like::shape::SqlType::DoublePrecision,
                unit: Some(procedure_like::shape::Unit::Kilogram),
                required: true,
            }],
        };
    ::diesel::allow_tables_to_appear_in_same_query!(
        fractioning_procedures,
        ::aps_fractioning_procedure_templates::fractioning_procedure_templates
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        fractioning_procedures,
        ::aps_resources::ownables
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        fractioning_procedures,
        ::aps_procedures::procedures
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        fractioning_procedures,
        ::aps_resources::resources
    );
}
pub use fractioning_procedures_model::*;
