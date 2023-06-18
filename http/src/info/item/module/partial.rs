use crate::{
    info::{item::charge::HChargeInfo, HItemInfoMode},
    shared::{HModRack, HState},
};

#[derive(serde::Serialize)]
pub(crate) struct HModuleInfoPartial {
    #[serde(with = "crate::util::serde_string")]
    pub(crate) id: rc::ReeId,
    #[serde(with = "crate::util::serde_string")]
    pub(crate) fit_id: rc::ReeId,
    pub(crate) type_id: rc::ReeInt,
    pub(crate) state: HState,
    pub(crate) rack: HModRack,
    pub(crate) pos: rc::ReeIdx,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) charge: Option<HChargeInfo>,
}
impl HModuleInfoPartial {
    pub(super) fn mk_info(
        core_ss: &mut rc::SolarSystem,
        core_module_info: &rc::SsModuleInfo,
        item_mode: HItemInfoMode,
    ) -> Self {
        Self {
            id: core_module_info.id,
            fit_id: core_module_info.fit_id,
            type_id: core_module_info.a_item_id,
            state: (&core_module_info.state).into(),
            rack: (&core_module_info.rack).into(),
            pos: core_module_info.pos,
            charge: core_module_info
                .ss_charge_info
                .as_ref()
                .map(|v| HChargeInfo::mk_info(core_ss, v, item_mode)),
        }
    }
}
