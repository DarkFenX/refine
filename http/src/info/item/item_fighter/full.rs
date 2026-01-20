use serde::Serialize;

use super::partial::HFighterInfoPartial;
use crate::info::{HItemInfoMode, item::extended::HItemExtendedInfo};

#[derive(Serialize)]
pub(crate) struct HFighterInfoFull {
    #[serde(flatten)]
    partial_info: HFighterInfoPartial,
    #[serde(flatten)]
    extended_info: HItemExtendedInfo,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HFighterInfoFull {
    pub(super) fn from_core(core_fighter: &mut rc::FighterMut, item_mode: HItemInfoMode) -> Self {
        Self {
            partial_info: HFighterInfoPartial::from_core(core_fighter, item_mode),
            extended_info: HItemExtendedInfo::from_core(core_fighter),
        }
    }
}
