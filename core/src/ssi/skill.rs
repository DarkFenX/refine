use std::{fmt, sync::Arc};

use crate::{
    ad,
    consts::State,
    defs::{ReeId, ReeInt},
    src::Src,
    util::Named,
};

use super::{bool_to_state, state_to_bool};

pub(crate) struct SsSkill {
    pub(crate) id: ReeId,
    pub(crate) fit_id: ReeId,
    pub(crate) a_item_id: ReeInt,
    pub(crate) level: ReeInt,
    pub(crate) state: State,
    pub(crate) a_item: Option<Arc<ad::AItem>>,
}
impl SsSkill {
    pub(crate) fn new(src: &Src, id: ReeId, fit_id: ReeId, a_item_id: ReeInt, level: ReeInt, state: bool) -> Self {
        Self {
            id,
            fit_id,
            a_item_id,
            level,
            state: bool_to_state(state),
            a_item: src.get_a_item(&a_item_id),
        }
    }
    pub(crate) fn get_bool_state(&self) -> bool {
        state_to_bool(self.state)
    }
    pub(crate) fn set_bool_state(&mut self, state: bool) {
        self.state = bool_to_state(state);
    }
}
impl Named for SsSkill {
    fn get_name() -> &'static str {
        "SsSkill"
    }
}
impl fmt::Display for SsSkill {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}(id={}, a_item_id={})", Self::get_name(), self.id, self.a_item_id)
    }
}
