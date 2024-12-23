use crate::info::HItemExtendedInfo;

use super::HSkillInfoPartial;

#[derive(serde::Serialize)]
pub(crate) struct HSkillInfoFull {
    #[serde(flatten)]
    pub(crate) partial_info: HSkillInfoPartial,
    #[serde(flatten)]
    pub(crate) extended_info: HItemExtendedInfo,
}
impl HSkillInfoFull {
    pub(super) fn mk_info(core_sol: &mut rc::SolarSystem, core_skill_info: &rc::SolSkillInfo) -> Self {
        let partial_info = HSkillInfoPartial::from(core_skill_info);
        let extended_info = HItemExtendedInfo::mk_info(core_sol, &partial_info.id);
        Self {
            partial_info,
            extended_info,
        }
    }
}
