use crate::{
    ad::{AAttrId, ABuff, ABuffAggrMode, ABuffId, AOp},
    rd::{RAttrId, RBuffModifier},
    util::RMap,
};

// Represents a dogma buff.
//
// A dogma buff applies modifications to a set of ships, and the modifications can stick for some
// time. For instance, fleet effects are implemented as dogma buffs.
pub(crate) struct RBuff {
    pub(crate) aid: ABuffId,
    pub(crate) aggr_mode: ABuffAggrMode,
    pub(crate) op: AOp,
    pub(crate) mods: Vec<RBuffModifier>,
}
impl RBuff {
    pub(in crate::rd) fn from_a_buff(a_buff: &ABuff) -> Self {
        Self {
            aid: a_buff.id,
            aggr_mode: a_buff.aggr_mode,
            op: a_buff.op,
            // Fields which depend on data not available during instantiation
            mods: Default::default(),
        }
    }
    pub(in crate::rd) fn fill_runtime(
        &mut self,
        a_buffs: &RMap<ABuffId, ABuff>,
        attr_aid_rid_map: &RMap<AAttrId, RAttrId>,
    ) {
        let a_buff = a_buffs.get(&self.aid).unwrap();
        self.mods.extend(
            a_buff
                .mods
                .iter()
                .filter_map(|a_buff_mod| RBuffModifier::try_from_a_buff_mod(a_buff_mod, attr_aid_rid_map)),
        )
    }
}
