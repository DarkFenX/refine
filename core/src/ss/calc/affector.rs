use crate::{ReeId, ReeIdx, ReeInt};

pub(in crate::ss::calc) struct AffectorSpec {
    item_id: ReeId,
    effect_id: ReeInt,
    modifier_idx: ReeIdx,
}
impl AffectorSpec {
    pub(in crate::ss::calc) fn new(item_id: ReeId, effect_id: ReeInt, modifier_idx: ReeIdx) -> AffectorSpec {
        AffectorSpec {
            item_id,
            effect_id,
            modifier_idx,
        }
    }
}
