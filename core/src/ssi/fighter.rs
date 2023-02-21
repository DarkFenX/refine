use std::{fmt, sync::Arc};

use crate::{consts::State, ct, util::Named, ReeId, ReeInt, Src};

pub(crate) struct Fighter {
    pub(crate) item_id: ReeId,
    pub(crate) fit_id: ReeId,
    pub(crate) type_id: ReeInt,
    pub(crate) citem: Option<Arc<ct::Item>>,
    pub(crate) state: State,
    pub(crate) amt_override: Option<ReeInt>,
}
impl Fighter {
    pub(crate) fn new(src: &Arc<Src>, item_id: ReeId, fit_id: ReeId, type_id: ReeInt, state: State) -> Fighter {
        Fighter {
            item_id,
            fit_id,
            type_id,
            citem: src.cache_handler.get_item(type_id),
            state,
            amt_override: None,
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
        write!(
            f,
            "{}(id={}, type_id={})",
            Fighter::get_name(),
            self.item_id,
            self.type_id
        )
    }
}
