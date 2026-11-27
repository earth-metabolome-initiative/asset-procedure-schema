#![allow(ambiguous_glob_reexports, reason = "TableModel trait names can collide across tables")]
extern crate self as aps_photograph_procedure_templates;
pub mod photograph_procedure_templates_model {
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
    /// Typed slots and parameters for photograph.
    #[table_model(ancestors(aps_resources::resources, aps_resources::procedure_templates))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_resources::ProcedureTemplate, foreign_key = id))]
    #[table_model(foreign_key((id), (::aps_resources::procedure_templates::id)))]
    #[table_model(
        foreign_key(
            (photographed_with_model_id),
            (::aps_commercial_products::camera_models::id)
        )
    )]
    #[table_model(
        foreign_key(
            (photographed_asset_model_id),
            (::aps_physical_asset_models::physical_asset_models::id)
        )
    )]
    #[table_model(
        foreign_key((photograph_model_id), (::aps_resources::photograph_models::id))
    )]
    #[table_model(
        foreign_key(
            (
                photographed_with_slot_id,
                id,
                photographed_with_model_id,
                photographed_with_slot_role_id,
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
                photographed_asset_slot_id,
                id,
                photographed_asset_model_id,
                photographed_asset_slot_role_id,
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
                photograph_slot_id,
                id,
                photograph_model_id,
                photograph_slot_role_id,
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
            (photographed_with_model_id, photographed_with_model_archived),
            (
                ::aps_commercial_products::camera_models::id,
                ::aps_commercial_products::camera_models::archived
            )
        )
    )]
    #[table_model(
        foreign_key(
            (photographed_asset_model_id, photographed_asset_model_archived),
            (
                ::aps_physical_asset_models::physical_asset_models::id,
                ::aps_physical_asset_models::physical_asset_models::archived
            )
        )
    )]
    #[table_model(
        foreign_key(
            (photograph_model_id, photograph_model_archived),
            (
                ::aps_resources::photograph_models::id,
                ::aps_resources::photograph_models::archived
            )
        )
    )]
    #[diesel(table_name = photograph_procedure_templates)]
    pub struct PhotographProcedureTemplate {
        /// Identity shared with the parent template.
        #[same_as(
            aps_resources::procedure_template_slots::procedure_template_id,
            (photographed_with_slot_id, photographed_asset_slot_id, photograph_slot_id)
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing the parent resource.
        #[same_as(aps_resources::procedure_templates::organization_id)]
        #[same_as(
            aps_resources::procedure_template_slots::organization_id,
            (photographed_with_slot_id, photographed_asset_slot_id, photograph_slot_id)
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Slot declaring the `photographed_with` assets.
        #[mandatory(aps_resources::procedure_template_slots)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        photographed_with_slot_id: ::rosetta_uuid::Uuid,
        /// Admissible model for the `photographed_with` slot.
        #[same_as(
            aps_resources::procedure_template_slots::asset_model_id,
            photographed_with_slot_id
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        photographed_with_model_id: ::rosetta_uuid::Uuid,
        /// Required role of the `photographed_with` slot.
        #[table_model(generated)]
        photographed_with_slot_role_id: String,
        /// Slot declaring the `photographed_asset` assets.
        #[mandatory(aps_resources::procedure_template_slots)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        photographed_asset_slot_id: ::rosetta_uuid::Uuid,
        /// Admissible model for the `photographed_asset` slot.
        #[same_as(
            aps_resources::procedure_template_slots::asset_model_id,
            photographed_asset_slot_id
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        photographed_asset_model_id: ::rosetta_uuid::Uuid,
        /// Required role of the `photographed_asset` slot.
        #[table_model(generated)]
        photographed_asset_slot_role_id: String,
        /// Slot declaring the `photograph` assets.
        #[mandatory(aps_resources::procedure_template_slots)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        photograph_slot_id: ::rosetta_uuid::Uuid,
        /// Admissible model for the `photograph` slot.
        #[same_as(aps_resources::procedure_template_slots::asset_model_id, photograph_slot_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        photograph_model_id: ::rosetta_uuid::Uuid,
        /// Required role of the `photograph` slot.
        #[table_model(generated)]
        photograph_slot_role_id: String,
        /// Archive state inherited from this resource.
        #[same_as(aps_resources::procedure_templates::archived)]
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
        /// Archive state of the row named by `photographed_with_model_id`.
        #[table_model(default = false)]
        #[infallible]
        photographed_with_model_archived: bool,
        /// Archive state of the row named by `photographed_asset_model_id`.
        #[table_model(default = false)]
        #[infallible]
        photographed_asset_model_archived: bool,
        /// Archive state of the row named by `photograph_model_id`.
        #[table_model(default = false)]
        #[infallible]
        photograph_model_archived: bool,
    }
    ::diesel_builders::prelude::unique_index!(
        photograph_procedure_templates::id,
        photograph_procedure_templates::archived
    );
    impl ::diesel_builders::GetColumn<aps_resources::procedure_templates::id>
        for PhotographProcedureTemplate
    {
        fn get_column_ref(
            &self,
        ) -> &<photograph_procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for PhotographProcedureTemplate {
        fn get_column_ref(
            &self,
        ) -> &<photograph_procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    /// Shape of `photograph_procedure_templates` the procedure runner renders.
    pub const PHOTOGRAPH_PROCEDURE_TEMPLATES_SHAPE: procedure_like::shape::TemplateShape =
        procedure_like::shape::TemplateShape {
            table_name: "photograph_procedure_templates",
            setpoints: &[],
            slots: &[
                procedure_like::shape::SlotShape {
                    name: "photographed_with",
                    role: procedure_like::shape::SlotRole::Instrument,
                    model_table_name: "camera_models",
                    slot_column: "photographed_with_slot_id",
                    model_column: "photographed_with_model_id",
                    placed_into: None,
                    retrieved_from: None,
                },
                procedure_like::shape::SlotShape {
                    name: "photographed_asset",
                    role: procedure_like::shape::SlotRole::Input,
                    model_table_name: "physical_asset_models",
                    slot_column: "photographed_asset_slot_id",
                    model_column: "photographed_asset_model_id",
                    placed_into: None,
                    retrieved_from: None,
                },
                procedure_like::shape::SlotShape {
                    name: "photograph",
                    role: procedure_like::shape::SlotRole::Produced,
                    model_table_name: "photograph_models",
                    slot_column: "photograph_slot_id",
                    model_column: "photograph_model_id",
                    placed_into: None,
                    retrieved_from: None,
                },
            ],
        };
    ::diesel::allow_tables_to_appear_in_same_query!(
        photograph_procedure_templates,
        ::aps_commercial_products::camera_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        photograph_procedure_templates,
        ::aps_resources::photograph_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        photograph_procedure_templates,
        ::aps_physical_asset_models::physical_asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        photograph_procedure_templates,
        ::aps_resources::procedure_template_slots
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        photograph_procedure_templates,
        ::aps_resources::procedure_templates
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        photograph_procedure_templates,
        ::aps_resources::resources
    );
}
pub use photograph_procedure_templates_model::*;
pub mod photograph_procedures_model {
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
    /// Recorded execution of a photograph template.
    #[table_model(ancestors(aps_resources::resources, aps_procedures::procedures))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[table_model(foreign_key((id), (::aps_procedures::procedures::id)))]
    #[table_model(
        foreign_key(
            (photograph_procedure_template_id),
            (::aps_photograph_procedure_templates::photograph_procedure_templates::id)
        )
    )]
    #[table_model(
        foreign_key(
            (photograph_procedure_template_id, photograph_procedure_template_archived),
            (
                ::aps_photograph_procedure_templates::photograph_procedure_templates::id,
                ::aps_photograph_procedure_templates::photograph_procedure_templates::archived
            )
        )
    )]
    #[diesel(table_name = photograph_procedures)]
    pub struct PhotographProcedure {
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
        photograph_procedure_template_id: ::rosetta_uuid::Uuid,
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
        /// `photograph_procedure_template_id`.
        #[table_model(default = false)]
        #[infallible]
        photograph_procedure_template_archived: bool,
    }
    impl ::diesel_builders::GetColumn<aps_procedures::procedures::id> for PhotographProcedure {
        fn get_column_ref(
            &self,
        ) -> &<photograph_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for PhotographProcedure {
        fn get_column_ref(
            &self,
        ) -> &<photograph_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl procedure_like::ProcedureTableLike for photograph_procedures::table {
        type ProcedureTemplateTable =
            aps_photograph_procedure_templates::photograph_procedure_templates::table;
    }
    /// Shape of `photograph_procedures` the procedure runner renders.
    pub const PHOTOGRAPH_PROCEDURES_SHAPE: procedure_like::shape::ProcedureShape =
        procedure_like::shape::ProcedureShape {
            table_name: "photograph_procedures",
            template_table_name: "photograph_procedure_templates",
            measurements: &[],
        };
    ::diesel::allow_tables_to_appear_in_same_query!(
        photograph_procedures,
        ::aps_photograph_procedure_templates::photograph_procedure_templates
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        photograph_procedures,
        ::aps_procedures::procedures
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        photograph_procedures,
        ::aps_resources::resources
    );
}
pub use photograph_procedures_model::*;
