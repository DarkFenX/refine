use crate::{
    api::{Fit, FitMut, ItemCommon, ItemMutCommon, ItemMutSealed, ItemSealed},
    num::SlotIndex,
    sol::SolarSystem,
    ud::{UBooster, UItemId},
};

pub struct Booster<'s> {
    pub(in crate::api) sol: &'s SolarSystem,
    pub(in crate::api) uid: UItemId,
}
impl<'s> Booster<'s> {
    pub(in crate::api) fn new(sol: &'s SolarSystem, uid: UItemId) -> Self {
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
impl<'s> ItemSealed for Booster<'s> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_uid(&self) -> UItemId {
        self.uid
    }
}
impl<'s> ItemCommon for Booster<'s> {}

pub struct BoosterMut<'s> {
    pub(in crate::api) sol: &'s mut SolarSystem,
    pub(in crate::api) uid: UItemId,
}
impl<'s> BoosterMut<'s> {
    pub(in crate::api) fn new(sol: &'s mut SolarSystem, uid: UItemId) -> Self {
        Self { sol, uid }
    }
    pub fn get_fit(&self) -> Fit<'_> {
        get_fit(self.sol, self.uid)
    }
    pub fn get_fit_mut(&mut self) -> FitMut<'_> {
        let fit_uid = get_u_booster(self.sol, self.uid).get_fit_uid();
        FitMut::new(self.sol, fit_uid)
    }
    pub fn get_slot(&self) -> Option<SlotIndex> {
        get_slot(self.sol, self.uid)
    }
    pub fn get_state(&self) -> bool {
        get_state(self.sol, self.uid)
    }
}
impl<'s> ItemSealed for BoosterMut<'s> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_uid(&self) -> UItemId {
        self.uid
    }
}
impl<'s> ItemMutSealed for BoosterMut<'s> {
    fn get_sol_mut(&mut self) -> &mut SolarSystem {
        self.sol
    }
}
impl<'s> ItemCommon for BoosterMut<'s> {}
impl<'s> ItemMutCommon for BoosterMut<'s> {}

fn get_fit(sol: &SolarSystem, booster_uid: UItemId) -> Fit<'_> {
    let fit_uid = get_u_booster(sol, booster_uid).get_fit_uid();
    Fit::new(sol, fit_uid)
}
fn get_slot(sol: &SolarSystem, booster_uid: UItemId) -> Option<SlotIndex> {
    get_u_booster(sol, booster_uid).get_slot()
}
fn get_state(sol: &SolarSystem, booster_uid: UItemId) -> bool {
    get_u_booster(sol, booster_uid).get_booster_state()
}
fn get_u_booster(sol: &SolarSystem, booster_uid: UItemId) -> &UBooster {
    sol.u_data.items.get(booster_uid).dc_booster().unwrap()
}
