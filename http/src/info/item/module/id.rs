use rc::ItemCommon;

use crate::info::{HItemInfoMode, item::charge::HChargeInfo};

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HModuleInfoId {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::ItemId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) charge: Option<HChargeInfo>,
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
