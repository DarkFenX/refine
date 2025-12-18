use crate::{
    api::{Fit, FitMut, ItemCommon, ItemMutCommon, ItemMutSealed, ItemSealed},
    def::SlotIndex,
    sol::SolarSystem,
    ud::{UImplant, UItemKey},
};

pub struct Implant<'a> {
    pub(in crate::api) sol: &'a SolarSystem,
    pub(in crate::api) key: UItemKey,
}
impl<'a> Implant<'a> {
    pub(in crate::api) fn new(sol: &'a SolarSystem, key: UItemKey) -> Self {
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
impl<'a> ItemSealed for Implant<'a> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_key(&self) -> UItemKey {
        self.key
    }
}
impl<'a> ItemCommon for Implant<'a> {}

pub struct ImplantMut<'a> {
    pub(in crate::api) sol: &'a mut SolarSystem,
    pub(in crate::api) key: UItemKey,
}
impl<'a> ImplantMut<'a> {
    pub(in crate::api) fn new(sol: &'a mut SolarSystem, key: UItemKey) -> Self {
        Self { sol, key }
    }
    pub fn get_fit(&self) -> Fit<'_> {
        get_fit(self.sol, self.key)
    }
    pub fn get_fit_mut(&mut self) -> FitMut<'_> {
        let fit_key = get_u_implant(self.sol, self.key).get_fit_key();
        FitMut::new(self.sol, fit_key)
    }
    pub fn get_slot(&self) -> Option<SlotIndex> {
        get_slot(self.sol, self.key)
    }
    pub fn get_state(&self) -> bool {
        get_state(self.sol, self.key)
    }
}
impl<'a> ItemSealed for ImplantMut<'a> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_key(&self) -> UItemKey {
        self.key
    }
}
impl<'a> ItemMutSealed for ImplantMut<'a> {
    fn get_sol_mut(&mut self) -> &mut SolarSystem {
        self.sol
    }
}
impl<'a> ItemCommon for ImplantMut<'a> {}
impl<'a> ItemMutCommon for ImplantMut<'a> {}

fn get_fit(sol: &SolarSystem, implant_key: UItemKey) -> Fit<'_> {
    let fit_key = get_u_implant(sol, implant_key).get_fit_key();
    Fit::new(sol, fit_key)
}
fn get_slot(sol: &SolarSystem, implant_key: UItemKey) -> Option<SlotIndex> {
    get_u_implant(sol, implant_key).get_slot()
}
fn get_state(sol: &SolarSystem, implant_key: UItemKey) -> bool {
    get_u_implant(sol, implant_key).get_implant_state()
}
fn get_u_implant(sol: &SolarSystem, implant_key: UItemKey) -> &UImplant {
    sol.u_data.items.get(implant_key).dc_implant().unwrap()
}
