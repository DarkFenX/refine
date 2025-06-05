use std::collections::HashMap;

use rc::{ItemCommon, Lender};

use crate::{
    info::{HItemInfoMode, item::item_autocharge::HAutochargeInfo},
    shared::HEffectId,
};

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HFighterInfoId {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    id: rc::ItemId,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    autocharges: HashMap<HEffectId, HAutochargeInfo>,
}
impl HFighterInfoId {
    pub(super) fn mk_info(core_fighter: &mut rc::FighterMut, item_mode: HItemInfoMode) -> Self {
        Self {
            id: core_fighter.get_item_id(),
            autocharges: core_fighter
                .iter_autocharges_mut()
                .map_into_iter(|mut autocharge| {
                    (
                        autocharge.get_cont_effect_id().into(),
                        HAutochargeInfo::mk_info(&mut autocharge, item_mode),
                    )
                })
                .collect(),
        }
    }
}
