use crate::{
    defs::{EItemId, Idx, SsFitId, SsItemId},
    ss::{
        item::{SsItemState, SsModule},
        SsModRack,
    },
};

use super::{SsChargeInfo, SsTgtInfo};

pub struct SsModuleInfo {
    pub id: SsItemId,
    pub fit_id: SsFitId,
    pub a_item_id: EItemId,
    pub state: SsItemState,
    pub rack: SsModRack,
    pub pos: Idx,
    pub ss_charge_info: Option<SsChargeInfo>,
    pub tgts: Vec<SsTgtInfo>,
}
impl SsModuleInfo {
    fn new(
        id: SsItemId,
        fit_id: SsFitId,
        a_item_id: EItemId,
        state: SsItemState,
        rack: SsModRack,
        pos: Idx,
        ss_charge_info: Option<SsChargeInfo>,
        tgts: Vec<SsTgtInfo>,
    ) -> Self {
        Self {
            id,
            fit_id,
            a_item_id,
            state,
            rack,
            pos,
            ss_charge_info,
            tgts,
        }
    }
    pub(in crate::ss) fn from_mod_and_charge(ss_module: &SsModule, ss_charge_info: Option<SsChargeInfo>) -> Self {
        SsModuleInfo::new(
            ss_module.id,
            ss_module.fit_id,
            ss_module.a_item_id,
            ss_module.state,
            ss_module.rack,
            ss_module.pos,
            ss_charge_info,
            ss_module
                .tgts
                .iter()
                .map(|(item_id, range)| SsTgtInfo::new(*item_id, *range))
                .collect(),
        )
    }
}
