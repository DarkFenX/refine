use crate::{
    defs::{EItemId, Idx, SolFitId, SolItemId},
    sol::{
        item::{SolItemState, SolModule},
        SolModRack,
    },
};

use super::{SolChargeInfo, SolProjInfo};

pub struct SolModuleInfo {
    pub id: SolItemId,
    pub fit_id: SolFitId,
    pub a_item_id: EItemId,
    pub state: SolItemState,
    pub rack: SolModRack,
    pub pos: Idx,
    pub charge_info: Option<SolChargeInfo>,
    pub projs: Vec<SolProjInfo>,
}
impl SolModuleInfo {
    fn new(
        id: SolItemId,
        fit_id: SolFitId,
        a_item_id: EItemId,
        state: SolItemState,
        rack: SolModRack,
        pos: Idx,
        charge_info: Option<SolChargeInfo>,
        projs: Vec<SolProjInfo>,
    ) -> Self {
        Self {
            id,
            fit_id,
            a_item_id,
            state,
            rack,
            pos,
            charge_info,
            projs,
        }
    }
    pub(in crate::sol) fn from_mod_and_charge(sol_module: &SolModule, charge_info: Option<SolChargeInfo>) -> Self {
        SolModuleInfo::new(
            sol_module.id,
            sol_module.fit_id,
            sol_module.a_item_id,
            sol_module.state,
            sol_module.rack,
            sol_module.pos,
            charge_info,
            sol_module
                .projs
                .iter()
                .map(|(item_id, range)| SolProjInfo::new(*item_id, *range))
                .collect(),
        )
    }
}
