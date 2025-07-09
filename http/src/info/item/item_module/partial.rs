use rc::ItemCommon;

use crate::{
    info::{
        HItemInfoMode,
        item::{item_charge::HChargeInfo, mutation::HItemMutationInfo, proj::HRangedProjInfo},
    },
    shared::{HModRack, HModuleState},
    util::TriStateField,
};

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HModuleInfoPartial {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    id: rc::ItemId,
    kind: &'static str,
    type_id: rc::ItemTypeId,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    fit_id: rc::FitId,
    state: HModuleState,
    rack: HModRack,
    pos: rc::Idx,
    #[serde(skip_serializing_if = "Option::is_none")]
    mutation: Option<HItemMutationInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    charge: Option<HChargeInfo>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    charge_count: TriStateField<rc::Count>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    projs: Vec<HRangedProjInfo>,
}
impl HModuleInfoPartial {
    pub(super) fn mk_info(core_module: &mut rc::ModuleMut, item_mode: HItemInfoMode) -> Self {
        let charge_info = core_module
            .get_charge_mut()
            .map(|mut charge| HChargeInfo::mk_info(&mut charge, item_mode));
        let charge_count = match charge_info.is_some() {
            true => match core_module.get_charge_count() {
                Some(charge_count) => TriStateField::Value(charge_count),
                None => TriStateField::None,
            },
            false => TriStateField::Absent,
        };
        Self {
            id: core_module.get_item_id(),
            kind: "module",
            type_id: core_module.get_type_id(),
            fit_id: core_module.get_fit().get_fit_id(),
            state: (&core_module.get_state()).into(),
            rack: (&core_module.get_rack()).into(),
            pos: core_module.get_pos(),
            mutation: match core_module.get_mutation() {
                Some(rc::Mutation::Effective(effective_mutation)) => Some(effective_mutation.into()),
                _ => None,
            },
            charge: charge_info,
            charge_count,
            projs: core_module
                .iter_projs()
                .map(|core_ranged_proj| core_ranged_proj.into())
                .collect(),
        }
    }
}
