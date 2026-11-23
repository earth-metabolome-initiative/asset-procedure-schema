#![allow(ambiguous_glob_reexports, reason = "TableModel trait names can collide across tables")]
extern crate self as aps_asset_model_ancestors;
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
                ::aps_assets::sample_models::id,
                ::aps_assets::sample_models::sample_source_model_id
            )
        )
    )]
    #[table_model(
        foreign_key(
            (sample_source_model_id, source_family_model_id),
            (
                ::aps_asset_model_ancestors::asset_model_ancestors::descendant_model_id,
                ::aps_asset_model_ancestors::asset_model_ancestors::ancestor_model_id
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
        ::aps_asset_model_ancestors::asset_model_ancestors
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
        ::aps_assets::sample_models
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
            aps_asset_model_ancestors::HarvestingProcedureTemplate,
            foreign_key = harvesting_procedure_template_id
        )
    )]
    #[table_model(foreign_key((id), (::aps_procedures::procedures::id)))]
    #[table_model(
        foreign_key(
            (harvesting_procedure_template_id),
            (::aps_asset_model_ancestors::harvesting_procedure_templates::id)
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
        type ProcedureTemplateTable =
            aps_asset_model_ancestors::harvesting_procedure_templates::table;
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
        ::aps_asset_model_ancestors::harvesting_procedure_templates
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
                ::aps_asset_model_ancestors::asset_model_ancestors::descendant_model_id,
                ::aps_asset_model_ancestors::asset_model_ancestors::ancestor_model_id
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
        ::aps_asset_model_ancestors::asset_model_ancestors
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
    #[diesel(primary_key(step_id, parent_template_id, child_slot_id))]
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
                ::aps_asset_model_ancestors::asset_model_ancestors::descendant_model_id,
                ::aps_asset_model_ancestors::asset_model_ancestors::ancestor_model_id
            )
        )
    )]
    #[table_model(
        foreign_key(
            (child_role_id, parent_role_id),
            (
                ::aps_asset_model_ancestors::slot_role_pairs::child_role_id,
                ::aps_asset_model_ancestors::slot_role_pairs::parent_role_id
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
        ::aps_asset_model_ancestors::asset_model_ancestors
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
        ::aps_asset_model_ancestors::slot_role_pairs
    );
}
pub use procedure_template_slot_bindings_model::*;
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
