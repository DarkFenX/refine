use serde::Serialize;

use super::{full::HServiceInfoFull, id::HServiceInfoId, partial::HServiceInfoPartial};
use crate::info::HItemInfoMode;

#[derive(Serialize)]
#[serde(untagged)]
pub(crate) enum HServiceInfo {
    Id(HServiceInfoId),
    Partial(HServiceInfoPartial),
    Full(HServiceInfoFull),
}
impl HServiceInfo {
    pub(in crate::info::item) fn mk_info(core_service: &mut rc::ServiceMut, item_mode: HItemInfoMode) -> Self {
        match item_mode {
            HItemInfoMode::Id => Self::Id(core_service.into()),
            HItemInfoMode::Partial => Self::Partial(core_service.into()),
            HItemInfoMode::Full => Self::Full(core_service.into()),
        }
    }
}
