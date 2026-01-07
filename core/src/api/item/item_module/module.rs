use crate::{
    api::{
        Adjustable, Charge, ChargeMut, Fit, FitMut, ItemCommon, ItemMutCommon, ItemMutSealed, ItemSealed, ModuleState,
    },
    def::{DefCount, Idx},
    misc::ModRack,
    sol::SolarSystem,
    ud::{UItemId, UModule},
    util::InfCount,
};

pub struct Module<'a> {
    pub(in crate::api) sol: &'a SolarSystem,
    pub(in crate::api) key: UItemId,
}
impl<'a> Module<'a> {
    pub(in crate::api) fn new(sol: &'a SolarSystem, key: UItemId) -> Self {
        Self { sol, key }
    }
    pub fn get_fit(&self) -> Fit<'_> {
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
    pub fn get_charge(&self) -> Option<Charge<'_>> {
        get_charge(self.sol, self.key)
    }
    pub fn get_charge_count(&self) -> Option<DefCount> {
        get_charge_count(self.sol, self.key)
    }
    pub fn get_reload_optionals(&self) -> Option<bool> {
        get_reload_optionals(self.sol, self.key)
    }
}
impl<'a> ItemSealed for Module<'a> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_key(&self) -> UItemId {
        self.key
    }
}
impl<'a> ItemCommon for Module<'a> {}

pub struct ModuleMut<'a> {
    pub(in crate::api) sol: &'a mut SolarSystem,
    pub(in crate::api) key: UItemId,
}
impl<'a> ModuleMut<'a> {
    pub(in crate::api) fn new(sol: &'a mut SolarSystem, key: UItemId) -> Self {
        Self { sol, key }
    }
    pub fn get_fit(&self) -> Fit<'_> {
        get_fit(self.sol, self.key)
    }
    pub fn get_fit_mut(&mut self) -> FitMut<'_> {
        let fit_key = get_u_module(self.sol, self.key).get_fit_uid();
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
    pub fn get_charge(&self) -> Option<Charge<'_>> {
        get_charge(self.sol, self.key)
    }
    pub fn get_charge_mut(&mut self) -> Option<ChargeMut<'_>> {
        get_u_module(self.sol, self.key)
            .get_charge_uid()
            .map(|charge_key| ChargeMut::new(self.sol, charge_key))
    }
    pub fn get_charge_count(&self) -> Option<DefCount> {
        get_charge_count(self.sol, self.key)
    }
    pub fn get_reload_optionals(&self) -> Option<bool> {
        get_reload_optionals(self.sol, self.key)
    }
    pub fn get_cycle_count_until_reload(&mut self) -> Option<DefCount> {
        match self.sol.svc.get_item_cycles_until_empty(&self.sol.u_data, self.key) {
            Some(InfCount::Count(count)) => Some(count),
            _ => None,
        }
    }
    pub fn get_spool_cycle_count(&mut self) -> Option<Adjustable> {
        self.sol.svc.get_effect_spool_cycle_count(&self.sol.u_data, self.key)
    }
}
impl<'a> ItemSealed for ModuleMut<'a> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_key(&self) -> UItemId {
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

fn get_fit(sol: &SolarSystem, module_key: UItemId) -> Fit<'_> {
    let fit_key = get_u_module(sol, module_key).get_fit_uid();
    Fit::new(sol, fit_key)
}
fn get_state(sol: &SolarSystem, module_key: UItemId) -> ModuleState {
    get_u_module(sol, module_key).get_module_state()
}
fn get_rack(sol: &SolarSystem, module_key: UItemId) -> ModRack {
    get_u_module(sol, module_key).get_rack()
}
fn get_pos(sol: &SolarSystem, module_key: UItemId) -> Idx {
    get_u_module(sol, module_key).get_pos()
}
fn get_charge(sol: &SolarSystem, module_key: UItemId) -> Option<Charge<'_>> {
    get_u_module(sol, module_key)
        .get_charge_uid()
        .map(|charge_key| Charge::new(sol, charge_key))
}
fn get_charge_count(sol: &SolarSystem, module_key: UItemId) -> Option<DefCount> {
    get_u_module(sol, module_key).get_charge_count(&sol.u_data)
}
fn get_reload_optionals(sol: &SolarSystem, module_key: UItemId) -> Option<bool> {
    get_u_module(sol, module_key).get_reload_optionals()
}
fn get_u_module(sol: &SolarSystem, module_key: UItemId) -> &UModule {
    sol.u_data.items.get(module_key).dc_module().unwrap()
}
