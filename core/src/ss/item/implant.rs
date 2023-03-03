use std::{fmt, sync::Arc};

use crate::{
    consts::{attrs, State},
    ct,
    util::Named,
    ReeId, ReeInt, Src,
};

pub(crate) struct Implant {
    pub(crate) item_id: ReeId,
    pub(crate) fit_id: ReeId,
    pub(crate) type_id: ReeInt,
    pub(crate) citem: Option<Arc<ct::Item>>,
    pub(crate) state: State,
}
impl Implant {
    pub(crate) fn new(src: &Arc<Src>, item_id: ReeId, fit_id: ReeId, type_id: ReeInt) -> Implant {
        Implant {
            item_id,
            fit_id,
            type_id,
            citem: src.cache_handler.get_item(&type_id),
            state: State::Offline,
        }
    }
    pub(crate) fn get_slot(&self) -> Option<ReeInt> {
        match &self.citem {
            None => None,
            Some(i) => match i.attr_vals.get(&attrs::IMPLANTNESS) {
                None => None,
                Some(v) => Some(v.round() as ReeInt),
            },
        }
    }
    pub(crate) fn get_bool_state(&self) -> bool {
        match self.state {
            State::Ghost => false,
            _ => true,
        }
    }
    pub(crate) fn set_bool_state(&mut self, state: bool) {
        self.state = match state {
            true => State::Offline,
            false => State::Ghost,
        }
    }
}
impl Named for Implant {
    fn get_name() -> &'static str {
        "ssi:Implant"
    }
}
impl fmt::Display for Implant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}(id={}, type_id={})",
            Implant::get_name(),
            self.item_id,
            self.type_id
        )
    }
}
