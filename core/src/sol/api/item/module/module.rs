use crate::sol::{
    FitId, Idx, ItemId, ItemKey, ItemTypeId, ModRack, SolarSystem,
    api::{Charge, ChargeMut},
    info::{ItemMutationInfo, ProjInfo},
    uad::item::{ModuleState, UadModule},
};

pub struct Module<'a> {
    pub(in crate::sol) sol: &'a SolarSystem,
    pub(in crate::sol) key: ItemKey,
}
impl<'a> Module<'a> {
    pub(in crate::sol) fn new(sol: &'a SolarSystem, key: ItemKey) -> Self {
        Self { sol, key }
    }
    pub fn get_item_id(&self) -> ItemId {
        get_item_id(self.sol, self.key)
    }
    pub fn get_type_id(&self) -> ItemTypeId {
        get_type_id(self.sol, self.key)
    }
    pub fn get_fit_id(&self) -> FitId {
        get_fit_id(self.sol, self.key)
    }
    pub fn get_state(&self) -> ModuleState {
        get_state(self.sol, self.key)
    }
    pub fn get_rack(&self) -> ModRack {
        get_rack(self.sol, self.key)
    }
    pub fn get_pos(&self) -> Idx {
        get_pos(self.sol, self.key)
    }
    pub fn get_mutation(&self) -> Option<ItemMutationInfo> {
        get_mutation(self.sol, self.key)
    }
    pub fn get_charge(&self) -> Option<Charge> {
        get_charge(self.sol, self.key)
    }
    pub fn get_projs(&self) -> Vec<ProjInfo> {
        get_projs(self.sol, self.key)
    }
}

pub struct ModuleMut<'a> {
    pub(in crate::sol) sol: &'a mut SolarSystem,
    pub(in crate::sol) key: ItemKey,
}
impl<'a> ModuleMut<'a> {
    pub(in crate::sol) fn new(sol: &'a mut SolarSystem, key: ItemKey) -> Self {
        Self { sol, key }
    }
    pub fn get_item_id(&self) -> ItemId {
        get_item_id(self.sol, self.key)
    }
    pub fn get_type_id(&self) -> ItemTypeId {
        get_type_id(self.sol, self.key)
    }
    pub fn get_fit_id(&self) -> FitId {
        get_fit_id(self.sol, self.key)
    }
    pub fn get_state(&self) -> ModuleState {
        get_state(self.sol, self.key)
    }
    pub fn get_rack(&self) -> ModRack {
        get_rack(self.sol, self.key)
    }
    pub fn get_pos(&self) -> Idx {
        get_pos(self.sol, self.key)
    }
    pub fn get_mutation(&self) -> Option<ItemMutationInfo> {
        get_mutation(self.sol, self.key)
    }
    pub fn get_charge(&self) -> Option<Charge> {
        get_charge(self.sol, self.key)
    }
    pub fn get_charge_mut(&mut self) -> Option<ChargeMut> {
        get_uad_module(self.sol, self.key)
            .get_charge_item_key()
            .map(|charge_key| ChargeMut::new(self.sol, charge_key))
    }
    pub fn get_projs(&self) -> Vec<ProjInfo> {
        get_projs(self.sol, self.key)
    }
}

fn get_item_id(sol: &SolarSystem, item_key: ItemKey) -> ItemId {
    sol.uad.items.id_by_key(item_key)
}
fn get_type_id(sol: &SolarSystem, item_key: ItemKey) -> ItemTypeId {
    get_uad_module(sol, item_key).get_a_item_id()
}
fn get_fit_id(sol: &SolarSystem, item_key: ItemKey) -> FitId {
    let fit_key = get_uad_module(sol, item_key).get_fit_key();
    sol.uad.fits.id_by_key(fit_key)
}
fn get_state(sol: &SolarSystem, item_key: ItemKey) -> ModuleState {
    get_uad_module(sol, item_key).get_module_state()
}
fn get_rack(sol: &SolarSystem, item_key: ItemKey) -> ModRack {
    get_uad_module(sol, item_key).get_rack()
}
fn get_pos(sol: &SolarSystem, item_key: ItemKey) -> Idx {
    get_uad_module(sol, item_key).get_pos()
}
fn get_mutation(sol: &SolarSystem, item_key: ItemKey) -> Option<ItemMutationInfo> {
    get_uad_module(sol, item_key).get_mutation_info(&sol.uad.src)
}
fn get_charge(sol: &SolarSystem, item_key: ItemKey) -> Option<Charge> {
    get_uad_module(sol, item_key)
        .get_charge_item_key()
        .map(|charge_key| Charge::new(sol, charge_key))
}
fn get_projs(sol: &SolarSystem, item_key: ItemKey) -> Vec<ProjInfo> {
    get_uad_module(sol, item_key)
        .get_projs()
        .iter()
        .map(|(&projectee_item_key, &range)| ProjInfo {
            item_id: sol.uad.items.id_by_key(projectee_item_key),
            range,
        })
        .collect()
}
fn get_uad_module(sol: &SolarSystem, item_key: ItemKey) -> &UadModule {
    sol.uad.items.get(item_key).get_module().unwrap()
}
