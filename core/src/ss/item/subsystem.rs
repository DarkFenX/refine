use std::{fmt, sync::Arc};

use crate::{consts::attrs, ct, util::Named, ReeId, ReeInt, Src};

pub(in crate::ss) struct Subsystem {
    pub(in crate::ss) item_id: ReeId,
    pub(in crate::ss) fit_id: ReeId,
    pub(in crate::ss) type_id: ReeInt,
    pub(in crate::ss) citem: Option<Arc<ct::Item>>,
}
impl Subsystem {
    pub(in crate::ss) fn new(src: &Arc<Src>, item_id: ReeId, fit_id: ReeId, type_id: ReeInt) -> Self {
        Self {
            item_id,
            fit_id,
            type_id,
            citem: src.cache_handler.get_item(&type_id),
        }
    }
    pub(in crate::ss) fn get_slot(&self) -> Option<ReeInt> {
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
        write!(f, "{}(id={}, type_id={})", Self::get_name(), self.item_id, self.type_id)
    }
}
