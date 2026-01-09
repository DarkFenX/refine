use full::HSkillInfoFull;
use id::HSkillInfoId;
use partial::HSkillInfoPartial;
use serde::Serialize;

use crate::info::HItemInfoMode;

mod full;
mod id;
mod partial;

#[derive(Serialize)]
#[serde(untagged)]
pub(crate) enum HSkillInfo {
    Id(HSkillInfoId),
    Partial(HSkillInfoPartial),
    Full(HSkillInfoFull),
}
impl HSkillInfo {
    pub(in crate::info::item) fn mk_info(core_skill: &mut rc::SkillMut, item_mode: HItemInfoMode) -> Self {
        match item_mode {
            HItemInfoMode::Id => Self::Id(core_skill.into()),
            HItemInfoMode::Partial => Self::Partial(core_skill.into()),
            HItemInfoMode::Full => Self::Full(core_skill.into()),
        }
    }
}
