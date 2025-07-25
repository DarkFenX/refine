use crate::{misc::SkillLevel, ud::UItemKey};

#[derive(Copy, Clone)]
pub(crate) struct UFitSkill {
    pub(crate) item_key: UItemKey,
    pub(crate) level: SkillLevel,
}
