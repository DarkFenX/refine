use std::collections::HashMap;

use crate::{
    info::{HItemInfoMode, item::autocharge::HAutochargeInfo},
    shared::HEffectId,
};

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HFighterInfoId {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::ItemId,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub(crate) autocharges: HashMap<HEffectId, HAutochargeInfo>,
}
impl HFighterInfoId {
    pub(super) fn mk_info(
        core_sol: &mut rc::SolarSystem,
        core_fighter_info: &rc::FighterInfo,
        item_mode: HItemInfoMode,
    ) -> Self {
        Self {
            id: core_fighter_info.id,
            autocharges: core_fighter_info
                .autocharges
                .iter()
                .map(|(k, v)| (k.into(), HAutochargeInfo::mk_info(core_sol, v, item_mode)))
                .collect(),
        }
    }
}
