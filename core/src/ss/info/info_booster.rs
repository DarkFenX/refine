use crate::{
    defs::{EItemId, SsFitId, SsItemId},
    ss::item::SsBooster,
};

pub struct SsBoosterInfo {
    pub id: SsItemId,
    pub fit_id: SsFitId,
    pub a_item_id: EItemId,
    pub enabled: bool,
}
impl SsBoosterInfo {
    fn new(id: SsItemId, fit_id: SsFitId, a_item_id: EItemId, enabled: bool) -> Self {
        Self {
            id,
            fit_id,
            a_item_id,
            enabled,
        }
    }
}
impl From<&SsBooster> for SsBoosterInfo {
    fn from(ss_booster: &SsBooster) -> Self {
        SsBoosterInfo::new(
            ss_booster.id,
            ss_booster.fit_id,
            ss_booster.a_item_id,
            ss_booster.get_bool_state(),
        )
    }
}
