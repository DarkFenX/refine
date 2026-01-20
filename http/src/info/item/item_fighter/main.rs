use serde::Serialize;

use super::{full::HFighterInfoFull, id::HFighterInfoId, partial::HFighterInfoPartial};
use crate::info::HItemInfoMode;

#[derive(Serialize)]
#[serde(untagged)]
pub(crate) enum HFighterInfo {
    Id(HFighterInfoId),
    Partial(HFighterInfoPartial),
    Full(HFighterInfoFull),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HFighterInfo {
    pub(in crate::info::item) fn from_core(core_fighter: &mut rc::FighterMut, item_mode: HItemInfoMode) -> Self {
        match item_mode {
            HItemInfoMode::Id => Self::Id(HFighterInfoId::from_core(core_fighter, item_mode)),
            HItemInfoMode::Partial => Self::Partial(HFighterInfoPartial::from_core(core_fighter, item_mode)),
            HItemInfoMode::Full => Self::Full(HFighterInfoFull::from_core(core_fighter, item_mode)),
        }
    }
}
