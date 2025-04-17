use crate::info::item::extended::HItemExtendedInfo;

use super::HFwEffectInfoPartial;

#[derive(serde::Serialize)]
pub(crate) struct HFwEffectInfoFull {
    #[serde(flatten)]
    pub(crate) partial_info: HFwEffectInfoPartial,
    #[serde(flatten)]
    pub(crate) extended_info: HItemExtendedInfo,
}
impl From<&mut rc::FwEffectMut<'_>> for HFwEffectInfoFull {
    fn from(core_fw_effect: &mut rc::FwEffectMut) -> Self {
        Self {
            partial_info: core_fw_effect.into(),
            extended_info: core_fw_effect.into(),
        }
    }
}
