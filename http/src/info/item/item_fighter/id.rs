use rc::{ItemCommon, Lender};
use serde::Serialize;
use serde_with::{DisplayFromStr, Map, serde_as};

use crate::info::{HItemInfoMode, item::item_autocharge::HAutochargeInfo};

#[serde_as]
#[derive(Serialize)]
pub(crate) struct HFighterInfoId {
    #[serde_as(as = "DisplayFromStr")]
    id: rc::ItemId,
    #[serde_as(as = "Map<DisplayFromStr, _>")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    autocharges: Vec<(rc::EffectId, HAutochargeInfo)>,
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
