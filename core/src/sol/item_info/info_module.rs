use crate::{
    defs::{EItemId, Idx, SolFitId, SolItemId},
    sol::{
        item::{SolItemState, SolModule},
        item_info::{SolChargeInfo, SolProjInfo},
        SolModRack,
    },
};

pub struct SolModuleInfo {
    pub id: SolItemId,
    pub type_id: EItemId,
    pub fit_id: SolFitId,
    pub state: SolItemState,
    pub rack: SolModRack,
    pub pos: Idx,
    pub charge_info: Option<SolChargeInfo>,
    pub projs: Vec<SolProjInfo>,
}
impl SolModuleInfo {
    fn new(
        id: SolItemId,
        type_id: EItemId,
        fit_id: SolFitId,
        state: SolItemState,
        rack: SolModRack,
        pos: Idx,
        charge_info: Option<SolChargeInfo>,
        projs: Vec<SolProjInfo>,
    ) -> Self {
        Self {
            id,
            type_id,
            fit_id,
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
            sol_module.get_type_id(),
            sol_module.get_fit_id(),
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
