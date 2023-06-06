use std::{fmt, sync::Arc};

use crate::{
    ad,
    consts::State,
    defs::{ReeId, ReeInt},
    src::Src,
    util::Named,
};

pub(crate) struct SsDrone {
    pub(crate) id: ReeId,
    pub(crate) fit_id: ReeId,
    pub(crate) a_item_id: ReeInt,
    pub(crate) state: State,
    pub(crate) a_item: Option<Arc<ad::AItem>>,
}
impl SsDrone {
    pub(crate) fn new(src: &Arc<Src>, id: ReeId, fit_id: ReeId, a_item_id: ReeInt, state: State) -> Self {
        Self {
            id,
            fit_id,
            a_item_id,
            state,
            a_item: src.a_handler.get_item(&a_item_id),
        }
    }
}
impl Named for SsDrone {
    fn get_name() -> &'static str {
        "SsDrone"
    }
}
impl fmt::Display for SsDrone {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}(id={}, a_item_id={})", Self::get_name(), self.id, self.a_item_id)
    }
}
