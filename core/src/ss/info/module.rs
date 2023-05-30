use crate::{
    consts::{ModRack, State},
    defs::{ReeId, ReeIdx, ReeInt},
    ss::{info::ChargeInfo, item::Module},
};

pub struct ModuleInfo {
    pub id: ReeId,
    pub fit_id: ReeId,
    pub type_id: ReeInt,
    pub state: State,
    pub rack: ModRack,
    pub pos: ReeIdx,
    pub charge_info: Option<ChargeInfo>,
}
impl ModuleInfo {
    fn new(
        id: ReeId,
        fit_id: ReeId,
        type_id: ReeInt,
        state: State,
        rack: ModRack,
        pos: ReeIdx,
        charge_info: Option<ChargeInfo>,
    ) -> Self {
        Self {
            id,
            fit_id,
            type_id,
            state,
            rack,
            pos,
            charge_info,
        }
    }
    pub(in crate::ss) fn from_mod_and_charge(module: &Module, charge_info: Option<ChargeInfo>) -> Self {
        ModuleInfo::new(
            module.id,
            module.fit_id,
            module.type_id,
            module.state,
            module.rack,
            module.pos,
            charge_info,
        )
    }
}
