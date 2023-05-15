use crate::{
    consts::State,
    defs::{ReeId, ReeIdx, ReeInt},
    ss::{info::ChargeInfo, item::Module},
};

pub struct ModuleInfo {
    pub item_id: ReeId,
    pub fit_id: ReeId,
    pub type_id: ReeInt,
    pub state: State,
    pub pos: ReeIdx,
    pub charge: Option<ChargeInfo>,
}
impl ModuleInfo {
    fn new(
        item_id: ReeId,
        fit_id: ReeId,
        type_id: ReeInt,
        state: State,
        pos: ReeIdx,
        charge: Option<ChargeInfo>,
    ) -> Self {
        Self {
            item_id,
            fit_id,
            type_id,
            state,
            pos,
            charge,
        }
    }
    pub(in crate::ss) fn from_mod_and_charge(module: &Module, charge_info: Option<ChargeInfo>) -> Self {
        ModuleInfo::new(
            module.item_id,
            module.fit_id,
            module.type_id,
            module.state,
            module.pos,
            charge_info,
        )
    }
}
