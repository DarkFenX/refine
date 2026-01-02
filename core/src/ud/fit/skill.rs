use crate::{misc::SkillLevel, ud::UItemId};

#[derive(Copy, Clone)]
pub(crate) struct UFitSkill {
    pub(crate) skill_key: UItemId,
    pub(crate) level: SkillLevel,
}
