//! The slots and steps a procedure template declares.

use aps_procedure_templates::*;
use aps_resources::asset_models;
use diesel::{
    Connection, QueryDsl, RunQueryDsl,
    dsl::{Eq, Find, Update},
    query_dsl::methods::ExecuteDsl,
};
use diesel_builders::{BuildableTable, BuilderResult, GetColumn, Insert, TableBuilder};
use rosetta_uuid::Uuid;
use validation_errors::ValidationError;

use crate::{
    SlotRole,
    validation::{IntoValidationError, flatten, validated},
};

/// The statement publishing a procedure template, which a generic caller of
/// [`ProcedureTemplateNode::publish`] bounds with `ExecuteDsl` on its
/// connection.
pub type Publish =
    Update<Find<procedure_templates::table, Uuid>, Eq<procedure_templates::state_id, &'static str>>;

/// A procedure template, which declares the slots its assets fill and the
/// steps it runs as other templates.
pub trait ProcedureTemplateNode:
    GetColumn<procedure_templates::id> + GetColumn<procedure_templates::organization_id>
{
    /// Inserts the slot `name`, which binds from `minimum_count` to
    /// `maximum_count` assets of `asset_model` or a model below it, playing
    /// `role`. A `maximum_count` of `None` leaves the number unbounded.
    ///
    /// # Errors
    ///
    /// Returns the error of the connection, or of an empty or overlong name or
    /// a negative count.
    ///
    /// # Example
    ///
    /// ```rust
    /// use aps_procedure_templates::*;
    /// use aps_test_utils::{Replica, Tenant};
    /// use aps_traits::{ProcedureTemplateNode, SlotRole};
    ///
    /// let mut replica = Replica::new();
    /// let tenant = replica.deliver(|conn| Ok(Tenant::insert(conn))).unwrap();
    /// replica.act_as(Some(tenant.user()));
    /// let slot = replica
    ///     .write(|conn| {
    ///         let dough = tenant.asset_model("Dough", None, conn);
    ///         let knead = tenant.procedure_template("Knead", conn);
    ///         Ok(knead.slot("dough", &dough, SlotRole::Input, 1, Some(1), conn).unwrap())
    ///     })
    ///     .unwrap();
    /// assert_eq!(slot.role_id(), "input");
    /// ```
    fn slot<C>(
        &self,
        name: &str,
        asset_model: &impl GetColumn<asset_models::id>,
        role: SlotRole,
        minimum_count: i32,
        maximum_count: Option<i32>,
        conn: &mut C,
    ) -> BuilderResult<ProcedureTemplateSlot, ValidationError>
    where
        TableBuilder<procedure_template_slots::table>:
            Insert<C, ValidationError: IntoValidationError>,
    {
        let builder = validated(procedure_template_slots::table::try_builder())?
            .id(Uuid::new_v4())
            .organization_id(GetColumn::<procedure_templates::organization_id>::get_column(self))
            .procedure_template_id(GetColumn::<procedure_templates::id>::get_column(self));
        let builder = validated(builder.try_name(name))?
            .asset_model_id(asset_model.get_column())
            .role_id(role.id());
        let builder =
            validated(builder.try_minimum_count(minimum_count))?.maximum_count(maximum_count);
        flatten(builder.insert(conn))
    }

    /// Inserts the step `name`, which runs the published template `child` at
    /// `position` among the steps of this template.
    ///
    /// # Errors
    ///
    /// Returns the error of the connection, which refuses a `child` that is not
    /// published, or of an empty or overlong name or a negative position.
    ///
    /// # Example
    ///
    /// ```rust
    /// use aps_procedure_templates::*;
    /// use aps_test_utils::{Replica, Tenant};
    /// use aps_traits::ProcedureTemplateNode;
    ///
    /// let mut replica = Replica::new();
    /// let tenant = replica.deliver(|conn| Ok(Tenant::insert(conn))).unwrap();
    /// replica.act_as(Some(tenant.user()));
    /// let step = replica
    ///     .write(|conn| {
    ///         let pizza = tenant.procedure_template("Make a pizza", conn);
    ///         let knead = tenant.procedure_template("Knead", conn);
    ///         knead.publish(conn)?;
    ///         Ok(pizza.step(&knead, "knead the dough", 0, conn).unwrap())
    ///     })
    ///     .unwrap();
    /// assert_eq!(*step.position(), 0);
    /// ```
    fn step<C>(
        &self,
        child: &impl GetColumn<procedure_templates::id>,
        name: &str,
        position: i32,
        conn: &mut C,
    ) -> BuilderResult<ProcedureTemplateStep, ValidationError>
    where
        TableBuilder<procedure_template_steps::table>:
            Insert<C, ValidationError: IntoValidationError>,
    {
        let builder = procedure_template_steps::table::builder()
            .id(Uuid::new_v4())
            .organization_id(GetColumn::<procedure_templates::organization_id>::get_column(self))
            .parent_template_id(GetColumn::<procedure_templates::id>::get_column(self))
            .child_template_id(child.get_column());
        let builder = validated(builder.try_name(name))?;
        flatten(validated(builder.try_position(position))?.insert(conn))
    }

    /// Publishes this draft template, freezing it so steps of other templates
    /// may run it and procedures may start from it.
    ///
    /// # Errors
    ///
    /// Returns the error of the connection, which refuses publishing a
    /// template whose slots and steps break the template rules.
    fn publish<C>(&self, conn: &mut C) -> diesel::QueryResult<usize>
    where
        C: Connection,
        Publish: ExecuteDsl<C>,
    {
        diesel::update(
            procedure_templates::table.find(GetColumn::<procedure_templates::id>::get_column(self)),
        )
        .set(diesel::ExpressionMethods::eq(procedure_templates::state_id, "published"))
        .execute(conn)
    }
}

impl<T> ProcedureTemplateNode for T where
    T: GetColumn<procedure_templates::id> + GetColumn<procedure_templates::organization_id>
{
}
