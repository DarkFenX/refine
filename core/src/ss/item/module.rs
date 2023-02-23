use std::{collections::HashMap, fmt, sync::Arc};

use crate::{consts::State, ct, util::Named, ReeFloat, ReeId, ReeIdx, ReeInt, Src};

pub(crate) struct Module {
    pub(crate) item_id: ReeId,
    pub(crate) fit_id: ReeId,
    pub(crate) type_id: ReeInt,
    pub(crate) citem: Option<Arc<ct::Item>>,
    pub(crate) mod_attrs: HashMap<ReeInt, ReeFloat>,
    pub(crate) state: State,
    pub(crate) pos: ReeIdx,
    pub(crate) charge: Option<ReeId>,
}
impl Module {
    pub(crate) fn new(
        src: &Arc<Src>,
        item_id: ReeId,
        fit_id: ReeId,
        type_id: ReeInt,
        state: State,
        pos: ReeIdx,
        charge: Option<ReeId>,
    ) -> Module {
        Module {
            item_id,
            fit_id,
            type_id,
            citem: src.cache_handler.get_item(type_id),
            mod_attrs: HashMap::new(),
            state,
            pos,
            charge,
        }
    }
}
impl Named for Module {
    fn get_name() -> &'static str {
        "ssi:Module"
    }
}
impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}(id={}, type_id={})",
            Module::get_name(),
            self.item_id,
            self.type_id
        )
    }
}
