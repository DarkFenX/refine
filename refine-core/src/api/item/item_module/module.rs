use crate::{
    Charge, ChargeMut, Count, Fit, FitMut, Index, ItemCommon, ItemMutCommon, ItemSpoolInfo, ModRack, ModuleState,
    OptionalReload, SolarSystem, Spool,
    api::{ItemSealed, active_stat_prepare, active_stat_rollback},
    misc::InfCount,
    stats::{StatItemChargeOptions, StatItemStateOptions},
    svc::cycle::CseqMap,
    ud::{UEffectUpdates, UItemId, UModule},
};

pub struct Module<'s> {
    pub(in crate::api) sol: &'s SolarSystem,
    pub(in crate::api) uid: UItemId,
}
impl<'s> Module<'s> {
    pub(in crate::api) fn new(sol: &'s SolarSystem, uid: UItemId) -> Self {
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
    pub fn get_optional_reload(&self) -> OptionalReload {
        get_optional_reload(self.sol, self.uid)
    }
    pub fn get_optional_reload_override(&self) -> Option<OptionalReload> {
        get_optional_reload_override(self.sol, self.uid)
    }
    pub fn get_spool(&self) -> Spool {
        get_spool(self.sol, self.uid)
    }
    pub fn get_spool_override(&self) -> Option<Spool> {
        get_spool_override(self.sol, self.uid)
    }
}
impl<'s> ItemSealed for Module<'s> {
    fn get_uid(&self) -> UItemId {
        self.uid
    }
}
impl<'s> ItemCommon for Module<'s> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
}

pub struct ModuleMut<'s> {
    pub(in crate::api) sol: &'s mut SolarSystem,
    pub(in crate::api) uid: UItemId,
}
impl<'s> ModuleMut<'s> {
    pub(in crate::api) fn new(sol: &'s mut SolarSystem, uid: UItemId) -> Self {
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
    pub fn get_spool(&self) -> Spool {
        get_spool(self.sol, self.uid)
    }
    pub fn get_spool_override(&self) -> Option<Spool> {
        get_spool_override(self.sol, self.uid)
    }
    pub fn get_optional_reload(&self) -> OptionalReload {
        get_optional_reload(self.sol, self.uid)
    }
    pub fn get_optional_reload_override(&self) -> Option<OptionalReload> {
        get_optional_reload_override(self.sol, self.uid)
    }
    pub fn get_charged_cycle_count(&mut self) -> Option<Count> {
        let mut reuse_eupdates = UEffectUpdates::new();
        let saved_state = active_stat_prepare(
            self,
            StatItemChargeOptions::Exclude,
            StatItemStateOptions::Switch,
            &mut reuse_eupdates,
        );
        let result = match self
            .sol
            .svc
            .get_item_charged_cycle_count(&mut CseqMap::new(), &self.sol.u_data, self.uid)
        {
            Some(InfCount::Count(count)) => Some(count),
            _ => None,
        };
        active_stat_rollback(self, saved_state, &mut reuse_eupdates);
        result
    }
    pub fn get_spool_cycle_count(&mut self) -> Option<ItemSpoolInfo> {
        self.sol.svc.get_effect_spool_cycle_count(&self.sol.u_data, self.uid)
    }
}
impl<'s> ItemSealed for ModuleMut<'s> {
    fn get_uid(&self) -> UItemId {
        self.uid
    }
}
impl<'s> ItemCommon for ModuleMut<'s> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
}
impl<'s> ItemMutCommon for ModuleMut<'s> {
    fn get_sol_mut(&mut self) -> &mut SolarSystem {
        self.sol
    }
}

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
fn get_spool(sol: &SolarSystem, module_uid: UItemId) -> Spool {
    sol.u_data.get_item_spool(module_uid, None)
}
fn get_spool_override(sol: &SolarSystem, module_uid: UItemId) -> Option<Spool> {
    get_u_module(sol, module_uid).get_spool_override()
}
fn get_optional_reload(sol: &SolarSystem, module_uid: UItemId) -> OptionalReload {
    sol.u_data.get_item_optional_reload(module_uid, None)
}
fn get_optional_reload_override(sol: &SolarSystem, module_uid: UItemId) -> Option<OptionalReload> {
    get_u_module(sol, module_uid).get_optional_reload_override()
}
fn get_u_module(sol: &SolarSystem, module_uid: UItemId) -> &UModule {
    sol.u_data.items.get(module_uid).dc_module().unwrap()
}
