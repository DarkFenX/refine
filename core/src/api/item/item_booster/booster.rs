use crate::{
    api::{Fit, FitMut, ItemCommon, ItemMutCommon, ItemMutSealed, ItemSealed},
    def::SlotIndex,
    sol::SolarSystem,
    ud::{UBooster, UItemId},
};

pub struct Booster<'a> {
    pub(in crate::api) sol: &'a SolarSystem,
    pub(in crate::api) key: UItemId,
}
impl<'a> Booster<'a> {
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
impl<'a> ItemSealed for Booster<'a> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_key(&self) -> UItemId {
        self.key
    }
}
impl<'a> ItemCommon for Booster<'a> {}

pub struct BoosterMut<'a> {
    pub(in crate::api) sol: &'a mut SolarSystem,
    pub(in crate::api) key: UItemId,
}
impl<'a> BoosterMut<'a> {
    pub(in crate::api) fn new(sol: &'a mut SolarSystem, key: UItemId) -> Self {
        Self { sol, key }
    }
    pub fn get_fit(&self) -> Fit<'_> {
        get_fit(self.sol, self.key)
    }
    pub fn get_fit_mut(&mut self) -> FitMut<'_> {
        let fit_key = get_u_booster(self.sol, self.key).get_fit_uid();
        FitMut::new(self.sol, fit_key)
    }
    pub fn get_slot(&self) -> Option<SlotIndex> {
        get_slot(self.sol, self.key)
    }
    pub fn get_state(&self) -> bool {
        get_state(self.sol, self.key)
    }
}
impl<'a> ItemSealed for BoosterMut<'a> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_key(&self) -> UItemId {
        self.key
    }
}
impl<'a> ItemMutSealed for BoosterMut<'a> {
    fn get_sol_mut(&mut self) -> &mut SolarSystem {
        self.sol
    }
}
impl<'a> ItemCommon for BoosterMut<'a> {}
impl<'a> ItemMutCommon for BoosterMut<'a> {}

fn get_fit(sol: &SolarSystem, booster_key: UItemId) -> Fit<'_> {
    let fit_key = get_u_booster(sol, booster_key).get_fit_uid();
    Fit::new(sol, fit_key)
}
fn get_slot(sol: &SolarSystem, booster_key: UItemId) -> Option<SlotIndex> {
    get_u_booster(sol, booster_key).get_slot()
}
fn get_state(sol: &SolarSystem, booster_key: UItemId) -> bool {
    get_u_booster(sol, booster_key).get_booster_state()
}
fn get_u_booster(sol: &SolarSystem, booster_key: UItemId) -> &UBooster {
    sol.u_data.items.get(booster_key).dc_booster().unwrap()
}
