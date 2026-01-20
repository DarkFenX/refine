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

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HSkillInfo {
    pub(in crate::info::item) fn from_core(core_skill: &mut rc::SkillMut, item_mode: HItemInfoMode) -> Self {
        match item_mode {
            HItemInfoMode::Id => Self::Id(HSkillInfoId::from_core(core_skill)),
            HItemInfoMode::Partial => Self::Partial(HSkillInfoPartial::from_core(core_skill)),
            HItemInfoMode::Full => Self::Full(HSkillInfoFull::from_core(core_skill)),
        }
    }
}
