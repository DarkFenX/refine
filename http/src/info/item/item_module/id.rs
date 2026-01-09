use rc::ItemCommon;
use serde::Serialize;
use serde_with::{DisplayFromStr, serde_as};

use crate::info::{HItemInfoMode, item::item_charge::HChargeInfo};

#[serde_as]
#[derive(Serialize)]
pub(crate) struct HModuleInfoId {
    #[serde_as(as = "DisplayFromStr")]
    id: rc::ItemId,
    #[serde(skip_serializing_if = "Option::is_none")]
    charge: Option<HChargeInfo>,
}
impl HModuleInfoId {
    pub(super) fn mk_info(core_module: &mut rc::ModuleMut, item_mode: HItemInfoMode) -> Self {
        Self {
            id: core_module.get_item_id(),
            charge: core_module
                .get_charge_mut()
                .map(|mut charge| HChargeInfo::mk_info(&mut charge, item_mode)),
        }
    }
}
