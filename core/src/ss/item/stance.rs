use std::{fmt, sync::Arc};

use crate::{
    ct,
    defs::{ReeId, ReeInt},
    src::Src,
    util::Named,
};

pub struct StanceInfo {
    pub item_id: ReeId,
    pub fit_id: ReeId,
    pub type_id: ReeInt,
}
impl StanceInfo {
    fn new(item_id: ReeId, fit_id: ReeId, type_id: ReeInt) -> Self {
        Self {
            item_id,
            fit_id,
            type_id,
        }
    }
}
impl From<&Stance> for StanceInfo {
    fn from(s: &Stance) -> Self {
        StanceInfo::new(s.item_id, s.fit_id, s.type_id)
    }
}

pub(in crate::ss) struct Stance {
    pub(in crate::ss) item_id: ReeId,
    pub(in crate::ss) fit_id: ReeId,
    pub(in crate::ss) type_id: ReeInt,
    pub(in crate::ss) citem: Option<Arc<ct::Item>>,
}
impl Stance {
    pub(in crate::ss) fn new(src: &Arc<Src>, item_id: ReeId, fit_id: ReeId, type_id: ReeInt) -> Self {
        Self {
            item_id,
            fit_id,
            type_id,
            citem: src.cache_handler.get_item(&type_id),
        }
    }
}
impl Named for Stance {
    fn get_name() -> &'static str {
        "ssi:Stance"
    }
}
impl fmt::Display for Stance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}(id={}, type_id={})", Self::get_name(), self.item_id, self.type_id)
    }
}
