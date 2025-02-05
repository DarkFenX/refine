use crate::{
    defs::{EItemId, Idx, SolFitId, SolItemId},
    sol::{
        info::{SolChargeInfo, SolItemMutationInfo, SolProjInfo},
        uad::item::{SolModule, SolModuleState},
        SolModRack,
    },
    src::Src,
};

pub struct SolModuleInfo {
    pub id: SolItemId,
    pub type_id: EItemId,
    pub fit_id: SolFitId,
    pub state: SolModuleState,
    pub rack: SolModRack,
    pub pos: Idx,
    pub mutation: Option<SolItemMutationInfo>,
    pub charge: Option<SolChargeInfo>,
    pub projs: Vec<SolProjInfo>,
}
impl SolModuleInfo {
    fn new(
        id: SolItemId,
        type_id: EItemId,
        fit_id: SolFitId,
        state: SolModuleState,
        rack: SolModRack,
        pos: Idx,
        mutation: Option<SolItemMutationInfo>,
        charge: Option<SolChargeInfo>,
        projs: Vec<SolProjInfo>,
    ) -> Self {
        Self {
            id,
            type_id,
            fit_id,
            state,
            rack,
            pos,
            mutation,
            charge,
            projs,
        }
    }
    pub(in crate::sol) fn from_mod_and_charge_with_source(
        src: &Src,
        sol_module: &SolModule,
        charge_info: Option<SolChargeInfo>,
    ) -> Self {
        SolModuleInfo::new(
            sol_module.get_id(),
            sol_module.get_type_id(),
            sol_module.get_fit_id(),
            sol_module.get_module_state(),
            sol_module.get_rack(),
            sol_module.get_pos(),
            sol_module.get_mutation_info(src),
            charge_info,
            sol_module
                .get_projs()
                .iter()
                .map(|(item_id, range)| SolProjInfo::new(*item_id, *range))
                .collect(),
        )
    }
}
