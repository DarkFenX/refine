use std::{fmt, sync::Arc};

use crate::{
    ad,
    consts::State,
    defs::{ReeId, ReeInt},
    src::Src,
    util::Named,
};

pub(crate) struct Drone {
    pub(crate) id: ReeId,
    pub(crate) fit_id: ReeId,
    pub(crate) type_id: ReeInt,
    pub(crate) state: State,
    pub(crate) aitem: Option<Arc<ad::AItem>>,
}
impl Drone {
    pub(crate) fn new(src: &Arc<Src>, id: ReeId, fit_id: ReeId, type_id: ReeInt, state: State) -> Self {
        Self {
            id,
            fit_id,
            type_id,
            state,
            aitem: src.ahandler.get_item(&type_id),
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
