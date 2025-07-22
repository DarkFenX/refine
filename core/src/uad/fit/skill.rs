use crate::{misc::SkillLevel, uad::UadItemKey};

#[derive(Copy, Clone)]
pub(crate) struct UadFitSkill {
    pub(crate) item_key: UadItemKey,
    pub(crate) level: SkillLevel,
}
