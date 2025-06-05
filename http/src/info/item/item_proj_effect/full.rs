use super::HProjEffectInfoPartial;
use crate::info::item::extended::HItemExtendedInfo;

#[derive(serde::Serialize)]
pub(crate) struct HProjEffectInfoFull {
    #[serde(flatten)]
    partial_info: HProjEffectInfoPartial,
    #[serde(flatten)]
    extended_info: HItemExtendedInfo,
}
impl From<&mut rc::ProjEffectMut<'_>> for HProjEffectInfoFull {
    fn from(core_proj_effect: &mut rc::ProjEffectMut) -> Self {
        Self {
            partial_info: core_proj_effect.into(),
            extended_info: core_proj_effect.into(),
        }
    }
}
