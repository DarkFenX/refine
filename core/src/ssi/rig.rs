use std::{fmt, sync::Arc};

use crate::{ct, consts::State, util::Named, ReeId, ReeInt, Src};

pub(crate) struct Rig {
    pub(crate) item_id: ReeId,
    pub(crate) fit_id: ReeId,
    pub(crate) type_id: ReeInt,
    pub(crate) citem: Option<Arc<ct::Item>>,
    pub(crate) state: State,
}
impl Rig {
    pub(crate) fn new(src: Arc<Src>, item_id: ReeId, fit_id: ReeId, type_id: ReeInt) -> Rig {
        Rig {
            item_id,
            fit_id,
            type_id,
            citem: src.cache_handler.get_item(type_id),
            state: State::Offline,
        }
    }
}
impl Named for Rig {
    fn get_name() -> &'static str {
        "ssi:Rig"
    }
}
impl fmt::Display for Rig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}(id={}, type_id={})",
            Rig::get_name(),
            self.item_id,
            self.type_id
        )
    }
}
