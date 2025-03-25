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
    pub(super) fn mk_info(
        core_sol: &mut rc::SolarSystem,
        core_module_info: &rc::ModuleInfo,
        item_mode: HItemInfoMode,
    ) -> Self {
        Self {
            id: core_module_info.id,
            charge: core_module_info
                .charge
                .as_ref()
                .map(|v| HChargeInfo::mk_info(core_sol, v, item_mode)),
        }
    }
}
