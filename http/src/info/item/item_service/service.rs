use serde::Serialize;

use super::{service_full::HServiceInfoFull, service_id::HServiceInfoId, service_partial::HServiceInfoPartial};
use crate::info::HItemInfoMode;

#[derive(Serialize)]
#[serde(untagged)]
pub(crate) enum HServiceInfo {
    Id(HServiceInfoId),
    Partial(HServiceInfoPartial),
    Full(HServiceInfoFull),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HServiceInfo {
    pub(in crate::info::item) fn from_core(core_service: &mut rc::ServiceMut, item_mode: HItemInfoMode) -> Self {
        match item_mode {
            HItemInfoMode::Id => Self::Id(HServiceInfoId::from_core(core_service)),
            HItemInfoMode::Partial => Self::Partial(HServiceInfoPartial::from_core(core_service)),
            HItemInfoMode::Full => Self::Full(HServiceInfoFull::from_core(core_service)),
        }
    }
}
