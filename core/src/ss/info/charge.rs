use crate::{
    defs::{ReeId, ReeInt},
    ss::item::SsCharge,
};

pub struct SsChargeInfo {
    pub id: ReeId,
    pub fit_id: ReeId,
    pub a_item_id: ReeInt,
    pub cont_id: ReeId,
}
impl SsChargeInfo {
    fn new(id: ReeId, fit_id: ReeId, a_item_id: ReeInt, cont_id: ReeId) -> Self {
        Self {
            id,
            fit_id,
            a_item_id,
            cont_id,
        }
    }
}
impl From<&SsCharge> for SsChargeInfo {
    fn from(ss_charge: &SsCharge) -> Self {
        SsChargeInfo::new(ss_charge.id, ss_charge.fit_id, ss_charge.a_item_id, ss_charge.cont_id)
    }
}
