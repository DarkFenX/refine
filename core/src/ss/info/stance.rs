use crate::{
    defs::{ItemId, SsFitId, SsItemId},
    ss::item::SsStance,
};

pub struct SsStanceInfo {
    pub id: SsItemId,
    pub fit_id: SsFitId,
    pub a_item_id: ItemId,
    pub enabled: bool,
}
impl SsStanceInfo {
    fn new(id: SsItemId, fit_id: SsFitId, a_item_id: ItemId, enabled: bool) -> Self {
        Self {
            id,
            fit_id,
            a_item_id,
            enabled,
        }
    }
}
impl From<&SsStance> for SsStanceInfo {
    fn from(ss_stance: &SsStance) -> Self {
        SsStanceInfo::new(
            ss_stance.id,
            ss_stance.fit_id,
            ss_stance.a_item_id,
            ss_stance.get_bool_state(),
        )
    }
}
