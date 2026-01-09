use rc::ItemCommon;
use serde::Serialize;
use serde_with::{DisplayFromStr, serde_as};

use crate::{
    info::{
        HItemInfoMode,
        item::{
            adj_count::HAdjustableCount, item_charge::HChargeInfo, mutation::HItemMutationInfo, proj::HRangedProjInfo,
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
    cycles_until_empty: TriStateField<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spool_cycles: Option<HAdjustableCount>,
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
                Some(charge_count) => TriStateField::Value(charge_count.into_u32()),
                None => TriStateField::None,
            },
            false => TriStateField::Absent,
        };
        let cycles_until_empty = match charge_info.is_some() {
            true => match core_module.get_cycle_count_until_reload() {
                Some(cycles_until_empty) => TriStateField::Value(cycles_until_empty.into_u32()),
                None => TriStateField::None,
            },
            false => TriStateField::Absent,
        };
        Self {
            id: core_module.get_item_id(),
            kind: "module",
            type_id: core_module.get_type_id().into_i32(),
            fit_id: core_module.get_fit().get_fit_id(),
            state: (&core_module.get_state()).into(),
            rack: (&core_module.get_rack()).into(),
            pos: core_module.get_pos().into_usize(),
            mutation: match core_module.get_mutation() {
                Some(rc::Mutation::Effective(effective_mutation)) => Some(effective_mutation.into()),
                _ => None,
            },
            charge: charge_info,
            charge_count,
            cycles_until_empty,
            spool_cycles: core_module.get_spool_cycle_count().map(Into::into),
            projs: core_module.iter_projs().map(HRangedProjInfo::from_core).collect(),
        }
    }
}
