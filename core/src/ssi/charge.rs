use std::{fmt, sync::Arc};

use crate::{ct, util::Named, ReeId, ReeInt, Src};

pub(crate) struct Charge {
    pub(crate) item_id: ReeId,
    pub(crate) fit_id: ReeId,
    pub(crate) type_id: ReeInt,
    pub(crate) citem: Option<Arc<ct::Item>>,
}
impl Charge {
    pub(crate) fn new(src: &Arc<Src>, item_id: ReeId, fit_id: ReeId, type_id: ReeInt) -> Charge {
        Charge {
            item_id,
            fit_id,
            type_id,
            citem: src.cache_handler.get_item(type_id),
        }
    }
}
impl Named for Charge {
    fn get_name() -> &'static str {
        "ssi:Charge"
    }
}
impl fmt::Display for Charge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}(id={}, type_id={})",
            Charge::get_name(),
            self.item_id,
            self.type_id
        )
    }
}
