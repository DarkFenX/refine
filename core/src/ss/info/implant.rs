use crate::{
    defs::{EItemId, SsFitId, SsItemId},
    ss::item::SsImplant,
};

pub struct SsImplantInfo {
    pub id: SsItemId,
    pub fit_id: SsFitId,
    pub a_item_id: EItemId,
    pub enabled: bool,
}
impl SsImplantInfo {
    fn new(id: SsItemId, fit_id: SsFitId, a_item_id: EItemId, enabled: bool) -> Self {
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
