use std::{fmt, sync::Arc};

use crate::{
    ad,
    defs::{ReeId, ReeInt},
    src::Src,
    util::Named,
};

pub(crate) struct Charge {
    pub(crate) id: ReeId,
    pub(crate) fit_id: ReeId,
    pub(crate) type_id: ReeInt,
    pub(crate) cont_id: ReeId,
    pub(crate) aitem: Option<Arc<ad::AItem>>,
}
impl Charge {
    pub(crate) fn new(src: &Arc<Src>, id: ReeId, fit_id: ReeId, type_id: ReeInt, cont_id: ReeId) -> Self {
        Self {
            id,
            fit_id,
            type_id,
            cont_id,
            aitem: src.ahandler.get_item(&type_id),
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
