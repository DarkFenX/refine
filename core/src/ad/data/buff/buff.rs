use crate::{
    ad::{ABuffAggrMode, ABuffModifier, AOp},
    defs::EBuffId,
};

/// Represents an adapted dogma buff.
///
/// A dogma buff applies modifications to multiple ships, and the modifications stick for some time.
/// For instance, fleet effects are implemented as dogma buffs.
pub struct ABuff {
    /// Buff ID.
    pub id: EBuffId,
    /// Defines how multiple modifications of the same attribute value are aggregated.
    pub aggr_mode: ABuffAggrMode,
    /// Operation to use when applying the buff's modifiers.
    pub op: AOp,
    /// Attribute modifiers carried by the buff
    pub mods: Vec<ABuffModifier>,
}
impl ABuff {
    /// Make a new dogma buff out of passed data.
    pub(crate) fn new(id: EBuffId, aggr_mode: ABuffAggrMode, op: AOp, mods: Vec<ABuffModifier>) -> Self {
        Self {
            id,
            aggr_mode,
            op,
            mods,
        }
    }
}
