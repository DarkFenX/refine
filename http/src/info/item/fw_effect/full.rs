use crate::info::HItemExtendedInfo;

use super::HFwEffectInfoPartial;

#[derive(serde::Serialize)]
pub(crate) struct HFwEffectInfoFull {
    #[serde(flatten)]
    pub(crate) partial_info: HFwEffectInfoPartial,
    #[serde(flatten)]
    pub(crate) extended_info: HItemExtendedInfo,
}
impl HFwEffectInfoFull {
    pub(super) fn mk_info(core_sol: &mut rc::SolarSystem, core_fw_effect_info: &rc::SolFwEffectInfo) -> Self {
        let partial_info = HFwEffectInfoPartial::from(core_fw_effect_info);
        let extended_info = HItemExtendedInfo::mk_info(core_sol, &partial_info.id);
        Self {
            partial_info,
            extended_info,
        }
    }
}
