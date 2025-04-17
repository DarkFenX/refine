use crate::info::{HItemInfoMode, item::extended::HItemExtendedInfo};

use super::HFighterInfoPartial;

#[derive(serde::Serialize)]
pub(crate) struct HFighterInfoFull {
    #[serde(flatten)]
    pub(crate) partial_info: HFighterInfoPartial,
    #[serde(flatten)]
    pub(crate) extended_info: HItemExtendedInfo,
}
impl HFighterInfoFull {
    pub(super) fn mk_info(core_fighter: &mut rc::FighterMut, item_mode: HItemInfoMode) -> Self {
        Self {
            partial_info: HFighterInfoPartial::mk_info(core_fighter, item_mode),
            extended_info: core_fighter.into(),
        }
    }
}
