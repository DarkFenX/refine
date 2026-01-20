use serde::Serialize;

use super::{full::HSubsystemInfoFull, id::HSubsystemInfoId, partial::HSubsystemInfoPartial};
use crate::info::HItemInfoMode;

#[derive(Serialize)]
#[serde(untagged)]
pub(crate) enum HSubsystemInfo {
    Id(HSubsystemInfoId),
    Partial(HSubsystemInfoPartial),
    Full(HSubsystemInfoFull),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HSubsystemInfo {
    pub(in crate::info::item) fn from_core(core_subsystem: &mut rc::SubsystemMut, item_mode: HItemInfoMode) -> Self {
        match item_mode {
            HItemInfoMode::Id => Self::Id(HSubsystemInfoId::from_core(core_subsystem)),
            HItemInfoMode::Partial => Self::Partial(HSubsystemInfoPartial::from_core(core_subsystem)),
            HItemInfoMode::Full => Self::Full(HSubsystemInfoFull::from_core(core_subsystem)),
        }
    }
}
