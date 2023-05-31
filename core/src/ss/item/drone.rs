use std::{fmt, sync::Arc};

use crate::{
    consts::State,
    defs::{ReeId, ReeInt},
    ert,
    src::Src,
    util::Named,
};

pub(in crate::ss) struct Drone {
    pub(in crate::ss) id: ReeId,
    pub(in crate::ss) fit_id: ReeId,
    pub(in crate::ss) type_id: ReeInt,
    pub(in crate::ss) state: State,
    pub(in crate::ss) cached_item: Option<Arc<ert::Item>>,
}
impl Drone {
    pub(in crate::ss) fn new(src: &Arc<Src>, id: ReeId, fit_id: ReeId, type_id: ReeInt, state: State) -> Self {
        Self {
            id,
            fit_id,
            type_id,
            state,
            cached_item: src.cache_handler.get_item(&type_id),
        }
    }
}
impl Named for Drone {
    fn get_name() -> &'static str {
        "ssi:Drone"
    }
}
impl fmt::Display for Drone {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}(id={}, type_id={})", Self::get_name(), self.id, self.type_id)
    }
}
