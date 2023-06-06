use crate::{
    defs::{ReeId, ReeInt},
    ssi,
};

pub struct SsStanceInfo {
    pub id: ReeId,
    pub fit_id: ReeId,
    pub a_item_id: ReeInt,
    pub enabled: bool,
}
impl SsStanceInfo {
    fn new(id: ReeId, fit_id: ReeId, a_item_id: ReeInt, enabled: bool) -> Self {
        Self {
            id,
            fit_id,
            a_item_id,
            enabled,
        }
    }
}
impl From<&ssi::SsStance> for SsStanceInfo {
    fn from(ss_stance: &ssi::SsStance) -> Self {
        SsStanceInfo::new(
            ss_stance.id,
            ss_stance.fit_id,
            ss_stance.a_item_id,
            ss_stance.get_bool_state(),
        )
    }
}
