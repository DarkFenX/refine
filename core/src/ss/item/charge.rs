use std::{fmt, sync::Arc};

use crate::{
    ct,
    defs::{ReeId, ReeInt},
    src::Src,
    util::Named,
};

pub(in crate::ss) struct Charge {
    pub(in crate::ss) id: ReeId,
    pub(in crate::ss) fit_id: ReeId,
    pub(in crate::ss) type_id: ReeInt,
    pub(in crate::ss) cont_id: ReeId,
    pub(in crate::ss) cached_item: Option<Arc<ct::Item>>,
}
impl Charge {
    pub(in crate::ss) fn new(src: &Arc<Src>, id: ReeId, fit_id: ReeId, type_id: ReeInt, cont_id: ReeId) -> Self {
        Self {
            id,
            fit_id,
            type_id,
            cont_id,
            cached_item: src.cache_handler.get_item(&type_id),
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
        write!(f, "{}(id={}, type_id={})", Self::get_name(), self.id, self.type_id)
    }
}
