use crate::{ad, util::Named};

// Represents a dogma buff.
//
// A dogma buff applies modifications to multiple ships, and the modifications stick for some time.
// For instance, fleet effects are implemented as dogma buffs.
pub(crate) struct RBuff {
    a_buff: ad::ABuff,
}
impl RBuff {
    pub(crate) fn new(a_buff: ad::ABuff) -> Self {
        Self { a_buff }
    }
    pub(crate) fn get_id(&self) -> ad::ABuffId {
        self.a_buff.id
    }
    // Defines how multiple modifications of the same attribute value are aggregated.
    pub(crate) fn get_aggr_mode(&self) -> ad::ABuffAggrMode {
        self.a_buff.aggr_mode
    }
    // Operation to use when applying the buff's modifiers.
    pub(crate) fn get_op(&self) -> ad::AOp {
        self.a_buff.op
    }
    // Attribute modifiers carried by the buff.
    pub(crate) fn get_mods(&self) -> &Vec<ad::ABuffModifier> {
        &self.a_buff.mods
    }
}
impl Named for RBuff {
    fn get_name() -> &'static str {
        "RBuff"
    }
}
