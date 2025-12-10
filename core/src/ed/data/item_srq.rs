use crate::{
    ed::{EItemId, ESkillLevel},
    util::Named,
};

pub struct EItemSkillReq {
    pub item_id: EItemId,
    pub skill_id: EItemId,
    pub level: ESkillLevel,
}
impl Named for EItemSkillReq {
    fn get_name() -> &'static str {
        "EItemSkillReq"
    }
}
