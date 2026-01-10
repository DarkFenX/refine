use crate::{
    api::{
        Adjustable, Charge, ChargeMut, Fit, FitMut, ItemCommon, ItemMutCommon, ItemMutSealed, ItemSealed, ModuleState,
    },
    misc::{InfCount, ModRack},
    num::{Count, Index},
    sol::SolarSystem,
    ud::{UItemId, UModule},
};

pub struct Module<'a> {
    pub(in crate::api) sol: &'a SolarSystem,
    pub(in crate::api) uid: UItemId,
}
impl<'a> Module<'a> {
    pub(in crate::api) fn new(sol: &'a SolarSystem, uid: UItemId) -> Self {
        Self { sol, uid }
    }
    pub fn get_fit(&self) -> Fit<'_> {
        get_fit(self.sol, self.uid)
    }
    pub fn get_state(&self) -> ModuleState {
        get_state(self.sol, self.uid)
    }
    pub fn get_rack(&self) -> ModRack {
        get_rack(self.sol, self.uid)
    }
    pub fn get_pos(&self) -> Index {
        get_pos(self.sol, self.uid)
    }
    pub fn get_charge(&self) -> Option<Charge<'_>> {
        get_charge(self.sol, self.uid)
    }
    pub fn get_charge_count(&self) -> Option<Count> {
        get_charge_count(self.sol, self.uid)
    }
    pub fn get_reload_optionals(&self) -> Option<bool> {
        get_reload_optionals(self.sol, self.uid)
    }
}
impl<'a> ItemSealed for Module<'a> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_uid(&self) -> UItemId {
        self.uid
    }
}
impl<'a> ItemCommon for Module<'a> {}

pub struct ModuleMut<'a> {
    pub(in crate::api) sol: &'a mut SolarSystem,
    pub(in crate::api) uid: UItemId,
}
impl<'a> ModuleMut<'a> {
    pub(in crate::api) fn new(sol: &'a mut SolarSystem, uid: UItemId) -> Self {
        Self { sol, uid }
    }
    pub fn get_fit(&self) -> Fit<'_> {
        get_fit(self.sol, self.uid)
    }
    pub fn get_fit_mut(&mut self) -> FitMut<'_> {
        let fit_uid = get_u_module(self.sol, self.uid).get_fit_uid();
        FitMut::new(self.sol, fit_uid)
    }
    pub fn get_state(&self) -> ModuleState {
        get_state(self.sol, self.uid)
    }
    pub fn get_rack(&self) -> ModRack {
        get_rack(self.sol, self.uid)
    }
    pub fn get_pos(&self) -> Index {
        get_pos(self.sol, self.uid)
    }
    pub fn get_charge(&self) -> Option<Charge<'_>> {
        get_charge(self.sol, self.uid)
    }
    pub fn get_charge_mut(&mut self) -> Option<ChargeMut<'_>> {
        get_u_module(self.sol, self.uid)
            .get_charge_uid()
            .map(|charge_uid| ChargeMut::new(self.sol, charge_uid))
    }
    pub fn get_charge_count(&self) -> Option<Count> {
        get_charge_count(self.sol, self.uid)
    }
    pub fn get_reload_optionals(&self) -> Option<bool> {
        get_reload_optionals(self.sol, self.uid)
    }
    pub fn get_cycle_count_until_reload(&mut self) -> Option<Count> {
        match self.sol.svc.get_item_cycles_until_empty(&self.sol.u_data, self.uid) {
            Some(InfCount::Count(count)) => Some(count),
            _ => None,
        }
    }
    pub fn get_spool_cycle_count(&mut self) -> Option<Adjustable<Count>> {
        self.sol.svc.get_effect_spool_cycle_count(&self.sol.u_data, self.uid)
    }
}
impl<'a> ItemSealed for ModuleMut<'a> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_uid(&self) -> UItemId {
        self.uid
    }
}
impl<'a> ItemMutSealed for ModuleMut<'a> {
    fn get_sol_mut(&mut self) -> &mut SolarSystem {
        self.sol
    }
}
impl<'a> ItemCommon for ModuleMut<'a> {}
impl<'a> ItemMutCommon for ModuleMut<'a> {}

fn get_fit(sol: &SolarSystem, module_uid: UItemId) -> Fit<'_> {
    let fit_uid = get_u_module(sol, module_uid).get_fit_uid();
    Fit::new(sol, fit_uid)
}
fn get_state(sol: &SolarSystem, module_uid: UItemId) -> ModuleState {
    get_u_module(sol, module_uid).get_module_state()
}
fn get_rack(sol: &SolarSystem, module_uid: UItemId) -> ModRack {
    get_u_module(sol, module_uid).get_rack()
}
fn get_pos(sol: &SolarSystem, module_uid: UItemId) -> Index {
    get_u_module(sol, module_uid).get_pos()
}
fn get_charge(sol: &SolarSystem, module_uid: UItemId) -> Option<Charge<'_>> {
    get_u_module(sol, module_uid)
        .get_charge_uid()
        .map(|charge_uid| Charge::new(sol, charge_uid))
}
fn get_charge_count(sol: &SolarSystem, module_uid: UItemId) -> Option<Count> {
    get_u_module(sol, module_uid).get_charge_count(&sol.u_data)
}
fn get_reload_optionals(sol: &SolarSystem, module_uid: UItemId) -> Option<bool> {
    get_u_module(sol, module_uid).get_reload_optionals()
}
fn get_u_module(sol: &SolarSystem, module_uid: UItemId) -> &UModule {
    sol.u_data.items.get(module_uid).dc_module().unwrap()
}
