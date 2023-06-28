use crate::{
    defs::{ItemId, SkillLevel},
    util::Named,
};

/// EVE item type skill requirement.
#[derive(Debug)]
pub struct EItemSkillReq {
    /// Refers an item type for which this skill requirement is defined.
    pub item_id: ItemId,
    /// Refers a skill item type which is needed to meet the skill requirement.
    pub skill_id: ItemId,
    /// Defines skill level which is needed to meet the skill requirement.
    pub level: SkillLevel,
}
impl EItemSkillReq {
    /// Make a new EVE item type skill requirement out of passed data.
    pub fn new(item_id: ItemId, skill_id: ItemId, level: SkillLevel) -> Self {
        Self {
            item_id,
            skill_id,
            level,
        }
    }
}
impl Named for EItemSkillReq {
    fn get_name() -> &'static str {
        "EItemSkillReq"
    }
}
