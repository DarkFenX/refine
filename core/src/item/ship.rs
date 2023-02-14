use std::sync::Arc;

use crate::{ct, ReeId, ReeInt, Src};

pub(crate) struct Ship {
    pub(crate) item_id: ReeId,
    pub(crate) fit_id: ReeId,
    pub(crate) type_id: ReeInt,
    pub(crate) citem: Option<Arc<ct::Item>>,
}
impl Ship {
    pub(crate) fn new(src: Arc<Src>, item_id: ReeId, fit_id: ReeId, type_id: ReeInt) -> Ship {
        Ship {
            item_id,
            fit_id,
            type_id,
            citem: src.cache_handler.get_item(type_id),
        }
    }
}
