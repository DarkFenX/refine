use crate::{
    ad,
    defs::{Amount, EItemId, SsFitId, SsItemId},
    shr::State,
    src::Src,
    util::Named,
};

use super::misc::EffectModes;

pub(in crate::ss) struct SsFighter {
    pub(in crate::ss) id: SsItemId,
    pub(in crate::ss) fit_id: SsFitId,
    pub(in crate::ss) a_item_id: EItemId,
    pub(in crate::ss) state: State,
    pub(in crate::ss) amt_override: Option<Amount>,
    pub(in crate::ss) effect_modes: EffectModes,
    pub(in crate::ss) a_item: Option<ad::ArcItem>,
}
impl SsFighter {
    pub(in crate::ss) fn new(src: &Src, id: SsItemId, fit_id: SsFitId, a_item_id: EItemId, state: State) -> Self {
        Self {
            id,
            fit_id,
            a_item_id,
            state,
            amt_override: None,
            effect_modes: EffectModes::new(),
            a_item: src.get_a_item(&a_item_id),
        }
    }
}
impl Named for SsFighter {
    fn get_name() -> &'static str {
        "SsFighter"
    }
}
impl std::fmt::Display for SsFighter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}(id={}, a_item_id={})", Self::get_name(), self.id, self.a_item_id)
    }
}
