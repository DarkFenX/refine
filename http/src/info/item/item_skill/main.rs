use serde::Serialize;

use super::{full::HSkillInfoFull, id::HSkillInfoId, partial::HSkillInfoPartial};
use crate::info::HItemInfoMode;

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
