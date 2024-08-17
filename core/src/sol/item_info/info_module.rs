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
    pub type_id: EItemId,
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
        type_id: EItemId,
        state: SolItemState,
        rack: SolModRack,
        pos: Idx,
        charge_info: Option<SolChargeInfo>,
        projs: Vec<SolProjInfo>,
    ) -> Self {
        Self {
            id,
            fit_id,
            type_id,
            state,
            rack,
            pos,
            charge_info,
            projs,
        }
    }
    pub(in crate::sol) fn from_mod_and_charge(sol_module: &SolModule, charge_info: Option<SolChargeInfo>) -> Self {
        SolModuleInfo::new(
            sol_module.get_id(),
            sol_module.get_fit_id(),
            sol_module.get_type_id(),
            sol_module.get_state(),
            sol_module.get_rack(),
            sol_module.get_pos(),
            charge_info,
            sol_module
                .get_projs()
                .iter()
                .map(|(item_id, range)| SolProjInfo::new(*item_id, *range))
                .collect(),
        )
    }
}
