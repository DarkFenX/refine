use std::{fmt, sync::Arc};

use crate::{
    consts::State,
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
    pub enabled: bool,
}
impl SkillInfo {
    fn new(item_id: ReeId, fit_id: ReeId, type_id: ReeInt, level: ReeInt, enabled: bool) -> Self {
        Self {
            item_id,
            fit_id,
            type_id,
            level,
            enabled,
        }
    }
}
impl From<&Skill> for SkillInfo {
    fn from(s: &Skill) -> Self {
        SkillInfo::new(s.item_id, s.fit_id, s.type_id, s.level, s.get_bool_state())
    }
}

pub(in crate::ss) struct Skill {
    pub(in crate::ss) item_id: ReeId,
    pub(in crate::ss) fit_id: ReeId,
    pub(in crate::ss) type_id: ReeInt,
    pub(in crate::ss) level: ReeInt,
    pub(in crate::ss) state: State,
    pub(in crate::ss) citem: Option<Arc<ct::Item>>,
}
impl Skill {
    pub(in crate::ss) fn new(src: &Arc<Src>, item_id: ReeId, fit_id: ReeId, type_id: ReeInt, level: ReeInt) -> Self {
        Self {
            item_id,
            fit_id,
            type_id,
            level,
            state: State::Offline,
            citem: src.cache_handler.get_item(&type_id),
        }
    }
    pub(in crate::ss) fn get_bool_state(&self) -> bool {
        match self.state {
            State::Ghost => false,
            _ => true,
        }
    }
    pub(in crate::ss) fn set_bool_state(&mut self, state: bool) {
        self.state = match state {
            true => State::Offline,
            false => State::Ghost,
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
