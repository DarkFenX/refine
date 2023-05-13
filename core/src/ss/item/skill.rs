use std::{fmt, sync::Arc};

use crate::{
    ct,
    defs::{ReeId, ReeInt},
    src::Src,
    util::Named,
};

pub struct SkillInfo {
    pub item_id: ReeId,
    pub fit_id: ReeId,
    pub type_id: ReeInt,
    pub level: ReeInt,
}
impl SkillInfo {
    fn new(item_id: ReeId, fit_id: ReeId, type_id: ReeInt, level: ReeInt) -> Self {
        Self {
            item_id,
            fit_id,
            type_id,
            level,
        }
    }
}
impl From<&Skill> for SkillInfo {
    fn from(s: &Skill) -> Self {
        SkillInfo::new(s.item_id, s.fit_id, s.type_id, s.level)
    }
}

pub(in crate::ss) struct Skill {
    pub(in crate::ss) item_id: ReeId,
    pub(in crate::ss) fit_id: ReeId,
    pub(in crate::ss) type_id: ReeInt,
    pub(in crate::ss) level: ReeInt,
    pub(in crate::ss) citem: Option<Arc<ct::Item>>,
}
impl Skill {
    pub(in crate::ss) fn new(src: &Arc<Src>, item_id: ReeId, fit_id: ReeId, type_id: ReeInt, level: ReeInt) -> Self {
        Self {
            item_id,
            fit_id,
            type_id,
            level,
            citem: src.cache_handler.get_item(&type_id),
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
        write!(f, "{}(id={}, type_id={})", Self::get_name(), self.item_id, self.type_id)
    }
}
