use std::{collections::HashMap, fmt, sync::Arc};

use crate::{ct, util::Named, ReeFloat, ReeId, ReeInt, Src};

pub(crate) struct Character {
    pub(crate) item_id: ReeId,
    pub(crate) fit_id: ReeId,
    pub(crate) type_id: ReeInt,
    pub(crate) citem: Option<Arc<ct::Item>>,
    pub(crate) mod_attrs: HashMap<ReeInt, ReeFloat>,
}
impl Character {
    pub(crate) fn new(src: &Arc<Src>, item_id: ReeId, fit_id: ReeId, type_id: ReeInt) -> Character {
        Character {
            item_id,
            fit_id,
            type_id,
            citem: src.cache_handler.get_item(type_id),
            mod_attrs: HashMap::new(),
        }
    }
}
impl Named for Character {
    fn get_name() -> &'static str {
        "ssi:Character"
    }
}
impl fmt::Display for Character {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}(id={}, type_id={})",
            Character::get_name(),
            self.item_id,
            self.type_id
        )
    }
}
