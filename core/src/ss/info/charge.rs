use crate::{
    defs::{ItemId, SsFitId, SsItemId},
    ss::item::SsCharge,
};

pub struct SsChargeInfo {
    pub id: SsItemId,
    pub fit_id: SsFitId,
    pub a_item_id: ItemId,
    pub cont_id: SsItemId,
}
impl SsChargeInfo {
    fn new(id: SsItemId, fit_id: SsFitId, a_item_id: ItemId, cont_id: SsItemId) -> Self {
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
