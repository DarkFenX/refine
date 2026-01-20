use serde::Serialize;

use super::{full::HImplantInfoFull, id::HImplantInfoId, partial::HImplantInfoPartial};
use crate::info::HItemInfoMode;

#[derive(Serialize)]
#[serde(untagged)]
pub(crate) enum HImplantInfo {
    Id(HImplantInfoId),
    Partial(HImplantInfoPartial),
    Full(HImplantInfoFull),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HImplantInfo {
    pub(in crate::info::item) fn from_core(core_implant: &mut rc::ImplantMut, item_mode: HItemInfoMode) -> Self {
        match item_mode {
            HItemInfoMode::Id => Self::Id(HImplantInfoId::from_core(core_implant)),
            HItemInfoMode::Partial => Self::Partial(HImplantInfoPartial::from_core(core_implant)),
            HItemInfoMode::Full => Self::Full(HImplantInfoFull::from_core(core_implant)),
        }
    }
}
