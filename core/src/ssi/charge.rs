use std::fmt;

use crate::{
    ad,
    consts::EffectMode,
    defs::{EffectId, ReeId, ReeInt},
    src::Src,
    util::{Named, OptMap},
};

pub(crate) struct SsCharge {
    pub(crate) id: ReeId,
    pub(crate) fit_id: ReeId,
    pub(crate) a_item_id: ReeInt,
    pub(crate) cont_id: ReeId,
    pub(crate) effect_modes: OptMap<EffectId, EffectMode>,
    pub(crate) a_item: Option<ad::ArcItem>,
}
impl SsCharge {
    pub(crate) fn new(src: &Src, id: ReeId, fit_id: ReeId, a_item_id: ReeInt, cont_id: ReeId) -> Self {
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
impl fmt::Display for SsCharge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}(id={}, a_item_id={})", Self::get_name(), self.id, self.a_item_id)
    }
}
