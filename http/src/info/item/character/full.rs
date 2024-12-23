use crate::info::HItemExtendedInfo;

use super::HCharacterInfoPartial;

#[derive(serde::Serialize)]
pub(crate) struct HCharacterInfoFull {
    #[serde(flatten)]
    pub(crate) partial_info: HCharacterInfoPartial,
    #[serde(flatten)]
    pub(crate) extended_info: HItemExtendedInfo,
}
impl HCharacterInfoFull {
    pub(super) fn mk_info(core_sol: &mut rc::SolarSystem, core_character_info: &rc::SolCharacterInfo) -> Self {
        let partial_info = HCharacterInfoPartial::from(core_character_info);
        let extended_info = HItemExtendedInfo::mk_info(core_sol, &partial_info.id);
        Self {
            partial_info,
            extended_info,
        }
    }
}
