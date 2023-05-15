use crate::{
    defs::{ReeId, ReeInt},
    ss::item::Booster,
};

pub struct BoosterInfo {
    pub item_id: ReeId,
    pub fit_id: ReeId,
    pub type_id: ReeInt,
    pub enabled: bool,
}
impl BoosterInfo {
    fn new(item_id: ReeId, fit_id: ReeId, type_id: ReeInt, enabled: bool) -> Self {
        Self {
            item_id,
            fit_id,
            type_id,
            enabled,
        }
    }
}
impl From<&Booster> for BoosterInfo {
    fn from(b: &Booster) -> Self {
        BoosterInfo::new(b.item_id, b.fit_id, b.type_id, b.get_bool_state())
    }
}
