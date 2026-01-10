use serde::Serialize;

use super::partial::HImplantInfoPartial;
use crate::info::item::extended::HItemExtendedInfo;

#[derive(Serialize)]
pub(crate) struct HImplantInfoFull {
    #[serde(flatten)]
    partial_info: HImplantInfoPartial,
    #[serde(flatten)]
    extended_info: HItemExtendedInfo,
}
impl From<&mut rc::ImplantMut<'_>> for HImplantInfoFull {
    fn from(core_implant: &mut rc::ImplantMut) -> Self {
        Self {
            partial_info: core_implant.into(),
            extended_info: core_implant.into(),
        }
    }
}
