use crate::{defines::ReeInt, util::Named};

/// Item type skill requirement.
#[derive(Debug)]
pub struct ItemSkillReq {
    /// Refers an item type for which this skill requirement is defined.
    pub item_id: ReeInt,
    /// Refers a skill item type which is needed to meet the skill requirement.
    pub skill_id: ReeInt,
    /// Defines skill level which is needed to meet the skill requirement.
    pub level: ReeInt,
}
impl ItemSkillReq {
    /// Make a new item type skill requirement out of passed data.
    pub fn new(item_id: ReeInt, skill_id: ReeInt, level: ReeInt) -> ItemSkillReq {
        ItemSkillReq {
            item_id,
            skill_id,
            level,
        }
    }
}
impl Named for ItemSkillReq {
    fn get_name() -> &'static str {
        "dh::ItemSkillReq"
    }
}
