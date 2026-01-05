use crate::{
    ed::{EGenInt, EItemId},
    util::LibNamed,
};

pub struct EItemSkillReq {
    pub item_id: EItemId,
    pub skill_id: EItemId,
    pub level: EGenInt,
}
impl LibNamed for EItemSkillReq {
    fn lib_get_name() -> &'static str {
        "EItemSkillReq"
    }
}
