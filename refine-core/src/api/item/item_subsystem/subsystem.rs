use crate::{
    api::{Fit, FitMut, ItemCommon, ItemMutCommon, ItemMutSealed, ItemSealed},
    num::SlotIndex,
    sol::SolarSystem,
    ud::{UItemId, USubsystem},
};

pub struct Subsystem<'a> {
    pub(in crate::api) sol: &'a SolarSystem,
    pub(in crate::api) uid: UItemId,
}
impl<'a> Subsystem<'a> {
    pub(in crate::api) fn new(sol: &'a SolarSystem, uid: UItemId) -> Self {
        Self { sol, uid }
    }
    pub fn get_fit(&self) -> Fit<'_> {
        get_fit(self.sol, self.uid)
    }
    pub fn get_slot(&self) -> Option<SlotIndex> {
        get_slot(self.sol, self.uid)
    }
    pub fn get_state(&self) -> bool {
        get_state(self.sol, self.uid)
    }
}
impl<'a> ItemSealed for Subsystem<'a> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_uid(&self) -> UItemId {
        self.uid
    }
}
impl<'a> ItemCommon for Subsystem<'a> {}

pub struct SubsystemMut<'a> {
    pub(in crate::api) sol: &'a mut SolarSystem,
    pub(in crate::api) uid: UItemId,
}
impl<'a> SubsystemMut<'a> {
    pub(in crate::api) fn new(sol: &'a mut SolarSystem, uid: UItemId) -> Self {
        Self { sol, uid }
    }
    pub fn get_fit(&self) -> Fit<'_> {
        get_fit(self.sol, self.uid)
    }
    pub fn get_fit_mut(&mut self) -> FitMut<'_> {
        let fit_uid = get_u_subsystem(self.sol, self.uid).get_fit_uid();
        FitMut::new(self.sol, fit_uid)
    }
    pub fn get_slot(&self) -> Option<SlotIndex> {
        get_slot(self.sol, self.uid)
    }
    pub fn get_state(&self) -> bool {
        get_state(self.sol, self.uid)
    }
}
impl<'a> ItemSealed for SubsystemMut<'a> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_uid(&self) -> UItemId {
        self.uid
    }
}
impl<'a> ItemMutSealed for SubsystemMut<'a> {
    fn get_sol_mut(&mut self) -> &mut SolarSystem {
        self.sol
    }
}
impl<'a> ItemCommon for SubsystemMut<'a> {}
impl<'a> ItemMutCommon for SubsystemMut<'a> {}

fn get_fit(sol: &SolarSystem, subsystem_uid: UItemId) -> Fit<'_> {
    let fit_uid = get_u_subsystem(sol, subsystem_uid).get_fit_uid();
    Fit::new(sol, fit_uid)
}
fn get_slot(sol: &SolarSystem, subsystem_uid: UItemId) -> Option<SlotIndex> {
    get_u_subsystem(sol, subsystem_uid).get_slot()
}
fn get_state(sol: &SolarSystem, subsystem_uid: UItemId) -> bool {
    get_u_subsystem(sol, subsystem_uid).get_subsystem_state()
}
fn get_u_subsystem(sol: &SolarSystem, subsystem_uid: UItemId) -> &USubsystem {
    sol.u_data.items.get(subsystem_uid).dc_subsystem().unwrap()
}
