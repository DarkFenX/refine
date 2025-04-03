use crate::{
    sol::{
        FitId, Idx, ItemId, ItemTypeId, ModRack,
        info::{ChargeInfo, ItemMutationInfo, ProjInfo},
        uad::item::{Module, ModuleState},
    },
    src::Src,
};

pub struct ModuleInfo {
    pub id: ItemId,
    pub type_id: ItemTypeId,
    pub fit_id: FitId,
    pub state: ModuleState,
    pub rack: ModRack,
    pub pos: Idx,
    pub mutation: Option<ItemMutationInfo>,
    pub charge: Option<ChargeInfo>,
    pub projs: Vec<ProjInfo>,
}
impl ModuleInfo {
    pub(in crate::sol) fn from_mod_and_charge_with_source(
        src: &Src,
        sol_module: &Module,
        charge_info: Option<ChargeInfo>,
    ) -> Self {
        Self {
            id: sol_module.get_item_id(),
            type_id: sol_module.get_a_item_id(),
            fit_id: sol_module.get_fit_id(),
            state: sol_module.get_module_state(),
            rack: sol_module.get_rack(),
            pos: sol_module.get_pos(),
            mutation: sol_module.get_mutation_info(src),
            charge: charge_info,
            projs: sol_module
                .get_projs()
                .iter()
                .map(|(&item_id, &range)| ProjInfo { item_id, range })
                .collect(),
        }
    }
}
