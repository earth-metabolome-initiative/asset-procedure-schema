#![allow(ambiguous_glob_reexports, reason = "TableModel trait names can collide across tables")]
extern crate self as aps_procedure_templates;
pub mod procedure_template_slots_model {
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
    /// A slot is a place in the procedure that assets fill, typed by the most
    /// general admissible asset model and by the role the assets play. A slot
    /// may name another slot of its template whose assets end up holding its
    /// assets, and another whose assets held them until the procedure took them
    /// out.
    #[table_model(error = ::validation_errors::ValidationError)]
    #[table_model(record_error = ::validation_errors::ValidationError)]
    #[diesel(
        belongs_to(
            aps_procedure_templates::ProcedureTemplate,
            foreign_key = procedure_template_id
        )
    )]
    #[diesel(belongs_to(aps_resources::AssetModel, foreign_key = asset_model_id))]
    #[diesel(belongs_to(aps_procedure_templates::SlotRole, foreign_key = role_id))]
    #[table_model(
        foreign_key(
            (procedure_template_id),
            (::aps_procedure_templates::procedure_templates::id)
        )
    )]
    #[table_model(foreign_key((asset_model_id), (::aps_resources::asset_models::id)))]
    #[table_model(foreign_key((role_id), (::aps_procedure_templates::slot_roles::id)))]
    #[table_model(
        foreign_key(
            (placed_into_slot_id, procedure_template_id, organization_id),
            (
                procedure_template_slots::id,
                procedure_template_slots::procedure_template_id,
                procedure_template_slots::organization_id
            )
        )
    )]
    #[table_model(
        foreign_key(
            (retrieved_from_slot_id, procedure_template_id, organization_id),
            (
                procedure_template_slots::id,
                procedure_template_slots::procedure_template_id,
                procedure_template_slots::organization_id
            )
        )
    )]
    #[table_model(
        foreign_key(
            (organization_id, procedure_template_id),
            (::aps_resources::resources::organization_id, ::aps_resources::resources::id)
        )
    )]
    #[diesel(table_name = procedure_template_slots)]
    pub struct ProcedureTemplateSlot {
        /// Identifier of the slot.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// connetto organization owning the template.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Template declaring the slot.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        procedure_template_id: ::rosetta_uuid::Uuid,
        /// Name, unique within the template.
        name: String,
        /// Free-text description of what fills the slot.
        description: Option<String>,
        /// Most general model the bound assets may have.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        asset_model_id: ::rosetta_uuid::Uuid,
        /// What the procedure does to the bound assets.
        #[infallible]
        role_id: String,
        /// Fewest assets a completed procedure binds.
        #[table_model(default = 1i32)]
        minimum_count: i32,
        /// Most assets a procedure binds. NULL leaves the number unbounded.
        #[infallible]
        maximum_count: Option<i32>,
        /// Slot whose assets hold this slot's assets once the procedure
        /// completes.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        placed_into_slot_id: Option<::rosetta_uuid::Uuid>,
        /// Slot whose assets held this slot's assets until the procedure took
        /// them out.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        retrieved_from_slot_id: Option<::rosetta_uuid::Uuid>,
    }
    ::diesel_builders::prelude::unique_index!(
        procedure_template_slots::procedure_template_id,
        procedure_template_slots::name
    );
    ::diesel_builders::prelude::unique_index!(
        procedure_template_slots::id,
        procedure_template_slots::procedure_template_id,
        procedure_template_slots::organization_id
    );
    ::diesel_builders::prelude::unique_index!(
        procedure_template_slots::id,
        procedure_template_slots::procedure_template_id,
        procedure_template_slots::asset_model_id
    );
    ::diesel_builders::prelude::unique_index!(
        procedure_template_slots::id,
        procedure_template_slots::procedure_template_id,
        procedure_template_slots::asset_model_id,
        procedure_template_slots::role_id
    );
    ::diesel_builders::prelude::unique_index!(
        procedure_template_slots::id,
        procedure_template_slots::procedure_template_id,
        procedure_template_slots::asset_model_id,
        procedure_template_slots::role_id,
        procedure_template_slots::organization_id
    );
    ::diesel_builders::prelude::unique_index!(
        procedure_template_slots::id,
        procedure_template_slots::procedure_template_id,
        procedure_template_slots::placed_into_slot_id,
        procedure_template_slots::organization_id
    );
    ::diesel_builders::prelude::unique_index!(
        procedure_template_slots::id,
        procedure_template_slots::procedure_template_id,
        procedure_template_slots::retrieved_from_slot_id,
        procedure_template_slots::organization_id
    );
    impl ::diesel_builders::ValidateColumn<procedure_template_slots::name>
        for <procedure_template_slots::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(name: &String) -> Result<(), Self::Error> {
            use diesel::Column;
            if name.is_empty() {
                return Err(
                    ::validation_errors::ValidationError::empty(
                        <crate::procedure_template_slots::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::procedure_template_slots::name::NAME,
                    ),
                );
            }
            if name.chars().count() > 255usize {
                return Err(
                    ::validation_errors::ValidationError::exceeds_max_length(
                        <crate::procedure_template_slots::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::procedure_template_slots::name::NAME,
                        255usize,
                    ),
                );
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateColumn<procedure_template_slots::description>
        for <procedure_template_slots::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(description: &String) -> Result<(), Self::Error> {
            use diesel::Column;
            if description.is_empty() {
                return Err(
                    ::validation_errors::ValidationError::empty(
                        <crate::procedure_template_slots::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::procedure_template_slots::description::NAME,
                    ),
                );
            }
            if description.chars().count() > 8192usize {
                return Err(
                    ::validation_errors::ValidationError::exceeds_max_length(
                        <crate::procedure_template_slots::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::procedure_template_slots::description::NAME,
                        8192usize,
                    ),
                );
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateColumn<procedure_template_slots::minimum_count>
        for <procedure_template_slots::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(minimum_count: &i32) -> Result<(), Self::Error> {
            use diesel::Column;
            if minimum_count < &0i32 {
                return Err(
                    ::validation_errors::ValidationError::greater_than_value(
                        <crate::procedure_template_slots::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::procedure_template_slots::minimum_count::NAME,
                        0f64,
                    ),
                );
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateRecord<procedure_template_slots::table>
        for <procedure_template_slots::table as ::diesel_builders::TableExt>::NewValues
    {
        #[inline]
        fn validate_record(&self) -> Result<(), ::validation_errors::ValidationError> {
            use diesel::Column;
            if let (Some(maximum_count), Some(minimum_count)) =
                (
                    <Self as diesel_builders::MayGetColumn<
                        procedure_template_slots::maximum_count,
                    >>::may_get_column_ref(self),
                    <Self as diesel_builders::MayGetColumn<
                        procedure_template_slots::minimum_count,
                    >>::may_get_column_ref(self),
                )
            {
                if (*maximum_count).zip(Some(1i32)).map(|(left, right)| left >= right)
                    == Some(false)
                {
                    return Err(
                        ::validation_errors::ValidationError::check_violated(
                            <crate::procedure_template_slots::table as ::diesel_builders::TableExt>::TABLE_NAME,
                            &[crate::procedure_template_slots::maximum_count::NAME],
                            "maximum_count >= 1",
                        ),
                    );
                }
                if maximum_count.as_ref().is_some_and(|maximum_count| maximum_count < minimum_count)
                {
                    return Err(
                        ::validation_errors::ValidationError::greater_than(
                            <crate::procedure_template_slots::table as ::diesel_builders::TableExt>::TABLE_NAME,
                            crate::procedure_template_slots::maximum_count::NAME,
                            crate::procedure_template_slots::minimum_count::NAME,
                        ),
                    );
                }
            }
            if let (Some(placed_into_slot_id), Some(id)) = (
                <Self as diesel_builders::MayGetColumn<
                    procedure_template_slots::placed_into_slot_id,
                >>::may_get_column_ref(self),
                <Self as diesel_builders::MayGetColumn<
                    procedure_template_slots::id,
                >>::may_get_column_ref(self),
            )
                && placed_into_slot_id
                    .as_ref()
                    .is_some_and(|placed_into_slot_id| placed_into_slot_id == id)
            {
                return Err(
                    ::validation_errors::ValidationError::equal(
                        <crate::procedure_template_slots::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::procedure_template_slots::placed_into_slot_id::NAME,
                        crate::procedure_template_slots::id::NAME,
                    ),
                );
            }
            if let (Some(retrieved_from_slot_id), Some(id)) = (
                <Self as diesel_builders::MayGetColumn<
                    procedure_template_slots::retrieved_from_slot_id,
                >>::may_get_column_ref(self),
                <Self as diesel_builders::MayGetColumn<
                    procedure_template_slots::id,
                >>::may_get_column_ref(self),
            )
                && retrieved_from_slot_id
                    .as_ref()
                    .is_some_and(|retrieved_from_slot_id| retrieved_from_slot_id == id)
            {
                return Err(
                    ::validation_errors::ValidationError::equal(
                        <crate::procedure_template_slots::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::procedure_template_slots::retrieved_from_slot_id::NAME,
                        crate::procedure_template_slots::id::NAME,
                    ),
                );
            }
            Ok(())
        }
    }
    ::diesel::allow_tables_to_appear_in_same_query!(
        procedure_template_slots,
        ::aps_resources::asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        procedure_template_slots,
        ::aps_procedure_templates::procedure_templates
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        procedure_template_slots,
        ::aps_resources::resources
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        procedure_template_slots,
        ::aps_procedure_templates::slot_roles
    );
}
pub use procedure_template_slots_model::*;
pub mod procedure_template_states_model {
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
    /// Lifecycle of a procedure template. Only a `draft` changes, a `published`
    /// template may be executed and a `deprecated` one is kept for history.
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(table_name = procedure_template_states)]
    pub struct ProcedureTemplateState {
        /// State name.
        id: String,
    }
    impl ::diesel_builders::ValidateColumn<procedure_template_states::id>
        for <procedure_template_states::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(id: &String) -> Result<(), Self::Error> {
            use diesel::Column;
            if id.is_empty() {
                return Err(
                    ::validation_errors::ValidationError::empty(
                        <crate::procedure_template_states::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::procedure_template_states::id::NAME,
                    ),
                );
            }
            if id.chars().count() > 63usize {
                return Err(
                    ::validation_errors::ValidationError::exceeds_max_length(
                        <crate::procedure_template_states::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::procedure_template_states::id::NAME,
                        63usize,
                    ),
                );
            }
            Ok(())
        }
    }
}
pub use procedure_template_states_model::*;
pub mod procedure_template_step_edges_model {
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
    /// The successor step starts only after the predecessor completes. A
    /// predecessor sits at a strictly lower position, so the steps of a parent
    /// form a DAG without a recursive check.
    #[table_model(error = ::validation_errors::ValidationError)]
    #[table_model(record_error = ::validation_errors::ValidationError)]
    #[diesel(primary_key(predecessor_step_id, successor_step_id))]
    #[table_model(
        foreign_key(
            (
                predecessor_step_id,
                parent_template_id,
                predecessor_position,
                organization_id
            ),
            (
                ::aps_procedure_templates::procedure_template_steps::id,
                ::aps_procedure_templates::procedure_template_steps::parent_template_id,
                ::aps_procedure_templates::procedure_template_steps::position,
                ::aps_procedure_templates::procedure_template_steps::organization_id
            )
        )
    )]
    #[table_model(
        foreign_key(
            (successor_step_id, parent_template_id, successor_position, organization_id),
            (
                ::aps_procedure_templates::procedure_template_steps::id,
                ::aps_procedure_templates::procedure_template_steps::parent_template_id,
                ::aps_procedure_templates::procedure_template_steps::position,
                ::aps_procedure_templates::procedure_template_steps::organization_id
            )
        )
    )]
    #[table_model(
        foreign_key(
            (organization_id, parent_template_id),
            (::aps_resources::resources::organization_id, ::aps_resources::resources::id)
        )
    )]
    #[diesel(table_name = procedure_template_step_edges)]
    pub struct ProcedureTemplateStepEdge {
        /// Step that completes first.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        predecessor_step_id: ::rosetta_uuid::Uuid,
        /// Step that starts after the predecessor completed.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        successor_step_id: ::rosetta_uuid::Uuid,
        /// Template both steps belong to.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        parent_template_id: ::rosetta_uuid::Uuid,
        /// connetto organization owning the parent template.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Position of the predecessor, kept equal by the cascade.
        #[infallible]
        predecessor_position: i32,
        /// Position of the successor, kept equal by the cascade.
        #[infallible]
        successor_position: i32,
    }
    impl ::diesel_builders::ValidateRecord<procedure_template_step_edges::table>
        for <procedure_template_step_edges::table as ::diesel_builders::TableExt>::NewValues
    {
        #[inline]
        fn validate_record(&self) -> Result<(), ::validation_errors::ValidationError> {
            use diesel::Column;
            if let (Some(predecessor_position), Some(successor_position)) = (
                <Self as diesel_builders::MayGetColumn<
                    procedure_template_step_edges::predecessor_position,
                >>::may_get_column_ref(self),
                <Self as diesel_builders::MayGetColumn<
                    procedure_template_step_edges::successor_position,
                >>::may_get_column_ref(self),
            ) && predecessor_position >= successor_position
            {
                return Err(
                    ::validation_errors::ValidationError::strictly_smaller_than(
                        <crate::procedure_template_step_edges::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::procedure_template_step_edges::predecessor_position::NAME,
                        crate::procedure_template_step_edges::successor_position::NAME,
                    ),
                );
            }
            Ok(())
        }
    }
    ::diesel::allow_tables_to_appear_in_same_query!(
        procedure_template_step_edges,
        ::aps_procedure_templates::procedure_template_steps
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        procedure_template_step_edges,
        ::aps_resources::resources
    );
}
pub use procedure_template_step_edges_model::*;
pub mod procedure_template_steps_model {
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
    /// A step runs a published child template inside a parent template. The
    /// same child may run in several steps of one parent, told apart by the
    /// step name. Steps with equal `position` may run in parallel. A step with
    /// a repeat slot runs once for every asset the parent procedure binds to
    /// that slot.
    #[table_model(error = ::validation_errors::ValidationError)]
    #[table_model(record_error = ::validation_errors::ValidationError)]
    #[table_model(
        foreign_key(
            (parent_template_id),
            (::aps_procedure_templates::procedure_templates::id)
        )
    )]
    #[table_model(
        foreign_key(
            (child_template_id),
            (::aps_procedure_templates::procedure_templates::id)
        )
    )]
    #[table_model(
        foreign_key(
            (repeat_slot_id, parent_template_id, organization_id),
            (
                ::aps_procedure_templates::procedure_template_slots::id,
                ::aps_procedure_templates::procedure_template_slots::procedure_template_id,
                ::aps_procedure_templates::procedure_template_slots::organization_id
            )
        )
    )]
    #[table_model(
        foreign_key(
            (organization_id, parent_template_id),
            (::aps_resources::resources::organization_id, ::aps_resources::resources::id)
        )
    )]
    #[diesel(table_name = procedure_template_steps)]
    pub struct ProcedureTemplateStep {
        /// Identifier of the step.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// connetto organization owning the parent template.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Template the step belongs to.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        parent_template_id: ::rosetta_uuid::Uuid,
        /// Template the step runs.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        child_template_id: ::rosetta_uuid::Uuid,
        /// Name, unique within the parent template.
        name: String,
        /// What the step achieves within the parent.
        description: Option<String>,
        /// Order of the step. A predecessor sits at a strictly lower position.
        position: i32,
        /// An optional step may be left without a completed procedure.
        #[table_model(default = false)]
        #[infallible]
        optional: bool,
        /// Parent slot whose every bound asset gets its own run of the step.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        repeat_slot_id: Option<::rosetta_uuid::Uuid>,
    }
    ::diesel_builders::prelude::unique_index!(
        procedure_template_steps::parent_template_id,
        procedure_template_steps::name
    );
    ::diesel_builders::prelude::unique_index!(
        procedure_template_steps::id,
        procedure_template_steps::parent_template_id
    );
    ::diesel_builders::prelude::unique_index!(
        procedure_template_steps::id,
        procedure_template_steps::parent_template_id,
        procedure_template_steps::position,
        procedure_template_steps::organization_id
    );
    ::diesel_builders::prelude::unique_index!(
        procedure_template_steps::id,
        procedure_template_steps::parent_template_id,
        procedure_template_steps::child_template_id
    );
    ::diesel_builders::prelude::unique_index!(
        procedure_template_steps::id,
        procedure_template_steps::parent_template_id,
        procedure_template_steps::child_template_id,
        procedure_template_steps::organization_id
    );
    impl ::diesel_builders::ValidateColumn<procedure_template_steps::name>
        for <procedure_template_steps::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(name: &String) -> Result<(), Self::Error> {
            use diesel::Column;
            if name.is_empty() {
                return Err(
                    ::validation_errors::ValidationError::empty(
                        <crate::procedure_template_steps::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::procedure_template_steps::name::NAME,
                    ),
                );
            }
            if name.chars().count() > 255usize {
                return Err(
                    ::validation_errors::ValidationError::exceeds_max_length(
                        <crate::procedure_template_steps::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::procedure_template_steps::name::NAME,
                        255usize,
                    ),
                );
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateColumn<procedure_template_steps::description>
        for <procedure_template_steps::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(description: &String) -> Result<(), Self::Error> {
            use diesel::Column;
            if description.is_empty() {
                return Err(
                    ::validation_errors::ValidationError::empty(
                        <crate::procedure_template_steps::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::procedure_template_steps::description::NAME,
                    ),
                );
            }
            if description.chars().count() > 8192usize {
                return Err(
                    ::validation_errors::ValidationError::exceeds_max_length(
                        <crate::procedure_template_steps::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::procedure_template_steps::description::NAME,
                        8192usize,
                    ),
                );
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateColumn<procedure_template_steps::position>
        for <procedure_template_steps::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(position: &i32) -> Result<(), Self::Error> {
            use diesel::Column;
            if position < &0i32 {
                return Err(
                    ::validation_errors::ValidationError::greater_than_value(
                        <crate::procedure_template_steps::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::procedure_template_steps::position::NAME,
                        0f64,
                    ),
                );
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateRecord<procedure_template_steps::table>
        for <procedure_template_steps::table as ::diesel_builders::TableExt>::NewValues
    {
        #[inline]
        fn validate_record(&self) -> Result<(), ::validation_errors::ValidationError> {
            use diesel::Column;
            if let (Some(parent_template_id), Some(child_template_id)) = (
                <Self as diesel_builders::MayGetColumn<
                    procedure_template_steps::parent_template_id,
                >>::may_get_column_ref(self),
                <Self as diesel_builders::MayGetColumn<
                    procedure_template_steps::child_template_id,
                >>::may_get_column_ref(self),
            ) && parent_template_id == child_template_id
            {
                return Err(
                    ::validation_errors::ValidationError::equal(
                        <crate::procedure_template_steps::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::procedure_template_steps::parent_template_id::NAME,
                        crate::procedure_template_steps::child_template_id::NAME,
                    ),
                );
            }
            Ok(())
        }
    }
    ::diesel::allow_tables_to_appear_in_same_query!(
        procedure_template_steps,
        ::aps_procedure_templates::procedure_template_slots
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        procedure_template_steps,
        ::aps_procedure_templates::procedure_templates
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        procedure_template_steps,
        ::aps_resources::resources
    );
}
pub use procedure_template_steps_model::*;
pub mod procedure_templates_model {
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
    /// A procedure template is the blueprint of a procedure. It declares the
    /// slots its assets fill, and, when composite, the steps it runs as other
    /// templates. Only a `draft` changes. Publishing freezes it, and a new
    /// version is a new draft naming the published one as
    /// `previous_version_id`.
    #[table_model(ancestors(aps_resources::resources, aps_resources::ownables))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[table_model(record_error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_resources::TableName, foreign_key = table_name_id))]
    #[diesel(
        belongs_to(
            aps_procedure_templates::ProcedureTemplateState,
            foreign_key = state_id
        )
    )]
    #[table_model(foreign_key((id), (::aps_resources::ownables::id)))]
    #[table_model(foreign_key((table_name_id), (::aps_resources::table_names::id)))]
    #[table_model(
        foreign_key(
            (state_id),
            (::aps_procedure_templates::procedure_template_states::id)
        )
    )]
    #[table_model(
        foreign_key(
            (previous_version_id, project_id, name, previous_version, organization_id),
            (
                procedure_templates::id,
                procedure_templates::project_id,
                procedure_templates::name,
                procedure_templates::version,
                procedure_templates::organization_id
            )
        )
    )]
    #[table_model(default(
        aps_resources::resources::kind,
        ::aps_authz_resource_kind::ResourceKind::Ownable
    ))]
    #[diesel(table_name = procedure_templates)]
    pub struct ProcedureTemplate {
        /// Identifier of the connetto ownable this template extends.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// connetto organization owning the row.
        #[same_as(aps_resources::ownables::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// connetto project holding the template.
        #[same_as(aps_resources::ownables::project_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        project_id: ::rosetta_uuid::Uuid,
        /// Most concrete APS table holding this template.
        #[infallible]
        table_name_id: String,
        /// Name shared by every version of the template, unique per version
        /// within the project.
        name: String,
        /// Free-text description of what the procedure achieves.
        description: Option<String>,
        /// Instruction a person follows to perform the procedure.
        instruction: Option<String>,
        /// Lifecycle state, `draft`, `published` or `deprecated`.
        #[table_model(default = "draft")]
        #[infallible]
        state_id: String,
        /// Version number, one more than the previous version's.
        #[table_model(default = 1i32)]
        version: i32,
        /// Published version this draft revises, absent on the first version.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        previous_version_id: Option<::rosetta_uuid::Uuid>,
        /// Version number of `previous_version_id`.
        #[infallible]
        previous_version: Option<i32>,
    }
    ::diesel_builders::prelude::unique_index!(procedure_templates::previous_version_id);
    ::diesel_builders::prelude::unique_index!(
        procedure_templates::project_id,
        procedure_templates::name,
        procedure_templates::version
    );
    ::diesel_builders::prelude::unique_index!(
        procedure_templates::id,
        procedure_templates::project_id,
        procedure_templates::name,
        procedure_templates::version,
        procedure_templates::organization_id
    );
    impl ::diesel_builders::ValidateColumn<procedure_templates::name>
        for <procedure_templates::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(name: &String) -> Result<(), Self::Error> {
            use diesel::Column;
            if name.is_empty() {
                return Err(::validation_errors::ValidationError::empty(
                    <crate::procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::procedure_templates::name::NAME,
                ));
            }
            if name.chars().count() > 255usize {
                return Err(::validation_errors::ValidationError::exceeds_max_length(
                    <crate::procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::procedure_templates::name::NAME,
                    255usize,
                ));
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateColumn<procedure_templates::description>
        for <procedure_templates::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(description: &String) -> Result<(), Self::Error> {
            use diesel::Column;
            if description.is_empty() {
                return Err(::validation_errors::ValidationError::empty(
                    <crate::procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::procedure_templates::description::NAME,
                ));
            }
            if description.chars().count() > 8192usize {
                return Err(::validation_errors::ValidationError::exceeds_max_length(
                    <crate::procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::procedure_templates::description::NAME,
                    8192usize,
                ));
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateColumn<procedure_templates::instruction>
        for <procedure_templates::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(instruction: &String) -> Result<(), Self::Error> {
            use diesel::Column;
            if instruction.is_empty() {
                return Err(::validation_errors::ValidationError::empty(
                    <crate::procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::procedure_templates::instruction::NAME,
                ));
            }
            if instruction.chars().count() > 65536usize {
                return Err(::validation_errors::ValidationError::exceeds_max_length(
                    <crate::procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::procedure_templates::instruction::NAME,
                    65536usize,
                ));
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateColumn<procedure_templates::version>
        for <procedure_templates::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(version: &i32) -> Result<(), Self::Error> {
            use diesel::Column;
            if version <= &0i32 {
                return Err(::validation_errors::ValidationError::strictly_greater_than_value(
                    <crate::procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::procedure_templates::version::NAME,
                    0f64,
                ));
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateRecord<procedure_templates::table>
        for <procedure_templates::table as ::diesel_builders::TableExt>::NewValues
    {
        #[inline]
        fn validate_record(&self) -> Result<(), ::validation_errors::ValidationError> {
            use diesel::Column;
            if let (Some(previous_version_id), Some(version)) = (
                <Self as diesel_builders::MayGetColumn<
                    procedure_templates::previous_version_id,
                >>::may_get_column_ref(self),
                <Self as diesel_builders::MayGetColumn<
                    procedure_templates::version,
                >>::may_get_column_ref(self),
            ) && (*previous_version_id).is_none() != (*version == 1i32)
            {
                return Err(
                    ::validation_errors::ValidationError::check_violated(
                        <crate::procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        &[
                            crate::procedure_templates::previous_version_id::NAME,
                            crate::procedure_templates::version::NAME,
                        ],
                        "(previous_version_id IS NULL) = (version = 1)",
                    ),
                );
            }
            if let (Some(previous_version_id), Some(previous_version)) =
                (
                    <Self as diesel_builders::MayGetColumn<
                        procedure_templates::previous_version_id,
                    >>::may_get_column_ref(self),
                    <Self as diesel_builders::MayGetColumn<
                        procedure_templates::previous_version,
                    >>::may_get_column_ref(self),
                )
                && (*previous_version_id).is_none() != (*previous_version).is_none()
            {
                return Err(::validation_errors::ValidationError::check_violated(
                    <crate::procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    &[
                        crate::procedure_templates::previous_version_id::NAME,
                        crate::procedure_templates::previous_version::NAME,
                    ],
                    "(previous_version_id IS NULL) = (previous_version IS NULL)",
                ));
            }
            if let (Some(version), Some(previous_version)) = (
                <Self as diesel_builders::MayGetColumn<
                    procedure_templates::version,
                >>::may_get_column_ref(self),
                <Self as diesel_builders::MayGetColumn<
                    procedure_templates::previous_version,
                >>::may_get_column_ref(self),
            )
                && Some(*version)
                    .zip(
                        (*previous_version)
                            .zip(Some(1i32))
                            .map(|(left, right)| left + right),
                    )
                    .map(|(left, right)| left == right) == Some(false)
            {
                return Err(
                    ::validation_errors::ValidationError::check_violated(
                        <crate::procedure_templates::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        &[
                            crate::procedure_templates::version::NAME,
                            crate::procedure_templates::previous_version::NAME,
                        ],
                        "version = previous_version + 1",
                    ),
                );
            }
            Ok(())
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id> for ProcedureTemplate {
        fn get_column_ref(
            &self,
        ) -> &<procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for ProcedureTemplate {
        fn get_column_ref(
            &self,
        ) -> &<procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    ::diesel::allow_tables_to_appear_in_same_query!(procedure_templates, ::aps_resources::ownables);
    ::diesel::allow_tables_to_appear_in_same_query!(
        procedure_templates,
        ::aps_procedure_templates::procedure_template_states
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        procedure_templates,
        ::aps_resources::resources
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        procedure_templates,
        ::aps_resources::table_names
    );
}
pub use procedure_templates_model::*;
pub mod slot_roles_model {
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
    /// How a procedure treats the asset bound to a slot. `input` exists before
    /// and after, `consumed` ceases to exist, `produced` is created by the
    /// procedure, `intermediate` is created and used up by steps of the
    /// procedure, and `instrument` performs it without entering the lineage of
    /// what is produced.
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(table_name = slot_roles)]
    pub struct SlotRole {
        /// Role name.
        id: String,
    }
    impl ::diesel_builders::ValidateColumn<slot_roles::id>
        for <slot_roles::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(id: &String) -> Result<(), Self::Error> {
            use diesel::Column;
            if id.is_empty() {
                return Err(::validation_errors::ValidationError::empty(
                    <crate::slot_roles::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::slot_roles::id::NAME,
                ));
            }
            if id.chars().count() > 63usize {
                return Err(::validation_errors::ValidationError::exceeds_max_length(
                    <crate::slot_roles::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::slot_roles::id::NAME,
                    63usize,
                ));
            }
            Ok(())
        }
    }
}
pub use slot_roles_model::*;
