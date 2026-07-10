use rc::ItemCommon;
use serde::Serialize;
use serde_with::{DisplayFromStr, serde_as};

use super::optional_reload::HItemOptionalReloadInfo;
use crate::{
    info::{
        HItemInfoMode,
        item::{
            count_info::HItemCountInfo, item_charge::HChargeInfo, mutation::HItemMutationInfo, proj::HRangedProjInfo,
        },
    },
    shared::{HModRack, HModuleState},
    util::TriStateField,
};

#[serde_as]
#[derive(Serialize)]
pub(crate) struct HModuleInfoPartial {
    #[serde_as(as = "DisplayFromStr")]
    id: rc::ItemId,
    kind: &'static str,
    type_id: i32,
    #[serde_as(as = "DisplayFromStr")]
    fit_id: rc::FitId,
    state: HModuleState,
    rack: HModRack,
    pos: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    mutation: Option<HItemMutationInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    charge: Option<HChargeInfo>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    charge_count: TriStateField<u32>,
    #[serde(skip_serializing_if = "TriStateField::is_absent")]
    charged_cycles: TriStateField<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spool_cycles: Option<HItemCountInfo>,
    optional_reload: HItemOptionalReloadInfo,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    projs: Vec<HRangedProjInfo>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HModuleInfoPartial {
    pub(super) fn from_core(core_module: &mut rc::ModuleMut, item_mode: HItemInfoMode) -> Self {
        let charge_info = core_module
            .get_charge_mut()
            .map(|mut charge| HChargeInfo::from_core(&mut charge, item_mode));
        let charge_count = match charge_info.is_some() {
            true => match core_module.get_charge_count() {
                Some(charge_count) => TriStateField::Value(charge_count.into_u32()),
                None => TriStateField::None,
            },
            false => TriStateField::Absent,
        };
        let charged_cycle_count = match charge_info.is_some() {
            true => match core_module.get_charged_cycle_count() {
                Some(charged_cycle_count) => TriStateField::Value(charged_cycle_count.into_u32()),
                None => TriStateField::None,
            },
            false => TriStateField::Absent,
        };
        Self {
            id: core_module.get_item_id(),
            kind: "module",
            type_id: core_module.get_type_id().into_i32(),
            fit_id: core_module.get_fit().get_fit_id(),
            state: HModuleState::from_core(core_module.get_state()),
            rack: HModRack::from_core(core_module.get_rack()),
            pos: core_module.get_pos().into_usize(),
            mutation: core_module.get_mutation().and_then(HItemMutationInfo::try_from_core),
            charge: charge_info,
            charge_count,
            charged_cycles: charged_cycle_count,
            spool_cycles: core_module
                .get_spool_cycle_count()
                .map(HItemCountInfo::from_core_item_spool),
            optional_reload: HItemOptionalReloadInfo::from_core(core_module.get_optional_reload()),
            projs: core_module.iter_projs().map(HRangedProjInfo::from_core).collect(),
        }
    }
}
