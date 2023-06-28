use crate::{
    defs::{ItemId, SsFitId, SsItemId},
    ss::item::SsBooster,
};

pub struct SsBoosterInfo {
    pub id: SsItemId,
    pub fit_id: SsFitId,
    pub a_item_id: ItemId,
    pub enabled: bool,
}
impl SsBoosterInfo {
    fn new(id: SsItemId, fit_id: SsFitId, a_item_id: ItemId, enabled: bool) -> Self {
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
