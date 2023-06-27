use crate::{
    defs::{ReeId, ReeInt},
    ss::item::SsImplant,
};

pub struct SsImplantInfo {
    pub id: ReeId,
    pub fit_id: ReeId,
    pub a_item_id: ReeInt,
    pub enabled: bool,
}
impl SsImplantInfo {
    fn new(id: ReeId, fit_id: ReeId, a_item_id: ReeInt, enabled: bool) -> Self {
        Self {
            id,
            fit_id,
            a_item_id,
            enabled,
        }
    }
}
impl From<&SsImplant> for SsImplantInfo {
    fn from(ss_implant: &SsImplant) -> Self {
        SsImplantInfo::new(
            ss_implant.id,
            ss_implant.fit_id,
            ss_implant.a_item_id,
            ss_implant.get_bool_state(),
        )
    }
}
