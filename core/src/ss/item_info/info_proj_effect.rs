use crate::{
    defs::{EItemId, SsItemId},
    ss::item::SsProjEffect,
};

pub struct SsProjEffectInfo {
    pub id: SsItemId,
    pub a_item_id: EItemId,
    pub enabled: bool,
    pub tgts: Vec<SsItemId>,
}
impl SsProjEffectInfo {
    fn new(id: SsItemId, a_item_id: EItemId, enabled: bool, tgts: Vec<SsItemId>) -> Self {
        Self {
            id,
            a_item_id,
            enabled,
            tgts,
        }
    }
}
impl From<&SsProjEffect> for SsProjEffectInfo {
    fn from(ss_proj_effect: &SsProjEffect) -> Self {
        SsProjEffectInfo::new(
            ss_proj_effect.id,
            ss_proj_effect.a_item_id,
            ss_proj_effect.get_bool_state(),
            ss_proj_effect.tgts.iter_tgts().map(|v| *v).collect(),
        )
    }
}
