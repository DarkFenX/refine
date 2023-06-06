use crate::{
    defs::{ReeId, ReeInt},
    ssi,
};

pub struct SsBoosterInfo {
    pub id: ReeId,
    pub fit_id: ReeId,
    pub a_item_id: ReeInt,
    pub enabled: bool,
}
impl SsBoosterInfo {
    fn new(id: ReeId, fit_id: ReeId, a_item_id: ReeInt, enabled: bool) -> Self {
        Self {
            id,
            fit_id,
            a_item_id,
            enabled,
        }
    }
}
impl From<&ssi::SsBooster> for SsBoosterInfo {
    fn from(ss_booster: &ssi::SsBooster) -> Self {
        SsBoosterInfo::new(
            ss_booster.id,
            ss_booster.fit_id,
            ss_booster.a_item_id,
            ss_booster.get_bool_state(),
        )
    }
}
