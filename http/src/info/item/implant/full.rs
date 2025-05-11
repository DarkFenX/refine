use super::HImplantInfoPartial;
use crate::info::item::extended::HItemExtendedInfo;

#[derive(serde::Serialize)]
pub(crate) struct HImplantInfoFull {
    #[serde(flatten)]
    pub(crate) partial_info: HImplantInfoPartial,
    #[serde(flatten)]
    pub(crate) extended_info: HItemExtendedInfo,
}
impl From<&mut rc::ImplantMut<'_>> for HImplantInfoFull {
    fn from(core_implant: &mut rc::ImplantMut) -> Self {
        Self {
            partial_info: core_implant.into(),
            extended_info: core_implant.into(),
        }
    }
}
