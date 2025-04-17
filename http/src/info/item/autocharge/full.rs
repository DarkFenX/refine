use crate::info::item::extended::HItemExtendedInfo;

use super::HAutochargeInfoPartial;

#[derive(serde::Serialize)]
pub(crate) struct HAutochargeInfoFull {
    #[serde(flatten)]
    pub(crate) partial_info: HAutochargeInfoPartial,
    #[serde(flatten)]
    pub(crate) extended_info: HItemExtendedInfo,
}
impl From<&mut rc::AutochargeMut<'_>> for HAutochargeInfoFull {
    fn from(core_autocharge: &mut rc::AutochargeMut) -> Self {
        Self {
            partial_info: core_autocharge.into(),
            extended_info: core_autocharge.into(),
        }
    }
}
