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
            foreign_key = aliquoted_with_model_id
        )
    )]
    #[table_model(
        foreign_key((id), (::aps_procedure_templates::procedure_templates::id))
    )]
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
                aliquoted_from_slot_id,
                id,
                aliquoted_from_model_id,
                aliquoted_from_slot_role_id,
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
                aliquoted_into_slot_id,
                id,
                aliquoted_into_model_id,
                aliquoted_into_slot_role_id,
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
                aliquoted_with_slot_id,
                id,
                aliquoted_with_model_id,
                aliquoted_with_slot_role_id,
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
                aliquot_slot_id,
                id,
                aliquot_model_id,
                aliquot_slot_role_id,
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
            (aliquot_slot_id, id, aliquoted_into_slot_id, organization_id),
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
    #[diesel(table_name = aliquoting_procedure_templates)]
    pub struct AliquotingProcedureTemplate {
        /// Identity shared with the parent template.
        #[same_as(
            aps_procedure_templates::procedure_template_slots::procedure_template_id,
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
        #[same_as(aps_procedure_templates::procedure_templates::organization_id)]
        #[same_as(
            aps_procedure_templates::procedure_template_slots::organization_id,
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
        /// Slot declaring the `aliquoted_from` assets.
        #[mandatory(aps_procedure_templates::procedure_template_slots)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        aliquoted_from_slot_id: ::rosetta_uuid::Uuid,
        /// Admissible model for the `aliquoted_from` slot.
        #[same_as(
            aps_procedure_templates::procedure_template_slots::asset_model_id,
            aliquoted_from_slot_id
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        aliquoted_from_model_id: ::rosetta_uuid::Uuid,
        /// Required role of the `aliquoted_from` slot.
        #[table_model(generated)]
        aliquoted_from_slot_role_id: String,
        /// Slot declaring the `aliquoted_into` assets.
        #[same_as(
            aps_procedure_templates::procedure_template_slots::placed_into_slot_id,
            aliquot_slot_id
        )]
        #[mandatory(aps_procedure_templates::procedure_template_slots)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        aliquoted_into_slot_id: ::rosetta_uuid::Uuid,
        /// Admissible model for the `aliquoted_into` slot.
        #[same_as(
            aps_procedure_templates::procedure_template_slots::asset_model_id,
            aliquoted_into_slot_id
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        aliquoted_into_model_id: ::rosetta_uuid::Uuid,
        /// Required role of the `aliquoted_into` slot.
        #[table_model(generated)]
        aliquoted_into_slot_role_id: String,
        /// Slot declaring the produced `aliquot` sample.
        #[mandatory(aps_procedure_templates::procedure_template_slots)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        aliquot_slot_id: ::rosetta_uuid::Uuid,
        /// Admissible model for the `aliquot` sample.
        #[same_as(
            aps_procedure_templates::procedure_template_slots::asset_model_id,
            aliquot_slot_id
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        aliquot_model_id: ::rosetta_uuid::Uuid,
        /// Required role of the `aliquot` slot.
        #[table_model(generated)]
        aliquot_slot_role_id: String,
        /// Slot declaring the `aliquoted_with` assets.
        #[mandatory(aps_procedure_templates::procedure_template_slots)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        aliquoted_with_slot_id: ::rosetta_uuid::Uuid,
        /// Admissible model for the `aliquoted_with` slot.
        #[same_as(
            aps_procedure_templates::procedure_template_slots::asset_model_id,
            aliquoted_with_slot_id
        )]
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
    }
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
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id> for AliquotingProcedureTemplate {
        fn get_column_ref(
            &self,
        ) -> &<aliquoting_procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_procedure_templates::procedure_templates::id>
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
        ::aps_resources::ownables
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        aliquoting_procedure_templates,
        ::aps_procedure_templates::procedure_template_slots
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        aliquoting_procedure_templates,
        ::aps_procedure_templates::procedure_templates
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
        ::aps_physical_asset_models::volume_measuring_device_models
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
        ::diesel::Associations,
        ::diesel_builders::prelude::TableModel,
    )]
    /// Recorded execution of a aliquoting template.
    #[table_model(ancestors(
        aps_resources::resources,
        aps_resources::ownables,
        aps_procedures::procedures
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[table_model(record_error = ::validation_errors::ValidationError)]
    #[diesel(
        belongs_to(
            aps_sample_models::AliquotingProcedureTemplate,
            foreign_key = aliquoting_procedure_template_id
        )
    )]
    #[table_model(foreign_key((id), (::aps_procedures::procedures::id)))]
    #[table_model(
        foreign_key(
            (aliquoting_procedure_template_id),
            (::aps_sample_models::aliquoting_procedure_templates::id)
        )
    )]
    #[table_model(default(
        aps_resources::resources::kind,
        ::aps_authz_resource_kind::ResourceKind::Ownable
    ))]
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
        #[infallible]
        status_id: String,
        /// Measured volume l.
        volume_l: Option<f64>,
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
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id> for AliquotingProcedure {
        fn get_column_ref(
            &self,
        ) -> &<aliquoting_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
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
        ::aps_resources::ownables
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
pub mod asset_model_ancestors_model {
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
        ::diesel_builders::prelude::TableModel,
    )]
    /// Reflexive transitive closure of `asset_models.parent_model_id`, written
    /// by the client together with the model. A non-reflexive row names the
    /// descendant's parent, and the two foreign keys through it make every row
    /// a true ancestor by induction. `check_asset_model_ancestors` makes the
    /// set complete at commit.
    #[table_model(error = ::validation_errors::ValidationError)]
    #[table_model(record_error = ::validation_errors::ValidationError)]
    #[diesel(primary_key(descendant_model_id, ancestor_model_id))]
    #[table_model(
        foreign_key((descendant_model_id), (::aps_resources::asset_models::id))
    )]
    #[table_model(
        foreign_key(
            (organization_id, descendant_model_id),
            (::aps_resources::resources::organization_id, ::aps_resources::resources::id)
        )
    )]
    #[table_model(
        foreign_key(
            (descendant_model_id, parent_model_id, organization_id),
            (
                ::aps_resources::asset_models::id,
                ::aps_resources::asset_models::parent_model_id,
                ::aps_resources::asset_models::organization_id
            )
        )
    )]
    #[table_model(
        foreign_key(
            (parent_model_id, ancestor_model_id),
            (
                asset_model_ancestors::descendant_model_id,
                asset_model_ancestors::ancestor_model_id
            )
        )
    )]
    #[diesel(table_name = asset_model_ancestors)]
    pub struct AssetModelAncestor {
        /// Model whose ancestors the row lists.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        descendant_model_id: ::rosetta_uuid::Uuid,
        /// Ancestor of the descendant, or the descendant itself.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        ancestor_model_id: ::rosetta_uuid::Uuid,
        /// connetto organization owning the descendant.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Parent of the descendant, through which the ancestor is reached.
        /// Null on the reflexive row.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        parent_model_id: Option<::rosetta_uuid::Uuid>,
    }
    impl ::diesel_builders::ValidateRecord<asset_model_ancestors::table>
        for <asset_model_ancestors::table as ::diesel_builders::TableExt>::NewValues
    {
        #[inline]
        fn validate_record(&self) -> Result<(), ::validation_errors::ValidationError> {
            use diesel::Column;
            if let (Some(parent_model_id), Some(descendant_model_id), Some(ancestor_model_id)) =
                (
                    <Self as diesel_builders::MayGetColumn<
                        asset_model_ancestors::parent_model_id,
                    >>::may_get_column_ref(self),
                    <Self as diesel_builders::MayGetColumn<
                        asset_model_ancestors::descendant_model_id,
                    >>::may_get_column_ref(self),
                    <Self as diesel_builders::MayGetColumn<
                        asset_model_ancestors::ancestor_model_id,
                    >>::may_get_column_ref(self),
                )
                && (*parent_model_id).is_none() != (*descendant_model_id == *ancestor_model_id)
            {
                return Err(
                    ::validation_errors::ValidationError::check_violated(
                        <crate::asset_model_ancestors::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        &[
                            crate::asset_model_ancestors::parent_model_id::NAME,
                            crate::asset_model_ancestors::descendant_model_id::NAME,
                            crate::asset_model_ancestors::ancestor_model_id::NAME,
                        ],
                        "(parent_model_id IS NULL) = (descendant_model_id = ancestor_model_id)",
                    ),
                );
            }
            Ok(())
        }
    }
    ::diesel::allow_tables_to_appear_in_same_query!(
        asset_model_ancestors,
        ::aps_resources::asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        asset_model_ancestors,
        ::aps_resources::resources
    );
}
pub use asset_model_ancestors_model::*;
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
            aps_weighing_device_models::WeighingDeviceModel,
            foreign_key = weighed_with_model_id
        )
    )]
    #[table_model(
        foreign_key((id), (::aps_procedure_templates::procedure_templates::id))
    )]
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
            aps_sample_models::FractioningProcedureTemplate,
            foreign_key = fractioning_procedure_template_id
        )
    )]
    #[table_model(foreign_key((id), (::aps_procedures::procedures::id)))]
    #[table_model(
        foreign_key(
            (fractioning_procedure_template_id),
            (::aps_sample_models::fractioning_procedure_templates::id)
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
        type ProcedureTemplateTable = aps_sample_models::fractioning_procedure_templates::table;
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
        ::aps_sample_models::fractioning_procedure_templates
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
    #[table_model(ancestors(
        aps_resources::resources,
        aps_resources::ownables,
        aps_procedure_templates::procedure_templates
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_procedure_templates::ProcedureTemplate, foreign_key = id))]
    #[diesel(
        belongs_to(aps_assets::SampleSourceModel, foreign_key = sample_source_model_id)
    )]
    #[table_model(
        foreign_key((id), (::aps_procedure_templates::procedure_templates::id))
    )]
    #[table_model(
        foreign_key((sample_source_model_id), (::aps_assets::sample_source_models::id))
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
            (sample_slot_id, id, sample_model_id, sample_slot_role_id, organization_id),
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
                ::aps_sample_models::asset_model_ancestors::descendant_model_id,
                ::aps_sample_models::asset_model_ancestors::ancestor_model_id
            )
        )
    )]
    #[table_model(default(
        aps_resources::resources::kind,
        ::aps_authz_resource_kind::ResourceKind::Ownable
    ))]
    #[diesel(table_name = harvesting_procedure_templates)]
    pub struct HarvestingProcedureTemplate {
        /// Identity shared with the parent template.
        #[same_as(
            aps_procedure_templates::procedure_template_slots::procedure_template_id,
            (sample_source_slot_id, sample_slot_id)
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing the parent resource.
        #[same_as(aps_procedure_templates::procedure_templates::organization_id)]
        #[same_as(
            aps_procedure_templates::procedure_template_slots::organization_id,
            (sample_source_slot_id, sample_slot_id)
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Slot declaring the `sample_source` assets.
        #[mandatory(aps_procedure_templates::procedure_template_slots)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        sample_source_slot_id: ::rosetta_uuid::Uuid,
        /// Admissible model for the `sample_source` slot.
        #[same_as(
            aps_procedure_templates::procedure_template_slots::asset_model_id,
            sample_source_slot_id
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        sample_source_model_id: ::rosetta_uuid::Uuid,
        /// Required role of the `sample_source` slot.
        #[table_model(generated)]
        sample_source_slot_role_id: String,
        /// Slot declaring the `sample` assets.
        #[mandatory(aps_procedure_templates::procedure_template_slots)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        sample_slot_id: ::rosetta_uuid::Uuid,
        /// Admissible model for the `sample` slot.
        #[same_as(
            aps_procedure_templates::procedure_template_slots::asset_model_id,
            sample_slot_id
        )]
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
    }
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id> for HarvestingProcedureTemplate {
        fn get_column_ref(
            &self,
        ) -> &<harvesting_procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_procedure_templates::procedure_templates::id>
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
        ::aps_sample_models::asset_model_ancestors
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        harvesting_procedure_templates,
        ::aps_resources::ownables
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        harvesting_procedure_templates,
        ::aps_procedure_templates::procedure_template_slots
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        harvesting_procedure_templates,
        ::aps_procedure_templates::procedure_templates
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
        ::aps_assets::sample_source_models
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
        ::diesel::Associations,
        ::diesel_builders::prelude::TableModel,
    )]
    /// Recorded execution of a harvesting template.
    #[table_model(ancestors(
        aps_resources::resources,
        aps_resources::ownables,
        aps_procedures::procedures
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(
        belongs_to(
            aps_sample_models::HarvestingProcedureTemplate,
            foreign_key = harvesting_procedure_template_id
        )
    )]
    #[table_model(foreign_key((id), (::aps_procedures::procedures::id)))]
    #[table_model(
        foreign_key(
            (harvesting_procedure_template_id),
            (::aps_sample_models::harvesting_procedure_templates::id)
        )
    )]
    #[table_model(default(
        aps_resources::resources::kind,
        ::aps_authz_resource_kind::ResourceKind::Ownable
    ))]
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
        #[infallible]
        status_id: String,
    }
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id> for HarvestingProcedure {
        fn get_column_ref(
            &self,
        ) -> &<harvesting_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
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
        ::aps_resources::ownables
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
    }
    impl ::diesel_builders::GetColumn<aps_resources::asset_models::id> for PackagingModel {
        fn get_column_ref(
            &self,
        ) -> &<packaging_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id> for PackagingModel {
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
    ::diesel::allow_tables_to_appear_in_same_query!(packaging_models, ::aps_resources::ownables);
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
    #[table_model(ancestors(
        aps_resources::resources,
        aps_resources::ownables,
        aps_procedure_templates::procedure_templates
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_procedure_templates::ProcedureTemplate, foreign_key = id))]
    #[diesel(
        belongs_to(
            aps_sample_models::PackagingModel,
            foreign_key = packaged_with_model_id
        )
    )]
    #[diesel(belongs_to(aps_sample_models::SampleModel, foreign_key = sample_model_id))]
    #[table_model(
        foreign_key((id), (::aps_procedure_templates::procedure_templates::id))
    )]
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
            (sample_slot_id, id, sample_model_id, sample_slot_role_id, organization_id),
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
    #[diesel(table_name = packaging_procedure_templates)]
    pub struct PackagingProcedureTemplate {
        /// Identity shared with the parent template.
        #[same_as(
            aps_procedure_templates::procedure_template_slots::procedure_template_id,
            (packaged_with_slot_id, sample_slot_id)
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing the parent resource.
        #[same_as(aps_procedure_templates::procedure_templates::organization_id)]
        #[same_as(
            aps_procedure_templates::procedure_template_slots::organization_id,
            (packaged_with_slot_id, sample_slot_id)
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Slot declaring the `packaged_with` assets.
        #[mandatory(aps_procedure_templates::procedure_template_slots)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        packaged_with_slot_id: ::rosetta_uuid::Uuid,
        /// Admissible model for the `packaged_with` slot.
        #[same_as(
            aps_procedure_templates::procedure_template_slots::asset_model_id,
            packaged_with_slot_id
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        packaged_with_model_id: ::rosetta_uuid::Uuid,
        /// Required role of the `packaged_with` slot.
        #[table_model(generated)]
        packaged_with_slot_role_id: String,
        /// Slot declaring the `sample` assets.
        #[mandatory(aps_procedure_templates::procedure_template_slots)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        sample_slot_id: ::rosetta_uuid::Uuid,
        /// Admissible model for the `sample` slot.
        #[same_as(
            aps_procedure_templates::procedure_template_slots::asset_model_id,
            sample_slot_id
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        sample_model_id: ::rosetta_uuid::Uuid,
        /// Required role of the `sample` slot.
        #[table_model(generated)]
        sample_slot_role_id: String,
    }
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id> for PackagingProcedureTemplate {
        fn get_column_ref(
            &self,
        ) -> &<packaging_procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_procedure_templates::procedure_templates::id>
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
        ::aps_resources::ownables
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        packaging_procedure_templates,
        ::aps_sample_models::packaging_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        packaging_procedure_templates,
        ::aps_procedure_templates::procedure_template_slots
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        packaging_procedure_templates,
        ::aps_procedure_templates::procedure_templates
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
        ::diesel::Associations,
        ::diesel_builders::prelude::TableModel,
    )]
    /// Recorded execution of a packaging template.
    #[table_model(ancestors(
        aps_resources::resources,
        aps_resources::ownables,
        aps_procedures::procedures
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(
        belongs_to(
            aps_sample_models::PackagingProcedureTemplate,
            foreign_key = packaging_procedure_template_id
        )
    )]
    #[table_model(foreign_key((id), (::aps_procedures::procedures::id)))]
    #[table_model(
        foreign_key(
            (packaging_procedure_template_id),
            (::aps_sample_models::packaging_procedure_templates::id)
        )
    )]
    #[table_model(default(
        aps_resources::resources::kind,
        ::aps_authz_resource_kind::ResourceKind::Ownable
    ))]
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
        #[infallible]
        status_id: String,
    }
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id> for PackagingProcedure {
        fn get_column_ref(
            &self,
        ) -> &<packaging_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
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
        ::aps_resources::ownables
    );
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
            foreign_key = measured_with_model_id
        )
    )]
    #[table_model(
        foreign_key((id), (::aps_procedure_templates::procedure_templates::id))
    )]
    #[table_model(
        foreign_key((source_sample_model_id), (::aps_sample_models::sample_models::id))
    )]
    #[table_model(
        foreign_key(
            (measured_with_model_id),
            (::aps_physical_asset_models::volume_measuring_device_models::id)
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
                measured_with_slot_id,
                id,
                measured_with_model_id,
                measured_with_slot_role_id,
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
                poured_from_slot_id,
                id,
                poured_from_model_id,
                poured_from_slot_role_id,
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
                poured_into_slot_id,
                id,
                poured_into_model_id,
                poured_into_slot_role_id,
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
                poured_sample_slot_id,
                id,
                poured_sample_model_id,
                poured_sample_slot_role_id,
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
            (poured_sample_slot_id, id, poured_into_slot_id, organization_id),
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
    #[diesel(table_name = pouring_procedure_templates)]
    pub struct PouringProcedureTemplate {
        /// Identity shared with the parent template.
        #[same_as(
            aps_procedure_templates::procedure_template_slots::procedure_template_id,
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
        #[same_as(aps_procedure_templates::procedure_templates::organization_id)]
        #[same_as(
            aps_procedure_templates::procedure_template_slots::organization_id,
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
        /// Slot declaring the `measured_with` assets.
        #[mandatory(aps_procedure_templates::procedure_template_slots)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        measured_with_slot_id: ::rosetta_uuid::Uuid,
        /// Admissible model for the `measured_with` slot.
        #[same_as(
            aps_procedure_templates::procedure_template_slots::asset_model_id,
            measured_with_slot_id
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        measured_with_model_id: ::rosetta_uuid::Uuid,
        /// Required role of the `measured_with` slot.
        #[table_model(generated)]
        measured_with_slot_role_id: String,
        /// Slot declaring the `poured_from` assets.
        #[mandatory(aps_procedure_templates::procedure_template_slots)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        poured_from_slot_id: ::rosetta_uuid::Uuid,
        /// Admissible model for the `poured_from` slot.
        #[same_as(
            aps_procedure_templates::procedure_template_slots::asset_model_id,
            poured_from_slot_id
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        poured_from_model_id: ::rosetta_uuid::Uuid,
        /// Required role of the `poured_from` slot.
        #[table_model(generated)]
        poured_from_slot_role_id: String,
        /// Slot declaring the `poured_into` assets.
        #[same_as(
            aps_procedure_templates::procedure_template_slots::placed_into_slot_id,
            poured_sample_slot_id
        )]
        #[mandatory(aps_procedure_templates::procedure_template_slots)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        poured_into_slot_id: ::rosetta_uuid::Uuid,
        /// Admissible model for the `poured_into` slot.
        #[same_as(
            aps_procedure_templates::procedure_template_slots::asset_model_id,
            poured_into_slot_id
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        poured_into_model_id: ::rosetta_uuid::Uuid,
        /// Required role of the `poured_into` slot.
        #[table_model(generated)]
        poured_into_slot_role_id: String,
        /// Slot declaring the produced `poured_sample` sample.
        #[mandatory(aps_procedure_templates::procedure_template_slots)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        poured_sample_slot_id: ::rosetta_uuid::Uuid,
        /// Admissible model for the `poured_sample` sample.
        #[same_as(
            aps_procedure_templates::procedure_template_slots::asset_model_id,
            poured_sample_slot_id
        )]
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
    }
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
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id> for PouringProcedureTemplate {
        fn get_column_ref(
            &self,
        ) -> &<pouring_procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_procedure_templates::procedure_templates::id>
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
        ::aps_resources::ownables
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        pouring_procedure_templates,
        ::aps_procedure_templates::procedure_template_slots
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        pouring_procedure_templates,
        ::aps_procedure_templates::procedure_templates
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
        ::aps_physical_asset_models::volume_measuring_device_models
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
        ::diesel::Associations,
        ::diesel_builders::prelude::TableModel,
    )]
    /// Recorded execution of a pouring template.
    #[table_model(ancestors(
        aps_resources::resources,
        aps_resources::ownables,
        aps_procedures::procedures
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[table_model(record_error = ::validation_errors::ValidationError)]
    #[diesel(
        belongs_to(
            aps_sample_models::PouringProcedureTemplate,
            foreign_key = pouring_procedure_template_id
        )
    )]
    #[table_model(foreign_key((id), (::aps_procedures::procedures::id)))]
    #[table_model(
        foreign_key(
            (pouring_procedure_template_id),
            (::aps_sample_models::pouring_procedure_templates::id)
        )
    )]
    #[table_model(default(
        aps_resources::resources::kind,
        ::aps_authz_resource_kind::ResourceKind::Ownable
    ))]
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
        #[infallible]
        status_id: String,
        /// Measured volume l.
        volume_l: Option<f64>,
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
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id> for PouringProcedure {
        fn get_column_ref(
            &self,
        ) -> &<pouring_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
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
    ::diesel::allow_tables_to_appear_in_same_query!(pouring_procedures, ::aps_resources::ownables);
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
pub mod procedure_assets_model {
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
    /// The only place a procedure names an asset. `asset_id` stays NULL when
    /// the asset's model is known but its identity is not tracked. The status
    /// is a copy of the procedure's, kept by the cascade, so closed procedures
    /// can be guarded and aborted or skipped ones excluded from the uniqueness
    /// of production and consumption. A row filling a child slot its step binds
    /// names the parent's row, and only rows without one count towards that
    /// uniqueness.
    #[table_model(error = ::validation_errors::ValidationError)]
    #[table_model(record_error = ::validation_errors::ValidationError)]
    #[table_model(
        foreign_key(
            (procedure_id, procedure_template_id, status_id, organization_id),
            (
                ::aps_procedures::procedures::id,
                ::aps_procedures::procedures::procedure_template_id,
                ::aps_procedures::procedures::status_id,
                ::aps_procedures::procedures::organization_id
            )
        )
    )]
    #[table_model(
        foreign_key(
            (slot_id, procedure_template_id, slot_asset_model_id, slot_role_id),
            (
                ::aps_procedure_templates::procedure_template_slots::id,
                ::aps_procedure_templates::procedure_template_slots::procedure_template_id,
                ::aps_procedure_templates::procedure_template_slots::asset_model_id,
                ::aps_procedure_templates::procedure_template_slots::role_id
            )
        )
    )]
    #[table_model(
        foreign_key(
            (asset_model_id, slot_asset_model_id),
            (
                ::aps_sample_models::asset_model_ancestors::descendant_model_id,
                ::aps_sample_models::asset_model_ancestors::ancestor_model_id
            )
        )
    )]
    #[table_model(
        foreign_key(
            (asset_id, asset_model_id),
            (::aps_assets::assets::id, ::aps_assets::assets::model_id)
        )
    )]
    #[table_model(
        foreign_key(
            (procedure_id, parent_procedure_id, organization_id),
            (
                ::aps_procedures::procedures::id,
                ::aps_procedures::procedures::parent_procedure_id,
                ::aps_procedures::procedures::organization_id
            )
        )
    )]
    #[table_model(
        foreign_key(
            (
                parent_procedure_asset_id,
                parent_procedure_id,
                asset_model_id,
                organization_id
            ),
            (
                procedure_assets::id,
                procedure_assets::procedure_id,
                procedure_assets::asset_model_id,
                procedure_assets::organization_id
            )
        )
    )]
    #[table_model(
        foreign_key(
            (organization_id, procedure_id),
            (::aps_resources::resources::organization_id, ::aps_resources::resources::id)
        )
    )]
    #[diesel(table_name = procedure_assets)]
    pub struct ProcedureAsset {
        /// Identifier of the binding.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// connetto organization owning the procedure.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Procedure binding the asset.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        procedure_id: ::rosetta_uuid::Uuid,
        /// Template of the procedure.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        procedure_template_id: ::rosetta_uuid::Uuid,
        /// Status of the procedure, kept equal by the cascade.
        #[infallible]
        status_id: String,
        /// Slot the asset fills.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        slot_id: ::rosetta_uuid::Uuid,
        /// Model of the slot.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        slot_asset_model_id: ::rosetta_uuid::Uuid,
        /// Role of the slot.
        #[infallible]
        slot_role_id: String,
        /// Model of the asset, equal to or below the slot's.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        asset_model_id: ::rosetta_uuid::Uuid,
        /// Tracked asset, NULL when only its model is known.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        asset_id: Option<::rosetta_uuid::Uuid>,
        /// Parent procedure's binding this row inherits through the step's slot
        /// binding.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        parent_procedure_asset_id: Option<::rosetta_uuid::Uuid>,
        /// Parent of the procedure, set exactly on a row that inherits a parent
        /// binding.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        parent_procedure_id: Option<::rosetta_uuid::Uuid>,
    }
    ::diesel_builders::prelude::unique_index!(
        procedure_assets::procedure_id,
        procedure_assets::slot_id,
        procedure_assets::asset_id
    );
    ::diesel_builders::prelude::unique_index!(
        procedure_assets::id,
        procedure_assets::procedure_id,
        procedure_assets::asset_model_id,
        procedure_assets::organization_id
    );
    impl ::diesel_builders::ValidateRecord<procedure_assets::table>
        for <procedure_assets::table as ::diesel_builders::TableExt>::NewValues
    {
        #[inline]
        fn validate_record(&self) -> Result<(), ::validation_errors::ValidationError> {
            use diesel::Column;
            if let (Some(parent_procedure_asset_id), Some(parent_procedure_id)) =
                (
                    <Self as diesel_builders::MayGetColumn<
                        procedure_assets::parent_procedure_asset_id,
                    >>::may_get_column_ref(self),
                    <Self as diesel_builders::MayGetColumn<
                        procedure_assets::parent_procedure_id,
                    >>::may_get_column_ref(self),
                )
                && (*parent_procedure_asset_id).is_none() != (*parent_procedure_id).is_none()
            {
                return Err(::validation_errors::ValidationError::check_violated(
                    <crate::procedure_assets::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    &[
                        crate::procedure_assets::parent_procedure_asset_id::NAME,
                        crate::procedure_assets::parent_procedure_id::NAME,
                    ],
                    "(parent_procedure_asset_id IS NULL) = (parent_procedure_id IS NULL)",
                ));
            }
            Ok(())
        }
    }
    ::diesel::allow_tables_to_appear_in_same_query!(
        procedure_assets,
        ::aps_sample_models::asset_model_ancestors
    );
    ::diesel::allow_tables_to_appear_in_same_query!(procedure_assets, ::aps_assets::assets);
    ::diesel::allow_tables_to_appear_in_same_query!(
        procedure_assets,
        ::aps_procedure_templates::procedure_template_slots
    );
    ::diesel::allow_tables_to_appear_in_same_query!(procedure_assets, ::aps_procedures::procedures);
    ::diesel::allow_tables_to_appear_in_same_query!(procedure_assets, ::aps_resources::resources);
}
pub use procedure_assets_model::*;
pub mod procedure_template_slot_bindings_model {
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
    /// Binds a slot of a step's child template to a slot of the parent
    /// template, so the child procedure must use the asset the parent procedure
    /// binds there. Two steps bound to one parent slot share that asset. The
    /// parent slot may narrow the child slot's model, and the pair of roles
    /// must be admitted by `slot_role_pairs`. A parent slot is produced by at
    /// most one child slot and consumed by at most one.
    #[diesel(primary_key(step_id, child_slot_id))]
    #[table_model(
        foreign_key(
            (step_id, parent_template_id, child_template_id, organization_id),
            (
                ::aps_procedure_templates::procedure_template_steps::id,
                ::aps_procedure_templates::procedure_template_steps::parent_template_id,
                ::aps_procedure_templates::procedure_template_steps::child_template_id,
                ::aps_procedure_templates::procedure_template_steps::organization_id
            )
        )
    )]
    #[table_model(
        foreign_key(
            (child_slot_id, child_template_id, child_asset_model_id, child_role_id),
            (
                ::aps_procedure_templates::procedure_template_slots::id,
                ::aps_procedure_templates::procedure_template_slots::procedure_template_id,
                ::aps_procedure_templates::procedure_template_slots::asset_model_id,
                ::aps_procedure_templates::procedure_template_slots::role_id
            )
        )
    )]
    #[table_model(
        foreign_key(
            (
                parent_slot_id,
                parent_template_id,
                parent_asset_model_id,
                parent_role_id,
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
            (parent_asset_model_id, child_asset_model_id),
            (
                ::aps_sample_models::asset_model_ancestors::descendant_model_id,
                ::aps_sample_models::asset_model_ancestors::ancestor_model_id
            )
        )
    )]
    #[table_model(
        foreign_key(
            (child_role_id, parent_role_id),
            (
                ::aps_sample_models::slot_role_pairs::child_role_id,
                ::aps_sample_models::slot_role_pairs::parent_role_id
            )
        )
    )]
    #[table_model(
        foreign_key(
            (organization_id, parent_template_id),
            (::aps_resources::resources::organization_id, ::aps_resources::resources::id)
        )
    )]
    #[diesel(table_name = procedure_template_slot_bindings)]
    pub struct ProcedureTemplateSlotBinding {
        /// Step whose child slot is bound.
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        step_id: ::rosetta_uuid::Uuid,
        /// Slot of the step's child template.
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        child_slot_id: ::rosetta_uuid::Uuid,
        /// Template the step belongs to.
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        parent_template_id: ::rosetta_uuid::Uuid,
        /// Template the step runs.
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        child_template_id: ::rosetta_uuid::Uuid,
        /// Slot of the parent template that fills the child slot.
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        parent_slot_id: ::rosetta_uuid::Uuid,
        /// Model of the child slot.
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        child_asset_model_id: ::rosetta_uuid::Uuid,
        /// Model of the parent slot, equal to or below the child slot's.
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        parent_asset_model_id: ::rosetta_uuid::Uuid,
        /// Role of the child slot.
        child_role_id: String,
        /// Role of the parent slot.
        parent_role_id: String,
        /// connetto organization owning the parent template.
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
    }
    ::diesel::allow_tables_to_appear_in_same_query!(
        procedure_template_slot_bindings,
        ::aps_sample_models::asset_model_ancestors
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        procedure_template_slot_bindings,
        ::aps_procedure_templates::procedure_template_slots
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        procedure_template_slot_bindings,
        ::aps_procedure_templates::procedure_template_steps
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        procedure_template_slot_bindings,
        ::aps_resources::resources
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        procedure_template_slot_bindings,
        ::aps_sample_models::slot_role_pairs
    );
}
pub use procedure_template_slot_bindings_model::*;
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
        aps_resources::ownables,
        aps_resources::asset_models,
        aps_physical_asset_models::physical_asset_models
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(
        belongs_to(aps_physical_asset_models::PhysicalAssetModel, foreign_key = id)
    )]
    #[diesel(
        belongs_to(aps_assets::SampleSourceModel, foreign_key = sample_source_model_id)
    )]
    #[table_model(
        foreign_key((id), (::aps_physical_asset_models::physical_asset_models::id))
    )]
    #[table_model(
        foreign_key((sample_source_model_id), (::aps_assets::sample_source_models::id))
    )]
    #[table_model(default(
        aps_resources::resources::kind,
        ::aps_authz_resource_kind::ResourceKind::Ownable
    ))]
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
    }
    ::diesel_builders::prelude::unique_index!(
        sample_models::id,
        sample_models::sample_source_model_id
    );
    impl ::diesel_builders::GetColumn<aps_resources::asset_models::id> for SampleModel {
        fn get_column_ref(
            &self,
        ) -> &<sample_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id> for SampleModel {
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
    ::diesel::allow_tables_to_appear_in_same_query!(sample_models, ::aps_resources::ownables);
    ::diesel::allow_tables_to_appear_in_same_query!(
        sample_models,
        ::aps_physical_asset_models::physical_asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(sample_models, ::aps_resources::resources);
    ::diesel::allow_tables_to_appear_in_same_query!(
        sample_models,
        ::aps_assets::sample_source_models
    );
}
pub use sample_models_model::*;
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
        aps_resources::ownables,
        aps_assets::assets,
        aps_assets::physical_assets
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_assets::PhysicalAsset, foreign_key = id))]
    #[diesel(
        belongs_to(aps_assets::SampleSourceModel, foreign_key = sample_source_model_id)
    )]
    #[table_model(foreign_key((id), (::aps_assets::physical_assets::id)))]
    #[table_model(
        foreign_key((sample_source_model_id), (::aps_assets::sample_source_models::id))
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
    #[table_model(default(
        aps_resources::resources::kind,
        ::aps_authz_resource_kind::ResourceKind::Ownable
    ))]
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
    }
    impl ::diesel_builders::GetColumn<aps_assets::assets::id> for Sample {
        fn get_column_ref(&self) -> &<samples::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id> for Sample {
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
    ::diesel::allow_tables_to_appear_in_same_query!(samples, ::aps_resources::ownables);
    ::diesel::allow_tables_to_appear_in_same_query!(samples, ::aps_assets::physical_assets);
    ::diesel::allow_tables_to_appear_in_same_query!(samples, ::aps_resources::resources);
    ::diesel::allow_tables_to_appear_in_same_query!(samples, ::aps_sample_models::sample_models);
    ::diesel::allow_tables_to_appear_in_same_query!(samples, ::aps_assets::sample_source_models);
}
pub use samples_model::*;
pub mod slot_role_pairs_model {
    #[derive(
        Clone,
        Default,
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
    /// Which role a step's child slot may play towards the parent slot it is
    /// bound to, so the child's effect on the asset fits what the parent
    /// declares.
    #[diesel(primary_key(child_role_id, parent_role_id))]
    #[table_model(
        foreign_key((child_role_id), (::aps_procedure_templates::slot_roles::id))
    )]
    #[table_model(
        foreign_key((parent_role_id), (::aps_procedure_templates::slot_roles::id))
    )]
    #[diesel(table_name = slot_role_pairs)]
    pub struct SlotRolePair {
        /// Role of the child template's slot.
        child_role_id: String,
        /// Role of the parent template's slot.
        parent_role_id: String,
    }
    ::diesel::allow_tables_to_appear_in_same_query!(
        slot_role_pairs,
        ::aps_procedure_templates::slot_roles
    );
}
pub use slot_role_pairs_model::*;
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
        ::aps_sample_models::sample_models
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
            aps_sample_models::SupernatantProcedureTemplate,
            foreign_key = supernatant_procedure_template_id
        )
    )]
    #[table_model(foreign_key((id), (::aps_procedures::procedures::id)))]
    #[table_model(
        foreign_key(
            (supernatant_procedure_template_id),
            (::aps_sample_models::supernatant_procedure_templates::id)
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
        ::aps_sample_models::supernatant_procedure_templates
    );
}
pub use supernatant_procedures_model::*;
