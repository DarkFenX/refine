use crate::{
    ed::{EInt, EItemId},
    util::LibNamed,
};

pub struct EItemSkillReq {
    pub item_id: EItemId,
    pub skill_id: EItemId,
    pub level: EInt,
}
impl LibNamed for EItemSkillReq {
    fn lib_get_name() -> &'static str {
        "EItemSkillReq"
    }
}
