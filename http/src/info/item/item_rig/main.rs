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
impl HRigInfo {
    pub(in crate::info::item) fn mk_info(core_rig: &mut rc::RigMut, item_mode: HItemInfoMode) -> Self {
        match item_mode {
            HItemInfoMode::Id => Self::Id(core_rig.into()),
            HItemInfoMode::Partial => Self::Partial(core_rig.into()),
            HItemInfoMode::Full => Self::Full(core_rig.into()),
        }
    }
}
