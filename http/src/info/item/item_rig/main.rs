use serde::Serialize;

use super::{full::HRigInfoFull, id::HRigInfoId, partial::HRigInfoPartial};
use crate::info::HItemInfoMode;

#[derive(Serialize)]
#[serde(untagged)]
pub(crate) enum HRigInfo {
    Id(HRigInfoId),
    Partial(HRigInfoPartial),
    Full(HRigInfoFull),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HRigInfo {
    pub(in crate::info::item) fn from_core(core_rig: &mut rc::RigMut, item_mode: HItemInfoMode) -> Self {
        match item_mode {
            HItemInfoMode::Id => Self::Id(HRigInfoId::from_core(core_rig)),
            HItemInfoMode::Partial => Self::Partial(HRigInfoPartial::from_core(core_rig)),
            HItemInfoMode::Full => Self::Full(HRigInfoFull::from_core(core_rig)),
        }
    }
}
