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
impl HFighterInfoFull {
    pub(super) fn mk_info(core_fighter: &mut rc::FighterMut, item_mode: HItemInfoMode) -> Self {
        Self {
            partial_info: HFighterInfoPartial::mk_info(core_fighter, item_mode),
            extended_info: core_fighter.into(),
        }
    }
}
