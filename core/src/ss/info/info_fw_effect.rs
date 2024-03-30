use crate::{
    defs::{EItemId, SsFitId, SsItemId},
    ss::item::SsFwEffect,
};

pub struct SsFwEffectInfo {
    pub id: SsItemId,
    pub fit_id: SsFitId,
    pub a_item_id: EItemId,
    pub enabled: bool,
}
impl SsFwEffectInfo {
    fn new(id: SsItemId, fit_id: SsFitId, a_item_id: EItemId, enabled: bool) -> Self {
        Self {
            id,
            fit_id,
            a_item_id,
            enabled,
        }
    }
}
impl From<&SsFwEffect> for SsFwEffectInfo {
    fn from(ss_fw_effect: &SsFwEffect) -> Self {
        SsFwEffectInfo::new(
            ss_fw_effect.id,
            ss_fw_effect.fit_id,
            ss_fw_effect.a_item_id,
            ss_fw_effect.get_bool_state(),
        )
    }
}
