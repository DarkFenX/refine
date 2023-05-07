use std::{fmt, sync::Arc};

use crate::{consts::State, ct, util::Named, ReeId, ReeInt, Src};

pub struct RigInfo {
    pub item_id: ReeId,
    pub fit_id: ReeId,
    pub type_id: ReeInt,
    pub state: bool,
}
impl RigInfo {
    fn new(item_id: ReeId, fit_id: ReeId, type_id: ReeInt, state: bool) -> Self {
        Self {
            item_id,
            fit_id,
            type_id,
            state,
        }
    }
}
impl From<&Rig> for RigInfo {
    fn from(r: &Rig) -> Self {
        RigInfo::new(r.item_id, r.fit_id, r.type_id, r.get_bool_state())
    }
}

pub(in crate::ss) struct Rig {
    pub(in crate::ss) item_id: ReeId,
    pub(in crate::ss) fit_id: ReeId,
    pub(in crate::ss) type_id: ReeInt,
    pub(in crate::ss) state: State,
    pub(in crate::ss) citem: Option<Arc<ct::Item>>,
}
impl Rig {
    pub(in crate::ss) fn new(src: &Arc<Src>, item_id: ReeId, fit_id: ReeId, type_id: ReeInt) -> Self {
        Self {
            item_id,
            fit_id,
            type_id,
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
impl Named for Rig {
    fn get_name() -> &'static str {
        "ssi:Rig"
    }
}
impl fmt::Display for Rig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}(id={}, type_id={})", Self::get_name(), self.item_id, self.type_id)
    }
}
