use std::{fmt, sync::Arc};

use crate::{ct, util::Named, ReeId, ReeInt, Src};

pub(crate) struct Skill {
    pub(crate) item_id: ReeId,
    pub(crate) fit_id: ReeId,
    pub(crate) type_id: ReeInt,
    pub(crate) citem: Option<Arc<ct::Item>>,
    pub(crate) level: ReeInt,
}
impl Skill {
    pub(crate) fn new(src: &Arc<Src>, item_id: ReeId, fit_id: ReeId, type_id: ReeInt, level: ReeInt) -> Skill {
        Skill {
            item_id,
            fit_id,
            type_id,
            citem: src.cache_handler.get_item(&type_id),
            level,
        }
    }
}
impl Named for Skill {
    fn get_name() -> &'static str {
        "ssi:Skill"
    }
}
impl fmt::Display for Skill {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}(id={}, type_id={})",
            Skill::get_name(),
            self.item_id,
            self.type_id
        )
    }
}
