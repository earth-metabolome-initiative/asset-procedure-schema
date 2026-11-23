#![allow(ambiguous_glob_reexports, reason = "TableModel trait names can collide across tables")]
extern crate self as aps_physical_asset_models;
pub mod container_compatibility_rules_model {
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
    /// Rule set that defines which physical asset models can be contained by a
    /// specific container model and in what quantity.
    #[table_model(ancestors(aps_resources::resources, aps_resources::ownables))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[table_model(record_error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_resources::TableName, foreign_key = table_name_id))]
    #[diesel(
        belongs_to(
            aps_physical_asset_models::ContainerModel,
            foreign_key = container_model_id
        )
    )]
    #[diesel(
        belongs_to(
            aps_physical_asset_models::PhysicalAssetModel,
            foreign_key = contained_asset_model_id
        )
    )]
    #[table_model(foreign_key((id), (::aps_resources::ownables::id)))]
    #[table_model(foreign_key((table_name_id), (::aps_resources::table_names::id)))]
    #[table_model(
        foreign_key(
            (container_model_id),
            (::aps_physical_asset_models::container_models::id)
        )
    )]
    #[table_model(
        foreign_key(
            (contained_asset_model_id),
            (::aps_physical_asset_models::physical_asset_models::id)
        )
    )]
    #[table_model(
        foreign_key(
            (container_model_id, project_id),
            (
                ::aps_resources::asset_models::id,
                ::aps_resources::asset_models::project_id
            )
        )
    )]
    #[table_model(
        foreign_key(
            (organization_id, container_model_id),
            (::aps_resources::resources::organization_id, ::aps_resources::resources::id)
        )
    )]
    #[table_model(default(
        aps_resources::resources::kind,
        ::aps_authz_resource_kind::ResourceKind::Ownable
    ))]
    #[diesel(table_name = container_compatibility_rules)]
    pub struct ContainerCompatibilityRule {
        /// Stable rule identifier inherited from `ownables`.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing this resource.
        #[same_as(aps_resources::ownables::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Project containing this ownable, the container model's project.
        #[same_as(aps_resources::ownables::project_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        project_id: ::rosetta_uuid::Uuid,
        /// Concrete APS table holding this row.
        table_name_id: String,
        /// Container model that receives contained assets.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        container_model_id: ::rosetta_uuid::Uuid,
        /// Physical asset model allowed inside the container model.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        contained_asset_model_id: ::rosetta_uuid::Uuid,
        /// Maximum allowed count of `contained_asset_model_id` in the
        /// container.
        #[table_model(default = 1i16)]
        quantity: i16,
    }
    ::diesel_builders::prelude::unique_index!(
        container_compatibility_rules::container_model_id,
        container_compatibility_rules::project_id,
        container_compatibility_rules::contained_asset_model_id
    );
    impl ::diesel_builders::ValidateColumn<container_compatibility_rules::table_name_id>
        for <container_compatibility_rules::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(table_name_id: &String) -> Result<(), Self::Error> {
            use diesel::Column;
            if table_name_id.as_str() != "container_compatibility_rules" {
                return Err(
                    ::validation_errors::ValidationError::check_violated(
                        <crate::container_compatibility_rules::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        &[crate::container_compatibility_rules::table_name_id::NAME],
                        "table_name_id = 'container_compatibility_rules'",
                    ),
                );
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateColumn<container_compatibility_rules::quantity>
        for <container_compatibility_rules::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(quantity: &i16) -> Result<(), Self::Error> {
            use diesel::Column;
            if quantity <= &0i16 {
                return Err(
                    ::validation_errors::ValidationError::strictly_greater_than_value(
                        <crate::container_compatibility_rules::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::container_compatibility_rules::quantity::NAME,
                        0f64,
                    ),
                );
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateRecord<container_compatibility_rules::table>
        for <container_compatibility_rules::table as ::diesel_builders::TableExt>::NewValues
    {
        #[inline]
        fn validate_record(&self) -> Result<(), ::validation_errors::ValidationError> {
            use diesel::Column;
            if let (Some(container_model_id), Some(contained_asset_model_id)) = (
                <Self as diesel_builders::MayGetColumn<
                    container_compatibility_rules::container_model_id,
                >>::may_get_column_ref(self),
                <Self as diesel_builders::MayGetColumn<
                    container_compatibility_rules::contained_asset_model_id,
                >>::may_get_column_ref(self),
            ) && container_model_id == contained_asset_model_id
            {
                return Err(
                    ::validation_errors::ValidationError::equal(
                        <crate::container_compatibility_rules::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::container_compatibility_rules::container_model_id::NAME,
                        crate::container_compatibility_rules::contained_asset_model_id::NAME,
                    ),
                );
            }
            Ok(())
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id> for ContainerCompatibilityRule {
        fn get_column_ref(
            &self,
        ) -> &<container_compatibility_rules::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for ContainerCompatibilityRule {
        fn get_column_ref(
            &self,
        ) -> &<container_compatibility_rules::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    ::diesel::allow_tables_to_appear_in_same_query!(
        container_compatibility_rules,
        ::aps_resources::asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        container_compatibility_rules,
        ::aps_physical_asset_models::container_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        container_compatibility_rules,
        ::aps_resources::ownables
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        container_compatibility_rules,
        ::aps_physical_asset_models::physical_asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        container_compatibility_rules,
        ::aps_resources::resources
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        container_compatibility_rules,
        ::aps_resources::table_names
    );
}
pub use container_compatibility_rules_model::*;
pub mod container_models_model {
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
    /// Catalog of reusable container models for physical container assets.
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
    #[diesel(table_name = container_models)]
    pub struct ContainerModel {
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
    impl ::diesel_builders::GetColumn<aps_resources::asset_models::id> for ContainerModel {
        fn get_column_ref(
            &self,
        ) -> &<container_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id> for ContainerModel {
        fn get_column_ref(
            &self,
        ) -> &<container_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_physical_asset_models::physical_asset_models::id>
        for ContainerModel
    {
        fn get_column_ref(
            &self,
        ) -> &<container_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for ContainerModel {
        fn get_column_ref(
            &self,
        ) -> &<container_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    /// Shape of `container_models` the procedure runner renders.
    pub const CONTAINER_MODELS_SHAPE: procedure_like::shape::AssetModelShape =
        procedure_like::shape::AssetModelShape {
            table_name: "container_models",
            asset_table_name: "containers",
            model_column: "container_model_id",
            required: &[
                procedure_like::shape::ColumnShape {
                    name: "organization_id",
                    label: "Organization containing this resource.",
                    sql_type: procedure_like::shape::SqlType::Uuid,
                    unit: None,
                    required: true,
                },
                procedure_like::shape::ColumnShape {
                    name: "container_model_id",
                    label: "Container model instantiated by this physical container.",
                    sql_type: procedure_like::shape::SqlType::Uuid,
                    unit: None,
                    required: true,
                },
            ],
        };
    ::diesel::allow_tables_to_appear_in_same_query!(
        container_models,
        ::aps_resources::asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(container_models, ::aps_resources::ownables);
    ::diesel::allow_tables_to_appear_in_same_query!(
        container_models,
        ::aps_physical_asset_models::physical_asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(container_models, ::aps_resources::resources);
}
pub use container_models_model::*;
pub mod physical_asset_models_model {
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
    #[table_model(ancestors(
        aps_resources::resources,
        aps_resources::ownables,
        aps_resources::asset_models
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_resources::AssetModel, foreign_key = id))]
    #[table_model(foreign_key((id), (::aps_resources::asset_models::id)))]
    #[table_model(default(
        aps_resources::resources::kind,
        ::aps_authz_resource_kind::ResourceKind::Ownable
    ))]
    #[diesel(table_name = physical_asset_models)]
    pub struct PhysicalAssetModel {
        /// Identifier of the asset model this row specialises.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// connetto organization owning the row.
        #[same_as(aps_resources::asset_models::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
    }
    impl ::diesel_builders::GetColumn<aps_resources::asset_models::id> for PhysicalAssetModel {
        fn get_column_ref(
            &self,
        ) -> &<physical_asset_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id> for PhysicalAssetModel {
        fn get_column_ref(
            &self,
        ) -> &<physical_asset_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for PhysicalAssetModel {
        fn get_column_ref(
            &self,
        ) -> &<physical_asset_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    /// Shape of `physical_asset_models` the procedure runner renders.
    pub const PHYSICAL_ASSET_MODELS_SHAPE: procedure_like::shape::AssetModelShape =
        procedure_like::shape::AssetModelShape {
            table_name: "physical_asset_models",
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
        physical_asset_models,
        ::aps_resources::asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        physical_asset_models,
        ::aps_resources::ownables
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        physical_asset_models,
        ::aps_resources::resources
    );
}
pub use physical_asset_models_model::*;
pub mod retrieval_procedure_templates_model {
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
    /// Typed slots and parameters for retrieval.
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
            aps_physical_asset_models::ContainerModel,
            foreign_key = retrieved_from_model_id
        )
    )]
    #[diesel(
        belongs_to(
            aps_physical_asset_models::PhysicalAssetModel,
            foreign_key = retrieved_asset_model_id
        )
    )]
    #[table_model(
        foreign_key((id), (::aps_procedure_templates::procedure_templates::id))
    )]
    #[table_model(
        foreign_key(
            (retrieved_from_model_id),
            (::aps_physical_asset_models::container_models::id)
        )
    )]
    #[table_model(
        foreign_key(
            (retrieved_asset_model_id),
            (::aps_physical_asset_models::physical_asset_models::id)
        )
    )]
    #[table_model(
        foreign_key(
            (
                retrieved_from_slot_id,
                id,
                retrieved_from_model_id,
                retrieved_from_slot_role_id,
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
                retrieved_asset_slot_id,
                id,
                retrieved_asset_model_id,
                retrieved_asset_slot_role_id,
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
            (retrieved_asset_slot_id, id, retrieved_from_slot_id, organization_id),
            (
                ::aps_procedure_templates::procedure_template_slots::id,
                ::aps_procedure_templates::procedure_template_slots::procedure_template_id,
                ::aps_procedure_templates::procedure_template_slots::retrieved_from_slot_id,
                ::aps_procedure_templates::procedure_template_slots::organization_id
            )
        )
    )]
    #[table_model(default(
        aps_resources::resources::kind,
        ::aps_authz_resource_kind::ResourceKind::Ownable
    ))]
    #[diesel(table_name = retrieval_procedure_templates)]
    pub struct RetrievalProcedureTemplate {
        /// Identity shared with the parent template.
        #[same_as(
            aps_procedure_templates::procedure_template_slots::procedure_template_id,
            (retrieved_from_slot_id, retrieved_asset_slot_id)
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing the parent resource.
        #[same_as(aps_procedure_templates::procedure_templates::organization_id)]
        #[same_as(
            aps_procedure_templates::procedure_template_slots::organization_id,
            (retrieved_from_slot_id, retrieved_asset_slot_id)
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Slot declaring the `retrieved_from` assets.
        #[same_as(
            aps_procedure_templates::procedure_template_slots::retrieved_from_slot_id,
            retrieved_asset_slot_id
        )]
        #[mandatory(aps_procedure_templates::procedure_template_slots)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        retrieved_from_slot_id: ::rosetta_uuid::Uuid,
        /// Admissible model for the `retrieved_from` slot.
        #[same_as(
            aps_procedure_templates::procedure_template_slots::asset_model_id,
            retrieved_from_slot_id
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        retrieved_from_model_id: ::rosetta_uuid::Uuid,
        /// Required role of the `retrieved_from` slot.
        #[table_model(generated)]
        retrieved_from_slot_role_id: String,
        /// Slot declaring the `retrieved_asset` assets.
        #[mandatory(aps_procedure_templates::procedure_template_slots)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        retrieved_asset_slot_id: ::rosetta_uuid::Uuid,
        /// Admissible model for the `retrieved_asset` slot.
        #[same_as(
            aps_procedure_templates::procedure_template_slots::asset_model_id,
            retrieved_asset_slot_id
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        retrieved_asset_model_id: ::rosetta_uuid::Uuid,
        /// Required role of the `retrieved_asset` slot.
        #[table_model(generated)]
        retrieved_asset_slot_role_id: String,
    }
    impl ::diesel_builders::ValidateRecord<retrieval_procedure_templates::table>
        for <retrieval_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
    {
        #[inline]
        fn validate_record(&self) -> Result<(), ::validation_errors::ValidationError> {
            use diesel::Column;
            if let (Some(retrieved_from_slot_id), Some(retrieved_asset_slot_id)) = (
                <Self as diesel_builders::MayGetColumn<
                    retrieval_procedure_templates::retrieved_from_slot_id,
                >>::may_get_column_ref(self),
                <Self as diesel_builders::MayGetColumn<
                    retrieval_procedure_templates::retrieved_asset_slot_id,
                >>::may_get_column_ref(self),
            ) && retrieved_from_slot_id == retrieved_asset_slot_id
            {
                return Err(
                    ::validation_errors::ValidationError::equal(
                        <crate::retrieval_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::retrieval_procedure_templates::retrieved_from_slot_id::NAME,
                        crate::retrieval_procedure_templates::retrieved_asset_slot_id::NAME,
                    ),
                );
            }
            Ok(())
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id> for RetrievalProcedureTemplate {
        fn get_column_ref(
            &self,
        ) -> &<retrieval_procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_procedure_templates::procedure_templates::id>
        for RetrievalProcedureTemplate
    {
        fn get_column_ref(
            &self,
        ) -> &<retrieval_procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for RetrievalProcedureTemplate {
        fn get_column_ref(
            &self,
        ) -> &<retrieval_procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    /// Shape of `retrieval_procedure_templates` the procedure runner renders.
    pub const RETRIEVAL_PROCEDURE_TEMPLATES_SHAPE: procedure_like::shape::TemplateShape =
        procedure_like::shape::TemplateShape {
            table_name: "retrieval_procedure_templates",
            setpoints: &[],
            slots: &[
                procedure_like::shape::SlotShape {
                    name: "retrieved_from",
                    role: procedure_like::shape::SlotRole::Input,
                    model_table_name: "container_models",
                    slot_column: "retrieved_from_slot_id",
                    model_column: "retrieved_from_model_id",
                    placed_into: None,
                    retrieved_from: None,
                },
                procedure_like::shape::SlotShape {
                    name: "retrieved_asset",
                    role: procedure_like::shape::SlotRole::Input,
                    model_table_name: "physical_asset_models",
                    slot_column: "retrieved_asset_slot_id",
                    model_column: "retrieved_asset_model_id",
                    placed_into: None,
                    retrieved_from: Some("retrieved_from"),
                },
            ],
        };
    ::diesel::allow_tables_to_appear_in_same_query!(
        retrieval_procedure_templates,
        ::aps_physical_asset_models::container_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        retrieval_procedure_templates,
        ::aps_resources::ownables
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        retrieval_procedure_templates,
        ::aps_physical_asset_models::physical_asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        retrieval_procedure_templates,
        ::aps_procedure_templates::procedure_template_slots
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        retrieval_procedure_templates,
        ::aps_procedure_templates::procedure_templates
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        retrieval_procedure_templates,
        ::aps_resources::resources
    );
}
pub use retrieval_procedure_templates_model::*;
pub mod storage_procedure_templates_model {
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
    /// Typed slots and parameters for storage.
    #[table_model(ancestors(
        aps_resources::resources,
        aps_resources::ownables,
        aps_procedure_templates::procedure_templates
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_procedure_templates::ProcedureTemplate, foreign_key = id))]
    #[diesel(
        belongs_to(
            aps_physical_asset_models::ContainerModel,
            foreign_key = stored_into_model_id
        )
    )]
    #[diesel(
        belongs_to(
            aps_physical_asset_models::PhysicalAssetModel,
            foreign_key = stored_asset_model_id
        )
    )]
    #[table_model(
        foreign_key((id), (::aps_procedure_templates::procedure_templates::id))
    )]
    #[table_model(
        foreign_key(
            (stored_into_model_id),
            (::aps_physical_asset_models::container_models::id)
        )
    )]
    #[table_model(
        foreign_key(
            (stored_asset_model_id),
            (::aps_physical_asset_models::physical_asset_models::id)
        )
    )]
    #[table_model(
        foreign_key(
            (
                stored_into_slot_id,
                id,
                stored_into_model_id,
                stored_into_slot_role_id,
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
                stored_asset_slot_id,
                id,
                stored_asset_model_id,
                stored_asset_slot_role_id,
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
            (stored_asset_slot_id, id, stored_into_slot_id, organization_id),
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
    #[diesel(table_name = storage_procedure_templates)]
    pub struct StorageProcedureTemplate {
        /// Identity shared with the parent template.
        #[same_as(
            aps_procedure_templates::procedure_template_slots::procedure_template_id,
            (stored_into_slot_id, stored_asset_slot_id)
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing the parent resource.
        #[same_as(aps_procedure_templates::procedure_templates::organization_id)]
        #[same_as(
            aps_procedure_templates::procedure_template_slots::organization_id,
            (stored_into_slot_id, stored_asset_slot_id)
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Slot declaring the `stored_into` assets.
        #[same_as(
            aps_procedure_templates::procedure_template_slots::placed_into_slot_id,
            stored_asset_slot_id
        )]
        #[mandatory(aps_procedure_templates::procedure_template_slots)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        stored_into_slot_id: ::rosetta_uuid::Uuid,
        /// Admissible model for the `stored_into` slot.
        #[same_as(
            aps_procedure_templates::procedure_template_slots::asset_model_id,
            stored_into_slot_id
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        stored_into_model_id: ::rosetta_uuid::Uuid,
        /// Required role of the `stored_into` slot.
        #[table_model(generated)]
        stored_into_slot_role_id: String,
        /// Slot declaring the `stored_asset` assets.
        #[mandatory(aps_procedure_templates::procedure_template_slots)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        stored_asset_slot_id: ::rosetta_uuid::Uuid,
        /// Admissible model for the `stored_asset` slot.
        #[same_as(
            aps_procedure_templates::procedure_template_slots::asset_model_id,
            stored_asset_slot_id
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        stored_asset_model_id: ::rosetta_uuid::Uuid,
        /// Required role of the `stored_asset` slot.
        #[table_model(generated)]
        stored_asset_slot_role_id: String,
        /// Target temperature in kelvin.
        #[table_model(default = 293.15f64)]
        temperature_k: f64,
        /// Allowed temperature deviation as a percentage of the target.
        #[table_model(default = 1f64)]
        temperature_tolerance_percentage: f64,
    }
    impl ::diesel_builders::ValidateColumn<storage_procedure_templates::temperature_k>
        for <storage_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(temperature_k: &f64) -> Result<(), Self::Error> {
            use diesel::Column;
            if temperature_k <= &0f64 {
                return Err(
                    ::validation_errors::ValidationError::strictly_greater_than_value(
                        <crate::storage_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::storage_procedure_templates::temperature_k::NAME,
                        0f64,
                    ),
                );
            }
            if temperature_k.partial_cmp(&f64::INFINITY).is_none_or(::core::cmp::Ordering::is_ge) {
                return Err(
                    ::validation_errors::ValidationError::check_violated(
                        <crate::storage_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        &[crate::storage_procedure_templates::temperature_k::NAME],
                        "temperature_k < 'Infinity'::DOUBLE PRECISION",
                    ),
                );
            }
            Ok(())
        }
    }
    impl
        ::diesel_builders::ValidateColumn<
            storage_procedure_templates::temperature_tolerance_percentage,
        > for <storage_procedure_templates::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(temperature_tolerance_percentage: &f64) -> Result<(), Self::Error> {
            use diesel::Column;
            if temperature_tolerance_percentage <= &0f64 {
                return Err(
                    ::validation_errors::ValidationError::strictly_greater_than_value(
                        <crate::storage_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::storage_procedure_templates::temperature_tolerance_percentage::NAME,
                        0f64,
                    ),
                );
            }
            if temperature_tolerance_percentage > &100f64 {
                return Err(
                    ::validation_errors::ValidationError::smaller_than_value(
                        <crate::storage_procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::storage_procedure_templates::temperature_tolerance_percentage::NAME,
                        100f64,
                    ),
                );
            }
            Ok(())
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id> for StorageProcedureTemplate {
        fn get_column_ref(
            &self,
        ) -> &<storage_procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_procedure_templates::procedure_templates::id>
        for StorageProcedureTemplate
    {
        fn get_column_ref(
            &self,
        ) -> &<storage_procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for StorageProcedureTemplate {
        fn get_column_ref(
            &self,
        ) -> &<storage_procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    /// Shape of `storage_procedure_templates` the procedure runner renders.
    pub const STORAGE_PROCEDURE_TEMPLATES_SHAPE: procedure_like::shape::TemplateShape =
        procedure_like::shape::TemplateShape {
            table_name: "storage_procedure_templates",
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
            ],
            slots: &[
                procedure_like::shape::SlotShape {
                    name: "stored_into",
                    role: procedure_like::shape::SlotRole::Input,
                    model_table_name: "container_models",
                    slot_column: "stored_into_slot_id",
                    model_column: "stored_into_model_id",
                    placed_into: None,
                    retrieved_from: None,
                },
                procedure_like::shape::SlotShape {
                    name: "stored_asset",
                    role: procedure_like::shape::SlotRole::Input,
                    model_table_name: "physical_asset_models",
                    slot_column: "stored_asset_slot_id",
                    model_column: "stored_asset_model_id",
                    placed_into: Some("stored_into"),
                    retrieved_from: None,
                },
            ],
        };
    ::diesel::allow_tables_to_appear_in_same_query!(
        storage_procedure_templates,
        ::aps_physical_asset_models::container_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        storage_procedure_templates,
        ::aps_resources::ownables
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        storage_procedure_templates,
        ::aps_physical_asset_models::physical_asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        storage_procedure_templates,
        ::aps_procedure_templates::procedure_template_slots
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        storage_procedure_templates,
        ::aps_procedure_templates::procedure_templates
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        storage_procedure_templates,
        ::aps_resources::resources
    );
}
pub use storage_procedure_templates_model::*;
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
    }
    impl ::diesel_builders::GetColumn<aps_resources::asset_models::id> for VolumeMeasuringDeviceModel {
        fn get_column_ref(
            &self,
        ) -> &<volume_measuring_device_models::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id> for VolumeMeasuringDeviceModel {
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
        ::aps_resources::ownables
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
pub mod volumetric_container_models_model {
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
    /// Specialization of container models with known maximum volume.
    #[table_model(ancestors(
        aps_resources::resources,
        aps_resources::ownables,
        aps_resources::asset_models,
        aps_physical_asset_models::physical_asset_models,
        aps_physical_asset_models::container_models
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_physical_asset_models::ContainerModel, foreign_key = id))]
    #[table_model(
        foreign_key((id), (::aps_physical_asset_models::container_models::id))
    )]
    #[table_model(default(
        aps_resources::resources::kind,
        ::aps_authz_resource_kind::ResourceKind::Ownable
    ))]
    #[diesel(table_name = volumetric_container_models)]
    pub struct VolumetricContainerModel {
        /// Stable model identifier inherited from `container_models`.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing this resource.
        #[same_as(aps_physical_asset_models::container_models::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Maximum internal volume.
        volume_l: f64,
    }
    impl ::diesel_builders::ValidateColumn<volumetric_container_models::volume_l>
        for <volumetric_container_models::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(volume_l: &f64) -> Result<(), Self::Error> {
            use diesel::Column;
            if volume_l <= &0f64 {
                return Err(
                    ::validation_errors::ValidationError::strictly_greater_than_value(
                        <crate::volumetric_container_models::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::volumetric_container_models::volume_l::NAME,
                        0f64,
                    ),
                );
            }
            if volume_l.partial_cmp(&f64::INFINITY).is_none_or(::core::cmp::Ordering::is_ge) {
                return Err(
                    ::validation_errors::ValidationError::check_violated(
                        <crate::volumetric_container_models::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        &[crate::volumetric_container_models::volume_l::NAME],
                        "volume_l < 'Infinity'::DOUBLE PRECISION",
                    ),
                );
            }
            Ok(())
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::asset_models::id> for VolumetricContainerModel {
        fn get_column_ref(
            &self,
        ) -> &<volumetric_container_models::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_physical_asset_models::container_models::id>
        for VolumetricContainerModel
    {
        fn get_column_ref(
            &self,
        ) -> &<volumetric_container_models::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id> for VolumetricContainerModel {
        fn get_column_ref(
            &self,
        ) -> &<volumetric_container_models::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_physical_asset_models::physical_asset_models::id>
        for VolumetricContainerModel
    {
        fn get_column_ref(
            &self,
        ) -> &<volumetric_container_models::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for VolumetricContainerModel {
        fn get_column_ref(
            &self,
        ) -> &<volumetric_container_models::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    /// Shape of `volumetric_container_models` the procedure runner renders.
    pub const VOLUMETRIC_CONTAINER_MODELS_SHAPE: procedure_like::shape::AssetModelShape =
        procedure_like::shape::AssetModelShape {
            table_name: "volumetric_container_models",
            asset_table_name: "volumetric_containers",
            model_column: "volumetric_container_model_id",
            required: &[
                procedure_like::shape::ColumnShape {
                    name: "organization_id",
                    label: "Organization containing this resource.",
                    sql_type: procedure_like::shape::SqlType::Uuid,
                    unit: None,
                    required: true,
                },
                procedure_like::shape::ColumnShape {
                    name: "volumetric_container_model_id",
                    label: "Volumetric model instantiated by this container.",
                    sql_type: procedure_like::shape::SqlType::Uuid,
                    unit: None,
                    required: true,
                },
            ],
        };
    ::diesel::allow_tables_to_appear_in_same_query!(
        volumetric_container_models,
        ::aps_resources::asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        volumetric_container_models,
        ::aps_physical_asset_models::container_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        volumetric_container_models,
        ::aps_resources::ownables
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        volumetric_container_models,
        ::aps_physical_asset_models::physical_asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        volumetric_container_models,
        ::aps_resources::resources
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        volumetric_container_models,
        ::aps_physical_asset_models::weighed_container_models
    );
}
pub use volumetric_container_models_model::*;
pub mod weighed_container_models_model {
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
    /// Specialization of container models with known empty mass.
    #[table_model(ancestors(
        aps_resources::resources,
        aps_resources::ownables,
        aps_resources::asset_models,
        aps_physical_asset_models::physical_asset_models,
        aps_physical_asset_models::container_models
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_physical_asset_models::ContainerModel, foreign_key = id))]
    #[table_model(
        foreign_key((id), (::aps_physical_asset_models::container_models::id))
    )]
    #[table_model(default(
        aps_resources::resources::kind,
        ::aps_authz_resource_kind::ResourceKind::Ownable
    ))]
    #[diesel(table_name = weighed_container_models)]
    pub struct WeighedContainerModel {
        /// Stable model identifier inherited from `container_models`.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing this resource.
        #[same_as(aps_physical_asset_models::container_models::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Empty container mass.
        mass_kg: f64,
    }
    impl ::diesel_builders::ValidateColumn<weighed_container_models::mass_kg>
        for <weighed_container_models::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(mass_kg: &f64) -> Result<(), Self::Error> {
            use diesel::Column;
            if mass_kg <= &0f64 {
                return Err(
                    ::validation_errors::ValidationError::strictly_greater_than_value(
                        <crate::weighed_container_models::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::weighed_container_models::mass_kg::NAME,
                        0f64,
                    ),
                );
            }
            if mass_kg.partial_cmp(&f64::INFINITY).is_none_or(::core::cmp::Ordering::is_ge) {
                return Err(
                    ::validation_errors::ValidationError::check_violated(
                        <crate::weighed_container_models::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        &[crate::weighed_container_models::mass_kg::NAME],
                        "mass_kg < 'Infinity'::DOUBLE PRECISION",
                    ),
                );
            }
            Ok(())
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::asset_models::id> for WeighedContainerModel {
        fn get_column_ref(
            &self,
        ) -> &<weighed_container_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_physical_asset_models::container_models::id>
        for WeighedContainerModel
    {
        fn get_column_ref(
            &self,
        ) -> &<weighed_container_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id> for WeighedContainerModel {
        fn get_column_ref(
            &self,
        ) -> &<weighed_container_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_physical_asset_models::physical_asset_models::id>
        for WeighedContainerModel
    {
        fn get_column_ref(
            &self,
        ) -> &<weighed_container_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for WeighedContainerModel {
        fn get_column_ref(
            &self,
        ) -> &<weighed_container_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    /// Shape of `weighed_container_models` the procedure runner renders.
    pub const WEIGHED_CONTAINER_MODELS_SHAPE: procedure_like::shape::AssetModelShape =
        procedure_like::shape::AssetModelShape {
            table_name: "weighed_container_models",
            asset_table_name: "containers",
            model_column: "container_model_id",
            required: &[
                procedure_like::shape::ColumnShape {
                    name: "organization_id",
                    label: "Organization containing this resource.",
                    sql_type: procedure_like::shape::SqlType::Uuid,
                    unit: None,
                    required: true,
                },
                procedure_like::shape::ColumnShape {
                    name: "container_model_id",
                    label: "Container model instantiated by this physical container.",
                    sql_type: procedure_like::shape::SqlType::Uuid,
                    unit: None,
                    required: true,
                },
            ],
        };
    ::diesel::allow_tables_to_appear_in_same_query!(
        weighed_container_models,
        ::aps_resources::asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        weighed_container_models,
        ::aps_physical_asset_models::container_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        weighed_container_models,
        ::aps_resources::ownables
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        weighed_container_models,
        ::aps_physical_asset_models::physical_asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        weighed_container_models,
        ::aps_resources::resources
    );
}
pub use weighed_container_models_model::*;
pub mod weighed_volumetric_container_models_model {
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
    /// Container models with both empty mass and maximum volume.
    #[table_model(ancestors(
        aps_resources::resources,
        aps_resources::ownables,
        aps_resources::asset_models,
        aps_physical_asset_models::physical_asset_models,
        aps_physical_asset_models::container_models,
        aps_physical_asset_models::volumetric_container_models,
        aps_physical_asset_models::weighed_container_models
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(
        belongs_to(aps_physical_asset_models::WeighedContainerModel, foreign_key = id)
    )]
    #[diesel(
        belongs_to(aps_physical_asset_models::VolumetricContainerModel, foreign_key = id)
    )]
    #[table_model(
        foreign_key((id), (::aps_physical_asset_models::weighed_container_models::id))
    )]
    #[table_model(
        foreign_key((id), (::aps_physical_asset_models::volumetric_container_models::id))
    )]
    #[table_model(default(
        aps_resources::resources::kind,
        ::aps_authz_resource_kind::ResourceKind::Ownable
    ))]
    #[diesel(table_name = weighed_volumetric_container_models)]
    pub struct WeighedVolumetricContainerModel {
        /// Model shared by the weighed and volumetric subtypes.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing this resource.
        #[same_as(aps_physical_asset_models::volumetric_container_models::organization_id)]
        #[same_as(aps_physical_asset_models::weighed_container_models::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
    }
    impl ::diesel_builders::GetColumn<aps_resources::asset_models::id>
        for WeighedVolumetricContainerModel
    {
        fn get_column_ref(
            &self,
        ) -> &<weighed_volumetric_container_models::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_physical_asset_models::container_models::id>
        for WeighedVolumetricContainerModel
    {
        fn get_column_ref(
            &self,
        ) -> &<weighed_volumetric_container_models::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id> for WeighedVolumetricContainerModel {
        fn get_column_ref(
            &self,
        ) -> &<weighed_volumetric_container_models::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_physical_asset_models::physical_asset_models::id>
        for WeighedVolumetricContainerModel
    {
        fn get_column_ref(
            &self,
        ) -> &<weighed_volumetric_container_models::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id>
        for WeighedVolumetricContainerModel
    {
        fn get_column_ref(
            &self,
        ) -> &<weighed_volumetric_container_models::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_physical_asset_models::volumetric_container_models::id>
        for WeighedVolumetricContainerModel
    {
        fn get_column_ref(
            &self,
        ) -> &<weighed_volumetric_container_models::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_physical_asset_models::weighed_container_models::id>
        for WeighedVolumetricContainerModel
    {
        fn get_column_ref(
            &self,
        ) -> &<weighed_volumetric_container_models::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    /// Shape of `weighed_volumetric_container_models` the procedure runner
    /// renders.
    pub const WEIGHED_VOLUMETRIC_CONTAINER_MODELS_SHAPE: procedure_like::shape::AssetModelShape =
        procedure_like::shape::AssetModelShape {
            table_name: "weighed_volumetric_container_models",
            asset_table_name: "volumetric_containers",
            model_column: "volumetric_container_model_id",
            required: &[
                procedure_like::shape::ColumnShape {
                    name: "organization_id",
                    label: "Organization containing this resource.",
                    sql_type: procedure_like::shape::SqlType::Uuid,
                    unit: None,
                    required: true,
                },
                procedure_like::shape::ColumnShape {
                    name: "volumetric_container_model_id",
                    label: "Volumetric model instantiated by this container.",
                    sql_type: procedure_like::shape::SqlType::Uuid,
                    unit: None,
                    required: true,
                },
            ],
        };
    ::diesel::allow_tables_to_appear_in_same_query!(
        weighed_volumetric_container_models,
        ::aps_resources::asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        weighed_volumetric_container_models,
        ::aps_physical_asset_models::container_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        weighed_volumetric_container_models,
        ::aps_resources::ownables
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        weighed_volumetric_container_models,
        ::aps_physical_asset_models::physical_asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        weighed_volumetric_container_models,
        ::aps_resources::resources
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        weighed_volumetric_container_models,
        ::aps_physical_asset_models::volumetric_container_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        weighed_volumetric_container_models,
        ::aps_physical_asset_models::weighed_container_models
    );
}
pub use weighed_volumetric_container_models_model::*;
