use std::{fmt, sync::Arc};

use crate::{consts::State, ct, util::Named, ReeId, ReeInt, Src};

pub(in crate::ss) struct Fighter {
    pub(in crate::ss) item_id: ReeId,
    pub(in crate::ss) fit_id: ReeId,
    pub(in crate::ss) type_id: ReeInt,
    pub(in crate::ss) citem: Option<Arc<ct::Item>>,
    pub(in crate::ss) state: State,
    pub(in crate::ss) amt_override: Option<ReeInt>,
}
impl Fighter {
    pub(in crate::ss) fn new(src: &Arc<Src>, item_id: ReeId, fit_id: ReeId, type_id: ReeInt, state: State) -> Self {
        Self {
            item_id,
            fit_id,
            type_id,
            citem: src.cache_handler.get_item(&type_id),
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
            Self::get_name(),
            self.item_id,
            self.type_id
        )
    }
}
