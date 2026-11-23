#![allow(ambiguous_glob_reexports, reason = "TableModel trait names can collide across tables")]
extern crate self as aps_procedures;
pub mod container_sealer_models_model {
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
    /// Catalog of models whose primary role is sealing container openings.
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
    #[diesel(table_name = container_sealer_models)]
    pub struct ContainerSealerModel {
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
    impl ::diesel_builders::GetColumn<aps_resources::asset_models::id> for ContainerSealerModel {
        fn get_column_ref(
            &self,
        ) -> &<container_sealer_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id> for ContainerSealerModel {
        fn get_column_ref(
            &self,
        ) -> &<container_sealer_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_physical_asset_models::physical_asset_models::id>
        for ContainerSealerModel
    {
        fn get_column_ref(
            &self,
        ) -> &<container_sealer_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for ContainerSealerModel {
        fn get_column_ref(
            &self,
        ) -> &<container_sealer_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    /// Shape of `container_sealer_models` the procedure runner renders.
    pub const CONTAINER_SEALER_MODELS_SHAPE: procedure_like::shape::AssetModelShape =
        procedure_like::shape::AssetModelShape {
            table_name: "container_sealer_models",
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
        container_sealer_models,
        ::aps_resources::asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        container_sealer_models,
        ::aps_resources::ownables
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        container_sealer_models,
        ::aps_physical_asset_models::physical_asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        container_sealer_models,
        ::aps_resources::resources
    );
}
pub use container_sealer_models_model::*;
pub mod container_sealing_procedure_templates_model {
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
    /// Typed slots and parameters for container sealing.
    #[table_model(ancestors(
        aps_resources::resources,
        aps_resources::ownables,
        aps_procedure_templates::procedure_templates
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_procedure_templates::ProcedureTemplate, foreign_key = id))]
    #[diesel(
        belongs_to(
            aps_physical_asset_models::VolumetricContainerModel,
            foreign_key = sealable_container_model_id
        )
    )]
    #[diesel(
        belongs_to(
            aps_procedures::ContainerSealerModel,
            foreign_key = sealed_with_model_id
        )
    )]
    #[table_model(
        foreign_key((id), (::aps_procedure_templates::procedure_templates::id))
    )]
    #[table_model(
        foreign_key(
            (sealable_container_model_id),
            (::aps_physical_asset_models::volumetric_container_models::id)
        )
    )]
    #[table_model(
        foreign_key(
            (sealed_with_model_id),
            (::aps_procedures::container_sealer_models::id)
        )
    )]
    #[table_model(
        foreign_key(
            (
                sealable_container_slot_id,
                id,
                sealable_container_model_id,
                sealable_container_slot_role_id,
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
                sealed_with_slot_id,
                id,
                sealed_with_model_id,
                sealed_with_slot_role_id,
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
    #[diesel(table_name = container_sealing_procedure_templates)]
    pub struct ContainerSealingProcedureTemplate {
        /// Identity shared with the parent template.
        #[same_as(
            aps_procedure_templates::procedure_template_slots::procedure_template_id,
            (sealable_container_slot_id, sealed_with_slot_id)
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing the parent resource.
        #[same_as(aps_procedure_templates::procedure_templates::organization_id)]
        #[same_as(
            aps_procedure_templates::procedure_template_slots::organization_id,
            (sealable_container_slot_id, sealed_with_slot_id)
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Slot declaring the `sealable_container` assets.
        #[mandatory(aps_procedure_templates::procedure_template_slots)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        sealable_container_slot_id: ::rosetta_uuid::Uuid,
        /// Admissible model for the `sealable_container` slot.
        #[same_as(
            aps_procedure_templates::procedure_template_slots::asset_model_id,
            sealable_container_slot_id
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        sealable_container_model_id: ::rosetta_uuid::Uuid,
        /// Required role of the `sealable_container` slot.
        #[table_model(generated)]
        sealable_container_slot_role_id: String,
        /// Slot declaring the `sealed_with` assets.
        #[mandatory(aps_procedure_templates::procedure_template_slots)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        sealed_with_slot_id: ::rosetta_uuid::Uuid,
        /// Admissible model for the `sealed_with` slot.
        #[same_as(
            aps_procedure_templates::procedure_template_slots::asset_model_id,
            sealed_with_slot_id
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        sealed_with_model_id: ::rosetta_uuid::Uuid,
        /// Required role of the `sealed_with` slot.
        #[table_model(generated)]
        sealed_with_slot_role_id: String,
    }
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id>
        for ContainerSealingProcedureTemplate
    {
        fn get_column_ref(
            &self,
        ) -> &<container_sealing_procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType{
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_procedure_templates::procedure_templates::id>
        for ContainerSealingProcedureTemplate
    {
        fn get_column_ref(
            &self,
        ) -> &<container_sealing_procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType{
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id>
        for ContainerSealingProcedureTemplate
    {
        fn get_column_ref(
            &self,
        ) -> &<container_sealing_procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType{
            &self.id
        }
    }
    /// Shape of `container_sealing_procedure_templates` the procedure runner
    /// renders.
    pub const CONTAINER_SEALING_PROCEDURE_TEMPLATES_SHAPE: procedure_like::shape::TemplateShape =
        procedure_like::shape::TemplateShape {
            table_name: "container_sealing_procedure_templates",
            setpoints: &[],
            slots: &[
                procedure_like::shape::SlotShape {
                    name: "sealable_container",
                    role: procedure_like::shape::SlotRole::Input,
                    model_table_name: "volumetric_container_models",
                    slot_column: "sealable_container_slot_id",
                    model_column: "sealable_container_model_id",
                    placed_into: None,
                    retrieved_from: None,
                },
                procedure_like::shape::SlotShape {
                    name: "sealed_with",
                    role: procedure_like::shape::SlotRole::Input,
                    model_table_name: "container_sealer_models",
                    slot_column: "sealed_with_slot_id",
                    model_column: "sealed_with_model_id",
                    placed_into: None,
                    retrieved_from: None,
                },
            ],
        };
    ::diesel::allow_tables_to_appear_in_same_query!(
        container_sealing_procedure_templates,
        ::aps_procedures::container_sealer_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        container_sealing_procedure_templates,
        ::aps_resources::ownables
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        container_sealing_procedure_templates,
        ::aps_procedure_templates::procedure_template_slots
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        container_sealing_procedure_templates,
        ::aps_procedure_templates::procedure_templates
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        container_sealing_procedure_templates,
        ::aps_resources::resources
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        container_sealing_procedure_templates,
        ::aps_physical_asset_models::volumetric_container_models
    );
}
pub use container_sealing_procedure_templates_model::*;
pub mod container_sealing_procedures_model {
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
    /// Recorded execution of a container sealing template.
    #[table_model(ancestors(
        aps_resources::resources,
        aps_resources::ownables,
        aps_procedures::procedures
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(
        belongs_to(
            aps_procedures::ContainerSealingProcedureTemplate,
            foreign_key = container_sealing_procedure_template_id
        )
    )]
    #[table_model(foreign_key((id), (::aps_procedures::procedures::id)))]
    #[table_model(
        foreign_key(
            (container_sealing_procedure_template_id),
            (::aps_procedures::container_sealing_procedure_templates::id)
        )
    )]
    #[table_model(default(
        aps_resources::resources::kind,
        ::aps_authz_resource_kind::ResourceKind::Ownable
    ))]
    #[diesel(table_name = container_sealing_procedures)]
    pub struct ContainerSealingProcedure {
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
        container_sealing_procedure_template_id: ::rosetta_uuid::Uuid,
        /// Execution status copied from the parent procedure.
        #[same_as(aps_procedures::procedures::status_id)]
        #[infallible]
        status_id: String,
    }
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id> for ContainerSealingProcedure {
        fn get_column_ref(
            &self,
        ) -> &<container_sealing_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_procedures::procedures::id> for ContainerSealingProcedure {
        fn get_column_ref(
            &self,
        ) -> &<container_sealing_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for ContainerSealingProcedure {
        fn get_column_ref(
            &self,
        ) -> &<container_sealing_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl procedure_like::ProcedureTableLike for container_sealing_procedures::table {
        type ProcedureTemplateTable = aps_procedures::container_sealing_procedure_templates::table;
    }
    /// Shape of `container_sealing_procedures` the procedure runner renders.
    pub const CONTAINER_SEALING_PROCEDURES_SHAPE: procedure_like::shape::ProcedureShape =
        procedure_like::shape::ProcedureShape {
            table_name: "container_sealing_procedures",
            template_table_name: "container_sealing_procedure_templates",
            measurements: &[],
        };
    ::diesel::allow_tables_to_appear_in_same_query!(
        container_sealing_procedures,
        ::aps_procedures::container_sealing_procedure_templates
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        container_sealing_procedures,
        ::aps_resources::ownables
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        container_sealing_procedures,
        ::aps_procedures::procedures
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        container_sealing_procedures,
        ::aps_resources::resources
    );
}
pub use container_sealing_procedures_model::*;
pub mod disposal_procedure_templates_model {
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
    /// Typed slots and parameters for disposal.
    #[table_model(ancestors(
        aps_resources::resources,
        aps_resources::ownables,
        aps_procedure_templates::procedure_templates
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_procedure_templates::ProcedureTemplate, foreign_key = id))]
    #[diesel(
        belongs_to(
            aps_physical_asset_models::PhysicalAssetModel,
            foreign_key = disposed_asset_model_id
        )
    )]
    #[table_model(
        foreign_key((id), (::aps_procedure_templates::procedure_templates::id))
    )]
    #[table_model(
        foreign_key(
            (disposed_asset_model_id),
            (::aps_physical_asset_models::physical_asset_models::id)
        )
    )]
    #[table_model(
        foreign_key(
            (
                disposed_asset_slot_id,
                id,
                disposed_asset_model_id,
                disposed_asset_slot_role_id,
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
    #[diesel(table_name = disposal_procedure_templates)]
    pub struct DisposalProcedureTemplate {
        /// Identity shared with the parent template.
        #[same_as(
            aps_procedure_templates::procedure_template_slots::procedure_template_id,
            disposed_asset_slot_id
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing the parent resource.
        #[same_as(aps_procedure_templates::procedure_templates::organization_id)]
        #[same_as(
            aps_procedure_templates::procedure_template_slots::organization_id,
            disposed_asset_slot_id
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Slot declaring the `disposed_asset` assets.
        #[mandatory(aps_procedure_templates::procedure_template_slots)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        disposed_asset_slot_id: ::rosetta_uuid::Uuid,
        /// Admissible model for the `disposed_asset` slot.
        #[same_as(
            aps_procedure_templates::procedure_template_slots::asset_model_id,
            disposed_asset_slot_id
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        disposed_asset_model_id: ::rosetta_uuid::Uuid,
        /// Required role of the `disposed_asset` slot.
        #[table_model(generated)]
        disposed_asset_slot_role_id: String,
    }
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id> for DisposalProcedureTemplate {
        fn get_column_ref(
            &self,
        ) -> &<disposal_procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_procedure_templates::procedure_templates::id>
        for DisposalProcedureTemplate
    {
        fn get_column_ref(
            &self,
        ) -> &<disposal_procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for DisposalProcedureTemplate {
        fn get_column_ref(
            &self,
        ) -> &<disposal_procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    /// Shape of `disposal_procedure_templates` the procedure runner renders.
    pub const DISPOSAL_PROCEDURE_TEMPLATES_SHAPE: procedure_like::shape::TemplateShape =
        procedure_like::shape::TemplateShape {
            table_name: "disposal_procedure_templates",
            setpoints: &[],
            slots: &[procedure_like::shape::SlotShape {
                name: "disposed_asset",
                role: procedure_like::shape::SlotRole::Consumed,
                model_table_name: "physical_asset_models",
                slot_column: "disposed_asset_slot_id",
                model_column: "disposed_asset_model_id",
                placed_into: None,
                retrieved_from: None,
            }],
        };
    ::diesel::allow_tables_to_appear_in_same_query!(
        disposal_procedure_templates,
        ::aps_resources::ownables
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        disposal_procedure_templates,
        ::aps_physical_asset_models::physical_asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        disposal_procedure_templates,
        ::aps_procedure_templates::procedure_template_slots
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        disposal_procedure_templates,
        ::aps_procedure_templates::procedure_templates
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        disposal_procedure_templates,
        ::aps_resources::resources
    );
}
pub use disposal_procedure_templates_model::*;
pub mod disposal_procedures_model {
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
    /// Recorded execution of a disposal template.
    #[table_model(ancestors(
        aps_resources::resources,
        aps_resources::ownables,
        aps_procedures::procedures
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(
        belongs_to(
            aps_procedures::DisposalProcedureTemplate,
            foreign_key = disposal_procedure_template_id
        )
    )]
    #[table_model(foreign_key((id), (::aps_procedures::procedures::id)))]
    #[table_model(
        foreign_key(
            (disposal_procedure_template_id),
            (::aps_procedures::disposal_procedure_templates::id)
        )
    )]
    #[table_model(default(
        aps_resources::resources::kind,
        ::aps_authz_resource_kind::ResourceKind::Ownable
    ))]
    #[diesel(table_name = disposal_procedures)]
    pub struct DisposalProcedure {
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
        disposal_procedure_template_id: ::rosetta_uuid::Uuid,
        /// Execution status copied from the parent procedure.
        #[same_as(aps_procedures::procedures::status_id)]
        #[infallible]
        status_id: String,
    }
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id> for DisposalProcedure {
        fn get_column_ref(
            &self,
        ) -> &<disposal_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_procedures::procedures::id> for DisposalProcedure {
        fn get_column_ref(
            &self,
        ) -> &<disposal_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for DisposalProcedure {
        fn get_column_ref(
            &self,
        ) -> &<disposal_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl procedure_like::ProcedureTableLike for disposal_procedures::table {
        type ProcedureTemplateTable = aps_procedures::disposal_procedure_templates::table;
    }
    /// Shape of `disposal_procedures` the procedure runner renders.
    pub const DISPOSAL_PROCEDURES_SHAPE: procedure_like::shape::ProcedureShape =
        procedure_like::shape::ProcedureShape {
            table_name: "disposal_procedures",
            template_table_name: "disposal_procedure_templates",
            measurements: &[],
        };
    ::diesel::allow_tables_to_appear_in_same_query!(
        disposal_procedures,
        ::aps_procedures::disposal_procedure_templates
    );
    ::diesel::allow_tables_to_appear_in_same_query!(disposal_procedures, ::aps_resources::ownables);
    ::diesel::allow_tables_to_appear_in_same_query!(
        disposal_procedures,
        ::aps_procedures::procedures
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        disposal_procedures,
        ::aps_resources::resources
    );
}
pub use disposal_procedures_model::*;
pub mod parameter_value_kinds_model {
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
    /// Kind of value a procedure template parameter takes.
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(table_name = parameter_value_kinds)]
    pub struct ParameterValueKind {
        /// Kind name.
        id: String,
    }
    impl ::diesel_builders::ValidateColumn<parameter_value_kinds::id>
        for <parameter_value_kinds::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(id: &String) -> Result<(), Self::Error> {
            use diesel::Column;
            if id.is_empty() {
                return Err(
                    ::validation_errors::ValidationError::empty(
                        <crate::parameter_value_kinds::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::parameter_value_kinds::id::NAME,
                    ),
                );
            }
            if id.chars().count() > 63usize {
                return Err(
                    ::validation_errors::ValidationError::exceeds_max_length(
                        <crate::parameter_value_kinds::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::parameter_value_kinds::id::NAME,
                        63usize,
                    ),
                );
            }
            Ok(())
        }
    }
}
pub use parameter_value_kinds_model::*;
pub mod procedure_parameter_values_model {
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
    /// The value a procedure records for a parameter its template declares. The
    /// status is a copy of the procedure's, kept by the cascade.
    #[table_model(error = ::validation_errors::ValidationError)]
    #[table_model(record_error = ::validation_errors::ValidationError)]
    #[diesel(primary_key(procedure_id, parameter_id))]
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
            (parameter_id, procedure_template_id, value_kind_id),
            (
                ::aps_procedures::procedure_template_parameters::id,
                ::aps_procedures::procedure_template_parameters::procedure_template_id,
                ::aps_procedures::procedure_template_parameters::value_kind_id
            )
        )
    )]
    #[table_model(
        foreign_key(
            (organization_id, procedure_id),
            (::aps_resources::resources::organization_id, ::aps_resources::resources::id)
        )
    )]
    #[diesel(table_name = procedure_parameter_values)]
    pub struct ProcedureParameterValue {
        /// Procedure recording the value.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        procedure_id: ::rosetta_uuid::Uuid,
        /// Parameter of the procedure's template.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        parameter_id: ::rosetta_uuid::Uuid,
        /// Template of the procedure.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        procedure_template_id: ::rosetta_uuid::Uuid,
        /// Status of the procedure, kept equal by the cascade.
        #[infallible]
        status_id: String,
        /// Kind of the parameter's value.
        #[infallible]
        value_kind_id: String,
        /// connetto organization owning the procedure.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Value of a number parameter, in the parameter's unit.
        number_value: Option<f64>,
        /// Value of a text parameter.
        text_value: Option<String>,
        /// Value of a boolean parameter.
        #[infallible]
        boolean_value: Option<bool>,
    }
    impl ::diesel_builders::ValidateColumn<procedure_parameter_values::number_value>
        for <procedure_parameter_values::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(number_value: &f64) -> Result<(), Self::Error> {
            use diesel::Column;
            if number_value.partial_cmp(&f64::NEG_INFINITY).is_none_or(::core::cmp::Ordering::is_le)
            {
                return Err(
                    ::validation_errors::ValidationError::check_violated(
                        <crate::procedure_parameter_values::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        &[crate::procedure_parameter_values::number_value::NAME],
                        "number_value > '-Infinity'::DOUBLE PRECISION",
                    ),
                );
            }
            if number_value.partial_cmp(&f64::INFINITY).is_none_or(::core::cmp::Ordering::is_ge) {
                return Err(
                    ::validation_errors::ValidationError::check_violated(
                        <crate::procedure_parameter_values::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        &[crate::procedure_parameter_values::number_value::NAME],
                        "number_value < 'Infinity'::DOUBLE PRECISION",
                    ),
                );
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateColumn<procedure_parameter_values::text_value>
        for <procedure_parameter_values::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(text_value: &String) -> Result<(), Self::Error> {
            use diesel::Column;
            if text_value.is_empty() {
                return Err(
                    ::validation_errors::ValidationError::empty(
                        <crate::procedure_parameter_values::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::procedure_parameter_values::text_value::NAME,
                    ),
                );
            }
            if text_value.chars().count() > 8192usize {
                return Err(
                    ::validation_errors::ValidationError::exceeds_max_length(
                        <crate::procedure_parameter_values::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::procedure_parameter_values::text_value::NAME,
                        8192usize,
                    ),
                );
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateRecord<procedure_parameter_values::table>
        for <procedure_parameter_values::table as ::diesel_builders::TableExt>::NewValues
    {
        #[inline]
        fn validate_record(&self) -> Result<(), ::validation_errors::ValidationError> {
            use diesel::Column;
            if let (Some(number_value), Some(value_kind_id)) =
                (
                    <Self as diesel_builders::MayGetColumn<
                        procedure_parameter_values::number_value,
                    >>::may_get_column_ref(self),
                    <Self as diesel_builders::MayGetColumn<
                        procedure_parameter_values::value_kind_id,
                    >>::may_get_column_ref(self),
                )
                && (*number_value).is_some() != (value_kind_id.as_str() == "number")
            {
                return Err(
                    ::validation_errors::ValidationError::check_violated(
                        <crate::procedure_parameter_values::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        &[
                            crate::procedure_parameter_values::number_value::NAME,
                            crate::procedure_parameter_values::value_kind_id::NAME,
                        ],
                        "(number_value IS NOT NULL) = (value_kind_id = 'number')",
                    ),
                );
            }
            if let (Some(text_value), Some(value_kind_id)) =
                (
                    <Self as diesel_builders::MayGetColumn<
                        procedure_parameter_values::text_value,
                    >>::may_get_column_ref(self),
                    <Self as diesel_builders::MayGetColumn<
                        procedure_parameter_values::value_kind_id,
                    >>::may_get_column_ref(self),
                )
                && text_value.as_deref().is_some() != (value_kind_id.as_str() == "text")
            {
                return Err(
                    ::validation_errors::ValidationError::check_violated(
                        <crate::procedure_parameter_values::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        &[
                            crate::procedure_parameter_values::text_value::NAME,
                            crate::procedure_parameter_values::value_kind_id::NAME,
                        ],
                        "(text_value IS NOT NULL) = (value_kind_id = 'text')",
                    ),
                );
            }
            if let (Some(boolean_value), Some(value_kind_id)) = (
                <Self as diesel_builders::MayGetColumn<
                    procedure_parameter_values::boolean_value,
                >>::may_get_column_ref(self),
                <Self as diesel_builders::MayGetColumn<
                    procedure_parameter_values::value_kind_id,
                >>::may_get_column_ref(self),
            ) && (*boolean_value).is_some() != (value_kind_id.as_str() == "boolean")
            {
                return Err(
                    ::validation_errors::ValidationError::check_violated(
                        <crate::procedure_parameter_values::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        &[
                            crate::procedure_parameter_values::boolean_value::NAME,
                            crate::procedure_parameter_values::value_kind_id::NAME,
                        ],
                        "(boolean_value IS NOT NULL) = (value_kind_id = 'boolean')",
                    ),
                );
            }
            Ok(())
        }
    }
    ::diesel::allow_tables_to_appear_in_same_query!(
        procedure_parameter_values,
        ::aps_procedures::procedure_template_parameters
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        procedure_parameter_values,
        ::aps_procedures::procedures
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        procedure_parameter_values,
        ::aps_resources::resources
    );
}
pub use procedure_parameter_values_model::*;
pub mod procedure_statuses_model {
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
    /// Lifecycle of a procedure. The triggers in 005-procedures enforce which
    /// transitions are allowed.
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(table_name = procedure_statuses)]
    pub struct ProcedureStatus {
        /// Status name.
        id: String,
    }
    impl ::diesel_builders::ValidateColumn<procedure_statuses::id>
        for <procedure_statuses::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(id: &String) -> Result<(), Self::Error> {
            use diesel::Column;
            if id.is_empty() {
                return Err(::validation_errors::ValidationError::empty(
                    <crate::procedure_statuses::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::procedure_statuses::id::NAME,
                ));
            }
            if id.chars().count() > 63usize {
                return Err(::validation_errors::ValidationError::exceeds_max_length(
                    <crate::procedure_statuses::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::procedure_statuses::id::NAME,
                    63usize,
                ));
            }
            Ok(())
        }
    }
}
pub use procedure_statuses_model::*;
pub mod procedure_template_parameters_model {
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
    /// A named procedure value with an optional default in its quantity kind's
    /// unit.
    #[table_model(error = ::validation_errors::ValidationError)]
    #[table_model(record_error = ::validation_errors::ValidationError)]
    #[diesel(
        belongs_to(
            aps_procedure_templates::ProcedureTemplate,
            foreign_key = procedure_template_id
        )
    )]
    #[diesel(
        belongs_to(aps_procedures::ParameterValueKind, foreign_key = value_kind_id)
    )]
    #[diesel(belongs_to(aps_procedures::QuantityKind, foreign_key = quantity_kind_id))]
    #[table_model(
        foreign_key(
            (procedure_template_id),
            (::aps_procedure_templates::procedure_templates::id)
        )
    )]
    #[table_model(
        foreign_key((value_kind_id), (::aps_procedures::parameter_value_kinds::id))
    )]
    #[table_model(
        foreign_key((quantity_kind_id), (::aps_procedures::quantity_kinds::id))
    )]
    #[table_model(
        foreign_key(
            (organization_id, procedure_template_id),
            (::aps_resources::resources::organization_id, ::aps_resources::resources::id)
        )
    )]
    #[diesel(table_name = procedure_template_parameters)]
    pub struct ProcedureTemplateParameter {
        /// Identifier of the parameter.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// connetto organization owning the template.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Template declaring the parameter.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        procedure_template_id: ::rosetta_uuid::Uuid,
        /// Name, unique within the template.
        name: String,
        /// Free-text description of what the value means.
        description: Option<String>,
        /// Kind of the value, `number`, `text` or `boolean`.
        #[infallible]
        value_kind_id: String,
        /// Quantity kind fixing the canonical unit of a number.
        #[infallible]
        quantity_kind_id: Option<String>,
        /// Whether a procedure completes only with a value recorded.
        #[table_model(default = false)]
        #[infallible]
        required: bool,
        /// Default of a number parameter.
        default_number: Option<f64>,
        /// Lowest admissible value before a procedure records a deviation.
        minimum_number: Option<f64>,
        /// Highest admissible value before a procedure records a deviation.
        maximum_number: Option<f64>,
        /// Default of a text parameter.
        default_text: Option<String>,
        /// Default of a boolean parameter.
        #[infallible]
        default_boolean: Option<bool>,
    }
    ::diesel_builders::prelude::unique_index!(
        procedure_template_parameters::procedure_template_id,
        procedure_template_parameters::name
    );
    ::diesel_builders::prelude::unique_index!(
        procedure_template_parameters::id,
        procedure_template_parameters::procedure_template_id,
        procedure_template_parameters::value_kind_id
    );
    impl ::diesel_builders::ValidateColumn<procedure_template_parameters::name>
        for <procedure_template_parameters::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(name: &String) -> Result<(), Self::Error> {
            use diesel::Column;
            if name.is_empty() {
                return Err(
                    ::validation_errors::ValidationError::empty(
                        <crate::procedure_template_parameters::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::procedure_template_parameters::name::NAME,
                    ),
                );
            }
            if name.chars().count() > 255usize {
                return Err(
                    ::validation_errors::ValidationError::exceeds_max_length(
                        <crate::procedure_template_parameters::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::procedure_template_parameters::name::NAME,
                        255usize,
                    ),
                );
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateColumn<procedure_template_parameters::description>
        for <procedure_template_parameters::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(description: &String) -> Result<(), Self::Error> {
            use diesel::Column;
            if description.is_empty() {
                return Err(
                    ::validation_errors::ValidationError::empty(
                        <crate::procedure_template_parameters::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::procedure_template_parameters::description::NAME,
                    ),
                );
            }
            if description.chars().count() > 8192usize {
                return Err(
                    ::validation_errors::ValidationError::exceeds_max_length(
                        <crate::procedure_template_parameters::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::procedure_template_parameters::description::NAME,
                        8192usize,
                    ),
                );
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateColumn<procedure_template_parameters::default_number>
        for <procedure_template_parameters::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(default_number: &f64) -> Result<(), Self::Error> {
            use diesel::Column;
            if default_number
                .partial_cmp(&f64::NEG_INFINITY)
                .is_none_or(::core::cmp::Ordering::is_le)
            {
                return Err(
                    ::validation_errors::ValidationError::check_violated(
                        <crate::procedure_template_parameters::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        &[crate::procedure_template_parameters::default_number::NAME],
                        "default_number > '-Infinity'::DOUBLE PRECISION",
                    ),
                );
            }
            if default_number.partial_cmp(&f64::INFINITY).is_none_or(::core::cmp::Ordering::is_ge) {
                return Err(
                    ::validation_errors::ValidationError::check_violated(
                        <crate::procedure_template_parameters::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        &[crate::procedure_template_parameters::default_number::NAME],
                        "default_number < 'Infinity'::DOUBLE PRECISION",
                    ),
                );
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateColumn<procedure_template_parameters::minimum_number>
        for <procedure_template_parameters::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(minimum_number: &f64) -> Result<(), Self::Error> {
            use diesel::Column;
            if minimum_number
                .partial_cmp(&f64::NEG_INFINITY)
                .is_none_or(::core::cmp::Ordering::is_le)
            {
                return Err(
                    ::validation_errors::ValidationError::check_violated(
                        <crate::procedure_template_parameters::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        &[crate::procedure_template_parameters::minimum_number::NAME],
                        "minimum_number > '-Infinity'::DOUBLE PRECISION",
                    ),
                );
            }
            if minimum_number.partial_cmp(&f64::INFINITY).is_none_or(::core::cmp::Ordering::is_ge) {
                return Err(
                    ::validation_errors::ValidationError::check_violated(
                        <crate::procedure_template_parameters::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        &[crate::procedure_template_parameters::minimum_number::NAME],
                        "minimum_number < 'Infinity'::DOUBLE PRECISION",
                    ),
                );
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateColumn<procedure_template_parameters::maximum_number>
        for <procedure_template_parameters::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(maximum_number: &f64) -> Result<(), Self::Error> {
            use diesel::Column;
            if maximum_number
                .partial_cmp(&f64::NEG_INFINITY)
                .is_none_or(::core::cmp::Ordering::is_le)
            {
                return Err(
                    ::validation_errors::ValidationError::check_violated(
                        <crate::procedure_template_parameters::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        &[crate::procedure_template_parameters::maximum_number::NAME],
                        "maximum_number > '-Infinity'::DOUBLE PRECISION",
                    ),
                );
            }
            if maximum_number.partial_cmp(&f64::INFINITY).is_none_or(::core::cmp::Ordering::is_ge) {
                return Err(
                    ::validation_errors::ValidationError::check_violated(
                        <crate::procedure_template_parameters::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        &[crate::procedure_template_parameters::maximum_number::NAME],
                        "maximum_number < 'Infinity'::DOUBLE PRECISION",
                    ),
                );
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateColumn<procedure_template_parameters::default_text>
        for <procedure_template_parameters::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(default_text: &String) -> Result<(), Self::Error> {
            use diesel::Column;
            if default_text.is_empty() {
                return Err(
                    ::validation_errors::ValidationError::empty(
                        <crate::procedure_template_parameters::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::procedure_template_parameters::default_text::NAME,
                    ),
                );
            }
            if default_text.chars().count() > 8192usize {
                return Err(
                    ::validation_errors::ValidationError::exceeds_max_length(
                        <crate::procedure_template_parameters::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::procedure_template_parameters::default_text::NAME,
                        8192usize,
                    ),
                );
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateRecord<procedure_template_parameters::table>
        for <procedure_template_parameters::table as ::diesel_builders::TableExt>::NewValues
    {
        #[inline]
        fn validate_record(&self) -> Result<(), ::validation_errors::ValidationError> {
            use diesel::Column;
            if let (Some(quantity_kind_id), Some(value_kind_id)) = (
                <Self as diesel_builders::MayGetColumn<
                    procedure_template_parameters::quantity_kind_id,
                >>::may_get_column_ref(self),
                <Self as diesel_builders::MayGetColumn<
                    procedure_template_parameters::value_kind_id,
                >>::may_get_column_ref(self),
            ) && quantity_kind_id.as_deref().is_some() != (value_kind_id.as_str() == "number")
            {
                return Err(
                    ::validation_errors::ValidationError::check_violated(
                        <crate::procedure_template_parameters::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        &[
                            crate::procedure_template_parameters::quantity_kind_id::NAME,
                            crate::procedure_template_parameters::value_kind_id::NAME,
                        ],
                        "(quantity_kind_id IS NOT NULL) = (value_kind_id = 'number')",
                    ),
                );
            }
            if let (Some(value_kind_id), Some(minimum_number), Some(maximum_number)) = (
                <Self as diesel_builders::MayGetColumn<
                    procedure_template_parameters::value_kind_id,
                >>::may_get_column_ref(self),
                <Self as diesel_builders::MayGetColumn<
                    procedure_template_parameters::minimum_number,
                >>::may_get_column_ref(self),
                <Self as diesel_builders::MayGetColumn<
                    procedure_template_parameters::maximum_number,
                >>::may_get_column_ref(self),
            ) && !((value_kind_id.as_str() == "number")
                || ((*minimum_number).is_none() && (*maximum_number).is_none()))
            {
                return Err(
                    ::validation_errors::ValidationError::check_violated(
                        <crate::procedure_template_parameters::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        &[
                            crate::procedure_template_parameters::value_kind_id::NAME,
                            crate::procedure_template_parameters::minimum_number::NAME,
                            crate::procedure_template_parameters::maximum_number::NAME,
                        ],
                        "value_kind_id = 'number' OR (minimum_number IS NULL AND maximum_number IS NULL)",
                    ),
                );
            }
            if let (Some(minimum_number), Some(maximum_number)) = (
                <Self as diesel_builders::MayGetColumn<
                    procedure_template_parameters::minimum_number,
                >>::may_get_column_ref(self),
                <Self as diesel_builders::MayGetColumn<
                    procedure_template_parameters::maximum_number,
                >>::may_get_column_ref(self),
            ) && minimum_number.as_ref().is_some_and(|minimum_number| {
                maximum_number
                    .as_ref()
                    .is_some_and(|maximum_number| minimum_number > maximum_number)
            }) {
                return Err(
                    ::validation_errors::ValidationError::smaller_than(
                        <crate::procedure_template_parameters::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::procedure_template_parameters::minimum_number::NAME,
                        crate::procedure_template_parameters::maximum_number::NAME,
                    ),
                );
            }
            if let (Some(default_number), Some(minimum_number), Some(maximum_number)) = (
                <Self as diesel_builders::MayGetColumn<
                    procedure_template_parameters::default_number,
                >>::may_get_column_ref(self),
                <Self as diesel_builders::MayGetColumn<
                    procedure_template_parameters::minimum_number,
                >>::may_get_column_ref(self),
                <Self as diesel_builders::MayGetColumn<
                    procedure_template_parameters::maximum_number,
                >>::may_get_column_ref(self),
            ) {
                if default_number.as_ref().is_some_and(|default_number| {
                    minimum_number
                        .as_ref()
                        .is_some_and(|minimum_number| default_number < minimum_number)
                }) {
                    return Err(
                        ::validation_errors::ValidationError::greater_than(
                            <crate::procedure_template_parameters::table as ::diesel_builders::TableExt>::TABLE_NAME,
                            crate::procedure_template_parameters::default_number::NAME,
                            crate::procedure_template_parameters::minimum_number::NAME,
                        ),
                    );
                }
                if default_number.as_ref().is_some_and(|default_number| {
                    maximum_number
                        .as_ref()
                        .is_some_and(|maximum_number| default_number > maximum_number)
                }) {
                    return Err(
                        ::validation_errors::ValidationError::smaller_than(
                            <crate::procedure_template_parameters::table as ::diesel_builders::TableExt>::TABLE_NAME,
                            crate::procedure_template_parameters::default_number::NAME,
                            crate::procedure_template_parameters::maximum_number::NAME,
                        ),
                    );
                }
            }
            if let (Some(default_number), Some(value_kind_id)) = (
                <Self as diesel_builders::MayGetColumn<
                    procedure_template_parameters::default_number,
                >>::may_get_column_ref(self),
                <Self as diesel_builders::MayGetColumn<
                    procedure_template_parameters::value_kind_id,
                >>::may_get_column_ref(self),
            ) && !((*default_number).is_none() || (value_kind_id.as_str() == "number"))
            {
                return Err(
                    ::validation_errors::ValidationError::check_violated(
                        <crate::procedure_template_parameters::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        &[
                            crate::procedure_template_parameters::default_number::NAME,
                            crate::procedure_template_parameters::value_kind_id::NAME,
                        ],
                        "default_number IS NULL OR value_kind_id = 'number'",
                    ),
                );
            }
            if let (Some(default_text), Some(value_kind_id)) = (
                <Self as diesel_builders::MayGetColumn<
                    procedure_template_parameters::default_text,
                >>::may_get_column_ref(self),
                <Self as diesel_builders::MayGetColumn<
                    procedure_template_parameters::value_kind_id,
                >>::may_get_column_ref(self),
            ) && !(default_text.as_deref().is_none() || (value_kind_id.as_str() == "text"))
            {
                return Err(
                    ::validation_errors::ValidationError::check_violated(
                        <crate::procedure_template_parameters::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        &[
                            crate::procedure_template_parameters::default_text::NAME,
                            crate::procedure_template_parameters::value_kind_id::NAME,
                        ],
                        "default_text IS NULL OR value_kind_id = 'text'",
                    ),
                );
            }
            if let (Some(default_boolean), Some(value_kind_id)) = (
                <Self as diesel_builders::MayGetColumn<
                    procedure_template_parameters::default_boolean,
                >>::may_get_column_ref(self),
                <Self as diesel_builders::MayGetColumn<
                    procedure_template_parameters::value_kind_id,
                >>::may_get_column_ref(self),
            ) && !((*default_boolean).is_none() || (value_kind_id.as_str() == "boolean"))
            {
                return Err(
                    ::validation_errors::ValidationError::check_violated(
                        <crate::procedure_template_parameters::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        &[
                            crate::procedure_template_parameters::default_boolean::NAME,
                            crate::procedure_template_parameters::value_kind_id::NAME,
                        ],
                        "default_boolean IS NULL OR value_kind_id = 'boolean'",
                    ),
                );
            }
            Ok(())
        }
    }
    ::diesel::allow_tables_to_appear_in_same_query!(
        procedure_template_parameters,
        ::aps_procedures::parameter_value_kinds
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        procedure_template_parameters,
        ::aps_procedure_templates::procedure_templates
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        procedure_template_parameters,
        ::aps_procedures::quantity_kinds
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        procedure_template_parameters,
        ::aps_resources::resources
    );
}
pub use procedure_template_parameters_model::*;
pub mod procedure_template_step_parameters_model {
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
    /// The value a parent template fixes for a parameter of a step's child
    /// template, replacing the child's default in that step.
    #[table_model(error = ::validation_errors::ValidationError)]
    #[table_model(record_error = ::validation_errors::ValidationError)]
    #[diesel(primary_key(step_id, parent_template_id, parameter_id))]
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
            (parameter_id, child_template_id, value_kind_id),
            (
                ::aps_procedures::procedure_template_parameters::id,
                ::aps_procedures::procedure_template_parameters::procedure_template_id,
                ::aps_procedures::procedure_template_parameters::value_kind_id
            )
        )
    )]
    #[table_model(
        foreign_key(
            (organization_id, parent_template_id),
            (::aps_resources::resources::organization_id, ::aps_resources::resources::id)
        )
    )]
    #[diesel(table_name = procedure_template_step_parameters)]
    pub struct ProcedureTemplateStepParameter {
        /// Step whose child parameter is fixed.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        step_id: ::rosetta_uuid::Uuid,
        /// Parameter of the step's child template.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        parameter_id: ::rosetta_uuid::Uuid,
        /// Template the step belongs to.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        parent_template_id: ::rosetta_uuid::Uuid,
        /// Template the step runs.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        child_template_id: ::rosetta_uuid::Uuid,
        /// Kind of the parameter's value.
        #[infallible]
        value_kind_id: String,
        /// connetto organization owning the parent template.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Value of a number parameter.
        number_value: Option<f64>,
        /// Value of a text parameter.
        text_value: Option<String>,
        /// Value of a boolean parameter.
        #[infallible]
        boolean_value: Option<bool>,
    }
    impl ::diesel_builders::ValidateColumn<procedure_template_step_parameters::number_value>
        for <procedure_template_step_parameters::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(number_value: &f64) -> Result<(), Self::Error> {
            use diesel::Column;
            if number_value.partial_cmp(&f64::NEG_INFINITY).is_none_or(::core::cmp::Ordering::is_le)
            {
                return Err(
                    ::validation_errors::ValidationError::check_violated(
                        <crate::procedure_template_step_parameters::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        &[crate::procedure_template_step_parameters::number_value::NAME],
                        "number_value > '-Infinity'::DOUBLE PRECISION",
                    ),
                );
            }
            if number_value.partial_cmp(&f64::INFINITY).is_none_or(::core::cmp::Ordering::is_ge) {
                return Err(
                    ::validation_errors::ValidationError::check_violated(
                        <crate::procedure_template_step_parameters::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        &[crate::procedure_template_step_parameters::number_value::NAME],
                        "number_value < 'Infinity'::DOUBLE PRECISION",
                    ),
                );
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateColumn<procedure_template_step_parameters::text_value>
        for <procedure_template_step_parameters::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(text_value: &String) -> Result<(), Self::Error> {
            use diesel::Column;
            if text_value.is_empty() {
                return Err(
                    ::validation_errors::ValidationError::empty(
                        <crate::procedure_template_step_parameters::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::procedure_template_step_parameters::text_value::NAME,
                    ),
                );
            }
            if text_value.chars().count() > 8192usize {
                return Err(
                    ::validation_errors::ValidationError::exceeds_max_length(
                        <crate::procedure_template_step_parameters::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::procedure_template_step_parameters::text_value::NAME,
                        8192usize,
                    ),
                );
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateRecord<procedure_template_step_parameters::table>
        for <procedure_template_step_parameters::table as ::diesel_builders::TableExt>::NewValues
    {
        #[inline]
        fn validate_record(&self) -> Result<(), ::validation_errors::ValidationError> {
            use diesel::Column;
            if let (Some(number_value), Some(value_kind_id)) = (
                <Self as diesel_builders::MayGetColumn<
                    procedure_template_step_parameters::number_value,
                >>::may_get_column_ref(self),
                <Self as diesel_builders::MayGetColumn<
                    procedure_template_step_parameters::value_kind_id,
                >>::may_get_column_ref(self),
            ) && (*number_value).is_some() != (value_kind_id.as_str() == "number")
            {
                return Err(
                    ::validation_errors::ValidationError::check_violated(
                        <crate::procedure_template_step_parameters::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        &[
                            crate::procedure_template_step_parameters::number_value::NAME,
                            crate::procedure_template_step_parameters::value_kind_id::NAME,
                        ],
                        "(number_value IS NOT NULL) = (value_kind_id = 'number')",
                    ),
                );
            }
            if let (Some(text_value), Some(value_kind_id)) = (
                <Self as diesel_builders::MayGetColumn<
                    procedure_template_step_parameters::text_value,
                >>::may_get_column_ref(self),
                <Self as diesel_builders::MayGetColumn<
                    procedure_template_step_parameters::value_kind_id,
                >>::may_get_column_ref(self),
            ) && text_value.as_deref().is_some() != (value_kind_id.as_str() == "text")
            {
                return Err(
                    ::validation_errors::ValidationError::check_violated(
                        <crate::procedure_template_step_parameters::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        &[
                            crate::procedure_template_step_parameters::text_value::NAME,
                            crate::procedure_template_step_parameters::value_kind_id::NAME,
                        ],
                        "(text_value IS NOT NULL) = (value_kind_id = 'text')",
                    ),
                );
            }
            if let (Some(boolean_value), Some(value_kind_id)) = (
                <Self as diesel_builders::MayGetColumn<
                    procedure_template_step_parameters::boolean_value,
                >>::may_get_column_ref(self),
                <Self as diesel_builders::MayGetColumn<
                    procedure_template_step_parameters::value_kind_id,
                >>::may_get_column_ref(self),
            ) && (*boolean_value).is_some() != (value_kind_id.as_str() == "boolean")
            {
                return Err(
                    ::validation_errors::ValidationError::check_violated(
                        <crate::procedure_template_step_parameters::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        &[
                            crate::procedure_template_step_parameters::boolean_value::NAME,
                            crate::procedure_template_step_parameters::value_kind_id::NAME,
                        ],
                        "(boolean_value IS NOT NULL) = (value_kind_id = 'boolean')",
                    ),
                );
            }
            Ok(())
        }
    }
    ::diesel::allow_tables_to_appear_in_same_query!(
        procedure_template_step_parameters,
        ::aps_procedures::procedure_template_parameters
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        procedure_template_step_parameters,
        ::aps_procedure_templates::procedure_template_steps
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        procedure_template_step_parameters,
        ::aps_resources::resources
    );
}
pub use procedure_template_step_parameters_model::*;
pub mod procedures_model {
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
    /// A procedure is one execution of a published template. A composite
    /// template runs as a parent procedure whose children each execute one of
    /// its steps, once, or once per asset of the step's repeat slot.
    #[table_model(ancestors(aps_resources::resources, aps_resources::ownables))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[table_model(record_error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_resources::TableName, foreign_key = table_name_id))]
    #[diesel(
        belongs_to(
            aps_procedure_templates::ProcedureTemplate,
            foreign_key = procedure_template_id
        )
    )]
    #[diesel(belongs_to(aps_procedures::ProcedureStatus, foreign_key = status_id))]
    #[diesel(belongs_to(aps_grantees::User, foreign_key = performed_by_id))]
    #[table_model(foreign_key((id), (::aps_resources::ownables::id)))]
    #[table_model(foreign_key((table_name_id), (::aps_resources::table_names::id)))]
    #[table_model(
        foreign_key(
            (procedure_template_id),
            (::aps_procedure_templates::procedure_templates::id)
        )
    )]
    #[table_model(foreign_key((status_id), (::aps_procedures::procedure_statuses::id)))]
    #[table_model(foreign_key((performed_by_id), (::aps_grantees::users::id)))]
    #[table_model(
        foreign_key(
            (parent_procedure_id, parent_template_id, organization_id),
            (
                procedures::id,
                procedures::procedure_template_id,
                procedures::organization_id
            )
        )
    )]
    #[table_model(
        foreign_key(
            (parent_procedure_id, root_procedure_id, organization_id),
            (procedures::id, procedures::root_procedure_id, procedures::organization_id)
        )
    )]
    #[table_model(
        foreign_key(
            (parent_procedure_id, project_id, organization_id),
            (procedures::id, procedures::project_id, procedures::organization_id)
        )
    )]
    #[table_model(
        foreign_key(
            (step_id, parent_template_id, procedure_template_id),
            (
                ::aps_procedure_templates::procedure_template_steps::id,
                ::aps_procedure_templates::procedure_template_steps::parent_template_id,
                ::aps_procedure_templates::procedure_template_steps::child_template_id
            )
        )
    )]
    #[table_model(default(
        aps_resources::resources::kind,
        ::aps_authz_resource_kind::ResourceKind::Ownable
    ))]
    #[diesel(table_name = procedures)]
    pub struct Procedure {
        /// Identifier of the connetto ownable this procedure extends.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// connetto organization owning the row.
        #[same_as(aps_resources::ownables::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// connetto project holding the procedure.
        #[same_as(aps_resources::ownables::project_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        project_id: ::rosetta_uuid::Uuid,
        /// Most concrete APS table holding this procedure.
        #[infallible]
        table_name_id: String,
        /// Template the procedure executes.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        procedure_template_id: ::rosetta_uuid::Uuid,
        /// Lifecycle status, `planned`, `in_progress`, `completed`, `aborted`
        /// or `skipped`. Only a step is skipped, and skipping a non-optional
        /// step records a deviation whose reason is in `note`.
        #[table_model(default = "planned")]
        #[infallible]
        status_id: String,
        /// Procedure whose step this procedure executes.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        parent_procedure_id: Option<::rosetta_uuid::Uuid>,
        /// Template of the parent procedure.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        parent_template_id: Option<::rosetta_uuid::Uuid>,
        /// The step of the parent template this procedure executes.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        step_id: Option<::rosetta_uuid::Uuid>,
        /// Top-level procedure of the run this procedure belongs to, itself for
        /// a procedure without a parent.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        root_procedure_id: ::rosetta_uuid::Uuid,
        /// Parent binding of the repeat slot this run of a repeated step is
        /// for.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        repeat_procedure_asset_id: Option<::rosetta_uuid::Uuid>,
        /// When the procedure is planned to happen.
        #[infallible]
        #[diesel(sql_type = ::rosetta_utc::diesel_impls::TimestampUTC)]
        planned_for: Option<::rosetta_utc::TimestampUTC>,
        /// When the procedure started, by the clock of the device that recorded
        /// it, which uploads the change later when offline. A time entered by
        /// hand may lie earlier, and none lies beyond the clock tolerance
        /// ahead.
        #[diesel(sql_type = ::rosetta_utc::diesel_impls::TimestampUTC)]
        started_at: Option<::rosetta_utc::TimestampUTC>,
        /// When the procedure completed, was aborted or was skipped, by the
        /// same device clock and under the same bound as `started_at`.
        #[diesel(sql_type = ::rosetta_utc::diesel_impls::TimestampUTC)]
        ended_at: Option<::rosetta_utc::TimestampUTC>,
        /// connetto user who performed the procedure.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        performed_by_id: Option<::rosetta_uuid::Uuid>,
        /// Free-text note, editable in every status.
        note: Option<String>,
    }
    ::diesel_builders::prelude::unique_index!(
        procedures::id,
        procedures::procedure_template_id,
        procedures::organization_id
    );
    ::diesel_builders::prelude::unique_index!(
        procedures::id,
        procedures::procedure_template_id,
        procedures::status_id,
        procedures::organization_id
    );
    ::diesel_builders::prelude::unique_index!(
        procedures::id,
        procedures::root_procedure_id,
        procedures::organization_id
    );
    ::diesel_builders::prelude::unique_index!(
        procedures::id,
        procedures::parent_procedure_id,
        procedures::organization_id
    );
    ::diesel_builders::prelude::unique_index!(
        procedures::id,
        procedures::project_id,
        procedures::organization_id
    );
    impl ::diesel_builders::ValidateColumn<procedures::started_at>
        for <procedures::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(started_at: &::rosetta_utc::TimestampUTC) -> Result<(), Self::Error> {
            use diesel::Column;
            if *started_at
                > ::rosetta_utc::TimestampUTC::from(
                    *::rosetta_utc::TimestampUTC::now() + ::core::time::Duration::from_secs(300),
                )
            {
                return Err(::validation_errors::ValidationError::check_violated(
                    <crate::procedures::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    &[crate::procedures::started_at::NAME],
                    "started_at <= now() + INTERVAL '5 minutes'",
                ));
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateColumn<procedures::ended_at>
        for <procedures::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(ended_at: &::rosetta_utc::TimestampUTC) -> Result<(), Self::Error> {
            use diesel::Column;
            if *ended_at
                > ::rosetta_utc::TimestampUTC::from(
                    *::rosetta_utc::TimestampUTC::now() + ::core::time::Duration::from_secs(300),
                )
            {
                return Err(::validation_errors::ValidationError::check_violated(
                    <crate::procedures::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    &[crate::procedures::ended_at::NAME],
                    "ended_at <= now() + INTERVAL '5 minutes'",
                ));
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateColumn<procedures::note>
        for <procedures::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(note: &String) -> Result<(), Self::Error> {
            use diesel::Column;
            if note.is_empty() {
                return Err(::validation_errors::ValidationError::empty(
                    <crate::procedures::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::procedures::note::NAME,
                ));
            }
            if note.chars().count() > 65536usize {
                return Err(::validation_errors::ValidationError::exceeds_max_length(
                    <crate::procedures::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::procedures::note::NAME,
                    65536usize,
                ));
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateRecord<procedures::table>
        for <procedures::table as ::diesel_builders::TableExt>::NewValues
    {
        #[inline]
        fn validate_record(&self) -> Result<(), ::validation_errors::ValidationError> {
            use diesel::Column;
            if let (Some(parent_procedure_id), Some(id)) = (
                <Self as diesel_builders::MayGetColumn<
                    procedures::parent_procedure_id,
                >>::may_get_column_ref(self),
                <Self as diesel_builders::MayGetColumn<
                    procedures::id,
                >>::may_get_column_ref(self),
            )
                && parent_procedure_id
                    .as_ref()
                    .is_some_and(|parent_procedure_id| parent_procedure_id == id)
            {
                return Err(
                    ::validation_errors::ValidationError::equal(
                        <crate::procedures::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::procedures::parent_procedure_id::NAME,
                        crate::procedures::id::NAME,
                    ),
                );
            }
            if let (Some(parent_procedure_id), Some(step_id)) = (
                <Self as diesel_builders::MayGetColumn<
                    procedures::parent_procedure_id,
                >>::may_get_column_ref(self),
                <Self as diesel_builders::MayGetColumn<
                    procedures::step_id,
                >>::may_get_column_ref(self),
            ) && (*parent_procedure_id).is_none() != (*step_id).is_none()
            {
                return Err(
                    ::validation_errors::ValidationError::check_violated(
                        <crate::procedures::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        &[
                            crate::procedures::parent_procedure_id::NAME,
                            crate::procedures::step_id::NAME,
                        ],
                        "(parent_procedure_id IS NULL) = (step_id IS NULL)",
                    ),
                );
            }
            if let (Some(parent_procedure_id), Some(parent_template_id)) = (
                <Self as diesel_builders::MayGetColumn<
                    procedures::parent_procedure_id,
                >>::may_get_column_ref(self),
                <Self as diesel_builders::MayGetColumn<
                    procedures::parent_template_id,
                >>::may_get_column_ref(self),
            ) && (*parent_procedure_id).is_none() != (*parent_template_id).is_none()
            {
                return Err(
                    ::validation_errors::ValidationError::check_violated(
                        <crate::procedures::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        &[
                            crate::procedures::parent_procedure_id::NAME,
                            crate::procedures::parent_template_id::NAME,
                        ],
                        "(parent_procedure_id IS NULL) = (parent_template_id IS NULL)",
                    ),
                );
            }
            if let (Some(repeat_procedure_asset_id), Some(step_id)) = (
                <Self as diesel_builders::MayGetColumn<
                    procedures::repeat_procedure_asset_id,
                >>::may_get_column_ref(self),
                <Self as diesel_builders::MayGetColumn<
                    procedures::step_id,
                >>::may_get_column_ref(self),
            ) && !((*repeat_procedure_asset_id).is_none() || (*step_id).is_some())
            {
                return Err(
                    ::validation_errors::ValidationError::check_violated(
                        <crate::procedures::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        &[
                            crate::procedures::repeat_procedure_asset_id::NAME,
                            crate::procedures::step_id::NAME,
                        ],
                        "repeat_procedure_asset_id IS NULL OR step_id IS NOT NULL",
                    ),
                );
            }
            if let (Some(status_id), Some(started_at), Some(ended_at)) = (
                <Self as diesel_builders::MayGetColumn<procedures::status_id>>::may_get_column_ref(
                    self,
                ),
                <Self as diesel_builders::MayGetColumn<procedures::started_at>>::may_get_column_ref(
                    self,
                ),
                <Self as diesel_builders::MayGetColumn<procedures::ended_at>>::may_get_column_ref(
                    self,
                ),
            ) && !((status_id.as_str() != "planned")
                || ((*started_at).is_none() && (*ended_at).is_none()))
            {
                return Err(::validation_errors::ValidationError::check_violated(
                    <crate::procedures::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    &[
                        crate::procedures::status_id::NAME,
                        crate::procedures::started_at::NAME,
                        crate::procedures::ended_at::NAME,
                    ],
                    "status_id <> 'planned' OR (started_at IS NULL AND ended_at IS NULL)",
                ));
            }
            if let (Some(status_id), Some(started_at), Some(ended_at)) = (
                <Self as diesel_builders::MayGetColumn<procedures::status_id>>::may_get_column_ref(
                    self,
                ),
                <Self as diesel_builders::MayGetColumn<procedures::started_at>>::may_get_column_ref(
                    self,
                ),
                <Self as diesel_builders::MayGetColumn<procedures::ended_at>>::may_get_column_ref(
                    self,
                ),
            ) && !((status_id.as_str() != "in_progress")
                || ((*started_at).is_some() && (*ended_at).is_none()))
            {
                return Err(::validation_errors::ValidationError::check_violated(
                    <crate::procedures::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    &[
                        crate::procedures::status_id::NAME,
                        crate::procedures::started_at::NAME,
                        crate::procedures::ended_at::NAME,
                    ],
                    "status_id <> 'in_progress' OR (started_at IS NOT NULL AND ended_at IS NULL)",
                ));
            }
            if let (Some(status_id), Some(started_at), Some(ended_at)) = (
                <Self as diesel_builders::MayGetColumn<procedures::status_id>>::may_get_column_ref(
                    self,
                ),
                <Self as diesel_builders::MayGetColumn<procedures::started_at>>::may_get_column_ref(
                    self,
                ),
                <Self as diesel_builders::MayGetColumn<procedures::ended_at>>::may_get_column_ref(
                    self,
                ),
            ) && !((status_id.as_str() != "completed")
                || ((*started_at).is_some() && (*ended_at).is_some()))
            {
                return Err(::validation_errors::ValidationError::check_violated(
                    <crate::procedures::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    &[
                        crate::procedures::status_id::NAME,
                        crate::procedures::started_at::NAME,
                        crate::procedures::ended_at::NAME,
                    ],
                    "status_id <> 'completed' OR (started_at IS NOT NULL AND ended_at IS NOT NULL)",
                ));
            }
            if let (Some(status_id), Some(ended_at)) = (
                <Self as diesel_builders::MayGetColumn<procedures::status_id>>::may_get_column_ref(
                    self,
                ),
                <Self as diesel_builders::MayGetColumn<procedures::ended_at>>::may_get_column_ref(
                    self,
                ),
            ) && !((status_id.as_str() != "aborted") || (*ended_at).is_some())
            {
                return Err(::validation_errors::ValidationError::check_violated(
                    <crate::procedures::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    &[crate::procedures::status_id::NAME, crate::procedures::ended_at::NAME],
                    "status_id <> 'aborted' OR ended_at IS NOT NULL",
                ));
            }
            if let (Some(status_id), Some(step_id), Some(started_at), Some(ended_at)) = (
                <Self as diesel_builders::MayGetColumn<procedures::status_id>>::may_get_column_ref(
                    self,
                ),
                <Self as diesel_builders::MayGetColumn<procedures::step_id>>::may_get_column_ref(
                    self,
                ),
                <Self as diesel_builders::MayGetColumn<procedures::started_at>>::may_get_column_ref(
                    self,
                ),
                <Self as diesel_builders::MayGetColumn<procedures::ended_at>>::may_get_column_ref(
                    self,
                ),
            ) && !((status_id.as_str() != "skipped")
                || (((*step_id).is_some() && (*started_at).is_none()) && (*ended_at).is_some()))
            {
                return Err(::validation_errors::ValidationError::check_violated(
                    <crate::procedures::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    &[
                        crate::procedures::status_id::NAME,
                        crate::procedures::step_id::NAME,
                        crate::procedures::started_at::NAME,
                        crate::procedures::ended_at::NAME,
                    ],
                    "status_id <> 'skipped' OR (step_id IS NOT NULL AND started_at IS NULL AND ended_at IS NOT NULL)",
                ));
            }
            if let (Some(parent_procedure_id), Some(root_procedure_id), Some(id)) = (
                <Self as diesel_builders::MayGetColumn<
                    procedures::parent_procedure_id,
                >>::may_get_column_ref(self),
                <Self as diesel_builders::MayGetColumn<
                    procedures::root_procedure_id,
                >>::may_get_column_ref(self),
                <Self as diesel_builders::MayGetColumn<
                    procedures::id,
                >>::may_get_column_ref(self),
            ) && (*parent_procedure_id).is_none() != (*root_procedure_id == *id)
            {
                return Err(
                    ::validation_errors::ValidationError::check_violated(
                        <crate::procedures::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        &[
                            crate::procedures::parent_procedure_id::NAME,
                            crate::procedures::root_procedure_id::NAME,
                            crate::procedures::id::NAME,
                        ],
                        "(parent_procedure_id IS NULL) = (root_procedure_id = id)",
                    ),
                );
            }
            if let (Some(ended_at), Some(started_at)) = (
                <Self as diesel_builders::MayGetColumn<procedures::ended_at>>::may_get_column_ref(
                    self,
                ),
                <Self as diesel_builders::MayGetColumn<procedures::started_at>>::may_get_column_ref(
                    self,
                ),
            ) && ended_at.as_ref().is_some_and(|ended_at| {
                started_at.as_ref().is_some_and(|started_at| ended_at < started_at)
            }) {
                return Err(::validation_errors::ValidationError::greater_than(
                    <crate::procedures::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::procedures::ended_at::NAME,
                    crate::procedures::started_at::NAME,
                ));
            }
            if let (Some(status_id), Some(performed_by_id)) = (
                <Self as diesel_builders::MayGetColumn<
                    procedures::status_id,
                >>::may_get_column_ref(self),
                <Self as diesel_builders::MayGetColumn<
                    procedures::performed_by_id,
                >>::may_get_column_ref(self),
            )
                && !(((status_id.as_str() != "in_progress")
                    && (status_id.as_str() != "completed"))
                    || (*performed_by_id).is_some())
            {
                return Err(
                    ::validation_errors::ValidationError::check_violated(
                        <crate::procedures::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        &[
                            crate::procedures::status_id::NAME,
                            crate::procedures::performed_by_id::NAME,
                        ],
                        "status_id NOT IN ('in_progress', 'completed') OR performed_by_id IS NOT NULL",
                    ),
                );
            }
            Ok(())
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id> for Procedure {
        fn get_column_ref(
            &self,
        ) -> &<procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for Procedure {
        fn get_column_ref(
            &self,
        ) -> &<procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl procedure_like::ProcedureTableLike for procedures::table {
        type ProcedureTemplateTable = aps_procedure_templates::procedure_templates::table;
    }
    ::diesel::allow_tables_to_appear_in_same_query!(procedures, ::aps_resources::ownables);
    ::diesel::allow_tables_to_appear_in_same_query!(
        procedures,
        ::aps_procedures::procedure_statuses
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        procedures,
        ::aps_procedure_templates::procedure_template_steps
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        procedures,
        ::aps_procedure_templates::procedure_templates
    );
    ::diesel::allow_tables_to_appear_in_same_query!(procedures, ::aps_resources::resources);
    ::diesel::allow_tables_to_appear_in_same_query!(procedures, ::aps_resources::table_names);
    ::diesel::allow_tables_to_appear_in_same_query!(procedures, ::aps_grantees::users);
}
pub use procedures_model::*;
pub mod quantity_kinds_model {
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
    /// Canonical units for generic numeric parameters.
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(table_name = quantity_kinds)]
    pub struct QuantityKind {
        /// Quantity name.
        id: String,
        /// Symbol displayed beside a stored value.
        unit_symbol: String,
        /// UCUM code of the stored unit.
        ucum_code: String,
    }
    impl ::diesel_builders::ValidateColumn<quantity_kinds::id>
        for <quantity_kinds::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(id: &String) -> Result<(), Self::Error> {
            use diesel::Column;
            if id.is_empty() {
                return Err(::validation_errors::ValidationError::empty(
                    <crate::quantity_kinds::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::quantity_kinds::id::NAME,
                ));
            }
            if id.chars().count() > 63usize {
                return Err(::validation_errors::ValidationError::exceeds_max_length(
                    <crate::quantity_kinds::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::quantity_kinds::id::NAME,
                    63usize,
                ));
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateColumn<quantity_kinds::unit_symbol>
        for <quantity_kinds::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(unit_symbol: &String) -> Result<(), Self::Error> {
            use diesel::Column;
            if unit_symbol.is_empty() {
                return Err(::validation_errors::ValidationError::empty(
                    <crate::quantity_kinds::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::quantity_kinds::unit_symbol::NAME,
                ));
            }
            if unit_symbol.chars().count() > 63usize {
                return Err(::validation_errors::ValidationError::exceeds_max_length(
                    <crate::quantity_kinds::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::quantity_kinds::unit_symbol::NAME,
                    63usize,
                ));
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateColumn<quantity_kinds::ucum_code>
        for <quantity_kinds::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(ucum_code: &String) -> Result<(), Self::Error> {
            use diesel::Column;
            if ucum_code.is_empty() {
                return Err(::validation_errors::ValidationError::empty(
                    <crate::quantity_kinds::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::quantity_kinds::ucum_code::NAME,
                ));
            }
            if ucum_code.chars().count() > 63usize {
                return Err(::validation_errors::ValidationError::exceeds_max_length(
                    <crate::quantity_kinds::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::quantity_kinds::ucum_code::NAME,
                    63usize,
                ));
            }
            Ok(())
        }
    }
}
pub use quantity_kinds_model::*;
pub mod retrieval_procedures_model {
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
    /// Recorded execution of a retrieval template.
    #[table_model(ancestors(
        aps_resources::resources,
        aps_resources::ownables,
        aps_procedures::procedures
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(
        belongs_to(
            aps_physical_asset_models::RetrievalProcedureTemplate,
            foreign_key = retrieval_procedure_template_id
        )
    )]
    #[table_model(foreign_key((id), (::aps_procedures::procedures::id)))]
    #[table_model(
        foreign_key(
            (retrieval_procedure_template_id),
            (::aps_physical_asset_models::retrieval_procedure_templates::id)
        )
    )]
    #[table_model(default(
        aps_resources::resources::kind,
        ::aps_authz_resource_kind::ResourceKind::Ownable
    ))]
    #[diesel(table_name = retrieval_procedures)]
    pub struct RetrievalProcedure {
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
        retrieval_procedure_template_id: ::rosetta_uuid::Uuid,
        /// Execution status copied from the parent procedure.
        #[same_as(aps_procedures::procedures::status_id)]
        #[infallible]
        status_id: String,
    }
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id> for RetrievalProcedure {
        fn get_column_ref(
            &self,
        ) -> &<retrieval_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_procedures::procedures::id> for RetrievalProcedure {
        fn get_column_ref(
            &self,
        ) -> &<retrieval_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for RetrievalProcedure {
        fn get_column_ref(
            &self,
        ) -> &<retrieval_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl procedure_like::ProcedureTableLike for retrieval_procedures::table {
        type ProcedureTemplateTable =
            aps_physical_asset_models::retrieval_procedure_templates::table;
    }
    /// Shape of `retrieval_procedures` the procedure runner renders.
    pub const RETRIEVAL_PROCEDURES_SHAPE: procedure_like::shape::ProcedureShape =
        procedure_like::shape::ProcedureShape {
            table_name: "retrieval_procedures",
            template_table_name: "retrieval_procedure_templates",
            measurements: &[],
        };
    ::diesel::allow_tables_to_appear_in_same_query!(
        retrieval_procedures,
        ::aps_resources::ownables
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        retrieval_procedures,
        ::aps_procedures::procedures
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        retrieval_procedures,
        ::aps_resources::resources
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        retrieval_procedures,
        ::aps_physical_asset_models::retrieval_procedure_templates
    );
}
pub use retrieval_procedures_model::*;
pub mod storage_procedures_model {
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
    /// Recorded execution of a storage template.
    #[table_model(ancestors(
        aps_resources::resources,
        aps_resources::ownables,
        aps_procedures::procedures
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[table_model(record_error = ::validation_errors::ValidationError)]
    #[diesel(
        belongs_to(
            aps_physical_asset_models::StorageProcedureTemplate,
            foreign_key = storage_procedure_template_id
        )
    )]
    #[table_model(foreign_key((id), (::aps_procedures::procedures::id)))]
    #[table_model(
        foreign_key(
            (storage_procedure_template_id),
            (::aps_physical_asset_models::storage_procedure_templates::id)
        )
    )]
    #[table_model(default(
        aps_resources::resources::kind,
        ::aps_authz_resource_kind::ResourceKind::Ownable
    ))]
    #[diesel(table_name = storage_procedures)]
    pub struct StorageProcedure {
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
        storage_procedure_template_id: ::rosetta_uuid::Uuid,
        /// Execution status copied from the parent procedure.
        #[same_as(aps_procedures::procedures::status_id)]
        #[infallible]
        status_id: String,
        /// Measured temperature k.
        temperature_k: Option<f64>,
    }
    impl ::diesel_builders::ValidateColumn<storage_procedures::temperature_k>
        for <storage_procedures::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(temperature_k: &f64) -> Result<(), Self::Error> {
            use diesel::Column;
            if temperature_k <= &0f64 {
                return Err(::validation_errors::ValidationError::strictly_greater_than_value(
                    <crate::storage_procedures::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::storage_procedures::temperature_k::NAME,
                    0f64,
                ));
            }
            if temperature_k.partial_cmp(&f64::INFINITY).is_none_or(::core::cmp::Ordering::is_ge) {
                return Err(::validation_errors::ValidationError::check_violated(
                    <crate::storage_procedures::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    &[crate::storage_procedures::temperature_k::NAME],
                    "temperature_k < 'Infinity'::DOUBLE PRECISION",
                ));
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateRecord<storage_procedures::table>
        for <storage_procedures::table as ::diesel_builders::TableExt>::NewValues
    {
        #[inline]
        fn validate_record(&self) -> Result<(), ::validation_errors::ValidationError> {
            use diesel::Column;
            if let (Some(status_id), Some(temperature_k)) = (
                <Self as diesel_builders::MayGetColumn<
                    storage_procedures::status_id,
                >>::may_get_column_ref(self),
                <Self as diesel_builders::MayGetColumn<
                    storage_procedures::temperature_k,
                >>::may_get_column_ref(self),
            ) && !((status_id.as_str() != "completed") || (*temperature_k).is_some())
            {
                return Err(
                    ::validation_errors::ValidationError::check_violated(
                        <crate::storage_procedures::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        &[
                            crate::storage_procedures::status_id::NAME,
                            crate::storage_procedures::temperature_k::NAME,
                        ],
                        "status_id <> 'completed' OR temperature_k IS NOT NULL",
                    ),
                );
            }
            Ok(())
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id> for StorageProcedure {
        fn get_column_ref(
            &self,
        ) -> &<storage_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_procedures::procedures::id> for StorageProcedure {
        fn get_column_ref(
            &self,
        ) -> &<storage_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for StorageProcedure {
        fn get_column_ref(
            &self,
        ) -> &<storage_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl procedure_like::ProcedureTableLike for storage_procedures::table {
        type ProcedureTemplateTable = aps_physical_asset_models::storage_procedure_templates::table;
    }
    /// Shape of `storage_procedures` the procedure runner renders.
    pub const STORAGE_PROCEDURES_SHAPE: procedure_like::shape::ProcedureShape =
        procedure_like::shape::ProcedureShape {
            table_name: "storage_procedures",
            template_table_name: "storage_procedure_templates",
            measurements: &[procedure_like::shape::ColumnShape {
                name: "temperature_k",
                label: "Measured temperature k.",
                sql_type: procedure_like::shape::SqlType::DoublePrecision,
                unit: Some(procedure_like::shape::Unit::Kelvin),
                required: true,
            }],
        };
    ::diesel::allow_tables_to_appear_in_same_query!(storage_procedures, ::aps_resources::ownables);
    ::diesel::allow_tables_to_appear_in_same_query!(
        storage_procedures,
        ::aps_procedures::procedures
    );
    ::diesel::allow_tables_to_appear_in_same_query!(storage_procedures, ::aps_resources::resources);
    ::diesel::allow_tables_to_appear_in_same_query!(
        storage_procedures,
        ::aps_physical_asset_models::storage_procedure_templates
    );
}
pub use storage_procedures_model::*;
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
    }
    impl ::diesel_builders::GetColumn<aps_resources::asset_models::id> for WeighingDeviceModel {
        fn get_column_ref(
            &self,
        ) -> &<weighing_device_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id> for WeighingDeviceModel {
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
        ::aps_resources::ownables
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
    #[table_model(ancestors(
        aps_resources::resources,
        aps_resources::ownables,
        aps_procedure_templates::procedure_templates
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_procedure_templates::ProcedureTemplate, foreign_key = id))]
    #[diesel(
        belongs_to(
            aps_physical_asset_models::PhysicalAssetModel,
            foreign_key = weighed_asset_model_id
        )
    )]
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
        foreign_key(
            (weighed_asset_model_id),
            (::aps_physical_asset_models::physical_asset_models::id)
        )
    )]
    #[table_model(
        foreign_key(
            (weighed_with_model_id),
            (::aps_procedures::weighing_device_models::id)
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
    #[table_model(default(
        aps_resources::resources::kind,
        ::aps_authz_resource_kind::ResourceKind::Ownable
    ))]
    #[diesel(table_name = weighing_procedure_templates)]
    pub struct WeighingProcedureTemplate {
        /// Identity shared with the parent template.
        #[same_as(
            aps_procedure_templates::procedure_template_slots::procedure_template_id,
            (weighed_asset_slot_id, weighed_with_slot_id)
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing the parent resource.
        #[same_as(aps_procedure_templates::procedure_templates::organization_id)]
        #[same_as(
            aps_procedure_templates::procedure_template_slots::organization_id,
            (weighed_asset_slot_id, weighed_with_slot_id)
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Slot declaring the `weighed_asset` assets.
        #[mandatory(aps_procedure_templates::procedure_template_slots)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        weighed_asset_slot_id: ::rosetta_uuid::Uuid,
        /// Admissible model for the `weighed_asset` slot.
        #[same_as(
            aps_procedure_templates::procedure_template_slots::asset_model_id,
            weighed_asset_slot_id
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        weighed_asset_model_id: ::rosetta_uuid::Uuid,
        /// Required role of the `weighed_asset` slot.
        #[table_model(generated)]
        weighed_asset_slot_role_id: String,
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
    }
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id> for WeighingProcedureTemplate {
        fn get_column_ref(
            &self,
        ) -> &<weighing_procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_procedure_templates::procedure_templates::id>
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
        ::aps_resources::ownables
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        weighing_procedure_templates,
        ::aps_physical_asset_models::physical_asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        weighing_procedure_templates,
        ::aps_procedure_templates::procedure_template_slots
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        weighing_procedure_templates,
        ::aps_procedure_templates::procedure_templates
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        weighing_procedure_templates,
        ::aps_resources::resources
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        weighing_procedure_templates,
        ::aps_procedures::weighing_device_models
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
        ::diesel::Associations,
        ::diesel_builders::prelude::TableModel,
    )]
    /// Recorded execution of a weighing template.
    #[table_model(ancestors(
        aps_resources::resources,
        aps_resources::ownables,
        aps_procedures::procedures
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[table_model(record_error = ::validation_errors::ValidationError)]
    #[diesel(
        belongs_to(
            aps_procedures::WeighingProcedureTemplate,
            foreign_key = weighing_procedure_template_id
        )
    )]
    #[table_model(foreign_key((id), (::aps_procedures::procedures::id)))]
    #[table_model(
        foreign_key(
            (weighing_procedure_template_id),
            (::aps_procedures::weighing_procedure_templates::id)
        )
    )]
    #[table_model(default(
        aps_resources::resources::kind,
        ::aps_authz_resource_kind::ResourceKind::Ownable
    ))]
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
        #[infallible]
        status_id: String,
        /// Mass in kilograms.
        mass_kg: Option<f64>,
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
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id> for WeighingProcedure {
        fn get_column_ref(
            &self,
        ) -> &<weighing_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
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
        type ProcedureTemplateTable = aps_procedures::weighing_procedure_templates::table;
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
    ::diesel::allow_tables_to_appear_in_same_query!(weighing_procedures, ::aps_resources::ownables);
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
        ::aps_procedures::weighing_procedure_templates
    );
}
pub use weighing_procedures_model::*;
