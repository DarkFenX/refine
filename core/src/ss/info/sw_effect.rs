use crate::{
    defs::{ReeId, ReeInt},
    ss::item::SwEffect,
};

pub struct SwEffectInfo {
    pub item_id: ReeId,
    pub type_id: ReeInt,
    pub enabled: bool,
}
impl SwEffectInfo {
    fn new(item_id: ReeId, type_id: ReeInt, enabled: bool) -> Self {
        Self {
            item_id,
            type_id,
            enabled,
        }
    }
}
impl From<&SwEffect> for SwEffectInfo {
    fn from(e: &SwEffect) -> Self {
        SwEffectInfo::new(e.item_id, e.type_id, e.get_bool_state())
    }
}
