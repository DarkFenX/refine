use crate::{
    consts::{ModRack, State},
    defs::{ReeId, ReeIdx, ReeInt},
    ss::item::SsModule,
};

use super::SsChargeInfo;

pub struct SsModuleInfo {
    pub id: ReeId,
    pub fit_id: ReeId,
    pub a_item_id: ReeInt,
    pub state: State,
    pub rack: ModRack,
    pub pos: ReeIdx,
    pub ss_charge_info: Option<SsChargeInfo>,
}
impl SsModuleInfo {
    fn new(
        id: ReeId,
        fit_id: ReeId,
        a_item_id: ReeInt,
        state: State,
        rack: ModRack,
        pos: ReeIdx,
        ss_charge_info: Option<SsChargeInfo>,
    ) -> Self {
        Self {
            id,
            fit_id,
            a_item_id,
            state,
            rack,
            pos,
            ss_charge_info,
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
        )
    }
}
