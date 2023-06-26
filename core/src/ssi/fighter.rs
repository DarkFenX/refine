use std::fmt;

use crate::{
    ad,
    consts::{EffectMode, State},
    defs::{EffectId, ReeId, ReeInt},
    src::Src,
    util::{Named, OptMap},
};

pub(crate) struct SsFighter {
    pub(crate) id: ReeId,
    pub(crate) fit_id: ReeId,
    pub(crate) a_item_id: ReeInt,
    pub(crate) state: State,
    pub(crate) amt_override: Option<ReeInt>,
    pub(crate) effect_modes: OptMap<EffectId, EffectMode>,
    pub(crate) a_item: Option<ad::ArcItem>,
}
impl SsFighter {
    pub(crate) fn new(src: &Src, id: ReeId, fit_id: ReeId, a_item_id: ReeInt, state: State) -> Self {
        Self {
            id,
            fit_id,
            a_item_id,
            state,
            amt_override: None,
            effect_modes: OptMap::new(),
            a_item: src.get_a_item(&a_item_id),
        }
    }
}
impl Named for SsFighter {
    fn get_name() -> &'static str {
        "SsFighter"
    }
}
impl fmt::Display for SsFighter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}(id={}, a_item_id={})", Self::get_name(), self.id, self.a_item_id)
    }
}
