use crate::{
    defs::{ItemId, SsItemId},
    ss::item::SsSwEffect,
};

pub struct SsSwEffectInfo {
    pub id: SsItemId,
    pub a_item_id: ItemId,
    pub enabled: bool,
}
impl SsSwEffectInfo {
    fn new(id: SsItemId, a_item_id: ItemId, enabled: bool) -> Self {
        Self { id, a_item_id, enabled }
    }
}
impl From<&SsSwEffect> for SsSwEffectInfo {
    fn from(ss_sw_effect: &SsSwEffect) -> Self {
        SsSwEffectInfo::new(ss_sw_effect.id, ss_sw_effect.a_item_id, ss_sw_effect.get_bool_state())
    }
}
