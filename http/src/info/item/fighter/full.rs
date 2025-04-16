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
    pub(super) fn mk_info(
        core_sol: &mut rc::SolarSystem,
        core_fighter_info: &rc::FighterInfo,
        item_mode: HItemInfoMode,
    ) -> Self {
        let partial_info = HFighterInfoPartial::mk_info(core_sol, core_fighter_info, item_mode);
        let extended_info = HItemExtendedInfo::from_item_id(core_sol, &partial_info.id);
        Self {
            partial_info,
            extended_info,
        }
    }
}
