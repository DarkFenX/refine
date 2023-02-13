use std::sync::Arc;

use crate::{ct, ReeId, ReeInt};

pub(crate) struct Skill {
    pub(crate) id: ReeId,
    pub(crate) fit_id: ReeId,
    pub(crate) type_id: ReeInt,
    pub(crate) item: Option<Arc<ct::Item>>,
    pub(crate) level: ReeInt,
}
impl Skill {
    pub fn new(id: ReeId, fit_id: ReeId, type_id: ReeInt) -> Skill {
        Skill {
            id,
            fit_id,
            type_id,
            item: None,
            level: 0,
        }
    }
    pub fn new_with_level(id: ReeId, fit_id: ReeId, type_id: ReeInt, level: ReeInt) -> Skill {
        Skill {
            id,
            fit_id,
            type_id,
            item: None,
            level,
        }
    }
}
