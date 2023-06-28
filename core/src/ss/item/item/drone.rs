use crate::{
    ad,
    consts::{EffectMode, State},
    defs::{EEffectId, EItemId, SsFitId, SsItemId},
    src::Src,
    util::{Named, OptMap},
};

pub(in crate::ss) struct SsDrone {
    pub(in crate::ss) id: SsItemId,
    pub(in crate::ss) fit_id: SsFitId,
    pub(in crate::ss) a_item_id: EItemId,
    pub(in crate::ss) state: State,
    pub(in crate::ss) effect_modes: OptMap<EEffectId, EffectMode>,
    pub(in crate::ss) a_item: Option<ad::ArcItem>,
}
impl SsDrone {
    pub(in crate::ss) fn new(src: &Src, id: SsItemId, fit_id: SsFitId, a_item_id: EItemId, state: State) -> Self {
        Self {
            id,
            fit_id,
            a_item_id,
            state,
            effect_modes: OptMap::new(),
            a_item: src.get_a_item(&a_item_id),
        }
    }
}
impl Named for SsDrone {
    fn get_name() -> &'static str {
        "SsDrone"
    }
}
impl std::fmt::Display for SsDrone {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}(id={}, a_item_id={})", Self::get_name(), self.id, self.a_item_id)
    }
}
