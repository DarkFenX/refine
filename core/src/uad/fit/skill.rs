use crate::{def::ItemKey, misc::SkillLevel};

#[derive(Copy, Clone)]
pub(crate) struct UadFitSkill {
    pub(crate) item_key: ItemKey,
    pub(crate) level: SkillLevel,
}
