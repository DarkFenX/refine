use crate::{
    def::SlotIndex,
    sol::{
        SolarSystem,
        api::{Fit, FitMut, ItemCommon, ItemMutCommon, ItemMutSealed, ItemSealed},
    },
    ud::{UItemKey, USubsystem},
};

pub struct Subsystem<'a> {
    pub(in crate::sol::api) sol: &'a SolarSystem,
    pub(in crate::sol::api) key: UItemKey,
}
impl<'a> Subsystem<'a> {
    pub(in crate::sol::api) fn new(sol: &'a SolarSystem, key: UItemKey) -> Self {
        Self { sol, key }
    }
    pub fn get_fit(&self) -> Fit<'_> {
        get_fit(self.sol, self.key)
    }
    pub fn get_slot(&self) -> Option<SlotIndex> {
        get_slot(self.sol, self.key)
    }
    pub fn get_state(&self) -> bool {
        get_state(self.sol, self.key)
    }
}
impl<'a> ItemSealed for Subsystem<'a> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_key(&self) -> UItemKey {
        self.key
    }
}
impl<'a> ItemCommon for Subsystem<'a> {}

pub struct SubsystemMut<'a> {
    pub(in crate::sol::api) sol: &'a mut SolarSystem,
    pub(in crate::sol::api) key: UItemKey,
}
impl<'a> SubsystemMut<'a> {
    pub(in crate::sol::api) fn new(sol: &'a mut SolarSystem, key: UItemKey) -> Self {
        Self { sol, key }
    }
    pub fn get_fit(&self) -> Fit<'_> {
        get_fit(self.sol, self.key)
    }
    pub fn get_fit_mut(&mut self) -> FitMut<'_> {
        let fit_key = get_u_subsystem(self.sol, self.key).get_fit_key();
        FitMut::new(self.sol, fit_key)
    }
    pub fn get_slot(&self) -> Option<SlotIndex> {
        get_slot(self.sol, self.key)
    }
    pub fn get_state(&self) -> bool {
        get_state(self.sol, self.key)
    }
}
impl<'a> ItemSealed for SubsystemMut<'a> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_key(&self) -> UItemKey {
        self.key
    }
}
impl<'a> ItemMutSealed for SubsystemMut<'a> {
    fn get_sol_mut(&mut self) -> &mut SolarSystem {
        self.sol
    }
}
impl<'a> ItemCommon for SubsystemMut<'a> {}
impl<'a> ItemMutCommon for SubsystemMut<'a> {}

fn get_fit(sol: &SolarSystem, subsystem_key: UItemKey) -> Fit<'_> {
    let fit_key = get_u_subsystem(sol, subsystem_key).get_fit_key();
    Fit::new(sol, fit_key)
}
fn get_slot(sol: &SolarSystem, subsystem_key: UItemKey) -> Option<SlotIndex> {
    get_u_subsystem(sol, subsystem_key).get_slot()
}
fn get_state(sol: &SolarSystem, subsystem_key: UItemKey) -> bool {
    get_u_subsystem(sol, subsystem_key).get_subsystem_state()
}
fn get_u_subsystem(sol: &SolarSystem, subsystem_key: UItemKey) -> &USubsystem {
    sol.u_data.items.get(subsystem_key).get_subsystem().unwrap()
}
