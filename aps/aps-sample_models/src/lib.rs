#![allow(ambiguous_glob_reexports, reason = "TableModel trait names can collide across tables")]
extern crate self as aps_sample_models;
pub mod aliquoting_procedure_templates_model {
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
    /// Typed slots and parameters for aliquoting.
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
            (aliquoted_from_model_id),
            (::aps_physical_asset_models::volumetric_container_models::id)
        )
    )]
    #[table_model(
        foreign_key(
            (aliquoted_into_model_id),
            (::aps_physical_asset_models::volumetric_container_models::id)
        )
    )]
    #[table_model(
        foreign_key((aliquot_model_id), (::aps_sample_models::sample_models::id))
    )]
    #[table_model(
        foreign_key(
            (aliquoted_with_model_id),
            (::aps_sample_models::volume_measuring_device_models::id)
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
                aliquoted_from_slot_id,
                id,
                aliquoted_from_model_id,
                aliquoted_from_slot_role_id,
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
                aliquoted_into_slot_id,
                id,
                aliquoted_into_model_id,
                aliquoted_into_slot_role_id,
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
                aliquoted_with_slot_id,
                id,
                aliquoted_with_model_id,
                aliquoted_with_slot_role_id,
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
                aliquot_slot_id,
                id,
                aliquot_model_id,
                aliquot_slot_role_id,
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
            (aliquot_slot_id, id, aliquoted_into_slot_id, organization_id),
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
            (aliquoted_from_model_id, aliquoted_from_model_archived),
            (
                ::aps_physical_asset_models::volumetric_container_models::id,
                ::aps_physical_asset_models::volumetric_container_models::archived
            )
        )
    )]
    #[table_model(
        foreign_key(
            (aliquoted_into_model_id, aliquoted_into_model_archived),
            (
                ::aps_physical_asset_models::volumetric_container_models::id,
                ::aps_physical_asset_models::volumetric_container_models::archived
            )
        )
    )]
    #[table_model(
        foreign_key(
            (aliquot_model_id, aliquot_model_archived),
            (
                ::aps_sample_models::sample_models::id,
                ::aps_sample_models::sample_models::archived
            )
        )
    )]
    #[table_model(
        foreign_key(
            (aliquoted_with_model_id, aliquoted_with_model_archived),
            (
                ::aps_sample_models::volume_measuring_device_models::id,
                ::aps_sample_models::volume_measuring_device_models::archived
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
    #[diesel(table_name = aliquoting_procedure_templates)]
    pub struct AliquotingProcedureTemplate {
        /// Identity shared with the parent template.
        #[same_as(
            aps_resources::procedure_template_slots::procedure_template_id,
            (
                source_sample_slot_id,
                aliquoted_from_slot_id,
                aliquoted_into_slot_id,
                aliquoted_with_slot_id,
                aliquot_slot_id
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
                aliquoted_from_slot_id,
                aliquoted_into_slot_id,
                aliquoted_with_slot_id,
                aliquot_slot_id
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
        /// Slot declaring the `aliquoted_from` assets.
        #[mandatory(aps_resources::procedure_template_slots)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        aliquoted_from_slot_id: ::rosetta_uuid::Uuid,
        /// Admissible model for the `aliquoted_from` slot.
        #[same_as(aps_resources::procedure_template_slots::asset_model_id, aliquoted_from_slot_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        aliquoted_from_model_id: ::rosetta_uuid::Uuid,
        /// Required role of the `aliquoted_from` slot.
        #[table_model(generated)]
        aliquoted_from_slot_role_id: String,
        /// Slot declaring the `aliquoted_into` assets.
        #[same_as(aps_resources::procedure_template_slots::placed_into_slot_id, aliquot_slot_id)]
        #[mandatory(aps_resources::procedure_template_slots)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        aliquoted_into_slot_id: ::rosetta_uuid::Uuid,
        /// Admissible model for the `aliquoted_into` slot.
        #[same_as(aps_resources::procedure_template_slots::asset_model_id, aliquoted_into_slot_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        aliquoted_into_model_id: ::rosetta_uuid::Uuid,
        /// Required role of the `aliquoted_into` slot.
        #[table_model(generated)]
        aliquoted_into_slot_role_id: String,
        /// Slot declaring the produced `aliquot` sample.
        #[mandatory(aps_resources::procedure_template_slots)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        aliquot_slot_id: ::rosetta_uuid::Uuid,
        /// Admissible model for the `aliquot` sample.
        #[same_as(aps_resources::procedure_template_slots::asset_model_id, aliquot_slot_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        aliquot_model_id: ::rosetta_uuid::Uuid,
        /// Required role of the `aliquot` slot.
        #[table_model(generated)]
        aliquot_slot_role_id: String,
        /// Slot declaring the `aliquoted_with` assets.
        #[mandatory(aps_resources::procedure_template_slots)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        aliquoted_with_slot_id: ::rosetta_uuid::Uuid,
        /// Admissible model for the `aliquoted_with` slot.
        #[same_as(aps_resources::procedure_template_slots::asset_model_id, aliquoted_with_slot_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        aliquoted_with_model_id: ::rosetta_uuid::Uuid,
        /// Required role of the `aliquoted_with` slot.
        #[table_model(generated)]
        aliquoted_with_slot_role_id: String,
        /// Volume in liters.
        volume_l: f64,
        /// Allowed volume deviation as a percentage of the target.
        #[table_model(default = 5f64)]
        volume_l_tolerance_percentage: f64,
        /// Archive state inherited from this resource.
        #[same_as(aps_resources::procedure_templates::archived)]
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
        /// Archive state of the row named by `aliquoted_from_model_id`.
        #[table_model(default = false)]
        #[infallible]
        aliquoted_from_model_archived: bool,
        /// Archive state of the row named by `aliquoted_into_model_id`.
        #[table_model(default = false)]
        #[infallible]
        aliquoted_into_model_archived: bool,
        /// Archive state of the row named by `aliquot_model_id`.
        #[table_model(default = false)]
        #[infallible]
        aliquot_model_archived: bool,
        /// Archive state of the row named by `aliquoted_with_model_id`.
        #[table_model(default = false)]
        #[infallible]
        aliquoted_with_model_archived: bool,
        /// Archive state of the row named by `source_sample_model_id`.
        #[table_model(default = false)]
        #[infallible]
        source_sample_model_archived: bool,
    }
    ::diesel_builders::prelude::unique_index!(
        aliquoting_procedure_templates::id,
        aliquoting_procedure_templates::archived
    );
    impl ::diesel_builders::ValidateColumn<aliquoting_procedure_templates::volume_l>
        for <aliquoting_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(volume_l: &f64) -> Result<(), Self::Error> {
            use diesel::Column;
            if volume_l <= &0f64 {
                return Err(
                    ::validation_errors::ValidationError::strictly_greater_than_value(
                        <crate::aliquoting_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::aliquoting_procedure_templates::volume_l::NAME,
                        0f64,
                    ),
                );
            }
            if volume_l.partial_cmp(&f64::INFINITY).is_none_or(::core::cmp::Ordering::is_ge) {
                return Err(
                    ::validation_errors::ValidationError::check_violated(
                        <crate::aliquoting_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        &[crate::aliquoting_procedure_templates::volume_l::NAME],
                        "volume_l < 'Infinity'::DOUBLE PRECISION",
                    ),
                );
            }
            Ok(())
        }
    }
    impl
        ::diesel_builders::ValidateColumn<
            aliquoting_procedure_templates::volume_l_tolerance_percentage,
        > for <aliquoting_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(volume_l_tolerance_percentage: &f64) -> Result<(), Self::Error> {
            use diesel::Column;
            if volume_l_tolerance_percentage <= &0f64 {
                return Err(
                    ::validation_errors::ValidationError::strictly_greater_than_value(
                        <crate::aliquoting_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::aliquoting_procedure_templates::volume_l_tolerance_percentage::NAME,
                        0f64,
                    ),
                );
            }
            if volume_l_tolerance_percentage > &100f64 {
                return Err(
                    ::validation_errors::ValidationError::smaller_than_value(
                        <crate::aliquoting_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::aliquoting_procedure_templates::volume_l_tolerance_percentage::NAME,
                        100f64,
                    ),
                );
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateRecord<aliquoting_procedure_templates::table>
        for <aliquoting_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
    {
        #[inline]
        fn validate_record(&self) -> Result<(), ::validation_errors::ValidationError> {
            use diesel::Column;
            if let (Some(aliquoted_from_slot_id), Some(aliquoted_into_slot_id)) = (
                <Self as diesel_builders::MayGetColumn<
                    aliquoting_procedure_templates::aliquoted_from_slot_id,
                >>::may_get_column_ref(self),
                <Self as diesel_builders::MayGetColumn<
                    aliquoting_procedure_templates::aliquoted_into_slot_id,
                >>::may_get_column_ref(self),
            ) && aliquoted_from_slot_id == aliquoted_into_slot_id
            {
                return Err(
                    ::validation_errors::ValidationError::equal(
                        <crate::aliquoting_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::aliquoting_procedure_templates::aliquoted_from_slot_id::NAME,
                        crate::aliquoting_procedure_templates::aliquoted_into_slot_id::NAME,
                    ),
                );
            }
            Ok(())
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::procedure_templates::id>
        for AliquotingProcedureTemplate
    {
        fn get_column_ref(
            &self,
        ) -> &<aliquoting_procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for AliquotingProcedureTemplate {
        fn get_column_ref(
            &self,
        ) -> &<aliquoting_procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    /// Shape of `aliquoting_procedure_templates` the procedure runner renders.
    pub const ALIQUOTING_PROCEDURE_TEMPLATES_SHAPE: procedure_like::shape::TemplateShape =
        procedure_like::shape::TemplateShape {
            table_name: "aliquoting_procedure_templates",
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
                    name: "aliquoted_from",
                    role: procedure_like::shape::SlotRole::Input,
                    model_table_name: "volumetric_container_models",
                    slot_column: "aliquoted_from_slot_id",
                    model_column: "aliquoted_from_model_id",
                    placed_into: None,
                    retrieved_from: None,
                },
                procedure_like::shape::SlotShape {
                    name: "aliquoted_into",
                    role: procedure_like::shape::SlotRole::Input,
                    model_table_name: "volumetric_container_models",
                    slot_column: "aliquoted_into_slot_id",
                    model_column: "aliquoted_into_model_id",
                    placed_into: None,
                    retrieved_from: None,
                },
                procedure_like::shape::SlotShape {
                    name: "aliquoted_with",
                    role: procedure_like::shape::SlotRole::Instrument,
                    model_table_name: "volume_measuring_device_models",
                    slot_column: "aliquoted_with_slot_id",
                    model_column: "aliquoted_with_model_id",
                    placed_into: None,
                    retrieved_from: None,
                },
                procedure_like::shape::SlotShape {
                    name: "aliquot",
                    role: procedure_like::shape::SlotRole::Produced,
                    model_table_name: "sample_models",
                    slot_column: "aliquot_slot_id",
                    model_column: "aliquot_model_id",
                    placed_into: Some("aliquoted_into"),
                    retrieved_from: None,
                },
            ],
        };
    ::diesel::allow_tables_to_appear_in_same_query!(
        aliquoting_procedure_templates,
        ::aps_resources::procedure_template_slots
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        aliquoting_procedure_templates,
        ::aps_resources::procedure_templates
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        aliquoting_procedure_templates,
        ::aps_resources::resources
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        aliquoting_procedure_templates,
        ::aps_sample_models::sample_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        aliquoting_procedure_templates,
        ::aps_sample_models::volume_measuring_device_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        aliquoting_procedure_templates,
        ::aps_physical_asset_models::volumetric_container_models
    );
}
pub use aliquoting_procedure_templates_model::*;
pub mod aliquoting_procedures_model {
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
    /// Recorded execution of a aliquoting template.
    #[table_model(ancestors(aps_resources::resources, aps_procedures::procedures))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[table_model(record_error = ::validation_errors::ValidationError)]
    #[table_model(foreign_key((id), (::aps_procedures::procedures::id)))]
    #[table_model(
        foreign_key(
            (aliquoting_procedure_template_id),
            (::aps_sample_models::aliquoting_procedure_templates::id)
        )
    )]
    #[table_model(
        foreign_key(
            (aliquoting_procedure_template_id, aliquoting_procedure_template_archived),
            (
                ::aps_sample_models::aliquoting_procedure_templates::id,
                ::aps_sample_models::aliquoting_procedure_templates::archived
            )
        )
    )]
    #[diesel(table_name = aliquoting_procedures)]
    pub struct AliquotingProcedure {
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
        aliquoting_procedure_template_id: ::rosetta_uuid::Uuid,
        /// Execution status copied from the parent procedure.
        #[same_as(aps_procedures::procedures::status_id)]
        #[table_model(default = "planned")]
        #[infallible]
        status_id: String,
        /// Measured volume l.
        volume_l: Option<f64>,
        /// Archive state inherited from this resource.
        #[same_as(aps_procedures::procedures::archived)]
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
        /// Archive state of the row named by
        /// `aliquoting_procedure_template_id`.
        #[table_model(default = false)]
        #[infallible]
        aliquoting_procedure_template_archived: bool,
    }
    impl ::diesel_builders::ValidateColumn<aliquoting_procedures::volume_l>
        for <aliquoting_procedures::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(volume_l: &f64) -> Result<(), Self::Error> {
            use diesel::Column;
            if volume_l <= &0f64 {
                return Err(
                    ::validation_errors::ValidationError::strictly_greater_than_value(
                        <crate::aliquoting_procedures::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::aliquoting_procedures::volume_l::NAME,
                        0f64,
                    ),
                );
            }
            if volume_l.partial_cmp(&f64::INFINITY).is_none_or(::core::cmp::Ordering::is_ge) {
                return Err(
                    ::validation_errors::ValidationError::check_violated(
                        <crate::aliquoting_procedures::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        &[crate::aliquoting_procedures::volume_l::NAME],
                        "volume_l < 'Infinity'::DOUBLE PRECISION",
                    ),
                );
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateRecord<aliquoting_procedures::table>
        for <aliquoting_procedures::table as ::diesel_builders::TableExt>::NewValues
    {
        #[inline]
        fn validate_record(&self) -> Result<(), ::validation_errors::ValidationError> {
            use diesel::Column;
            if let (Some(status_id), Some(volume_l)) = (
                <Self as diesel_builders::MayGetColumn<
                    aliquoting_procedures::status_id,
                >>::may_get_column_ref(self),
                <Self as diesel_builders::MayGetColumn<
                    aliquoting_procedures::volume_l,
                >>::may_get_column_ref(self),
            ) && !((status_id.as_str() != "completed") || (*volume_l).is_some())
            {
                return Err(
                    ::validation_errors::ValidationError::check_violated(
                        <crate::aliquoting_procedures::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        &[
                            crate::aliquoting_procedures::status_id::NAME,
                            crate::aliquoting_procedures::volume_l::NAME,
                        ],
                        "status_id <> 'completed' OR volume_l IS NOT NULL",
                    ),
                );
            }
            Ok(())
        }
    }
    impl ::diesel_builders::GetColumn<aps_procedures::procedures::id> for AliquotingProcedure {
        fn get_column_ref(
            &self,
        ) -> &<aliquoting_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for AliquotingProcedure {
        fn get_column_ref(
            &self,
        ) -> &<aliquoting_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl procedure_like::ProcedureTableLike for aliquoting_procedures::table {
        type ProcedureTemplateTable = aps_sample_models::aliquoting_procedure_templates::table;
    }
    /// Shape of `aliquoting_procedures` the procedure runner renders.
    pub const ALIQUOTING_PROCEDURES_SHAPE: procedure_like::shape::ProcedureShape =
        procedure_like::shape::ProcedureShape {
            table_name: "aliquoting_procedures",
            template_table_name: "aliquoting_procedure_templates",
            measurements: &[procedure_like::shape::ColumnShape {
                name: "volume_l",
                label: "Measured volume l.",
                sql_type: procedure_like::shape::SqlType::DoublePrecision,
                unit: Some(procedure_like::shape::Unit::Liter),
                required: true,
            }],
        };
    ::diesel::allow_tables_to_appear_in_same_query!(
        aliquoting_procedures,
        ::aps_sample_models::aliquoting_procedure_templates
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        aliquoting_procedures,
        ::aps_procedures::procedures
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        aliquoting_procedures,
        ::aps_resources::resources
    );
}
pub use aliquoting_procedures_model::*;
pub mod harvesting_procedure_templates_model {
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
    /// Typed slots and parameters for harvesting.
    #[table_model(ancestors(aps_resources::resources, aps_resources::procedure_templates))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_resources::ProcedureTemplate, foreign_key = id))]
    #[table_model(foreign_key((id), (::aps_resources::procedure_templates::id)))]
    #[table_model(
        foreign_key(
            (sample_source_model_id),
            (::aps_sample_models::sample_source_models::id)
        )
    )]
    #[table_model(
        foreign_key(
            (
                sample_source_slot_id,
                id,
                sample_source_model_id,
                sample_source_slot_role_id,
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
            (sample_slot_id, id, sample_model_id, sample_slot_role_id, organization_id),
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
            (sample_model_id, source_family_model_id),
            (
                ::aps_sample_models::sample_models::id,
                ::aps_sample_models::sample_models::sample_source_model_id
            )
        )
    )]
    #[table_model(
        foreign_key(
            (sample_source_model_id, source_family_model_id),
            (
                ::aps_resources::asset_model_ancestors::descendant_model_id,
                ::aps_resources::asset_model_ancestors::ancestor_model_id
            )
        )
    )]
    #[table_model(
        foreign_key(
            (sample_source_model_id, sample_source_model_archived),
            (
                ::aps_sample_models::sample_source_models::id,
                ::aps_sample_models::sample_source_models::archived
            )
        )
    )]
    #[table_model(
        foreign_key(
            (sample_model_id, sample_model_archived),
            (
                ::aps_sample_models::sample_models::id,
                ::aps_sample_models::sample_models::archived
            )
        )
    )]
    #[table_model(
        foreign_key(
            (source_family_model_id, source_family_model_archived),
            (
                ::aps_sample_models::sample_source_models::id,
                ::aps_sample_models::sample_source_models::archived
            )
        )
    )]
    #[diesel(table_name = harvesting_procedure_templates)]
    pub struct HarvestingProcedureTemplate {
        /// Identity shared with the parent template.
        #[same_as(
            aps_resources::procedure_template_slots::procedure_template_id,
            (sample_source_slot_id, sample_slot_id)
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing the parent resource.
        #[same_as(aps_resources::procedure_templates::organization_id)]
        #[same_as(
            aps_resources::procedure_template_slots::organization_id,
            (sample_source_slot_id, sample_slot_id)
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Slot declaring the `sample_source` assets.
        #[mandatory(aps_resources::procedure_template_slots)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        sample_source_slot_id: ::rosetta_uuid::Uuid,
        /// Admissible model for the `sample_source` slot.
        #[same_as(aps_resources::procedure_template_slots::asset_model_id, sample_source_slot_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        sample_source_model_id: ::rosetta_uuid::Uuid,
        /// Required role of the `sample_source` slot.
        #[table_model(generated)]
        sample_source_slot_role_id: String,
        /// Slot declaring the `sample` assets.
        #[mandatory(aps_resources::procedure_template_slots)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        sample_slot_id: ::rosetta_uuid::Uuid,
        /// Admissible model for the `sample` slot.
        #[same_as(aps_resources::procedure_template_slots::asset_model_id, sample_slot_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        sample_model_id: ::rosetta_uuid::Uuid,
        /// Source family required by the sample model.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        source_family_model_id: ::rosetta_uuid::Uuid,
        /// Required role of the `sample` slot.
        #[table_model(generated)]
        sample_slot_role_id: String,
        /// Archive state inherited from this resource.
        #[same_as(aps_resources::procedure_templates::archived)]
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
        /// Archive state of the row named by `sample_source_model_id`.
        #[table_model(default = false)]
        #[infallible]
        sample_source_model_archived: bool,
        /// Archive state of the row named by `sample_model_id`.
        #[table_model(default = false)]
        #[infallible]
        sample_model_archived: bool,
        /// Archive state of the row named by `source_family_model_id`.
        #[table_model(default = false)]
        #[infallible]
        source_family_model_archived: bool,
    }
    ::diesel_builders::prelude::unique_index!(
        harvesting_procedure_templates::id,
        harvesting_procedure_templates::archived
    );
    impl ::diesel_builders::GetColumn<aps_resources::procedure_templates::id>
        for HarvestingProcedureTemplate
    {
        fn get_column_ref(
            &self,
        ) -> &<harvesting_procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for HarvestingProcedureTemplate {
        fn get_column_ref(
            &self,
        ) -> &<harvesting_procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    /// Shape of `harvesting_procedure_templates` the procedure runner renders.
    pub const HARVESTING_PROCEDURE_TEMPLATES_SHAPE: procedure_like::shape::TemplateShape =
        procedure_like::shape::TemplateShape {
            table_name: "harvesting_procedure_templates",
            setpoints: &[],
            slots: &[
                procedure_like::shape::SlotShape {
                    name: "sample_source",
                    role: procedure_like::shape::SlotRole::Input,
                    model_table_name: "sample_source_models",
                    slot_column: "sample_source_slot_id",
                    model_column: "sample_source_model_id",
                    placed_into: None,
                    retrieved_from: None,
                },
                procedure_like::shape::SlotShape {
                    name: "sample",
                    role: procedure_like::shape::SlotRole::Produced,
                    model_table_name: "sample_models",
                    slot_column: "sample_slot_id",
                    model_column: "sample_model_id",
                    placed_into: None,
                    retrieved_from: None,
                },
            ],
        };
    ::diesel::allow_tables_to_appear_in_same_query!(
        harvesting_procedure_templates,
        ::aps_resources::asset_model_ancestors
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        harvesting_procedure_templates,
        ::aps_resources::procedure_template_slots
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        harvesting_procedure_templates,
        ::aps_resources::procedure_templates
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        harvesting_procedure_templates,
        ::aps_resources::resources
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        harvesting_procedure_templates,
        ::aps_sample_models::sample_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        harvesting_procedure_templates,
        ::aps_sample_models::sample_source_models
    );
}
pub use harvesting_procedure_templates_model::*;
pub mod harvesting_procedures_model {
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
        ::diesel_builders::prelude::TableModel,
    )]
    /// Recorded execution of a harvesting template.
    #[table_model(ancestors(aps_resources::resources, aps_procedures::procedures))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[table_model(foreign_key((id), (::aps_procedures::procedures::id)))]
    #[table_model(
        foreign_key(
            (harvesting_procedure_template_id),
            (::aps_sample_models::harvesting_procedure_templates::id)
        )
    )]
    #[table_model(
        foreign_key(
            (harvesting_procedure_template_id, harvesting_procedure_template_archived),
            (
                ::aps_sample_models::harvesting_procedure_templates::id,
                ::aps_sample_models::harvesting_procedure_templates::archived
            )
        )
    )]
    #[diesel(table_name = harvesting_procedures)]
    pub struct HarvestingProcedure {
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
        harvesting_procedure_template_id: ::rosetta_uuid::Uuid,
        /// Execution status copied from the parent procedure.
        #[same_as(aps_procedures::procedures::status_id)]
        #[table_model(default = "planned")]
        #[infallible]
        status_id: String,
        /// Archive state inherited from this resource.
        #[same_as(aps_procedures::procedures::archived)]
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
        /// Archive state of the row named by
        /// `harvesting_procedure_template_id`.
        #[table_model(default = false)]
        #[infallible]
        harvesting_procedure_template_archived: bool,
    }
    impl ::diesel_builders::GetColumn<aps_procedures::procedures::id> for HarvestingProcedure {
        fn get_column_ref(
            &self,
        ) -> &<harvesting_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for HarvestingProcedure {
        fn get_column_ref(
            &self,
        ) -> &<harvesting_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl procedure_like::ProcedureTableLike for harvesting_procedures::table {
        type ProcedureTemplateTable = aps_sample_models::harvesting_procedure_templates::table;
    }
    /// Shape of `harvesting_procedures` the procedure runner renders.
    pub const HARVESTING_PROCEDURES_SHAPE: procedure_like::shape::ProcedureShape =
        procedure_like::shape::ProcedureShape {
            table_name: "harvesting_procedures",
            template_table_name: "harvesting_procedure_templates",
            measurements: &[],
        };
    ::diesel::allow_tables_to_appear_in_same_query!(
        harvesting_procedures,
        ::aps_sample_models::harvesting_procedure_templates
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        harvesting_procedures,
        ::aps_procedures::procedures
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        harvesting_procedures,
        ::aps_resources::resources
    );
}
pub use harvesting_procedures_model::*;
pub mod packaging_models_model {
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
    /// Catalog of physical asset models used as packaging.
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
    #[diesel(table_name = packaging_models)]
    pub struct PackagingModel {
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
    ::diesel_builders::prelude::unique_index!(packaging_models::id, packaging_models::archived);
    impl ::diesel_builders::GetColumn<aps_resources::asset_models::id> for PackagingModel {
        fn get_column_ref(
            &self,
        ) -> &<packaging_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_physical_asset_models::physical_asset_models::id>
        for PackagingModel
    {
        fn get_column_ref(
            &self,
        ) -> &<packaging_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for PackagingModel {
        fn get_column_ref(
            &self,
        ) -> &<packaging_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    /// Shape of `packaging_models` the procedure runner renders.
    pub const PACKAGING_MODELS_SHAPE: procedure_like::shape::AssetModelShape =
        procedure_like::shape::AssetModelShape {
            table_name: "packaging_models",
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
        packaging_models,
        ::aps_resources::asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        packaging_models,
        ::aps_physical_asset_models::physical_asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(packaging_models, ::aps_resources::resources);
}
pub use packaging_models_model::*;
pub mod packaging_procedure_templates_model {
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
    /// Typed slots and parameters for packaging.
    #[table_model(ancestors(aps_resources::resources, aps_resources::procedure_templates))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_resources::ProcedureTemplate, foreign_key = id))]
    #[table_model(foreign_key((id), (::aps_resources::procedure_templates::id)))]
    #[table_model(
        foreign_key(
            (packaged_with_model_id),
            (::aps_sample_models::packaging_models::id)
        )
    )]
    #[table_model(
        foreign_key((sample_model_id), (::aps_sample_models::sample_models::id))
    )]
    #[table_model(
        foreign_key(
            (
                packaged_with_slot_id,
                id,
                packaged_with_model_id,
                packaged_with_slot_role_id,
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
            (sample_slot_id, id, sample_model_id, sample_slot_role_id, organization_id),
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
            (packaged_with_model_id, packaged_with_model_archived),
            (
                ::aps_sample_models::packaging_models::id,
                ::aps_sample_models::packaging_models::archived
            )
        )
    )]
    #[table_model(
        foreign_key(
            (sample_model_id, sample_model_archived),
            (
                ::aps_sample_models::sample_models::id,
                ::aps_sample_models::sample_models::archived
            )
        )
    )]
    #[diesel(table_name = packaging_procedure_templates)]
    pub struct PackagingProcedureTemplate {
        /// Identity shared with the parent template.
        #[same_as(
            aps_resources::procedure_template_slots::procedure_template_id,
            (packaged_with_slot_id, sample_slot_id)
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing the parent resource.
        #[same_as(aps_resources::procedure_templates::organization_id)]
        #[same_as(
            aps_resources::procedure_template_slots::organization_id,
            (packaged_with_slot_id, sample_slot_id)
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Slot declaring the `packaged_with` assets.
        #[mandatory(aps_resources::procedure_template_slots)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        packaged_with_slot_id: ::rosetta_uuid::Uuid,
        /// Admissible model for the `packaged_with` slot.
        #[same_as(aps_resources::procedure_template_slots::asset_model_id, packaged_with_slot_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        packaged_with_model_id: ::rosetta_uuid::Uuid,
        /// Required role of the `packaged_with` slot.
        #[table_model(generated)]
        packaged_with_slot_role_id: String,
        /// Slot declaring the `sample` assets.
        #[mandatory(aps_resources::procedure_template_slots)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        sample_slot_id: ::rosetta_uuid::Uuid,
        /// Admissible model for the `sample` slot.
        #[same_as(aps_resources::procedure_template_slots::asset_model_id, sample_slot_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        sample_model_id: ::rosetta_uuid::Uuid,
        /// Required role of the `sample` slot.
        #[table_model(generated)]
        sample_slot_role_id: String,
        /// Archive state inherited from this resource.
        #[same_as(aps_resources::procedure_templates::archived)]
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
        /// Archive state of the row named by `packaged_with_model_id`.
        #[table_model(default = false)]
        #[infallible]
        packaged_with_model_archived: bool,
        /// Archive state of the row named by `sample_model_id`.
        #[table_model(default = false)]
        #[infallible]
        sample_model_archived: bool,
    }
    ::diesel_builders::prelude::unique_index!(
        packaging_procedure_templates::id,
        packaging_procedure_templates::archived
    );
    impl ::diesel_builders::GetColumn<aps_resources::procedure_templates::id>
        for PackagingProcedureTemplate
    {
        fn get_column_ref(
            &self,
        ) -> &<packaging_procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for PackagingProcedureTemplate {
        fn get_column_ref(
            &self,
        ) -> &<packaging_procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    /// Shape of `packaging_procedure_templates` the procedure runner renders.
    pub const PACKAGING_PROCEDURE_TEMPLATES_SHAPE: procedure_like::shape::TemplateShape =
        procedure_like::shape::TemplateShape {
            table_name: "packaging_procedure_templates",
            setpoints: &[],
            slots: &[
                procedure_like::shape::SlotShape {
                    name: "packaged_with",
                    role: procedure_like::shape::SlotRole::Input,
                    model_table_name: "packaging_models",
                    slot_column: "packaged_with_slot_id",
                    model_column: "packaged_with_model_id",
                    placed_into: None,
                    retrieved_from: None,
                },
                procedure_like::shape::SlotShape {
                    name: "sample",
                    role: procedure_like::shape::SlotRole::Input,
                    model_table_name: "sample_models",
                    slot_column: "sample_slot_id",
                    model_column: "sample_model_id",
                    placed_into: None,
                    retrieved_from: None,
                },
            ],
        };
    ::diesel::allow_tables_to_appear_in_same_query!(
        packaging_procedure_templates,
        ::aps_sample_models::packaging_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        packaging_procedure_templates,
        ::aps_resources::procedure_template_slots
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        packaging_procedure_templates,
        ::aps_resources::procedure_templates
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        packaging_procedure_templates,
        ::aps_resources::resources
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        packaging_procedure_templates,
        ::aps_sample_models::sample_models
    );
}
pub use packaging_procedure_templates_model::*;
pub mod packaging_procedures_model {
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
        ::diesel_builders::prelude::TableModel,
    )]
    /// Recorded execution of a packaging template.
    #[table_model(ancestors(aps_resources::resources, aps_procedures::procedures))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[table_model(foreign_key((id), (::aps_procedures::procedures::id)))]
    #[table_model(
        foreign_key(
            (packaging_procedure_template_id),
            (::aps_sample_models::packaging_procedure_templates::id)
        )
    )]
    #[table_model(
        foreign_key(
            (packaging_procedure_template_id, packaging_procedure_template_archived),
            (
                ::aps_sample_models::packaging_procedure_templates::id,
                ::aps_sample_models::packaging_procedure_templates::archived
            )
        )
    )]
    #[diesel(table_name = packaging_procedures)]
    pub struct PackagingProcedure {
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
        packaging_procedure_template_id: ::rosetta_uuid::Uuid,
        /// Execution status copied from the parent procedure.
        #[same_as(aps_procedures::procedures::status_id)]
        #[table_model(default = "planned")]
        #[infallible]
        status_id: String,
        /// Archive state inherited from this resource.
        #[same_as(aps_procedures::procedures::archived)]
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
        /// Archive state of the row named by `packaging_procedure_template_id`.
        #[table_model(default = false)]
        #[infallible]
        packaging_procedure_template_archived: bool,
    }
    impl ::diesel_builders::GetColumn<aps_procedures::procedures::id> for PackagingProcedure {
        fn get_column_ref(
            &self,
        ) -> &<packaging_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for PackagingProcedure {
        fn get_column_ref(
            &self,
        ) -> &<packaging_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl procedure_like::ProcedureTableLike for packaging_procedures::table {
        type ProcedureTemplateTable = aps_sample_models::packaging_procedure_templates::table;
    }
    /// Shape of `packaging_procedures` the procedure runner renders.
    pub const PACKAGING_PROCEDURES_SHAPE: procedure_like::shape::ProcedureShape =
        procedure_like::shape::ProcedureShape {
            table_name: "packaging_procedures",
            template_table_name: "packaging_procedure_templates",
            measurements: &[],
        };
    ::diesel::allow_tables_to_appear_in_same_query!(
        packaging_procedures,
        ::aps_sample_models::packaging_procedure_templates
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        packaging_procedures,
        ::aps_procedures::procedures
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        packaging_procedures,
        ::aps_resources::resources
    );
}
pub use packaging_procedures_model::*;
pub mod pouring_procedure_templates_model {
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
    /// Typed slots and parameters for pouring.
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
            (measured_with_model_id),
            (::aps_sample_models::volume_measuring_device_models::id)
        )
    )]
    #[table_model(
        foreign_key(
            (poured_from_model_id),
            (::aps_physical_asset_models::volumetric_container_models::id)
        )
    )]
    #[table_model(
        foreign_key(
            (poured_into_model_id),
            (::aps_physical_asset_models::volumetric_container_models::id)
        )
    )]
    #[table_model(
        foreign_key((poured_sample_model_id), (::aps_sample_models::sample_models::id))
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
                measured_with_slot_id,
                id,
                measured_with_model_id,
                measured_with_slot_role_id,
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
                poured_from_slot_id,
                id,
                poured_from_model_id,
                poured_from_slot_role_id,
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
                poured_into_slot_id,
                id,
                poured_into_model_id,
                poured_into_slot_role_id,
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
                poured_sample_slot_id,
                id,
                poured_sample_model_id,
                poured_sample_slot_role_id,
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
            (poured_sample_slot_id, id, poured_into_slot_id, organization_id),
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
            (measured_with_model_id, measured_with_model_archived),
            (
                ::aps_sample_models::volume_measuring_device_models::id,
                ::aps_sample_models::volume_measuring_device_models::archived
            )
        )
    )]
    #[table_model(
        foreign_key(
            (poured_from_model_id, poured_from_model_archived),
            (
                ::aps_physical_asset_models::volumetric_container_models::id,
                ::aps_physical_asset_models::volumetric_container_models::archived
            )
        )
    )]
    #[table_model(
        foreign_key(
            (poured_into_model_id, poured_into_model_archived),
            (
                ::aps_physical_asset_models::volumetric_container_models::id,
                ::aps_physical_asset_models::volumetric_container_models::archived
            )
        )
    )]
    #[table_model(
        foreign_key(
            (poured_sample_model_id, poured_sample_model_archived),
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
    #[diesel(table_name = pouring_procedure_templates)]
    pub struct PouringProcedureTemplate {
        /// Identity shared with the parent template.
        #[same_as(
            aps_resources::procedure_template_slots::procedure_template_id,
            (
                source_sample_slot_id,
                measured_with_slot_id,
                poured_from_slot_id,
                poured_into_slot_id,
                poured_sample_slot_id
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
                measured_with_slot_id,
                poured_from_slot_id,
                poured_into_slot_id,
                poured_sample_slot_id
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
        /// Slot declaring the `measured_with` assets.
        #[mandatory(aps_resources::procedure_template_slots)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        measured_with_slot_id: ::rosetta_uuid::Uuid,
        /// Admissible model for the `measured_with` slot.
        #[same_as(aps_resources::procedure_template_slots::asset_model_id, measured_with_slot_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        measured_with_model_id: ::rosetta_uuid::Uuid,
        /// Required role of the `measured_with` slot.
        #[table_model(generated)]
        measured_with_slot_role_id: String,
        /// Slot declaring the `poured_from` assets.
        #[mandatory(aps_resources::procedure_template_slots)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        poured_from_slot_id: ::rosetta_uuid::Uuid,
        /// Admissible model for the `poured_from` slot.
        #[same_as(aps_resources::procedure_template_slots::asset_model_id, poured_from_slot_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        poured_from_model_id: ::rosetta_uuid::Uuid,
        /// Required role of the `poured_from` slot.
        #[table_model(generated)]
        poured_from_slot_role_id: String,
        /// Slot declaring the `poured_into` assets.
        #[same_as(
            aps_resources::procedure_template_slots::placed_into_slot_id,
            poured_sample_slot_id
        )]
        #[mandatory(aps_resources::procedure_template_slots)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        poured_into_slot_id: ::rosetta_uuid::Uuid,
        /// Admissible model for the `poured_into` slot.
        #[same_as(aps_resources::procedure_template_slots::asset_model_id, poured_into_slot_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        poured_into_model_id: ::rosetta_uuid::Uuid,
        /// Required role of the `poured_into` slot.
        #[table_model(generated)]
        poured_into_slot_role_id: String,
        /// Slot declaring the produced `poured_sample` sample.
        #[mandatory(aps_resources::procedure_template_slots)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        poured_sample_slot_id: ::rosetta_uuid::Uuid,
        /// Admissible model for the `poured_sample` sample.
        #[same_as(aps_resources::procedure_template_slots::asset_model_id, poured_sample_slot_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        poured_sample_model_id: ::rosetta_uuid::Uuid,
        /// Required role of the `poured_sample` slot.
        #[table_model(generated)]
        poured_sample_slot_role_id: String,
        /// Volume in liters.
        volume_l: f64,
        /// Allowed volume deviation as a percentage of the target.
        #[table_model(default = 5f64)]
        volume_l_tolerance_percentage: f64,
        /// Archive state inherited from this resource.
        #[same_as(aps_resources::procedure_templates::archived)]
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
        /// Archive state of the row named by `measured_with_model_id`.
        #[table_model(default = false)]
        #[infallible]
        measured_with_model_archived: bool,
        /// Archive state of the row named by `poured_from_model_id`.
        #[table_model(default = false)]
        #[infallible]
        poured_from_model_archived: bool,
        /// Archive state of the row named by `poured_into_model_id`.
        #[table_model(default = false)]
        #[infallible]
        poured_into_model_archived: bool,
        /// Archive state of the row named by `poured_sample_model_id`.
        #[table_model(default = false)]
        #[infallible]
        poured_sample_model_archived: bool,
        /// Archive state of the row named by `source_sample_model_id`.
        #[table_model(default = false)]
        #[infallible]
        source_sample_model_archived: bool,
    }
    ::diesel_builders::prelude::unique_index!(
        pouring_procedure_templates::id,
        pouring_procedure_templates::archived
    );
    impl ::diesel_builders::ValidateColumn<pouring_procedure_templates::volume_l>
        for <pouring_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(volume_l: &f64) -> Result<(), Self::Error> {
            use diesel::Column;
            if volume_l <= &0f64 {
                return Err(
                    ::validation_errors::ValidationError::strictly_greater_than_value(
                        <crate::pouring_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::pouring_procedure_templates::volume_l::NAME,
                        0f64,
                    ),
                );
            }
            if volume_l.partial_cmp(&f64::INFINITY).is_none_or(::core::cmp::Ordering::is_ge) {
                return Err(
                    ::validation_errors::ValidationError::check_violated(
                        <crate::pouring_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        &[crate::pouring_procedure_templates::volume_l::NAME],
                        "volume_l < 'Infinity'::DOUBLE PRECISION",
                    ),
                );
            }
            Ok(())
        }
    }
    impl
        ::diesel_builders::ValidateColumn<
            pouring_procedure_templates::volume_l_tolerance_percentage,
        > for <pouring_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(volume_l_tolerance_percentage: &f64) -> Result<(), Self::Error> {
            use diesel::Column;
            if volume_l_tolerance_percentage <= &0f64 {
                return Err(
                    ::validation_errors::ValidationError::strictly_greater_than_value(
                        <crate::pouring_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::pouring_procedure_templates::volume_l_tolerance_percentage::NAME,
                        0f64,
                    ),
                );
            }
            if volume_l_tolerance_percentage > &100f64 {
                return Err(
                    ::validation_errors::ValidationError::smaller_than_value(
                        <crate::pouring_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::pouring_procedure_templates::volume_l_tolerance_percentage::NAME,
                        100f64,
                    ),
                );
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateRecord<pouring_procedure_templates::table>
        for <pouring_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
    {
        #[inline]
        fn validate_record(&self) -> Result<(), ::validation_errors::ValidationError> {
            use diesel::Column;
            if let (Some(poured_from_slot_id), Some(poured_into_slot_id)) = (
                <Self as diesel_builders::MayGetColumn<
                    pouring_procedure_templates::poured_from_slot_id,
                >>::may_get_column_ref(self),
                <Self as diesel_builders::MayGetColumn<
                    pouring_procedure_templates::poured_into_slot_id,
                >>::may_get_column_ref(self),
            ) && poured_from_slot_id == poured_into_slot_id
            {
                return Err(
                    ::validation_errors::ValidationError::equal(
                        <crate::pouring_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::pouring_procedure_templates::poured_from_slot_id::NAME,
                        crate::pouring_procedure_templates::poured_into_slot_id::NAME,
                    ),
                );
            }
            Ok(())
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::procedure_templates::id>
        for PouringProcedureTemplate
    {
        fn get_column_ref(
            &self,
        ) -> &<pouring_procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for PouringProcedureTemplate {
        fn get_column_ref(
            &self,
        ) -> &<pouring_procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    /// Shape of `pouring_procedure_templates` the procedure runner renders.
    pub const POURING_PROCEDURE_TEMPLATES_SHAPE: procedure_like::shape::TemplateShape =
        procedure_like::shape::TemplateShape {
            table_name: "pouring_procedure_templates",
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
                    name: "measured_with",
                    role: procedure_like::shape::SlotRole::Instrument,
                    model_table_name: "volume_measuring_device_models",
                    slot_column: "measured_with_slot_id",
                    model_column: "measured_with_model_id",
                    placed_into: None,
                    retrieved_from: None,
                },
                procedure_like::shape::SlotShape {
                    name: "poured_from",
                    role: procedure_like::shape::SlotRole::Input,
                    model_table_name: "volumetric_container_models",
                    slot_column: "poured_from_slot_id",
                    model_column: "poured_from_model_id",
                    placed_into: None,
                    retrieved_from: None,
                },
                procedure_like::shape::SlotShape {
                    name: "poured_into",
                    role: procedure_like::shape::SlotRole::Input,
                    model_table_name: "volumetric_container_models",
                    slot_column: "poured_into_slot_id",
                    model_column: "poured_into_model_id",
                    placed_into: None,
                    retrieved_from: None,
                },
                procedure_like::shape::SlotShape {
                    name: "poured_sample",
                    role: procedure_like::shape::SlotRole::Produced,
                    model_table_name: "sample_models",
                    slot_column: "poured_sample_slot_id",
                    model_column: "poured_sample_model_id",
                    placed_into: Some("poured_into"),
                    retrieved_from: None,
                },
            ],
        };
    ::diesel::allow_tables_to_appear_in_same_query!(
        pouring_procedure_templates,
        ::aps_resources::procedure_template_slots
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        pouring_procedure_templates,
        ::aps_resources::procedure_templates
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        pouring_procedure_templates,
        ::aps_resources::resources
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        pouring_procedure_templates,
        ::aps_sample_models::sample_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        pouring_procedure_templates,
        ::aps_sample_models::volume_measuring_device_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        pouring_procedure_templates,
        ::aps_physical_asset_models::volumetric_container_models
    );
}
pub use pouring_procedure_templates_model::*;
pub mod pouring_procedures_model {
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
    /// Recorded execution of a pouring template.
    #[table_model(ancestors(aps_resources::resources, aps_procedures::procedures))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[table_model(record_error = ::validation_errors::ValidationError)]
    #[table_model(foreign_key((id), (::aps_procedures::procedures::id)))]
    #[table_model(
        foreign_key(
            (pouring_procedure_template_id),
            (::aps_sample_models::pouring_procedure_templates::id)
        )
    )]
    #[table_model(
        foreign_key(
            (pouring_procedure_template_id, pouring_procedure_template_archived),
            (
                ::aps_sample_models::pouring_procedure_templates::id,
                ::aps_sample_models::pouring_procedure_templates::archived
            )
        )
    )]
    #[diesel(table_name = pouring_procedures)]
    pub struct PouringProcedure {
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
        pouring_procedure_template_id: ::rosetta_uuid::Uuid,
        /// Execution status copied from the parent procedure.
        #[same_as(aps_procedures::procedures::status_id)]
        #[table_model(default = "planned")]
        #[infallible]
        status_id: String,
        /// Measured volume l.
        volume_l: Option<f64>,
        /// Archive state inherited from this resource.
        #[same_as(aps_procedures::procedures::archived)]
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
        /// Archive state of the row named by `pouring_procedure_template_id`.
        #[table_model(default = false)]
        #[infallible]
        pouring_procedure_template_archived: bool,
    }
    impl ::diesel_builders::ValidateColumn<pouring_procedures::volume_l>
        for <pouring_procedures::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(volume_l: &f64) -> Result<(), Self::Error> {
            use diesel::Column;
            if volume_l <= &0f64 {
                return Err(::validation_errors::ValidationError::strictly_greater_than_value(
                    <crate::pouring_procedures::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::pouring_procedures::volume_l::NAME,
                    0f64,
                ));
            }
            if volume_l.partial_cmp(&f64::INFINITY).is_none_or(::core::cmp::Ordering::is_ge) {
                return Err(::validation_errors::ValidationError::check_violated(
                    <crate::pouring_procedures::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    &[crate::pouring_procedures::volume_l::NAME],
                    "volume_l < 'Infinity'::DOUBLE PRECISION",
                ));
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateRecord<pouring_procedures::table>
        for <pouring_procedures::table as ::diesel_builders::TableExt>::NewValues
    {
        #[inline]
        fn validate_record(&self) -> Result<(), ::validation_errors::ValidationError> {
            use diesel::Column;
            if let (Some(status_id), Some(volume_l)) = (
                <Self as diesel_builders::MayGetColumn<
                    pouring_procedures::status_id,
                >>::may_get_column_ref(self),
                <Self as diesel_builders::MayGetColumn<
                    pouring_procedures::volume_l,
                >>::may_get_column_ref(self),
            ) && !((status_id.as_str() != "completed") || (*volume_l).is_some())
            {
                return Err(
                    ::validation_errors::ValidationError::check_violated(
                        <crate::pouring_procedures::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        &[
                            crate::pouring_procedures::status_id::NAME,
                            crate::pouring_procedures::volume_l::NAME,
                        ],
                        "status_id <> 'completed' OR volume_l IS NOT NULL",
                    ),
                );
            }
            Ok(())
        }
    }
    impl ::diesel_builders::GetColumn<aps_procedures::procedures::id> for PouringProcedure {
        fn get_column_ref(
            &self,
        ) -> &<pouring_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for PouringProcedure {
        fn get_column_ref(
            &self,
        ) -> &<pouring_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl procedure_like::ProcedureTableLike for pouring_procedures::table {
        type ProcedureTemplateTable = aps_sample_models::pouring_procedure_templates::table;
    }
    /// Shape of `pouring_procedures` the procedure runner renders.
    pub const POURING_PROCEDURES_SHAPE: procedure_like::shape::ProcedureShape =
        procedure_like::shape::ProcedureShape {
            table_name: "pouring_procedures",
            template_table_name: "pouring_procedure_templates",
            measurements: &[procedure_like::shape::ColumnShape {
                name: "volume_l",
                label: "Measured volume l.",
                sql_type: procedure_like::shape::SqlType::DoublePrecision,
                unit: Some(procedure_like::shape::Unit::Liter),
                required: true,
            }],
        };
    ::diesel::allow_tables_to_appear_in_same_query!(
        pouring_procedures,
        ::aps_sample_models::pouring_procedure_templates
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        pouring_procedures,
        ::aps_procedures::procedures
    );
    ::diesel::allow_tables_to_appear_in_same_query!(pouring_procedures, ::aps_resources::resources);
}
pub use pouring_procedures_model::*;
pub mod sample_models_model {
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
    /// Catalog of sample models and the source-model family they require.
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
    #[table_model(
        foreign_key(
            (sample_source_model_id),
            (::aps_sample_models::sample_source_models::id)
        )
    )]
    #[table_model(
        foreign_key(
            (sample_source_model_id, sample_source_model_archived),
            (
                ::aps_sample_models::sample_source_models::id,
                ::aps_sample_models::sample_source_models::archived
            )
        )
    )]
    #[diesel(table_name = sample_models)]
    pub struct SampleModel {
        /// Stable model identifier inherited from `physical_asset_models`.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing this resource.
        #[same_as(aps_physical_asset_models::physical_asset_models::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Source-model family that sample instances of this model must
        /// reference.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        sample_source_model_id: ::rosetta_uuid::Uuid,
        /// Archive state inherited from this resource.
        #[same_as(aps_physical_asset_models::physical_asset_models::archived)]
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
        /// Archive state of the row named by `sample_source_model_id`.
        #[table_model(default = false)]
        #[infallible]
        sample_source_model_archived: bool,
    }
    ::diesel_builders::prelude::unique_index!(
        sample_models::id,
        sample_models::sample_source_model_id
    );
    ::diesel_builders::prelude::unique_index!(sample_models::id, sample_models::archived);
    impl ::diesel_builders::GetColumn<aps_resources::asset_models::id> for SampleModel {
        fn get_column_ref(
            &self,
        ) -> &<sample_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_physical_asset_models::physical_asset_models::id>
        for SampleModel
    {
        fn get_column_ref(
            &self,
        ) -> &<sample_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for SampleModel {
        fn get_column_ref(
            &self,
        ) -> &<sample_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    /// Shape of `sample_models` the procedure runner renders.
    pub const SAMPLE_MODELS_SHAPE: procedure_like::shape::AssetModelShape =
        procedure_like::shape::AssetModelShape {
            table_name: "sample_models",
            asset_table_name: "samples",
            model_column: "sample_model_id",
            required: &[
                procedure_like::shape::ColumnShape {
                    name: "organization_id",
                    label: "Organization containing this resource.",
                    sql_type: procedure_like::shape::SqlType::Uuid,
                    unit: None,
                    required: true,
                },
                procedure_like::shape::ColumnShape {
                    name: "sample_model_id",
                    label: "Sample model instantiated by this physical sample.",
                    sql_type: procedure_like::shape::SqlType::Uuid,
                    unit: None,
                    required: true,
                },
                procedure_like::shape::ColumnShape {
                    name: "sample_source_model_id",
                    label: "Required source-model family of the sample model.",
                    sql_type: procedure_like::shape::SqlType::Uuid,
                    unit: None,
                    required: true,
                },
            ],
        };
    ::diesel::allow_tables_to_appear_in_same_query!(sample_models, ::aps_resources::asset_models);
    ::diesel::allow_tables_to_appear_in_same_query!(
        sample_models,
        ::aps_physical_asset_models::physical_asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(sample_models, ::aps_resources::resources);
    ::diesel::allow_tables_to_appear_in_same_query!(
        sample_models,
        ::aps_sample_models::sample_source_models
    );
}
pub use sample_models_model::*;
pub mod sample_source_models_model {
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
    /// Catalog of model definitions for assets that can be sampled from.
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
    #[diesel(table_name = sample_source_models)]
    pub struct SampleSourceModel {
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
        sample_source_models::id,
        sample_source_models::archived
    );
    impl ::diesel_builders::GetColumn<aps_resources::asset_models::id> for SampleSourceModel {
        fn get_column_ref(
            &self,
        ) -> &<sample_source_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_physical_asset_models::physical_asset_models::id>
        for SampleSourceModel
    {
        fn get_column_ref(
            &self,
        ) -> &<sample_source_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for SampleSourceModel {
        fn get_column_ref(
            &self,
        ) -> &<sample_source_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    /// Shape of `sample_source_models` the procedure runner renders.
    pub const SAMPLE_SOURCE_MODELS_SHAPE: procedure_like::shape::AssetModelShape =
        procedure_like::shape::AssetModelShape {
            table_name: "sample_source_models",
            asset_table_name: "sample_sources",
            model_column: "sample_source_model_id",
            required: &[
                procedure_like::shape::ColumnShape {
                    name: "organization_id",
                    label: "Organization containing this resource.",
                    sql_type: procedure_like::shape::SqlType::Uuid,
                    unit: None,
                    required: true,
                },
                procedure_like::shape::ColumnShape {
                    name: "sample_source_model_id",
                    label: "Source model instantiated by this physical source asset.",
                    sql_type: procedure_like::shape::SqlType::Uuid,
                    unit: None,
                    required: true,
                },
            ],
        };
    ::diesel::allow_tables_to_appear_in_same_query!(
        sample_source_models,
        ::aps_resources::asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        sample_source_models,
        ::aps_physical_asset_models::physical_asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        sample_source_models,
        ::aps_resources::resources
    );
}
pub use sample_source_models_model::*;
pub mod sample_sources_model {
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
    /// Physical assets that can act as concrete sample sources.
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
            (sample_source_model_id),
            (::aps_sample_models::sample_source_models::id)
        )
    )]
    #[table_model(
        foreign_key(
            (sample_source_model_id, sample_source_model_archived),
            (
                ::aps_sample_models::sample_source_models::id,
                ::aps_sample_models::sample_source_models::archived
            )
        )
    )]
    #[diesel(table_name = sample_sources)]
    pub struct SampleSource {
        /// Stable asset identifier inherited from `physical_assets`.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing this resource.
        #[same_as(aps_assets::physical_assets::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Source model instantiated by this physical source asset.
        #[same_as(aps_assets::physical_assets::physical_asset_model_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        sample_source_model_id: ::rosetta_uuid::Uuid,
        /// Archive state inherited from this resource.
        #[same_as(aps_assets::physical_assets::archived)]
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
        /// Archive state of the row named by `sample_source_model_id`.
        #[table_model(default = false)]
        #[infallible]
        sample_source_model_archived: bool,
    }
    impl ::diesel_builders::GetColumn<aps_assets::assets::id> for SampleSource {
        fn get_column_ref(
            &self,
        ) -> &<sample_sources::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_assets::physical_assets::id> for SampleSource {
        fn get_column_ref(
            &self,
        ) -> &<sample_sources::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for SampleSource {
        fn get_column_ref(
            &self,
        ) -> &<sample_sources::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    ::diesel::allow_tables_to_appear_in_same_query!(sample_sources, ::aps_assets::assets);
    ::diesel::allow_tables_to_appear_in_same_query!(sample_sources, ::aps_assets::physical_assets);
    ::diesel::allow_tables_to_appear_in_same_query!(sample_sources, ::aps_resources::resources);
    ::diesel::allow_tables_to_appear_in_same_query!(
        sample_sources,
        ::aps_sample_models::sample_source_models
    );
}
pub use sample_sources_model::*;
pub mod samples_model {
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
    /// Physical samples with their model's required source family.
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
            (sample_source_model_id),
            (::aps_sample_models::sample_source_models::id)
        )
    )]
    #[table_model(
        foreign_key(
            (sample_model_id, sample_source_model_id),
            (
                ::aps_sample_models::sample_models::id,
                ::aps_sample_models::sample_models::sample_source_model_id
            )
        )
    )]
    #[table_model(
        foreign_key(
            (sample_model_id, sample_model_archived),
            (
                ::aps_sample_models::sample_models::id,
                ::aps_sample_models::sample_models::archived
            )
        )
    )]
    #[table_model(
        foreign_key(
            (sample_source_model_id, sample_source_model_archived),
            (
                ::aps_sample_models::sample_source_models::id,
                ::aps_sample_models::sample_source_models::archived
            )
        )
    )]
    #[diesel(table_name = samples)]
    pub struct Sample {
        /// Stable asset identifier inherited from `physical_assets`.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing this resource.
        #[same_as(aps_assets::physical_assets::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Sample model instantiated by this physical sample.
        #[same_as(aps_assets::physical_assets::physical_asset_model_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        sample_model_id: ::rosetta_uuid::Uuid,
        /// Required source-model family of the sample model.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        sample_source_model_id: ::rosetta_uuid::Uuid,
        /// Archive state inherited from this resource.
        #[same_as(aps_assets::physical_assets::archived)]
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
        /// Archive state of the row named by `sample_model_id`.
        #[table_model(default = false)]
        #[infallible]
        sample_model_archived: bool,
        /// Archive state of the row named by `sample_source_model_id`.
        #[table_model(default = false)]
        #[infallible]
        sample_source_model_archived: bool,
    }
    impl ::diesel_builders::GetColumn<aps_assets::assets::id> for Sample {
        fn get_column_ref(&self) -> &<samples::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_assets::physical_assets::id> for Sample {
        fn get_column_ref(&self) -> &<samples::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for Sample {
        fn get_column_ref(&self) -> &<samples::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    ::diesel::allow_tables_to_appear_in_same_query!(samples, ::aps_assets::assets);
    ::diesel::allow_tables_to_appear_in_same_query!(samples, ::aps_assets::physical_assets);
    ::diesel::allow_tables_to_appear_in_same_query!(samples, ::aps_resources::resources);
    ::diesel::allow_tables_to_appear_in_same_query!(samples, ::aps_sample_models::sample_models);
    ::diesel::allow_tables_to_appear_in_same_query!(
        samples,
        ::aps_sample_models::sample_source_models
    );
}
pub use samples_model::*;
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
        foreign_key((supernatant_model_id), (::aps_sample_models::sample_models::id))
    )]
    #[table_model(
        foreign_key(
            (transferred_with_model_id),
            (::aps_sample_models::volume_measuring_device_models::id)
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
                stratified_source_slot_id,
                id,
                stratified_source_model_id,
                stratified_source_slot_role_id,
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
                supernatant_destination_slot_id,
                id,
                supernatant_destination_model_id,
                supernatant_destination_slot_role_id,
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
                transferred_with_slot_id,
                id,
                transferred_with_model_id,
                transferred_with_slot_role_id,
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
                supernatant_slot_id,
                id,
                supernatant_model_id,
                supernatant_slot_role_id,
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
            (supernatant_slot_id, id, supernatant_destination_slot_id, organization_id),
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
            (stratified_source_model_id, stratified_source_model_archived),
            (
                ::aps_physical_asset_models::volumetric_container_models::id,
                ::aps_physical_asset_models::volumetric_container_models::archived
            )
        )
    )]
    #[table_model(
        foreign_key(
            (supernatant_destination_model_id, supernatant_destination_model_archived),
            (
                ::aps_physical_asset_models::volumetric_container_models::id,
                ::aps_physical_asset_models::volumetric_container_models::archived
            )
        )
    )]
    #[table_model(
        foreign_key(
            (supernatant_model_id, supernatant_model_archived),
            (
                ::aps_sample_models::sample_models::id,
                ::aps_sample_models::sample_models::archived
            )
        )
    )]
    #[table_model(
        foreign_key(
            (transferred_with_model_id, transferred_with_model_archived),
            (
                ::aps_sample_models::volume_measuring_device_models::id,
                ::aps_sample_models::volume_measuring_device_models::archived
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
    #[diesel(table_name = supernatant_procedure_templates)]
    pub struct SupernatantProcedureTemplate {
        /// Identity shared with the parent template.
        #[same_as(
            aps_resources::procedure_template_slots::procedure_template_id,
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
        #[same_as(aps_resources::procedure_templates::organization_id)]
        #[same_as(
            aps_resources::procedure_template_slots::organization_id,
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
        /// Slot declaring the `stratified_source` assets.
        #[mandatory(aps_resources::procedure_template_slots)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        stratified_source_slot_id: ::rosetta_uuid::Uuid,
        /// Admissible model for the `stratified_source` slot.
        #[same_as(
            aps_resources::procedure_template_slots::asset_model_id,
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
            aps_resources::procedure_template_slots::placed_into_slot_id,
            supernatant_slot_id
        )]
        #[mandatory(aps_resources::procedure_template_slots)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        supernatant_destination_slot_id: ::rosetta_uuid::Uuid,
        /// Admissible model for the `supernatant_destination` slot.
        #[same_as(
            aps_resources::procedure_template_slots::asset_model_id,
            supernatant_destination_slot_id
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        supernatant_destination_model_id: ::rosetta_uuid::Uuid,
        /// Required role of the `supernatant_destination` slot.
        #[table_model(generated)]
        supernatant_destination_slot_role_id: String,
        /// Slot declaring the produced `supernatant` sample.
        #[mandatory(aps_resources::procedure_template_slots)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        supernatant_slot_id: ::rosetta_uuid::Uuid,
        /// Admissible model for the `supernatant` sample.
        #[same_as(aps_resources::procedure_template_slots::asset_model_id, supernatant_slot_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        supernatant_model_id: ::rosetta_uuid::Uuid,
        /// Required role of the `supernatant` slot.
        #[table_model(generated)]
        supernatant_slot_role_id: String,
        /// Slot declaring the `transferred_with` assets.
        #[mandatory(aps_resources::procedure_template_slots)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        transferred_with_slot_id: ::rosetta_uuid::Uuid,
        /// Admissible model for the `transferred_with` slot.
        #[same_as(
            aps_resources::procedure_template_slots::asset_model_id,
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
        /// Archive state inherited from this resource.
        #[same_as(aps_resources::procedure_templates::archived)]
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
        /// Archive state of the row named by `stratified_source_model_id`.
        #[table_model(default = false)]
        #[infallible]
        stratified_source_model_archived: bool,
        /// Archive state of the row named by
        /// `supernatant_destination_model_id`.
        #[table_model(default = false)]
        #[infallible]
        supernatant_destination_model_archived: bool,
        /// Archive state of the row named by `supernatant_model_id`.
        #[table_model(default = false)]
        #[infallible]
        supernatant_model_archived: bool,
        /// Archive state of the row named by `transferred_with_model_id`.
        #[table_model(default = false)]
        #[infallible]
        transferred_with_model_archived: bool,
        /// Archive state of the row named by `source_sample_model_id`.
        #[table_model(default = false)]
        #[infallible]
        source_sample_model_archived: bool,
    }
    ::diesel_builders::prelude::unique_index!(
        supernatant_procedure_templates::id,
        supernatant_procedure_templates::archived
    );
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
    impl ::diesel_builders::GetColumn<aps_resources::procedure_templates::id>
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
        ::aps_resources::procedure_template_slots
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        supernatant_procedure_templates,
        ::aps_resources::procedure_templates
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        supernatant_procedure_templates,
        ::aps_resources::resources
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        supernatant_procedure_templates,
        ::aps_sample_models::sample_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        supernatant_procedure_templates,
        ::aps_sample_models::volume_measuring_device_models
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
        ::diesel_builders::prelude::TableModel,
    )]
    /// Recorded execution of a supernatant template.
    #[table_model(ancestors(aps_resources::resources, aps_procedures::procedures))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[table_model(record_error = ::validation_errors::ValidationError)]
    #[table_model(foreign_key((id), (::aps_procedures::procedures::id)))]
    #[table_model(
        foreign_key(
            (supernatant_procedure_template_id),
            (::aps_sample_models::supernatant_procedure_templates::id)
        )
    )]
    #[table_model(
        foreign_key(
            (supernatant_procedure_template_id, supernatant_procedure_template_archived),
            (
                ::aps_sample_models::supernatant_procedure_templates::id,
                ::aps_sample_models::supernatant_procedure_templates::archived
            )
        )
    )]
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
        #[table_model(default = "planned")]
        #[infallible]
        status_id: String,
        /// Measured volume l.
        volume_l: Option<f64>,
        /// Archive state inherited from this resource.
        #[same_as(aps_procedures::procedures::archived)]
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
        /// Archive state of the row named by
        /// `supernatant_procedure_template_id`.
        #[table_model(default = false)]
        #[infallible]
        supernatant_procedure_template_archived: bool,
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
        type ProcedureTemplateTable = aps_sample_models::supernatant_procedure_templates::table;
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
        ::aps_procedures::procedures
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        supernatant_procedures,
        ::aps_resources::resources
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        supernatant_procedures,
        ::aps_sample_models::supernatant_procedure_templates
    );
}
pub use supernatant_procedures_model::*;
pub mod volume_measuring_device_models_model {
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
    /// Catalog of volume-measuring device model definitions.
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
    #[diesel(table_name = volume_measuring_device_models)]
    pub struct VolumeMeasuringDeviceModel {
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
        volume_measuring_device_models::id,
        volume_measuring_device_models::archived
    );
    impl ::diesel_builders::GetColumn<aps_resources::asset_models::id> for VolumeMeasuringDeviceModel {
        fn get_column_ref(
            &self,
        ) -> &<volume_measuring_device_models::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_physical_asset_models::physical_asset_models::id>
        for VolumeMeasuringDeviceModel
    {
        fn get_column_ref(
            &self,
        ) -> &<volume_measuring_device_models::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for VolumeMeasuringDeviceModel {
        fn get_column_ref(
            &self,
        ) -> &<volume_measuring_device_models::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    /// Shape of `volume_measuring_device_models` the procedure runner renders.
    pub const VOLUME_MEASURING_DEVICE_MODELS_SHAPE: procedure_like::shape::AssetModelShape =
        procedure_like::shape::AssetModelShape {
            table_name: "volume_measuring_device_models",
            asset_table_name: "volume_measuring_devices",
            model_column: "volume_measuring_device_model_id",
            required: &[
                procedure_like::shape::ColumnShape {
                    name: "organization_id",
                    label: "Organization containing this resource.",
                    sql_type: procedure_like::shape::SqlType::Uuid,
                    unit: None,
                    required: true,
                },
                procedure_like::shape::ColumnShape {
                    name: "volume_measuring_device_model_id",
                    label: "Volume-measuring model instantiated by this physical asset.",
                    sql_type: procedure_like::shape::SqlType::Uuid,
                    unit: None,
                    required: true,
                },
            ],
        };
    ::diesel::allow_tables_to_appear_in_same_query!(
        volume_measuring_device_models,
        ::aps_resources::asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        volume_measuring_device_models,
        ::aps_physical_asset_models::physical_asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        volume_measuring_device_models,
        ::aps_resources::resources
    );
}
pub use volume_measuring_device_models_model::*;
pub mod volume_measuring_devices_model {
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
    /// Physical volume-measuring devices tracked in APS inventory.
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
            (volume_measuring_device_model_id),
            (::aps_sample_models::volume_measuring_device_models::id)
        )
    )]
    #[table_model(
        foreign_key(
            (volume_measuring_device_model_id, volume_measuring_device_model_archived),
            (
                ::aps_sample_models::volume_measuring_device_models::id,
                ::aps_sample_models::volume_measuring_device_models::archived
            )
        )
    )]
    #[diesel(table_name = volume_measuring_devices)]
    pub struct VolumeMeasuringDevice {
        /// Stable asset identifier inherited from `physical_assets`.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing this resource.
        #[same_as(aps_assets::physical_assets::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Volume-measuring model instantiated by this physical asset.
        #[same_as(aps_assets::physical_assets::physical_asset_model_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        volume_measuring_device_model_id: ::rosetta_uuid::Uuid,
        /// Archive state inherited from this resource.
        #[same_as(aps_assets::physical_assets::archived)]
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
        /// Archive state of the row named by
        /// `volume_measuring_device_model_id`.
        #[table_model(default = false)]
        #[infallible]
        volume_measuring_device_model_archived: bool,
    }
    impl ::diesel_builders::GetColumn<aps_assets::assets::id> for VolumeMeasuringDevice {
        fn get_column_ref(
            &self,
        ) -> &<volume_measuring_devices::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_assets::physical_assets::id> for VolumeMeasuringDevice {
        fn get_column_ref(
            &self,
        ) -> &<volume_measuring_devices::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for VolumeMeasuringDevice {
        fn get_column_ref(
            &self,
        ) -> &<volume_measuring_devices::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    ::diesel::allow_tables_to_appear_in_same_query!(volume_measuring_devices, ::aps_assets::assets);
    ::diesel::allow_tables_to_appear_in_same_query!(
        volume_measuring_devices,
        ::aps_assets::physical_assets
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        volume_measuring_devices,
        ::aps_resources::resources
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        volume_measuring_devices,
        ::aps_sample_models::volume_measuring_device_models
    );
}
pub use volume_measuring_devices_model::*;
