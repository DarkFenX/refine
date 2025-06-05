use super::HAutochargeInfoPartial;
use crate::info::item::extended::HItemExtendedInfo;

#[derive(serde::Serialize)]
pub(crate) struct HAutochargeInfoFull {
    #[serde(flatten)]
    partial_info: HAutochargeInfoPartial,
    #[serde(flatten)]
    extended_info: HItemExtendedInfo,
}
impl From<&mut rc::AutochargeMut<'_>> for HAutochargeInfoFull {
    fn from(core_autocharge: &mut rc::AutochargeMut) -> Self {
        Self {
            partial_info: core_autocharge.into(),
            extended_info: core_autocharge.into(),
        }
    }
}
