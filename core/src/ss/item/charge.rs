use std::{fmt, sync::Arc};

use crate::{
    consts::State,
    ct,
    defs::{ReeId, ReeInt},
    src::Src,
    ss::item::{bool_to_state, state_to_bool},
    util::Named,
};

pub(in crate::ss) struct Charge {
    pub(in crate::ss) item_id: ReeId,
    pub(in crate::ss) fit_id: ReeId,
    pub(in crate::ss) type_id: ReeInt,
    pub(in crate::ss) container_id: ReeId,
    pub(in crate::ss) state: State,
    pub(in crate::ss) cached_item: Option<Arc<ct::Item>>,
}
impl Charge {
    pub(in crate::ss) fn new(
        src: &Arc<Src>,
        item_id: ReeId,
        fit_id: ReeId,
        type_id: ReeInt,
        container_id: ReeId,
    ) -> Self {
        Self {
            item_id,
            fit_id,
            type_id,
            container_id,
            state: State::Offline,
            cached_item: src.cache_handler.get_item(&type_id),
        }
    }
    pub(in crate::ss) fn get_bool_state(&self) -> bool {
        state_to_bool(self.state)
    }
    pub(in crate::ss) fn set_bool_state(&mut self, state: bool) {
        self.state = bool_to_state(state);
    }
}
impl Named for Charge {
    fn get_name() -> &'static str {
        "ssi:Charge"
    }
}
impl fmt::Display for Charge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}(id={}, type_id={})", Self::get_name(), self.item_id, self.type_id)
    }
}
