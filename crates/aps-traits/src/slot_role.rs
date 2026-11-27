//! The roles of `slot_roles`.

/// What a procedure does to the assets bound to a slot, a row of `slot_roles`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SlotRole {
    /// The asset exists before and after the procedure.
    Input,
    /// The procedure uses the asset up.
    Consumed,
    /// The procedure creates the asset.
    Produced,
    /// Steps of the procedure create and use up the asset.
    Intermediate,
    /// The asset performs the procedure without entering the lineage of what
    /// it produces.
    Instrument,
}

impl SlotRole {
    /// The identifier of the role in `slot_roles`.
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::Input => "input",
            Self::Consumed => "consumed",
            Self::Produced => "produced",
            Self::Intermediate => "intermediate",
            Self::Instrument => "instrument",
        }
    }

    /// The role `id` names in `slot_roles`, if it names one.
    #[must_use]
    pub fn from_id(id: &str) -> Option<Self> {
        [Self::Input, Self::Consumed, Self::Produced, Self::Intermediate, Self::Instrument]
            .into_iter()
            .find(|role| role.id() == id)
    }
}
