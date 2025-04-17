use std::collections::HashMap;

use rc::{ItemCommon, Lender};

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
    pub(super) fn mk_info(core_fighter: &mut rc::FighterMut, item_mode: HItemInfoMode) -> Self {
        let mut autocharges = HashMap::new();
        let mut autocharge_iter = core_fighter.iter_autocharges_mut();
        while let Some(mut autocharge) = autocharge_iter.next() {
            autocharges.insert(
                autocharge.get_cont_effect_id().into(),
                HAutochargeInfo::mk_info(&mut autocharge, item_mode),
            );
        }
        Self {
            id: core_fighter.get_item_id(),
            autocharges,
        }
    }
}
