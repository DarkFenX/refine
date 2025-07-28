use crate::{
    ad::{ABuff, ABuffAggrMode, ABuffId, ABuffModifier, AOp},
    util::{GetId, Named},
};

// Represents a dogma buff.
//
// A dogma buff applies modifications to multiple ships, and the modifications stick for some time.
// For instance, fleet effects are implemented as dogma buffs.
pub(crate) struct RBuff {
    a_buff: ABuff,
}
impl RBuff {
    pub(in crate::rd) fn new(a_buff: ABuff) -> Self {
        Self { a_buff }
    }
    // Defines how multiple modifications of the same attribute value are aggregated.
    pub(crate) fn get_aggr_mode(&self) -> ABuffAggrMode {
        self.a_buff.aggr_mode
    }
    // Operation to use when applying the buff's modifiers.
    pub(crate) fn get_op(&self) -> AOp {
        self.a_buff.op
    }
    // Attribute modifiers carried by the buff.
    pub(crate) fn get_mods(&self) -> &Vec<ABuffModifier> {
        &self.a_buff.mods
    }
}
impl GetId<ABuffId> for RBuff {
    fn get_id(&self) -> ABuffId {
        self.a_buff.id
    }
}
impl Named for RBuff {
    fn get_name() -> &'static str {
        "RBuff"
    }
}
