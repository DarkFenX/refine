use crate::info::item::extended::HItemExtendedInfo;

use super::HProjEffectInfoPartial;

#[derive(serde::Serialize)]
pub(crate) struct HProjEffectInfoFull {
    #[serde(flatten)]
    pub(crate) partial_info: HProjEffectInfoPartial,
    #[serde(flatten)]
    pub(crate) extended_info: HItemExtendedInfo,
}
impl HProjEffectInfoFull {
    pub(super) fn mk_info(core_sol: &mut rc::SolarSystem, core_proj_effect_info: &rc::ProjEffectInfo) -> Self {
        let partial_info = HProjEffectInfoPartial::from(core_proj_effect_info);
        let extended_info = HItemExtendedInfo::mk_info(core_sol, &partial_info.id);
        Self {
            partial_info,
            extended_info,
        }
    }
}
