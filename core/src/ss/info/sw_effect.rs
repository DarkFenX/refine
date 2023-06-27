use crate::{
    defs::{ReeId, ReeInt},
    ss::item::SsSwEffect,
};

pub struct SsSwEffectInfo {
    pub id: ReeId,
    pub a_item_id: ReeInt,
    pub enabled: bool,
}
impl SsSwEffectInfo {
    fn new(id: ReeId, a_item_id: ReeInt, enabled: bool) -> Self {
        Self { id, a_item_id, enabled }
    }
}
impl From<&SsSwEffect> for SsSwEffectInfo {
    fn from(ss_sw_effect: &SsSwEffect) -> Self {
        SsSwEffectInfo::new(ss_sw_effect.id, ss_sw_effect.a_item_id, ss_sw_effect.get_bool_state())
    }
}
