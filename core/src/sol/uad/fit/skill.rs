use crate::sol::{ItemKey, SkillLevel};

#[derive(Copy, Clone)]
pub(in crate::sol) struct FitSkill {
    pub(in crate::sol) item_key: ItemKey,
    pub(in crate::sol) level: SkillLevel,
}
