//! The steps of a procedure template and the edges ordering them.

use std::collections::BTreeMap;

use aps_procedure_templates::{
    ProcedureTemplateStep, ProcedureTemplateStepEdge, procedure_template_step_edges,
    procedure_template_steps,
};
use aps_sample_models::{ProcedureTemplateSlotBinding, procedure_template_slot_bindings};
use diesel_builders::GetColumnExt;
use rosetta_uuid::Uuid;

/// A step of a procedure template, with the slot bindings handing its child
/// template's slots the parent's assets.
#[derive(Debug, Clone, PartialEq)]
pub struct StepNode {
    /// The step.
    step: ProcedureTemplateStep,
    /// The bindings of the step, sorted by child slot.
    bindings: Vec<ProcedureTemplateSlotBinding>,
    /// Indices of the steps that complete before this one starts.
    predecessors: Vec<usize>,
    /// Indices of the steps that start after this one completes.
    successors: Vec<usize>,
}

impl StepNode {
    /// The step.
    #[must_use]
    pub const fn step(&self) -> &ProcedureTemplateStep {
        &self.step
    }

    /// The bindings of the step, sorted by child slot.
    #[must_use]
    pub fn bindings(&self) -> &[ProcedureTemplateSlotBinding] {
        &self.bindings
    }

    /// The binding filling `child_slot`, a slot of the template the step runs,
    /// if the step binds it.
    #[must_use]
    pub fn binding_of(&self, child_slot: Uuid) -> Option<&ProcedureTemplateSlotBinding> {
        self.bindings
            .binary_search_by(|binding| {
                binding
                    .get_column_ref::<procedure_template_slot_bindings::child_slot_id>()
                    .cmp(&child_slot)
            })
            .ok()
            .map(|index| &self.bindings[index])
    }
}

/// The steps of a procedure template in `position` order, which is
/// topological for its step edges. A template running no step has an empty
/// task graph.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TaskGraph {
    /// The steps, sorted by position and then by identifier.
    steps: Vec<StepNode>,
}

impl TaskGraph {
    /// Builds the task graph of `steps`, ordered by `edges` and binding slots
    /// through `bindings`, all of one parent template.
    ///
    /// # Panics
    ///
    /// Panics when an edge or a binding names a step outside `steps`, which
    /// the composite keys of the schema rule out.
    pub(crate) fn new(
        mut steps: Vec<ProcedureTemplateStep>,
        edges: Vec<ProcedureTemplateStepEdge>,
        bindings: Vec<ProcedureTemplateSlotBinding>,
    ) -> Self {
        steps.sort_unstable_by_key(|step| {
            (
                step.get_column::<procedure_template_steps::position>(),
                step.get_column::<procedure_template_steps::id>(),
            )
        });
        let index: BTreeMap<Uuid, usize> = steps
            .iter()
            .enumerate()
            .map(|(position, step)| (step.get_column::<procedure_template_steps::id>(), position))
            .collect();
        let mut steps: Vec<StepNode> = steps
            .into_iter()
            .map(|step| {
                StepNode {
                    step,
                    bindings: Vec::new(),
                    predecessors: Vec::new(),
                    successors: Vec::new(),
                }
            })
            .collect();
        for edge in edges {
            let predecessor =
                index[edge.get_column_ref::<procedure_template_step_edges::predecessor_step_id>()];
            let successor =
                index[edge.get_column_ref::<procedure_template_step_edges::successor_step_id>()];
            steps[predecessor].successors.push(successor);
            steps[successor].predecessors.push(predecessor);
        }
        for binding in bindings {
            let step = index[binding.get_column_ref::<procedure_template_slot_bindings::step_id>()];
            steps[step].bindings.push(binding);
        }
        for step in &mut steps {
            step.predecessors.sort_unstable();
            step.successors.sort_unstable();
            step.bindings.sort_unstable_by_key(|binding| {
                binding.get_column::<procedure_template_slot_bindings::child_slot_id>()
            });
        }
        Self { steps }
    }

    /// Whether the template runs no step.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }

    /// The steps in `position` order.
    #[must_use]
    pub fn steps(&self) -> &[StepNode] {
        &self.steps
    }

    /// The steps that complete before `step` starts.
    pub fn predecessors<'graph>(
        &'graph self,
        step: &'graph StepNode,
    ) -> impl Iterator<Item = &'graph StepNode> {
        step.predecessors.iter().map(|&index| &self.steps[index])
    }

    /// The steps that start after `step` completes.
    pub fn successors<'graph>(
        &'graph self,
        step: &'graph StepNode,
    ) -> impl Iterator<Item = &'graph StepNode> {
        step.successors.iter().map(|&index| &self.steps[index])
    }

    /// The steps waiting on no other step, of which a template may have
    /// several.
    pub fn roots(&self) -> impl Iterator<Item = &StepNode> {
        self.steps.iter().filter(|step| step.predecessors.is_empty())
    }

    /// The steps no other step waits on.
    pub fn sinks(&self) -> impl Iterator<Item = &StepNode> {
        self.steps.iter().filter(|step| step.successors.is_empty())
    }

    /// Whether the steps form one chain, each waiting on the one before it.
    #[must_use]
    pub fn is_simple_path(&self) -> bool {
        self.roots().count() <= 1
            && self
                .steps
                .iter()
                .all(|step| step.predecessors.len() <= 1 && step.successors.len() <= 1)
    }
}
