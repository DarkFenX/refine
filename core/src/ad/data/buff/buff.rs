use crate::ad::{ABuffAggrMode, ABuffId, ABuffModifier, AOp};

/// Represents an adapted dogma buff.
///
/// A dogma buff applies modifications to multiple ships, and the modifications stick for some time.
/// For instance, fleet effects are implemented as dogma buffs.
pub struct ABuff {
    /// Buff ID.
    pub id: ABuffId,
    /// Defines how multiple modifications of the same attribute value are aggregated.
    pub aggr_mode: ABuffAggrMode,
    /// Operation to use when applying the buff's modifiers.
    pub op: AOp,
    /// Attribute modifiers carried by the buff
    pub mods: Vec<ABuffModifier>,
}
