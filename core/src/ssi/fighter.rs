use std::{fmt, sync::Arc};

use crate::{
    adt,
    consts::State,
    defs::{ReeId, ReeInt},
    src::Src,
    util::Named,
};

pub(crate) struct Fighter {
    pub(crate) id: ReeId,
    pub(crate) fit_id: ReeId,
    pub(crate) type_id: ReeInt,
    pub(crate) state: State,
    pub(crate) amt_override: Option<ReeInt>,
    pub(crate) cached_item: Option<Arc<adt::AItem>>,
}
impl Fighter {
    pub(crate) fn new(src: &Arc<Src>, id: ReeId, fit_id: ReeId, type_id: ReeInt, state: State) -> Self {
        Self {
            id,
            fit_id,
            type_id,
            state,
            amt_override: None,
            cached_item: src.cache_handler.get_item(&type_id),
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
        write!(f, "{}(id={}, type_id={})", Self::get_name(), self.id, self.type_id)
    }
}
