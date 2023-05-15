use std::{fmt, sync::Arc};

use crate::{
    consts::State,
    ct,
    defs::{ReeId, ReeInt},
    src::Src,
    util::Named,
};

pub(in crate::ss) struct Charge {
    pub(in crate::ss) item_id: ReeId,
    pub(in crate::ss) fit_id: ReeId,
    pub(in crate::ss) type_id: ReeInt,
    pub(in crate::ss) cont: ReeId,
    pub(in crate::ss) state: State,
    pub(in crate::ss) citem: Option<Arc<ct::Item>>,
}
impl Charge {
    pub(in crate::ss) fn new(src: &Arc<Src>, item_id: ReeId, fit_id: ReeId, type_id: ReeInt, cont: ReeId) -> Self {
        Self {
            item_id,
            fit_id,
            type_id,
            cont,
            state: State::Offline,
            citem: src.cache_handler.get_item(&type_id),
        }
    }
    pub(in crate::ss) fn get_bool_state(&self) -> bool {
        match self.state {
            State::Ghost => false,
            _ => true,
        }
    }
    pub(in crate::ss) fn set_bool_state(&mut self, state: bool) {
        self.state = match state {
            true => State::Offline,
            false => State::Ghost,
        }
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
