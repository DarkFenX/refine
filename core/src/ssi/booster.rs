use std::{fmt, sync::Arc};

use crate::{
    consts::{attrs, State},
    ct,
    util::Named,
    ReeId, ReeInt, Src,
};

pub(crate) struct Booster {
    pub(crate) item_id: ReeId,
    pub(crate) fit_id: ReeId,
    pub(crate) type_id: ReeInt,
    pub(crate) citem: Option<Arc<ct::Item>>,
    pub(crate) state: State,
}
impl Booster {
    pub(crate) fn new(src: &Arc<Src>, item_id: ReeId, fit_id: ReeId, type_id: ReeInt) -> Booster {
        Booster {
            item_id,
            fit_id,
            type_id,
            citem: src.cache_handler.get_item(type_id),
            state: State::Online,
        }
    }
    pub(crate) fn get_slot(&self) -> Option<ReeInt> {
        match &self.citem {
            None => None,
            Some(i) => match i.attr_vals.get(&attrs::BOOSTERNESS) {
                None => None,
                Some(v) => Some(v.round() as ReeInt),
            },
        }
    }
}
impl Named for Booster {
    fn get_name() -> &'static str {
        "ssi:Booster"
    }
}
impl fmt::Display for Booster {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}(id={}, type_id={})",
            Booster::get_name(),
            self.item_id,
            self.type_id
        )
    }
}
