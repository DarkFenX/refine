use crate::sol::{
    Idx, ItemKey, ModRack, SolarSystem,
    api::{Charge, ChargeMut, Fit, FitMut, ItemCommon, ItemMutCommon, ItemMutSealed, ItemSealed},
    info::ItemMutationInfo,
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
    pub fn get_fit(&self) -> Fit {
        get_fit(self.sol, self.key)
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
}
impl<'a> ItemSealed for Module<'a> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_key(&self) -> ItemKey {
        self.key
    }
}
impl<'a> ItemCommon for Module<'a> {}

pub struct ModuleMut<'a> {
    pub(in crate::sol) sol: &'a mut SolarSystem,
    pub(in crate::sol) key: ItemKey,
}
impl<'a> ModuleMut<'a> {
    pub(in crate::sol) fn new(sol: &'a mut SolarSystem, key: ItemKey) -> Self {
        Self { sol, key }
    }
    pub fn get_fit(&self) -> Fit {
        get_fit(self.sol, self.key)
    }
    pub fn get_fit_mut(&mut self) -> FitMut {
        let fit_key = get_uad_module(self.sol, self.key).get_fit_key();
        FitMut::new(self.sol, fit_key)
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
}
impl<'a> ItemSealed for ModuleMut<'a> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_key(&self) -> ItemKey {
        self.key
    }
}
impl<'a> ItemMutSealed for ModuleMut<'a> {
    fn get_sol_mut(&mut self) -> &mut SolarSystem {
        self.sol
    }
}
impl<'a> ItemCommon for ModuleMut<'a> {}
impl<'a> ItemMutCommon for ModuleMut<'a> {}

fn get_fit(sol: &SolarSystem, item_key: ItemKey) -> Fit {
    let fit_key = get_uad_module(sol, item_key).get_fit_key();
    Fit::new(sol, fit_key)
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
fn get_uad_module(sol: &SolarSystem, item_key: ItemKey) -> &UadModule {
    sol.uad.items.get(item_key).get_module().unwrap()
}
