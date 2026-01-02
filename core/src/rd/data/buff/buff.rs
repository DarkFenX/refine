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
    pub(crate) id: ABuffId,
    pub(crate) aggr_mode: ABuffAggrMode,
    pub(crate) op: AOp,
    // Fields which depend on slab keys
    pub(crate) mods: Vec<RBuffModifier>,
}
impl RBuff {
    pub(in crate::rd) fn from_a_buff(a_buff: &ABuff) -> Self {
        Self {
            id: a_buff.id,
            aggr_mode: a_buff.aggr_mode,
            op: a_buff.op,
            // Fields which depend on slab keys
            mods: Default::default(),
        }
    }
    pub(in crate::rd) fn fill_key_dependents(
        &mut self,
        a_buffs: &RMap<ABuffId, ABuff>,
        attr_id_key_map: &RMap<AAttrId, RAttrId>,
    ) {
        let a_buff = a_buffs.get(&self.id).unwrap();
        self.mods.extend(
            a_buff
                .mods
                .iter()
                .filter_map(|a_buff_mod| RBuffModifier::try_from_a_buff_mod(a_buff_mod, attr_id_key_map)),
        )
    }
}
