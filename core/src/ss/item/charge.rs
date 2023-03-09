use std::{fmt, sync::Arc};

use crate::{ct, util::Named, ReeId, ReeInt, Src};

pub(in crate::ss) struct Charge {
    pub(in crate::ss) item_id: ReeId,
    pub(in crate::ss) fit_id: ReeId,
    pub(in crate::ss) type_id: ReeInt,
    pub(in crate::ss) citem: Option<Arc<ct::Item>>,
    pub(in crate::ss) cont: ReeId,
}
impl Charge {
    pub(in crate::ss) fn new(src: &Arc<Src>, item_id: ReeId, fit_id: ReeId, type_id: ReeInt, cont: ReeId) -> Charge {
        Charge {
            item_id,
            fit_id,
            type_id,
            citem: src.cache_handler.get_item(&type_id),
            cont,
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
