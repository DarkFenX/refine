use crate::{
    api::{Fit, FitMut, ItemCommon, ItemMutCommon, ItemMutSealed, ItemSealed},
    def::SlotIndex,
    sol::SolarSystem,
    ud::{UItemId, USubsystem},
};

pub struct Subsystem<'a> {
    pub(in crate::api) sol: &'a SolarSystem,
    pub(in crate::api) key: UItemId,
}
impl<'a> Subsystem<'a> {
    pub(in crate::api) fn new(sol: &'a SolarSystem, key: UItemId) -> Self {
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
    fn get_key(&self) -> UItemId {
        self.key
    }
}
impl<'a> ItemCommon for Subsystem<'a> {}

pub struct SubsystemMut<'a> {
    pub(in crate::api) sol: &'a mut SolarSystem,
    pub(in crate::api) key: UItemId,
}
impl<'a> SubsystemMut<'a> {
    pub(in crate::api) fn new(sol: &'a mut SolarSystem, key: UItemId) -> Self {
        Self { sol, key }
    }
    pub fn get_fit(&self) -> Fit<'_> {
        get_fit(self.sol, self.key)
    }
    pub fn get_fit_mut(&mut self) -> FitMut<'_> {
        let fit_key = get_u_subsystem(self.sol, self.key).get_fit_uid();
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
    fn get_key(&self) -> UItemId {
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

fn get_fit(sol: &SolarSystem, subsystem_key: UItemId) -> Fit<'_> {
    let fit_key = get_u_subsystem(sol, subsystem_key).get_fit_uid();
    Fit::new(sol, fit_key)
}
fn get_slot(sol: &SolarSystem, subsystem_key: UItemId) -> Option<SlotIndex> {
    get_u_subsystem(sol, subsystem_key).get_slot()
}
fn get_state(sol: &SolarSystem, subsystem_key: UItemId) -> bool {
    get_u_subsystem(sol, subsystem_key).get_subsystem_state()
}
fn get_u_subsystem(sol: &SolarSystem, subsystem_key: UItemId) -> &USubsystem {
    sol.u_data.items.get(subsystem_key).dc_subsystem().unwrap()
}
