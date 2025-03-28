use crate::sol::{ItemId, SkillLevel};

#[derive(Copy, Clone)]
pub(in crate::sol) struct FitSkill {
    pub(in crate::sol) item_id: ItemId,
    pub(in crate::sol) level: SkillLevel,
}
