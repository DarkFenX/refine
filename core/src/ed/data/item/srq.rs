use crate::{
    ed::{EGenInt, EItemId},
    util::Named,
};

pub struct EItemSkillReq {
    pub item_id: EItemId,
    pub skill_id: EItemId,
    pub level: EGenInt,
}
impl Named for EItemSkillReq {
    fn get_name() -> &'static str {
        "EItemSkillReq"
    }
}
