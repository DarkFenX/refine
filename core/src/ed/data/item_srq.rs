use crate::{
    ed::{EItemId, ESkillLevel},
    util::Named,
};

/// EVE item type skill requirement.
pub struct EItemSkillReq {
    /// Refers an item type for which this skill requirement is defined.
    pub item_id: EItemId,
    /// Refers a skill item type which is needed to meet the skill requirement.
    pub skill_id: EItemId,
    /// Defines skill level which is needed to meet the skill requirement.
    pub level: ESkillLevel,
}
impl Named for EItemSkillReq {
    fn get_name() -> &'static str {
        "EItemSkillReq"
    }
}
