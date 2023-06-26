use std::fmt;

use crate::{
    ad,
    consts::{attrs, EffectMode, State},
    defs::{EffectId, ReeId, ReeInt},
    src::Src,
    util::{Named, OptMap},
};

use super::{bool_to_state, state_to_bool};

pub(crate) struct SsBooster {
    pub(crate) id: ReeId,
    pub(crate) fit_id: ReeId,
    pub(crate) a_item_id: ReeInt,
    pub(crate) state: State,
    pub(crate) effect_modes: OptMap<EffectId, EffectMode>,
    pub(crate) a_item: Option<ad::ArcItem>,
}
impl SsBooster {
    pub(crate) fn new(src: &Src, id: ReeId, fit_id: ReeId, a_item_id: ReeInt, state: bool) -> Self {
        Self {
            id,
            fit_id,
            a_item_id,
            state: bool_to_state(state),
            effect_modes: OptMap::new(),
            a_item: src.get_a_item(&a_item_id),
        }
    }
    pub(crate) fn get_bool_state(&self) -> bool {
        state_to_bool(self.state)
    }
    pub(crate) fn set_bool_state(&mut self, state: bool) {
        self.state = bool_to_state(state);
    }
    pub(crate) fn get_slot(&self) -> Option<ReeInt> {
        match &self.a_item {
            None => None,
            Some(a_item) => match a_item.attr_vals.get(&attrs::BOOSTERNESS) {
                None => None,
                Some(value) => Some(value.round() as ReeInt),
            },
        }
    }
}
impl Named for SsBooster {
    fn get_name() -> &'static str {
        "SsBooster"
    }
}
impl fmt::Display for SsBooster {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}(id={}, a_item_id={})", Self::get_name(), self.id, self.a_item_id)
    }
}
