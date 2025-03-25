use crate::sol::{ItemId, SkillLevel};

#[derive(Copy, Clone)]
pub(in crate::sol) struct FitSkill {
    pub(in crate::sol) item_id: ItemId,
    pub(in crate::sol) level: SkillLevel,
}
impl FitSkill {
    pub(in crate::sol) fn new(item_id: ItemId, level: SkillLevel) -> Self {
        Self { item_id, level }
    }
}
