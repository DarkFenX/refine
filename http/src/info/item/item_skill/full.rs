use serde::Serialize;

use super::partial::HSkillInfoPartial;
use crate::info::item::extended::HItemExtendedInfo;

#[derive(Serialize)]
pub(crate) struct HSkillInfoFull {
    #[serde(flatten)]
    partial_info: HSkillInfoPartial,
    #[serde(flatten)]
    extended_info: HItemExtendedInfo,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HSkillInfoFull {
    pub(super) fn from_core(core_skill: &mut rc::SkillMut) -> Self {
        Self {
            partial_info: HSkillInfoPartial::from_core(core_skill),
            extended_info: HItemExtendedInfo::from_core(core_skill),
        }
    }
}
