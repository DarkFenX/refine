use std::{fmt, sync::Arc};

use crate::{
    consts::State,
    ct,
    defines::{ReeId, ReeInt},
    src::Src,
    util::Named,
};

pub struct FighterInfo {
    pub item_id: ReeId,
    pub fit_id: ReeId,
    pub type_id: ReeInt,
    pub state: State,
    pub amt_override: Option<ReeInt>,
}
impl FighterInfo {
    fn new(item_id: ReeId, fit_id: ReeId, type_id: ReeInt, state: State, amt_override: Option<ReeInt>) -> Self {
        Self {
            item_id,
            fit_id,
            type_id,
            state,
            amt_override,
        }
    }
}
impl From<&Fighter> for FighterInfo {
    fn from(f: &Fighter) -> Self {
        FighterInfo::new(f.item_id, f.fit_id, f.type_id, f.state, f.amt_override)
    }
}

pub(in crate::ss) struct Fighter {
    pub(in crate::ss) item_id: ReeId,
    pub(in crate::ss) fit_id: ReeId,
    pub(in crate::ss) type_id: ReeInt,
    pub(in crate::ss) state: State,
    pub(in crate::ss) amt_override: Option<ReeInt>,
    pub(in crate::ss) citem: Option<Arc<ct::Item>>,
}
impl Fighter {
    pub(in crate::ss) fn new(src: &Arc<Src>, item_id: ReeId, fit_id: ReeId, type_id: ReeInt, state: State) -> Self {
        Self {
            item_id,
            fit_id,
            type_id,
            state,
            amt_override: None,
            citem: src.cache_handler.get_item(&type_id),
        }
    }
}
impl Named for Fighter {
    fn get_name() -> &'static str {
        "ssi:Fighter"
    }
}
impl fmt::Display for Fighter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}(id={}, type_id={})", Self::get_name(), self.item_id, self.type_id)
    }
}
