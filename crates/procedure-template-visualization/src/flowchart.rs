//! The flowchart of a procedure template graph. Each step is a node nested in
//! the node of the procedure running it, solid arrows order the steps of one
//! template, and each slot is a node shaped by its role and coloured by the
//! slot its assets come from, with a dashed arrow from the parent's slot a
//! step binding hands down.

use core::convert::Infallible;
use std::{collections::BTreeMap, rc::Rc};

use aps_asset_model_ancestors::procedure_template_slot_bindings;
use aps_procedure_templates::{
    ProcedureTemplateSlot, procedure_template_slots, procedure_template_steps, procedure_templates,
};
use aps_traits::SlotRole;
use diesel_builders::GetColumnExt;
use mermaid_builder::{
    prelude::{
        ArrowShape, Color, ConfigurationBuilder, DiagramBuilder, Direction, Flowchart,
        FlowchartBuilder, FlowchartConfigurationBuilder, FlowchartEdgeBuilder, FlowchartNode,
        FlowchartNodeBuilder, FlowchartNodeShape, LineStyle, Renderer, StyleClass,
        StyleClassBuilder, StyleProperty, Unit,
    },
    traits::{EdgeBuilder, NodeBuilder},
};
use procedure_template_visitor::{
    ProcedureTemplateGraph, StepNode, TemplateNode, Visitor, walk_template,
};
use rosetta_uuid::Uuid;

use crate::{Error, table_icons::template_icon};

/// Fill of a step node directly under the root, darkened at each level below.
const STEP_FILL: (u8, u8, u8) = (236, 237, 238);

/// Stroke of a step node directly under the root, darkened at each level
/// below.
const STEP_STROKE: (u8, u8, u8) = (213, 212, 211);

/// Lightness lost by a step node at each level below the root.
const DARKEN_PER_LEVEL: u8 = 3;

/// Draws `graph` as a Mermaid flowchart titled by its root template.
///
/// # Errors
///
/// Returns an error when a slot names a role outside `slot_roles` or the
/// flowchart builder refuses a node, an edge or a style class.
///
/// # Example
///
/// ```rust
/// use aps_test_utils::{Replica, Tenant};
/// use procedure_template_visitor::ProcedureTemplateGraph;
/// use procedure_template_visualization::flowchart;
///
/// let mut replica = Replica::new();
/// let tenant = replica.deliver(|conn| Ok(Tenant::insert(conn))).unwrap();
/// replica.act_as(Some(tenant.user()));
/// let graph = replica
///     .write(|conn| {
///         let pizza = tenant.pizza(&["Add olives"], conn);
///         ProcedureTemplateGraph::new(pizza.template(), conn)
///     })
///     .unwrap();
/// let mermaid = flowchart(&graph).unwrap().to_string();
/// assert!(mermaid.contains("Add olives"));
/// assert!(mermaid.contains("dough (input)"));
/// ```
pub fn flowchart(graph: &ProcedureTemplateGraph) -> Result<Flowchart, Error> {
    let mut owners = Owners::default();
    let Ok(()) = graph.accept(&mut owners);
    let root_name = graph.root().template().get_column_ref::<procedure_templates::name>();
    let mut builder = FlowchartBuilder::default().configuration(
        FlowchartConfigurationBuilder::default()
            .renderer(Renderer::EclipseLayoutKernel)
            .direction(Direction::TopToBottom)
            .title(root_name)?,
    )?;
    let arrow = builder.style_class(
        StyleClassBuilder::default()
            .name("step_arrow")?
            .property(StyleProperty::Stroke(Color::from((26, 26, 26))))?,
    )?;
    let number_of_owners = u16::try_from(owners.slots.len()).unwrap_or(u16::MAX);
    let mut classes = BTreeMap::new();
    for (index, (owner, color)) in
        owners.slots.iter().zip(Color::maximally_distinct(number_of_owners, 70, 80)).enumerate()
    {
        let node = builder.style_class(
            StyleClassBuilder::default()
                .name(format!("slot_{index}"))?
                .property(StyleProperty::Fill(color))?
                .property(StyleProperty::Stroke(color.darken(30)))?
                .property(StyleProperty::BorderRadius(Unit::Pixel(2)))?,
        )?;
        let edge = builder.style_class(
            StyleClassBuilder::default()
                .name(format!("binding_{index}"))?
                .property(StyleProperty::Stroke(color.darken(30)))?
                .property(StyleProperty::StrokeWidth(Unit::Pixel(2)))?,
        )?;
        classes.insert(*owner, SlotClasses { node, edge });
    }
    let mut drawing = Drawing { builder, open: Vec::new(), nodes: BTreeMap::new(), classes, arrow };
    graph.accept(&mut drawing)?;
    Ok(drawing.builder.into())
}

/// The slots whose procedures bind their own assets, in the order the walk
/// meets them.
#[derive(Debug, Default)]
struct Owners {
    /// Identifiers of the slots.
    slots: Vec<Uuid>,
}

impl<'graph> Visitor<'graph> for Owners {
    type Error = Infallible;

    fn visit_slot(
        &mut self,
        graph: &'graph ProcedureTemplateGraph,
        path: &[&'graph StepNode],
        _template: &'graph TemplateNode,
        slot: &'graph ProcedureTemplateSlot,
    ) -> Result<(), Self::Error> {
        let owner =
            graph.slot_owner(path, slot).slot().get_column::<procedure_template_slots::id>();
        if !self.slots.contains(&owner) {
            self.slots.push(owner);
        }
        Ok(())
    }
}

/// The style classes of the slots sharing one owner.
#[derive(Debug)]
struct SlotClasses {
    /// Class of their nodes.
    node: Rc<StyleClass>,
    /// Class of the dashed arrows handing their assets down.
    edge: Rc<StyleClass>,
}

/// The visitor drawing the flowchart.
struct Drawing {
    /// The flowchart drawn so far.
    builder: FlowchartBuilder,
    /// The step nodes the walk is inside, innermost last.
    open: Vec<FlowchartNodeBuilder>,
    /// The nodes drawn, by the step identifiers of their path followed by
    /// their own: the step's for a step node, the slot's for a slot node.
    nodes: BTreeMap<Vec<Uuid>, Rc<FlowchartNode>>,
    /// The style classes of each owning slot.
    classes: BTreeMap<Uuid, SlotClasses>,
    /// The class of the arrows ordering steps.
    arrow: Rc<StyleClass>,
}

impl Drawing {
    /// Places `node` inside the innermost open step node, or at the top of
    /// the flowchart outside every step.
    fn attach(&mut self, node: Rc<FlowchartNode>) -> Result<(), Error> {
        if let Some(parent) = self.open.pop() {
            self.open.push(parent.subnode(node)?);
        }
        Ok(())
    }

    /// The node drawn for `id` under the steps of `path`.
    ///
    /// # Panics
    ///
    /// Panics when the walk has not drawn it yet, which the order of the walk
    /// rules out: slots before steps, and every step of a template before its
    /// arrows.
    fn node(&self, path: &[&StepNode], id: Uuid) -> Rc<FlowchartNode> {
        Rc::clone(
            self.nodes.get(&key(path, id)).expect("the walk draws a node before pointing at it"),
        )
    }
}

impl<'graph> Visitor<'graph> for Drawing {
    type Error = Error;

    fn visit_template(
        &mut self,
        graph: &'graph ProcedureTemplateGraph,
        path: &[&'graph StepNode],
        template: &'graph TemplateNode,
    ) -> Result<(), Self::Error> {
        if let Some(step) = path.last() {
            self.open.push(step_node(graph, path, template, step)?);
        }
        walk_template(self, graph, path, template)?;
        let task_graph = template.task_graph();
        for step in task_graph.steps() {
            for successor in task_graph.successors(step) {
                self.builder.edge(
                    FlowchartEdgeBuilder::default()
                        .source(self.node(path, step_id(step)))?
                        .destination(self.node(path, step_id(successor)))?
                        .right_arrow_shape(ArrowShape::Normal)?
                        .style_class(Rc::clone(&self.arrow))?,
                )?;
            }
        }
        if let Some((step, parents)) = path.split_last() {
            let builder = self.open.pop().expect("the step node opened above");
            let node = self.builder.node(builder)?;
            self.attach(Rc::clone(&node))?;
            self.nodes.insert(key(parents, step_id(step)), node);
        }
        Ok(())
    }

    fn visit_slot(
        &mut self,
        graph: &'graph ProcedureTemplateGraph,
        path: &[&'graph StepNode],
        _template: &'graph TemplateNode,
        slot: &'graph ProcedureTemplateSlot,
    ) -> Result<(), Self::Error> {
        let name = slot.get_column_ref::<procedure_template_slots::name>();
        let role_id = slot.get_column_ref::<procedure_template_slots::role_id>();
        let role = SlotRole::from_id(role_id)
            .ok_or_else(|| Error::UnknownSlotRole { slot: name.clone(), role: role_id.clone() })?;
        let owner =
            graph.slot_owner(path, slot).slot().get_column::<procedure_template_slots::id>();
        let classes = &self.classes[&owner];
        let edge_class = Rc::clone(&classes.edge);
        let node = self.builder.node(
            FlowchartNodeBuilder::default()
                .label(format!("{name} ({})", role.id()))?
                .shape(role_shape(role))
                .style_class(Rc::clone(&classes.node))?,
        )?;
        self.attach(Rc::clone(&node))?;
        let slot_id = slot.get_column::<procedure_template_slots::id>();
        if let Some((step, parents)) = path.split_last()
            && let Some(binding) = step.binding_of(slot_id)
        {
            let parent_slot =
                binding.get_column::<procedure_template_slot_bindings::parent_slot_id>();
            self.builder.edge(
                FlowchartEdgeBuilder::default()
                    .source(self.node(parents, parent_slot))?
                    .destination(Rc::clone(&node))?
                    .line_style(LineStyle::Dashed)
                    .style_class(edge_class)?,
            )?;
        }
        self.nodes.insert(key(path, slot_id), node);
        Ok(())
    }
}

/// The open node of `step`, which runs `template` at the end of `path`:
/// labelled by the step and the template when their names differ, dashed
/// when the step is optional, and darker the deeper it sits.
fn step_node(
    graph: &ProcedureTemplateGraph,
    path: &[&StepNode],
    template: &TemplateNode,
    step: &StepNode,
) -> Result<FlowchartNodeBuilder, Error> {
    let step_name = step.step().get_column_ref::<procedure_template_steps::name>();
    let template_name = template.template().get_column_ref::<procedure_templates::name>();
    let mut label = if step_name == template_name {
        step_name.clone()
    } else {
        format!("{step_name}: {template_name}")
    };
    if let Some(icon) =
        template_icon(template.template().get_column_ref::<procedure_templates::table_name_id>())
    {
        label = format!("{icon} {label}");
    }
    if let Some(repeat_slot) = step.step().get_column::<procedure_template_steps::repeat_slot_id>()
    {
        let repeat_slot = graph
            .parent_of(step)
            .slot(repeat_slot)
            .expect("a step repeats over a slot of its parent template")
            .get_column_ref::<procedure_template_slots::name>();
        label = format!("{label}, once per {repeat_slot}");
    }
    let darken = u8::try_from(path.len() - 1).unwrap_or(u8::MAX).saturating_mul(DARKEN_PER_LEVEL);
    let mut node = FlowchartNodeBuilder::default()
        .label(label)?
        .shape(FlowchartNodeShape::RoundEdges)
        .style_property(StyleProperty::BorderRadius(Unit::Pixel(3)))?
        .style_property(StyleProperty::Fill(Color::from(STEP_FILL).darken(darken)))?
        .style_property(StyleProperty::Stroke(Color::from(STEP_STROKE).darken(darken)))?;
    if step.step().get_column::<procedure_template_steps::optional>() {
        node = node.style_property(StyleProperty::StrokeDasharray(5, 5))?;
    }
    Ok(node)
}

/// The shape of a slot node of `role`.
const fn role_shape(role: SlotRole) -> FlowchartNodeShape {
    match role {
        SlotRole::Input => FlowchartNodeShape::Rectangle,
        SlotRole::Consumed => FlowchartNodeShape::ReverseTrapezoid,
        SlotRole::Produced => FlowchartNodeShape::Trapezoid,
        SlotRole::Intermediate => FlowchartNodeShape::Hexagon,
        SlotRole::Instrument => FlowchartNodeShape::Subprocess,
    }
}

/// The identifier of `step`.
fn step_id(step: &StepNode) -> Uuid {
    step.step().get_column::<procedure_template_steps::id>()
}

/// The key of the node drawn for `id` under the steps of `path`.
fn key(path: &[&StepNode], id: Uuid) -> Vec<Uuid> {
    path.iter().map(|step| step_id(step)).chain([id]).collect()
}
