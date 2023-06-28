use crate::{
    ad,
    consts::EffectMode,
    defs::{EffectId, ItemId, SsFitId, SsItemId},
    src::Src,
    util::{Named, OptMap},
};

pub(in crate::ss) struct SsCharge {
    pub(in crate::ss) id: SsItemId,
    pub(in crate::ss) fit_id: SsFitId,
    pub(in crate::ss) a_item_id: ItemId,
    pub(in crate::ss) cont_id: SsItemId,
    pub(in crate::ss) effect_modes: OptMap<EffectId, EffectMode>,
    pub(in crate::ss) a_item: Option<ad::ArcItem>,
}
impl SsCharge {
    pub(in crate::ss) fn new(src: &Src, id: SsItemId, fit_id: SsFitId, a_item_id: ItemId, cont_id: SsItemId) -> Self {
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
