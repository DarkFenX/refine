use crate::{num::SkillLevel, ud::UItemId};

#[derive(Copy, Clone)]
pub(crate) struct UFitSkill {
    pub(crate) skill_uid: UItemId,
    pub(crate) level: SkillLevel,
}
