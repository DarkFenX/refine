use crate::{
    consts::{ModRack, State},
    defs::{EItemId, Idx, SsFitId, SsItemId},
    ss::item::SsModule,
};

use super::SsChargeInfo;

pub struct SsModuleInfo {
    pub id: SsItemId,
    pub fit_id: SsFitId,
    pub a_item_id: EItemId,
    pub state: State,
    pub rack: ModRack,
    pub pos: Idx,
    pub ss_charge_info: Option<SsChargeInfo>,
}
impl SsModuleInfo {
    fn new(
        id: SsItemId,
        fit_id: SsFitId,
        a_item_id: EItemId,
        state: State,
        rack: ModRack,
        pos: Idx,
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
