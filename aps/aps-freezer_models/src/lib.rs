#![allow(ambiguous_glob_reexports, reason = "TableModel trait names can collide across tables")]
extern crate self as aps_freezer_models;
pub mod freezer_models_model {
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
    /// Catalog of freezer model definitions.
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
    #[diesel(table_name = freezer_models)]
    pub struct FreezerModel {
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
    impl ::diesel_builders::GetColumn<aps_resources::asset_models::id> for FreezerModel {
        fn get_column_ref(
            &self,
        ) -> &<freezer_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id> for FreezerModel {
        fn get_column_ref(
            &self,
        ) -> &<freezer_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_physical_asset_models::physical_asset_models::id>
        for FreezerModel
    {
        fn get_column_ref(
            &self,
        ) -> &<freezer_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for FreezerModel {
        fn get_column_ref(
            &self,
        ) -> &<freezer_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    /// Shape of `freezer_models` the procedure runner renders.
    pub const FREEZER_MODELS_SHAPE: procedure_like::shape::AssetModelShape =
        procedure_like::shape::AssetModelShape {
            table_name: "freezer_models",
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
    ::diesel::allow_tables_to_appear_in_same_query!(freezer_models, ::aps_resources::asset_models);
    ::diesel::allow_tables_to_appear_in_same_query!(freezer_models, ::aps_resources::ownables);
    ::diesel::allow_tables_to_appear_in_same_query!(
        freezer_models,
        ::aps_physical_asset_models::physical_asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(freezer_models, ::aps_resources::resources);
}
pub use freezer_models_model::*;
pub mod freezing_procedure_templates_model {
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
    /// Typed slots and parameters for freezing.
    #[table_model(ancestors(
        aps_resources::resources,
        aps_resources::ownables,
        aps_procedure_templates::procedure_templates
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_procedure_templates::ProcedureTemplate, foreign_key = id))]
    #[diesel(
        belongs_to(aps_freezer_models::FreezerModel, foreign_key = frozen_with_model_id)
    )]
    #[diesel(
        belongs_to(
            aps_physical_asset_models::VolumetricContainerModel,
            foreign_key = frozen_container_model_id
        )
    )]
    #[table_model(
        foreign_key((id), (::aps_procedure_templates::procedure_templates::id))
    )]
    #[table_model(
        foreign_key((frozen_with_model_id), (::aps_freezer_models::freezer_models::id))
    )]
    #[table_model(
        foreign_key(
            (frozen_container_model_id),
            (::aps_physical_asset_models::volumetric_container_models::id)
        )
    )]
    #[table_model(
        foreign_key(
            (
                frozen_with_slot_id,
                id,
                frozen_with_model_id,
                frozen_with_slot_role_id,
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
                frozen_container_slot_id,
                id,
                frozen_container_model_id,
                frozen_container_slot_role_id,
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
    #[diesel(table_name = freezing_procedure_templates)]
    pub struct FreezingProcedureTemplate {
        /// Identity shared with the parent template.
        #[same_as(
            aps_procedure_templates::procedure_template_slots::procedure_template_id,
            (frozen_with_slot_id, frozen_container_slot_id)
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing the parent resource.
        #[same_as(aps_procedure_templates::procedure_templates::organization_id)]
        #[same_as(
            aps_procedure_templates::procedure_template_slots::organization_id,
            (frozen_with_slot_id, frozen_container_slot_id)
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Slot declaring the `frozen_with` assets.
        #[mandatory(aps_procedure_templates::procedure_template_slots)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        frozen_with_slot_id: ::rosetta_uuid::Uuid,
        /// Admissible model for the `frozen_with` slot.
        #[same_as(
            aps_procedure_templates::procedure_template_slots::asset_model_id,
            frozen_with_slot_id
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        frozen_with_model_id: ::rosetta_uuid::Uuid,
        /// Required role of the `frozen_with` slot.
        #[table_model(generated)]
        frozen_with_slot_role_id: String,
        /// Slot declaring the `frozen_container` assets.
        #[mandatory(aps_procedure_templates::procedure_template_slots)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        frozen_container_slot_id: ::rosetta_uuid::Uuid,
        /// Admissible model for the `frozen_container` slot.
        #[same_as(
            aps_procedure_templates::procedure_template_slots::asset_model_id,
            frozen_container_slot_id
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        frozen_container_model_id: ::rosetta_uuid::Uuid,
        /// Required role of the `frozen_container` slot.
        #[table_model(generated)]
        frozen_container_slot_role_id: String,
        /// Target temperature in kelvin.
        #[table_model(default = 203.15f64)]
        temperature_k: f64,
        /// Allowed temperature deviation as a percentage of the target.
        #[table_model(default = 5f64)]
        temperature_tolerance_percentage: f64,
        /// Treatment duration in seconds.
        #[table_model(default = 43200f64)]
        duration_s: f64,
        /// Allowed duration deviation as a percentage of the target.
        #[table_model(default = 5f64)]
        duration_tolerance_percentage: f64,
    }
    impl ::diesel_builders::ValidateColumn<freezing_procedure_templates::temperature_k>
        for <freezing_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(temperature_k: &f64) -> Result<(), Self::Error> {
            use diesel::Column;
            if temperature_k <= &0f64 {
                return Err(
                    ::validation_errors::ValidationError::strictly_greater_than_value(
                        <crate::freezing_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::freezing_procedure_templates::temperature_k::NAME,
                        0f64,
                    ),
                );
            }
            if temperature_k.partial_cmp(&f64::INFINITY).is_none_or(::core::cmp::Ordering::is_ge) {
                return Err(
                    ::validation_errors::ValidationError::check_violated(
                        <crate::freezing_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        &[crate::freezing_procedure_templates::temperature_k::NAME],
                        "temperature_k < 'Infinity'::DOUBLE PRECISION",
                    ),
                );
            }
            Ok(())
        }
    }
    impl
        ::diesel_builders::ValidateColumn<
            freezing_procedure_templates::temperature_tolerance_percentage,
        > for <freezing_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(temperature_tolerance_percentage: &f64) -> Result<(), Self::Error> {
            use diesel::Column;
            if temperature_tolerance_percentage <= &0f64 {
                return Err(
                    ::validation_errors::ValidationError::strictly_greater_than_value(
                        <crate::freezing_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::freezing_procedure_templates::temperature_tolerance_percentage::NAME,
                        0f64,
                    ),
                );
            }
            if temperature_tolerance_percentage > &100f64 {
                return Err(
                    ::validation_errors::ValidationError::smaller_than_value(
                        <crate::freezing_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::freezing_procedure_templates::temperature_tolerance_percentage::NAME,
                        100f64,
                    ),
                );
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateColumn<freezing_procedure_templates::duration_s>
        for <freezing_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(duration_s: &f64) -> Result<(), Self::Error> {
            use diesel::Column;
            if duration_s <= &1800f64 {
                return Err(
                    ::validation_errors::ValidationError::strictly_greater_than_value(
                        <crate::freezing_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::freezing_procedure_templates::duration_s::NAME,
                        1800f64,
                    ),
                );
            }
            if duration_s > &604800f64 {
                return Err(
                    ::validation_errors::ValidationError::smaller_than_value(
                        <crate::freezing_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::freezing_procedure_templates::duration_s::NAME,
                        604800f64,
                    ),
                );
            }
            Ok(())
        }
    }
    impl
        ::diesel_builders::ValidateColumn<
            freezing_procedure_templates::duration_tolerance_percentage,
        > for <freezing_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(duration_tolerance_percentage: &f64) -> Result<(), Self::Error> {
            use diesel::Column;
            if duration_tolerance_percentage <= &0f64 {
                return Err(
                    ::validation_errors::ValidationError::strictly_greater_than_value(
                        <crate::freezing_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::freezing_procedure_templates::duration_tolerance_percentage::NAME,
                        0f64,
                    ),
                );
            }
            if duration_tolerance_percentage > &100f64 {
                return Err(
                    ::validation_errors::ValidationError::smaller_than_value(
                        <crate::freezing_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::freezing_procedure_templates::duration_tolerance_percentage::NAME,
                        100f64,
                    ),
                );
            }
            Ok(())
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id> for FreezingProcedureTemplate {
        fn get_column_ref(
            &self,
        ) -> &<freezing_procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_procedure_templates::procedure_templates::id>
        for FreezingProcedureTemplate
    {
        fn get_column_ref(
            &self,
        ) -> &<freezing_procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for FreezingProcedureTemplate {
        fn get_column_ref(
            &self,
        ) -> &<freezing_procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    /// Shape of `freezing_procedure_templates` the procedure runner renders.
    pub const FREEZING_PROCEDURE_TEMPLATES_SHAPE: procedure_like::shape::TemplateShape =
        procedure_like::shape::TemplateShape {
            table_name: "freezing_procedure_templates",
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
            ],
            slots: &[
                procedure_like::shape::SlotShape {
                    name: "frozen_with",
                    role: procedure_like::shape::SlotRole::Instrument,
                    model_table_name: "freezer_models",
                    slot_column: "frozen_with_slot_id",
                    model_column: "frozen_with_model_id",
                    placed_into: None,
                    retrieved_from: None,
                },
                procedure_like::shape::SlotShape {
                    name: "frozen_container",
                    role: procedure_like::shape::SlotRole::Input,
                    model_table_name: "volumetric_container_models",
                    slot_column: "frozen_container_slot_id",
                    model_column: "frozen_container_model_id",
                    placed_into: None,
                    retrieved_from: None,
                },
            ],
        };
    ::diesel::allow_tables_to_appear_in_same_query!(
        freezing_procedure_templates,
        ::aps_freezer_models::freezer_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        freezing_procedure_templates,
        ::aps_resources::ownables
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        freezing_procedure_templates,
        ::aps_procedure_templates::procedure_template_slots
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        freezing_procedure_templates,
        ::aps_procedure_templates::procedure_templates
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        freezing_procedure_templates,
        ::aps_resources::resources
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        freezing_procedure_templates,
        ::aps_physical_asset_models::volumetric_container_models
    );
}
pub use freezing_procedure_templates_model::*;
pub mod freezing_procedures_model {
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
    /// Recorded execution of a freezing template.
    #[table_model(ancestors(
        aps_resources::resources,
        aps_resources::ownables,
        aps_procedures::procedures
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[table_model(record_error = ::validation_errors::ValidationError)]
    #[diesel(
        belongs_to(
            aps_freezer_models::FreezingProcedureTemplate,
            foreign_key = freezing_procedure_template_id
        )
    )]
    #[table_model(foreign_key((id), (::aps_procedures::procedures::id)))]
    #[table_model(
        foreign_key(
            (freezing_procedure_template_id),
            (::aps_freezer_models::freezing_procedure_templates::id)
        )
    )]
    #[table_model(default(
        aps_resources::resources::kind,
        ::aps_authz_resource_kind::ResourceKind::Ownable
    ))]
    #[diesel(table_name = freezing_procedures)]
    pub struct FreezingProcedure {
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
        freezing_procedure_template_id: ::rosetta_uuid::Uuid,
        /// Execution status copied from the parent procedure.
        #[same_as(aps_procedures::procedures::status_id)]
        #[infallible]
        status_id: String,
        /// Measured temperature k.
        temperature_k: Option<f64>,
    }
    impl ::diesel_builders::ValidateColumn<freezing_procedures::temperature_k>
        for <freezing_procedures::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(temperature_k: &f64) -> Result<(), Self::Error> {
            use diesel::Column;
            if temperature_k <= &0f64 {
                return Err(::validation_errors::ValidationError::strictly_greater_than_value(
                    <crate::freezing_procedures::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::freezing_procedures::temperature_k::NAME,
                    0f64,
                ));
            }
            if temperature_k.partial_cmp(&f64::INFINITY).is_none_or(::core::cmp::Ordering::is_ge) {
                return Err(::validation_errors::ValidationError::check_violated(
                    <crate::freezing_procedures::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    &[crate::freezing_procedures::temperature_k::NAME],
                    "temperature_k < 'Infinity'::DOUBLE PRECISION",
                ));
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateRecord<freezing_procedures::table>
        for <freezing_procedures::table as ::diesel_builders::TableExt>::NewValues
    {
        #[inline]
        fn validate_record(&self) -> Result<(), ::validation_errors::ValidationError> {
            use diesel::Column;
            if let (Some(status_id), Some(temperature_k)) = (
                <Self as diesel_builders::MayGetColumn<
                    freezing_procedures::status_id,
                >>::may_get_column_ref(self),
                <Self as diesel_builders::MayGetColumn<
                    freezing_procedures::temperature_k,
                >>::may_get_column_ref(self),
            ) && !((status_id.as_str() != "completed") || (*temperature_k).is_some())
            {
                return Err(
                    ::validation_errors::ValidationError::check_violated(
                        <crate::freezing_procedures::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        &[
                            crate::freezing_procedures::status_id::NAME,
                            crate::freezing_procedures::temperature_k::NAME,
                        ],
                        "status_id <> 'completed' OR temperature_k IS NOT NULL",
                    ),
                );
            }
            Ok(())
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id> for FreezingProcedure {
        fn get_column_ref(
            &self,
        ) -> &<freezing_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_procedures::procedures::id> for FreezingProcedure {
        fn get_column_ref(
            &self,
        ) -> &<freezing_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for FreezingProcedure {
        fn get_column_ref(
            &self,
        ) -> &<freezing_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl procedure_like::ProcedureTableLike for freezing_procedures::table {
        type ProcedureTemplateTable = aps_freezer_models::freezing_procedure_templates::table;
    }
    /// Shape of `freezing_procedures` the procedure runner renders.
    pub const FREEZING_PROCEDURES_SHAPE: procedure_like::shape::ProcedureShape =
        procedure_like::shape::ProcedureShape {
            table_name: "freezing_procedures",
            template_table_name: "freezing_procedure_templates",
            measurements: &[procedure_like::shape::ColumnShape {
                name: "temperature_k",
                label: "Measured temperature k.",
                sql_type: procedure_like::shape::SqlType::DoublePrecision,
                unit: Some(procedure_like::shape::Unit::Kelvin),
                required: true,
            }],
        };
    ::diesel::allow_tables_to_appear_in_same_query!(
        freezing_procedures,
        ::aps_freezer_models::freezing_procedure_templates
    );
    ::diesel::allow_tables_to_appear_in_same_query!(freezing_procedures, ::aps_resources::ownables);
    ::diesel::allow_tables_to_appear_in_same_query!(
        freezing_procedures,
        ::aps_procedures::procedures
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        freezing_procedures,
        ::aps_resources::resources
    );
}
pub use freezing_procedures_model::*;
