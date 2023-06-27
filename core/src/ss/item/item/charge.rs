use crate::{
    ad,
    consts::EffectMode,
    defs::{EffectId, ReeId, ReeInt},
    src::Src,
    util::{Named, OptMap},
};

pub(in crate::ss) struct SsCharge {
    pub(in crate::ss) id: ReeId,
    pub(in crate::ss) fit_id: ReeId,
    pub(in crate::ss) a_item_id: ReeInt,
    pub(in crate::ss) cont_id: ReeId,
    pub(in crate::ss) effect_modes: OptMap<EffectId, EffectMode>,
    pub(in crate::ss) a_item: Option<ad::ArcItem>,
}
impl SsCharge {
    pub(in crate::ss) fn new(src: &Src, id: ReeId, fit_id: ReeId, a_item_id: ReeInt, cont_id: ReeId) -> Self {
        Self {
            id,
            fit_id,
            a_item_id,
            cont_id,
            effect_modes: OptMap::new(),
            a_item: src.get_a_item(&a_item_id),
        }
    }
}
impl Named for SsCharge {
    fn get_name() -> &'static str {
        "SsCharge"
    }
}
impl std::fmt::Display for SsCharge {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}(id={}, a_item_id={})", Self::get_name(), self.id, self.a_item_id)
    }
}
