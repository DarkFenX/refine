use rc::ItemCommon;

use crate::{
    info::{
        HItemInfoMode,
        item::{charge::HChargeInfo, mutation::HItemMutationInfo, proj::HRangedProjInfo},
    },
    shared::{HModRack, HModuleState},
};

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HModuleInfoPartial {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::ItemId,
    pub(crate) kind: &'static str,
    pub(crate) type_id: rc::ItemTypeId,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) fit_id: rc::FitId,
    pub(crate) state: HModuleState,
    pub(crate) rack: HModRack,
    pub(crate) pos: rc::Idx,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) mutation: Option<HItemMutationInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) charge: Option<HChargeInfo>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) projs: Vec<HRangedProjInfo>,
}
impl HModuleInfoPartial {
    pub(super) fn mk_info(core_module: &mut rc::ModuleMut, item_mode: HItemInfoMode) -> Self {
        Self {
            id: core_module.get_item_id(),
            kind: "module",
            type_id: core_module.get_type_id(),
            fit_id: core_module.get_fit().get_fit_id(),
            state: (&core_module.get_state()).into(),
            rack: (&core_module.get_rack()).into(),
            pos: core_module.get_pos(),
            mutation: core_module.get_mutation().as_ref().map(|v| v.into()),
            charge: core_module
                .get_charge_mut()
                .map(|mut charge| HChargeInfo::mk_info(&mut charge, item_mode)),
            projs: core_module
                .iter_projs()
                .map(|core_ranged_proj| core_ranged_proj.into())
                .collect(),
        }
    }
}
