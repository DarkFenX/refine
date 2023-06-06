use std::{fmt, sync::Arc};

use crate::{
    ad,
    consts::State,
    defs::{ReeId, ReeInt},
    src::Src,
    util::Named,
};

use super::{bool_to_state, state_to_bool};

pub(crate) struct SsSwEffect {
    pub(crate) id: ReeId,
    pub(crate) a_item_id: ReeInt,
    pub(crate) state: State,
    pub(crate) a_item: Option<Arc<ad::AItem>>,
}
impl SsSwEffect {
    pub(crate) fn new(src: &Arc<Src>, id: ReeId, a_item_id: ReeInt, state: bool) -> Self {
        Self {
            id,
            a_item_id,
            state: bool_to_state(state),
            a_item: src.ahandler.get_item(&a_item_id),
        }
    }
    pub(crate) fn get_bool_state(&self) -> bool {
        state_to_bool(self.state)
    }
    pub(crate) fn set_bool_state(&mut self, state: bool) {
        self.state = bool_to_state(state);
    }
}
impl Named for SsSwEffect {
    fn get_name() -> &'static str {
        "SsSwEffect"
    }
}
impl fmt::Display for SsSwEffect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}(id={}, a_item_id={})", Self::get_name(), self.id, self.a_item_id)
    }
}
