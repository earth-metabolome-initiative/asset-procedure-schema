//! A published template for making a pizza, the example the visitor and
//! visualization tests run on.

use std::fmt::Debug;

use aps_physical_asset_models::physical_asset_models;
use aps_procedure_templates::{
    procedure_template_slots, procedure_template_step_edges, procedure_template_steps,
    procedure_templates,
};
use aps_resources::resources;
use aps_sample_models::{asset_model_ancestors, procedure_template_slot_bindings};
use aps_traits::{
    IntoValidationError, ProcedureTemplateNode, ProcedureTemplateStepNode, Publish, SlotRole,
};
use diesel::{Connection, query_dsl::methods::ExecuteDsl};
use diesel_builders::{Insert, LoadMany, NestedModel, TableBuilder};

use crate::Tenant;

/// The template [`Tenant::pizza`] inserts and the asset models of its slots.
#[derive(Debug, Clone)]
pub struct Pizza {
    /// The root template, "Make a pizza".
    template: NestedModel<procedure_templates::table>,
    /// Model of the flour the dough consumes.
    flour: NestedModel<physical_asset_models::table>,
    /// Model of the dough, produced and then consumed.
    dough: NestedModel<physical_asset_models::table>,
    /// Model of the pizza the run produces.
    pizza: NestedModel<physical_asset_models::table>,
    /// Model of the oven baking it.
    oven: NestedModel<physical_asset_models::table>,
}

impl Pizza {
    /// The root template, "Make a pizza".
    #[must_use]
    pub const fn template(&self) -> &NestedModel<procedure_templates::table> {
        &self.template
    }

    /// Model of the flour the dough consumes.
    #[must_use]
    pub const fn flour(&self) -> &NestedModel<physical_asset_models::table> {
        &self.flour
    }

    /// Model of the dough, produced and then consumed.
    #[must_use]
    pub const fn dough(&self) -> &NestedModel<physical_asset_models::table> {
        &self.dough
    }

    /// Model of the pizza the run produces.
    #[must_use]
    pub const fn pizza(&self) -> &NestedModel<physical_asset_models::table> {
        &self.pizza
    }

    /// Model of the oven baking it.
    #[must_use]
    pub const fn oven(&self) -> &NestedModel<physical_asset_models::table> {
        &self.oven
    }
}

impl Tenant {
    /// Inserts and publishes "Make a pizza", whose slots are `flour`
    /// (consumed), `dough` (intermediate), `pizza` (produced) and `oven`
    /// (instrument). Its steps prepare the dough from the flour, then add each
    /// of `toppings` to the dough, all at once, then bake the dough into the
    /// pizza in the oven. Every step runs a published template of its own
    /// whose slots the step binds to the root's.
    ///
    /// # Panics
    ///
    /// Panics when the connection refuses a row, which includes calling it
    /// twice for one tenant since model names are unique in a project.
    pub fn pizza<C>(&self, toppings: &[&str], conn: &mut C) -> Pizza
    where
        C: Connection,
        Publish: ExecuteDsl<C>,
        TableBuilder<resources::table>: Insert<C, ValidationError: Debug>,
        TableBuilder<physical_asset_models::table>: Insert<C, ValidationError: Debug>,
        TableBuilder<asset_model_ancestors::table>: Insert<C, ValidationError: Debug>,
        (asset_model_ancestors::descendant_model_id,): LoadMany<C>,
        TableBuilder<procedure_templates::table>: Insert<C, ValidationError: Debug>,
        TableBuilder<procedure_template_slots::table>:
            Insert<C, ValidationError: IntoValidationError>,
        TableBuilder<procedure_template_steps::table>:
            Insert<C, ValidationError: IntoValidationError>,
        TableBuilder<procedure_template_step_edges::table>:
            Insert<C, ValidationError: IntoValidationError>,
        TableBuilder<procedure_template_slot_bindings::table>:
            Insert<C, ValidationError: IntoValidationError>,
    {
        let flour = self.physical_asset_model("Flour", conn);
        let dough = self.physical_asset_model("Dough", conn);
        let pizza = self.physical_asset_model("Pizza", conn);
        let oven = self.physical_asset_model("Oven", conn);

        let prepare = self.procedure_template("Prepare dough", conn);
        let prepare_flour =
            prepare.slot("flour", &flour, SlotRole::Consumed, 1, None, conn).expect("flour slot");
        let prepare_dough =
            prepare.slot("dough", &dough, SlotRole::Produced, 1, None, conn).expect("dough slot");
        prepare.publish(conn).expect("publish prepare dough");

        let toppings: Vec<_> = toppings
            .iter()
            .map(|&topping| {
                let template = self.procedure_template(topping, conn);
                let slot = template
                    .slot("dough", &dough, SlotRole::Input, 1, None, conn)
                    .expect("topped dough slot");
                template.publish(conn).expect("publish topping");
                (topping, template, slot)
            })
            .collect();

        let bake = self.procedure_template("Bake pizza", conn);
        let bake_dough =
            bake.slot("dough", &dough, SlotRole::Consumed, 1, None, conn).expect("dough slot");
        let bake_pizza =
            bake.slot("pizza", &pizza, SlotRole::Produced, 1, None, conn).expect("pizza slot");
        let bake_oven =
            bake.slot("oven", &oven, SlotRole::Instrument, 1, None, conn).expect("oven slot");
        bake.publish(conn).expect("publish bake pizza");

        let template = self.procedure_template("Make a pizza", conn);
        let root_flour =
            template.slot("flour", &flour, SlotRole::Consumed, 1, None, conn).expect("flour slot");
        let root_dough = template
            .slot("dough", &dough, SlotRole::Intermediate, 1, None, conn)
            .expect("dough slot");
        let root_pizza =
            template.slot("pizza", &pizza, SlotRole::Produced, 1, None, conn).expect("pizza slot");
        let root_oven =
            template.slot("oven", &oven, SlotRole::Instrument, 1, None, conn).expect("oven slot");

        let prepare_step =
            template.step(&prepare, "prepare the dough", 0, conn).expect("prepare step");
        prepare_step.bind(&prepare_flour, &root_flour, conn).expect("bind flour");
        prepare_step.bind(&prepare_dough, &root_dough, conn).expect("bind prepared dough");
        let bake_step = template.step(&bake, "bake", 2, conn).expect("bake step");
        bake_step.bind(&bake_dough, &root_dough, conn).expect("bind baked dough");
        bake_step.bind(&bake_pizza, &root_pizza, conn).expect("bind pizza");
        bake_step.bind(&bake_oven, &root_oven, conn).expect("bind oven");
        for (topping, topping_template, topping_dough) in &toppings {
            let step = template.step(topping_template, topping, 1, conn).expect("topping step");
            step.bind(topping_dough, &root_dough, conn).expect("bind topped dough");
            prepare_step.precedes(&step, conn).expect("dough before topping");
            step.precedes(&bake_step, conn).expect("topping before baking");
        }
        if toppings.is_empty() {
            prepare_step.precedes(&bake_step, conn).expect("dough before baking");
        }
        template.publish(conn).expect("publish make a pizza");

        Pizza { template, flour, dough, pizza, oven }
    }
}
