use crate::sol::{
    FitId, Idx, ItemId, ItemTypeId, ModRack,
    info::{ChargeInfo, ItemMutationInfo, ProjInfo},
    uad::{
        Uad,
        item::{Module, ModuleState},
    },
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
    pub(in crate::sol) fn from_module(uad: &Uad, module: &Module) -> Self {
        Self {
            id: module.get_item_id(),
            type_id: module.get_a_item_id(),
            fit_id: module.get_fit_id(),
            state: module.get_module_state(),
            rack: module.get_rack(),
            pos: module.get_pos(),
            mutation: module.get_mutation_info(&uad.src),
            charge: module
                .get_charge_item_key()
                .map(|charge_key| ChargeInfo::from_charge(uad, uad.items.get(charge_key).get_charge().unwrap())),
            projs: module
                .get_projs()
                .iter()
                .map(|(&projectee_item_key, &range)| ProjInfo {
                    item_id: uad.items.id_by_key(projectee_item_key),
                    range,
                })
                .collect(),
        }
    }
}
