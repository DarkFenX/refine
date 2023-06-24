use full::HSkillInfoFull;
use id::HSkillInfoId;
use partial::HSkillInfoPartial;

use crate::info::HItemInfoMode;

mod full;
mod id;
mod partial;

#[derive(serde::Serialize)]
#[serde(untagged)]
pub(crate) enum HSkillInfo {
    Id(HSkillInfoId),
    Partial(HSkillInfoPartial),
    Full(HSkillInfoFull),
}
impl HSkillInfo {
    pub(crate) fn mk_info(
        core_ss: &mut rc::SolarSystem,
        core_skill_info: &rc::SsSkillInfo,
        item_mode: HItemInfoMode,
    ) -> Self {
        match item_mode {
            HItemInfoMode::Id => Self::Id(core_skill_info.into()),
            HItemInfoMode::Partial => Self::Partial(core_skill_info.into()),
            HItemInfoMode::Full => Self::Full(HSkillInfoFull::mk_info(core_ss, core_skill_info)),
        }
    }
}
