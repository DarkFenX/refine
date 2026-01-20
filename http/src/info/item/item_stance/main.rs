use serde::Serialize;

use super::{full::HStanceInfoFull, id::HStanceInfoId, partial::HStanceInfoPartial};
use crate::info::HItemInfoMode;

#[derive(Serialize)]
#[serde(untagged)]
pub(crate) enum HStanceInfo {
    Id(HStanceInfoId),
    Partial(HStanceInfoPartial),
    Full(HStanceInfoFull),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HStanceInfo {
    pub(in crate::info::item) fn from_core(core_stance: &mut rc::StanceMut, item_mode: HItemInfoMode) -> Self {
        match item_mode {
            HItemInfoMode::Id => Self::Id(HStanceInfoId::from_core(core_stance)),
            HItemInfoMode::Partial => Self::Partial(HStanceInfoPartial::from_core(core_stance)),
            HItemInfoMode::Full => Self::Full(HStanceInfoFull::from_core(core_stance)),
        }
    }
}
