use std::{collections::HashMap, fmt, sync::Arc};

use crate::{consts::State, ct, util::Named, ReeFloat, ReeId, ReeInt, Src};

pub(crate) struct Drone {
    pub(crate) item_id: ReeId,
    pub(crate) fit_id: ReeId,
    pub(crate) type_id: ReeInt,
    pub(crate) citem: Option<Arc<ct::Item>>,
    pub(crate) mod_attrs: HashMap<ReeInt, ReeFloat>,
    pub(crate) state: State,
}
impl Drone {
    pub(crate) fn new(src: &Arc<Src>, item_id: ReeId, fit_id: ReeId, type_id: ReeInt, state: State) -> Drone {
        Drone {
            item_id,
            fit_id,
            type_id,
            citem: src.cache_handler.get_item(type_id),
            mod_attrs: HashMap::new(),
            state,
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
        write!(
            f,
            "{}(id={}, type_id={})",
            Drone::get_name(),
            self.item_id,
            self.type_id
        )
    }
}
