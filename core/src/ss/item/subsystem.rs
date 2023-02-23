use std::{collections::HashMap, fmt, sync::Arc};

use crate::{consts::attrs, ct, util::Named, ReeFloat, ReeId, ReeInt, Src};

pub(crate) struct Subsystem {
    pub(crate) item_id: ReeId,
    pub(crate) fit_id: ReeId,
    pub(crate) type_id: ReeInt,
    pub(crate) citem: Option<Arc<ct::Item>>,
    pub(crate) mod_attrs: HashMap<ReeInt, ReeFloat>,
}
impl Subsystem {
    pub(crate) fn new(src: &Arc<Src>, item_id: ReeId, fit_id: ReeId, type_id: ReeInt) -> Subsystem {
        Subsystem {
            item_id,
            fit_id,
            type_id,
            citem: src.cache_handler.get_item(type_id),
            mod_attrs: HashMap::new(),
        }
    }
    pub(crate) fn get_slot(&self) -> Option<ReeInt> {
        match &self.citem {
            None => None,
            Some(i) => match i.attr_vals.get(&attrs::SUBSYSTEM_SLOT) {
                None => None,
                Some(v) => Some(v.round() as ReeInt),
            },
        }
    }
}
impl Named for Subsystem {
    fn get_name() -> &'static str {
        "ssi:Subsystem"
    }
}
impl fmt::Display for Subsystem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}(id={}, type_id={})",
            Subsystem::get_name(),
            self.item_id,
            self.type_id
        )
    }
}
