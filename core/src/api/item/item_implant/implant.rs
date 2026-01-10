use crate::{
    api::{Fit, FitMut, ItemCommon, ItemMutCommon, ItemMutSealed, ItemSealed},
    num::SlotIndex,
    sol::SolarSystem,
    ud::{UImplant, UItemId},
};

pub struct Implant<'a> {
    pub(in crate::api) sol: &'a SolarSystem,
    pub(in crate::api) uid: UItemId,
}
impl<'a> Implant<'a> {
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
impl<'a> ItemSealed for Implant<'a> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_uid(&self) -> UItemId {
        self.uid
    }
}
impl<'a> ItemCommon for Implant<'a> {}

pub struct ImplantMut<'a> {
    pub(in crate::api) sol: &'a mut SolarSystem,
    pub(in crate::api) uid: UItemId,
}
impl<'a> ImplantMut<'a> {
    pub(in crate::api) fn new(sol: &'a mut SolarSystem, uid: UItemId) -> Self {
        Self { sol, uid }
    }
    pub fn get_fit(&self) -> Fit<'_> {
        get_fit(self.sol, self.uid)
    }
    pub fn get_fit_mut(&mut self) -> FitMut<'_> {
        let fit_uid = get_u_implant(self.sol, self.uid).get_fit_uid();
        FitMut::new(self.sol, fit_uid)
    }
    pub fn get_slot(&self) -> Option<SlotIndex> {
        get_slot(self.sol, self.uid)
    }
    pub fn get_state(&self) -> bool {
        get_state(self.sol, self.uid)
    }
}
impl<'a> ItemSealed for ImplantMut<'a> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_uid(&self) -> UItemId {
        self.uid
    }
}
impl<'a> ItemMutSealed for ImplantMut<'a> {
    fn get_sol_mut(&mut self) -> &mut SolarSystem {
        self.sol
    }
}
impl<'a> ItemCommon for ImplantMut<'a> {}
impl<'a> ItemMutCommon for ImplantMut<'a> {}

fn get_fit(sol: &SolarSystem, implant_uid: UItemId) -> Fit<'_> {
    let fit_uid = get_u_implant(sol, implant_uid).get_fit_uid();
    Fit::new(sol, fit_uid)
}
fn get_slot(sol: &SolarSystem, implant_uid: UItemId) -> Option<SlotIndex> {
    get_u_implant(sol, implant_uid).get_slot()
}
fn get_state(sol: &SolarSystem, implant_uid: UItemId) -> bool {
    get_u_implant(sol, implant_uid).get_implant_state()
}
fn get_u_implant(sol: &SolarSystem, implant_uid: UItemId) -> &UImplant {
    sol.u_data.items.get(implant_uid).dc_implant().unwrap()
}
