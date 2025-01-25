use crate::defs::{SkillLevel, SolItemId};

#[derive(Copy, Clone)]
pub(in crate::sol) struct SolFitSkill {
    pub(in crate::sol) item_id: SolItemId,
    pub(in crate::sol) level: SkillLevel,
}
impl SolFitSkill {
    pub(in crate::sol) fn new(item_id: SolItemId, level: SkillLevel) -> Self {
        Self { item_id, level }
    }
}
