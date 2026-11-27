#![allow(ambiguous_glob_reexports, reason = "TableModel trait names can collide across tables")]
extern crate self as aps_weighing_device_models;
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
    #[table_model(ancestors(aps_resources::resources, aps_resources::procedure_templates))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[table_model(record_error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_resources::ProcedureTemplate, foreign_key = id))]
    #[table_model(foreign_key((id), (::aps_resources::procedure_templates::id)))]
    #[table_model(
        foreign_key((source_sample_model_id), (::aps_sample_models::sample_models::id))
    )]
    #[table_model(
        foreign_key(
            (weighed_with_model_id),
            (::aps_weighing_device_models::weighing_device_models::id)
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
    #[table_model(
        foreign_key((fraction_model_id), (::aps_sample_models::sample_models::id))
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
                weighed_with_slot_id,
                id,
                weighed_with_model_id,
                weighed_with_slot_role_id,
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
                fragment_container_slot_id,
                id,
                fragment_container_model_id,
                fragment_container_slot_role_id,
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
                fragment_placed_into_slot_id,
                id,
                fragment_placed_into_model_id,
                fragment_placed_into_slot_role_id,
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
                fraction_slot_id,
                id,
                fraction_model_id,
                fraction_slot_role_id,
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
            (fraction_slot_id, id, fragment_placed_into_slot_id, organization_id),
            (
                ::aps_resources::procedure_template_slots::id,
                ::aps_resources::procedure_template_slots::procedure_template_id,
                ::aps_resources::procedure_template_slots::placed_into_slot_id,
                ::aps_resources::procedure_template_slots::organization_id
            )
        )
    )]
    #[table_model(
        foreign_key(
            (weighed_with_model_id, weighed_with_model_archived),
            (
                ::aps_weighing_device_models::weighing_device_models::id,
                ::aps_weighing_device_models::weighing_device_models::archived
            )
        )
    )]
    #[table_model(
        foreign_key(
            (fragment_container_model_id, fragment_container_model_archived),
            (
                ::aps_physical_asset_models::volumetric_container_models::id,
                ::aps_physical_asset_models::volumetric_container_models::archived
            )
        )
    )]
    #[table_model(
        foreign_key(
            (fragment_placed_into_model_id, fragment_placed_into_model_archived),
            (
                ::aps_physical_asset_models::volumetric_container_models::id,
                ::aps_physical_asset_models::volumetric_container_models::archived
            )
        )
    )]
    #[table_model(
        foreign_key(
            (fraction_model_id, fraction_model_archived),
            (
                ::aps_sample_models::sample_models::id,
                ::aps_sample_models::sample_models::archived
            )
        )
    )]
    #[table_model(
        foreign_key(
            (source_sample_model_id, source_sample_model_archived),
            (
                ::aps_sample_models::sample_models::id,
                ::aps_sample_models::sample_models::archived
            )
        )
    )]
    #[diesel(table_name = fractioning_procedure_templates)]
    pub struct FractioningProcedureTemplate {
        /// Identity shared with the parent template.
        #[same_as(
            aps_resources::procedure_template_slots::procedure_template_id,
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
        #[same_as(aps_resources::procedure_templates::organization_id)]
        #[same_as(
            aps_resources::procedure_template_slots::organization_id,
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
        #[mandatory(aps_resources::procedure_template_slots)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        source_sample_slot_id: ::rosetta_uuid::Uuid,
        /// Admissible model for the source sample.
        #[same_as(aps_resources::procedure_template_slots::asset_model_id, source_sample_slot_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        source_sample_model_id: ::rosetta_uuid::Uuid,
        /// Required role of the source sample.
        #[table_model(generated)]
        source_sample_slot_role_id: String,
        /// Slot declaring the `weighed_with` assets.
        #[mandatory(aps_resources::procedure_template_slots)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        weighed_with_slot_id: ::rosetta_uuid::Uuid,
        /// Admissible model for the `weighed_with` slot.
        #[same_as(aps_resources::procedure_template_slots::asset_model_id, weighed_with_slot_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        weighed_with_model_id: ::rosetta_uuid::Uuid,
        /// Required role of the `weighed_with` slot.
        #[table_model(generated)]
        weighed_with_slot_role_id: String,
        /// Slot declaring the `fragment_container` assets.
        #[mandatory(aps_resources::procedure_template_slots)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        fragment_container_slot_id: ::rosetta_uuid::Uuid,
        /// Admissible model for the `fragment_container` slot.
        #[same_as(
            aps_resources::procedure_template_slots::asset_model_id,
            fragment_container_slot_id
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        fragment_container_model_id: ::rosetta_uuid::Uuid,
        /// Required role of the `fragment_container` slot.
        #[table_model(generated)]
        fragment_container_slot_role_id: String,
        /// Slot declaring the `fragment_placed_into` assets.
        #[same_as(aps_resources::procedure_template_slots::placed_into_slot_id, fraction_slot_id)]
        #[mandatory(aps_resources::procedure_template_slots)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        fragment_placed_into_slot_id: ::rosetta_uuid::Uuid,
        /// Admissible model for the `fragment_placed_into` slot.
        #[same_as(
            aps_resources::procedure_template_slots::asset_model_id,
            fragment_placed_into_slot_id
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        fragment_placed_into_model_id: ::rosetta_uuid::Uuid,
        /// Required role of the `fragment_placed_into` slot.
        #[table_model(generated)]
        fragment_placed_into_slot_role_id: String,
        /// Slot declaring the produced `fraction` sample.
        #[mandatory(aps_resources::procedure_template_slots)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        fraction_slot_id: ::rosetta_uuid::Uuid,
        /// Admissible model for the `fraction` sample.
        #[same_as(aps_resources::procedure_template_slots::asset_model_id, fraction_slot_id)]
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
        /// Archive state inherited from this resource.
        #[same_as(aps_resources::procedure_templates::archived)]
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
        /// Archive state of the row named by `weighed_with_model_id`.
        #[table_model(default = false)]
        #[infallible]
        weighed_with_model_archived: bool,
        /// Archive state of the row named by `fragment_container_model_id`.
        #[table_model(default = false)]
        #[infallible]
        fragment_container_model_archived: bool,
        /// Archive state of the row named by `fragment_placed_into_model_id`.
        #[table_model(default = false)]
        #[infallible]
        fragment_placed_into_model_archived: bool,
        /// Archive state of the row named by `fraction_model_id`.
        #[table_model(default = false)]
        #[infallible]
        fraction_model_archived: bool,
        /// Archive state of the row named by `source_sample_model_id`.
        #[table_model(default = false)]
        #[infallible]
        source_sample_model_archived: bool,
    }
    ::diesel_builders::prelude::unique_index!(
        fractioning_procedure_templates::id,
        fractioning_procedure_templates::archived
    );
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
    impl ::diesel_builders::GetColumn<aps_resources::procedure_templates::id>
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
        ::aps_resources::procedure_template_slots
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        fractioning_procedure_templates,
        ::aps_resources::procedure_templates
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        fractioning_procedure_templates,
        ::aps_resources::resources
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        fractioning_procedure_templates,
        ::aps_sample_models::sample_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        fractioning_procedure_templates,
        ::aps_physical_asset_models::volumetric_container_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        fractioning_procedure_templates,
        ::aps_weighing_device_models::weighing_device_models
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
        ::diesel_builders::prelude::TableModel,
    )]
    /// Recorded execution of a fractioning template.
    #[table_model(ancestors(aps_resources::resources, aps_procedures::procedures))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[table_model(record_error = ::validation_errors::ValidationError)]
    #[table_model(foreign_key((id), (::aps_procedures::procedures::id)))]
    #[table_model(
        foreign_key(
            (fractioning_procedure_template_id),
            (::aps_weighing_device_models::fractioning_procedure_templates::id)
        )
    )]
    #[table_model(
        foreign_key(
            (fractioning_procedure_template_id, fractioning_procedure_template_archived),
            (
                ::aps_weighing_device_models::fractioning_procedure_templates::id,
                ::aps_weighing_device_models::fractioning_procedure_templates::archived
            )
        )
    )]
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
        #[table_model(default = "planned")]
        #[infallible]
        status_id: String,
        /// Mass in kilograms.
        mass_kg: Option<f64>,
        /// Archive state inherited from this resource.
        #[same_as(aps_procedures::procedures::archived)]
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
        /// Archive state of the row named by
        /// `fractioning_procedure_template_id`.
        #[table_model(default = false)]
        #[infallible]
        fractioning_procedure_template_archived: bool,
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
            aps_weighing_device_models::fractioning_procedure_templates::table;
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
        ::aps_weighing_device_models::fractioning_procedure_templates
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
pub mod weighing_device_models_model {
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
    /// Catalog of weighing-device model definitions.
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
    #[diesel(table_name = weighing_device_models)]
    pub struct WeighingDeviceModel {
        /// Stable model identifier inherited from `physical_asset_models`.
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
    ::diesel_builders::prelude::unique_index!(
        weighing_device_models::id,
        weighing_device_models::archived
    );
    impl ::diesel_builders::GetColumn<aps_resources::asset_models::id> for WeighingDeviceModel {
        fn get_column_ref(
            &self,
        ) -> &<weighing_device_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_physical_asset_models::physical_asset_models::id>
        for WeighingDeviceModel
    {
        fn get_column_ref(
            &self,
        ) -> &<weighing_device_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for WeighingDeviceModel {
        fn get_column_ref(
            &self,
        ) -> &<weighing_device_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    /// Shape of `weighing_device_models` the procedure runner renders.
    pub const WEIGHING_DEVICE_MODELS_SHAPE: procedure_like::shape::AssetModelShape =
        procedure_like::shape::AssetModelShape {
            table_name: "weighing_device_models",
            asset_table_name: "weighing_devices",
            model_column: "weighing_device_model_id",
            required: &[
                procedure_like::shape::ColumnShape {
                    name: "organization_id",
                    label: "Organization containing this resource.",
                    sql_type: procedure_like::shape::SqlType::Uuid,
                    unit: None,
                    required: true,
                },
                procedure_like::shape::ColumnShape {
                    name: "weighing_device_model_id",
                    label: "Weighing-device model instantiated by this physical asset.",
                    sql_type: procedure_like::shape::SqlType::Uuid,
                    unit: None,
                    required: true,
                },
            ],
        };
    ::diesel::allow_tables_to_appear_in_same_query!(
        weighing_device_models,
        ::aps_resources::asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        weighing_device_models,
        ::aps_physical_asset_models::physical_asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        weighing_device_models,
        ::aps_resources::resources
    );
}
pub use weighing_device_models_model::*;
pub mod weighing_devices_model {
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
    /// Physical weighing devices tracked in APS inventory.
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
            (weighing_device_model_id),
            (::aps_weighing_device_models::weighing_device_models::id)
        )
    )]
    #[table_model(
        foreign_key(
            (weighing_device_model_id, weighing_device_model_archived),
            (
                ::aps_weighing_device_models::weighing_device_models::id,
                ::aps_weighing_device_models::weighing_device_models::archived
            )
        )
    )]
    #[diesel(table_name = weighing_devices)]
    pub struct WeighingDevice {
        /// Stable asset identifier inherited from `physical_assets`.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing this resource.
        #[same_as(aps_assets::physical_assets::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Weighing-device model instantiated by this physical asset.
        #[same_as(aps_assets::physical_assets::physical_asset_model_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        weighing_device_model_id: ::rosetta_uuid::Uuid,
        /// Archive state inherited from this resource.
        #[same_as(aps_assets::physical_assets::archived)]
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
        /// Archive state of the row named by `weighing_device_model_id`.
        #[table_model(default = false)]
        #[infallible]
        weighing_device_model_archived: bool,
    }
    impl ::diesel_builders::GetColumn<aps_assets::assets::id> for WeighingDevice {
        fn get_column_ref(
            &self,
        ) -> &<weighing_devices::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_assets::physical_assets::id> for WeighingDevice {
        fn get_column_ref(
            &self,
        ) -> &<weighing_devices::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for WeighingDevice {
        fn get_column_ref(
            &self,
        ) -> &<weighing_devices::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    ::diesel::allow_tables_to_appear_in_same_query!(weighing_devices, ::aps_assets::assets);
    ::diesel::allow_tables_to_appear_in_same_query!(
        weighing_devices,
        ::aps_assets::physical_assets
    );
    ::diesel::allow_tables_to_appear_in_same_query!(weighing_devices, ::aps_resources::resources);
    ::diesel::allow_tables_to_appear_in_same_query!(
        weighing_devices,
        ::aps_weighing_device_models::weighing_device_models
    );
}
pub use weighing_devices_model::*;
pub mod weighing_procedure_templates_model {
    #[derive(
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
    /// Typed slots and parameters for weighing.
    #[table_model(ancestors(aps_resources::resources, aps_resources::procedure_templates))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_resources::ProcedureTemplate, foreign_key = id))]
    #[table_model(foreign_key((id), (::aps_resources::procedure_templates::id)))]
    #[table_model(
        foreign_key(
            (weighed_asset_model_id),
            (::aps_physical_asset_models::physical_asset_models::id)
        )
    )]
    #[table_model(
        foreign_key(
            (weighed_with_model_id),
            (::aps_weighing_device_models::weighing_device_models::id)
        )
    )]
    #[table_model(
        foreign_key(
            (
                weighed_asset_slot_id,
                id,
                weighed_asset_model_id,
                weighed_asset_slot_role_id,
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
                weighed_with_slot_id,
                id,
                weighed_with_model_id,
                weighed_with_slot_role_id,
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
            (weighed_asset_model_id, weighed_asset_model_archived),
            (
                ::aps_physical_asset_models::physical_asset_models::id,
                ::aps_physical_asset_models::physical_asset_models::archived
            )
        )
    )]
    #[table_model(
        foreign_key(
            (weighed_with_model_id, weighed_with_model_archived),
            (
                ::aps_weighing_device_models::weighing_device_models::id,
                ::aps_weighing_device_models::weighing_device_models::archived
            )
        )
    )]
    #[diesel(table_name = weighing_procedure_templates)]
    pub struct WeighingProcedureTemplate {
        /// Identity shared with the parent template.
        #[same_as(
            aps_resources::procedure_template_slots::procedure_template_id,
            (weighed_asset_slot_id, weighed_with_slot_id)
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing the parent resource.
        #[same_as(aps_resources::procedure_templates::organization_id)]
        #[same_as(
            aps_resources::procedure_template_slots::organization_id,
            (weighed_asset_slot_id, weighed_with_slot_id)
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Slot declaring the `weighed_asset` assets.
        #[mandatory(aps_resources::procedure_template_slots)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        weighed_asset_slot_id: ::rosetta_uuid::Uuid,
        /// Admissible model for the `weighed_asset` slot.
        #[same_as(aps_resources::procedure_template_slots::asset_model_id, weighed_asset_slot_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        weighed_asset_model_id: ::rosetta_uuid::Uuid,
        /// Required role of the `weighed_asset` slot.
        #[table_model(generated)]
        weighed_asset_slot_role_id: String,
        /// Slot declaring the `weighed_with` assets.
        #[mandatory(aps_resources::procedure_template_slots)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        weighed_with_slot_id: ::rosetta_uuid::Uuid,
        /// Admissible model for the `weighed_with` slot.
        #[same_as(aps_resources::procedure_template_slots::asset_model_id, weighed_with_slot_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        weighed_with_model_id: ::rosetta_uuid::Uuid,
        /// Required role of the `weighed_with` slot.
        #[table_model(generated)]
        weighed_with_slot_role_id: String,
        /// Archive state inherited from this resource.
        #[same_as(aps_resources::procedure_templates::archived)]
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
        /// Archive state of the row named by `weighed_asset_model_id`.
        #[table_model(default = false)]
        #[infallible]
        weighed_asset_model_archived: bool,
        /// Archive state of the row named by `weighed_with_model_id`.
        #[table_model(default = false)]
        #[infallible]
        weighed_with_model_archived: bool,
    }
    ::diesel_builders::prelude::unique_index!(
        weighing_procedure_templates::id,
        weighing_procedure_templates::archived
    );
    impl ::diesel_builders::GetColumn<aps_resources::procedure_templates::id>
        for WeighingProcedureTemplate
    {
        fn get_column_ref(
            &self,
        ) -> &<weighing_procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for WeighingProcedureTemplate {
        fn get_column_ref(
            &self,
        ) -> &<weighing_procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    /// Shape of `weighing_procedure_templates` the procedure runner renders.
    pub const WEIGHING_PROCEDURE_TEMPLATES_SHAPE: procedure_like::shape::TemplateShape =
        procedure_like::shape::TemplateShape {
            table_name: "weighing_procedure_templates",
            setpoints: &[],
            slots: &[
                procedure_like::shape::SlotShape {
                    name: "weighed_asset",
                    role: procedure_like::shape::SlotRole::Input,
                    model_table_name: "physical_asset_models",
                    slot_column: "weighed_asset_slot_id",
                    model_column: "weighed_asset_model_id",
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
            ],
        };
    ::diesel::allow_tables_to_appear_in_same_query!(
        weighing_procedure_templates,
        ::aps_physical_asset_models::physical_asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        weighing_procedure_templates,
        ::aps_resources::procedure_template_slots
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        weighing_procedure_templates,
        ::aps_resources::procedure_templates
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        weighing_procedure_templates,
        ::aps_resources::resources
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        weighing_procedure_templates,
        ::aps_weighing_device_models::weighing_device_models
    );
}
pub use weighing_procedure_templates_model::*;
pub mod weighing_procedures_model {
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
    /// Recorded execution of a weighing template.
    #[table_model(ancestors(aps_resources::resources, aps_procedures::procedures))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[table_model(record_error = ::validation_errors::ValidationError)]
    #[table_model(foreign_key((id), (::aps_procedures::procedures::id)))]
    #[table_model(
        foreign_key(
            (weighing_procedure_template_id),
            (::aps_weighing_device_models::weighing_procedure_templates::id)
        )
    )]
    #[table_model(
        foreign_key(
            (weighing_procedure_template_id, weighing_procedure_template_archived),
            (
                ::aps_weighing_device_models::weighing_procedure_templates::id,
                ::aps_weighing_device_models::weighing_procedure_templates::archived
            )
        )
    )]
    #[diesel(table_name = weighing_procedures)]
    pub struct WeighingProcedure {
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
        weighing_procedure_template_id: ::rosetta_uuid::Uuid,
        /// Execution status copied from the parent procedure.
        #[same_as(aps_procedures::procedures::status_id)]
        #[table_model(default = "planned")]
        #[infallible]
        status_id: String,
        /// Mass in kilograms.
        mass_kg: Option<f64>,
        /// Archive state inherited from this resource.
        #[same_as(aps_procedures::procedures::archived)]
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
        /// Archive state of the row named by `weighing_procedure_template_id`.
        #[table_model(default = false)]
        #[infallible]
        weighing_procedure_template_archived: bool,
    }
    impl ::diesel_builders::ValidateColumn<weighing_procedures::mass_kg>
        for <weighing_procedures::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(mass_kg: &f64) -> Result<(), Self::Error> {
            use diesel::Column;
            if mass_kg <= &0f64 {
                return Err(::validation_errors::ValidationError::strictly_greater_than_value(
                    <crate::weighing_procedures::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::weighing_procedures::mass_kg::NAME,
                    0f64,
                ));
            }
            if mass_kg.partial_cmp(&f64::INFINITY).is_none_or(::core::cmp::Ordering::is_ge) {
                return Err(::validation_errors::ValidationError::check_violated(
                    <crate::weighing_procedures::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    &[crate::weighing_procedures::mass_kg::NAME],
                    "mass_kg < 'Infinity'::DOUBLE PRECISION",
                ));
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateRecord<weighing_procedures::table>
        for <weighing_procedures::table as ::diesel_builders::TableExt>::NewValues
    {
        #[inline]
        fn validate_record(&self) -> Result<(), ::validation_errors::ValidationError> {
            use diesel::Column;
            if let (Some(status_id), Some(mass_kg)) = (
                <Self as diesel_builders::MayGetColumn<
                    weighing_procedures::status_id,
                >>::may_get_column_ref(self),
                <Self as diesel_builders::MayGetColumn<
                    weighing_procedures::mass_kg,
                >>::may_get_column_ref(self),
            ) && !((status_id.as_str() != "completed") || (*mass_kg).is_some())
            {
                return Err(
                    ::validation_errors::ValidationError::check_violated(
                        <crate::weighing_procedures::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        &[
                            crate::weighing_procedures::status_id::NAME,
                            crate::weighing_procedures::mass_kg::NAME,
                        ],
                        "status_id <> 'completed' OR mass_kg IS NOT NULL",
                    ),
                );
            }
            Ok(())
        }
    }
    impl ::diesel_builders::GetColumn<aps_procedures::procedures::id> for WeighingProcedure {
        fn get_column_ref(
            &self,
        ) -> &<weighing_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for WeighingProcedure {
        fn get_column_ref(
            &self,
        ) -> &<weighing_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl procedure_like::ProcedureTableLike for weighing_procedures::table {
        type ProcedureTemplateTable =
            aps_weighing_device_models::weighing_procedure_templates::table;
    }
    /// Shape of `weighing_procedures` the procedure runner renders.
    pub const WEIGHING_PROCEDURES_SHAPE: procedure_like::shape::ProcedureShape =
        procedure_like::shape::ProcedureShape {
            table_name: "weighing_procedures",
            template_table_name: "weighing_procedure_templates",
            measurements: &[procedure_like::shape::ColumnShape {
                name: "mass_kg",
                label: "Mass in kilograms.",
                sql_type: procedure_like::shape::SqlType::DoublePrecision,
                unit: Some(procedure_like::shape::Unit::Kilogram),
                required: true,
            }],
        };
    ::diesel::allow_tables_to_appear_in_same_query!(
        weighing_procedures,
        ::aps_procedures::procedures
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        weighing_procedures,
        ::aps_resources::resources
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        weighing_procedures,
        ::aps_weighing_device_models::weighing_procedure_templates
    );
}
pub use weighing_procedures_model::*;
