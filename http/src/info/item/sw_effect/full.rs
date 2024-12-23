use crate::info::HItemExtendedInfo;

use super::HSwEffectInfoPartial;

#[derive(serde::Serialize)]
pub(crate) struct HSwEffectInfoFull {
    #[serde(flatten)]
    pub(crate) partial_info: HSwEffectInfoPartial,
    #[serde(flatten)]
    pub(crate) extended_info: HItemExtendedInfo,
}
impl HSwEffectInfoFull {
    pub(super) fn mk_info(core_sol: &mut rc::SolarSystem, core_sw_effect_info: &rc::SolSwEffectInfo) -> Self {
        let partial_info = HSwEffectInfoPartial::from(core_sw_effect_info);
        let extended_info = HItemExtendedInfo::mk_info(core_sol, &partial_info.id);
        Self {
            partial_info,
            extended_info,
        }
    }
}
