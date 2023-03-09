use std::{fmt, sync::Arc};

use crate::{ct, util::Named, ReeId, ReeInt, Src};

pub(in crate::ss) struct Ship {
    pub(in crate::ss) item_id: ReeId,
    pub(in crate::ss) fit_id: ReeId,
    pub(in crate::ss) type_id: ReeInt,
    pub(in crate::ss) citem: Option<Arc<ct::Item>>,
}
impl Ship {
    pub(in crate::ss) fn new(src: &Arc<Src>, item_id: ReeId, fit_id: ReeId, type_id: ReeInt) -> Ship {
        Ship {
            item_id,
            fit_id,
            type_id,
            citem: src.cache_handler.get_item(&type_id),
        }
    }
}
impl Named for Ship {
    fn get_name() -> &'static str {
        "ssi:Ship"
    }
}
impl fmt::Display for Ship {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}(id={}, type_id={})", Ship::get_name(), self.item_id, self.type_id)
    }
}
