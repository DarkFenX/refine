use std::{fmt, sync::Arc};

use crate::{
    ad,
    defs::{ReeId, ReeInt},
    src::Src,
    util::Named,
};

pub(crate) struct SsCharge {
    pub(crate) id: ReeId,
    pub(crate) fit_id: ReeId,
    pub(crate) a_item_id: ReeInt,
    pub(crate) cont_id: ReeId,
    pub(crate) a_item: Option<Arc<ad::AItem>>,
}
impl SsCharge {
    pub(crate) fn new(src: &Arc<Src>, id: ReeId, fit_id: ReeId, a_item_id: ReeInt, cont_id: ReeId) -> Self {
        Self {
            id,
            fit_id,
            a_item_id,
            cont_id,
            a_item: src.ahandler.get_item(&a_item_id),
        }
    }
}
impl Named for SsCharge {
    fn get_name() -> &'static str {
        "SsCharge"
    }
}
impl fmt::Display for SsCharge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}(id={}, a_item_id={})", Self::get_name(), self.id, self.a_item_id)
    }
}
