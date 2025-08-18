use crate::{
    sol::{
        SolarSystem,
        api::{Fit, FitMut, ItemCommon, ItemMutCommon, ItemMutSealed, ItemSealed},
    },
    ud::{UFwEffect, UItemKey},
};

pub struct FwEffect<'a> {
    pub(in crate::sol::api) sol: &'a SolarSystem,
    pub(in crate::sol::api) key: UItemKey,
}
impl<'a> FwEffect<'a> {
    pub(in crate::sol::api) fn new(sol: &'a SolarSystem, key: UItemKey) -> Self {
        Self { sol, key }
    }
    pub fn get_fit(&self) -> Fit<'_> {
        get_fit(self.sol, self.key)
    }
    pub fn get_state(&self) -> bool {
        get_state(self.sol, self.key)
    }
}
impl<'a> ItemSealed for FwEffect<'a> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_key(&self) -> UItemKey {
        self.key
    }
}
impl<'a> ItemCommon for FwEffect<'a> {}

pub struct FwEffectMut<'a> {
    pub(in crate::sol::api) sol: &'a mut SolarSystem,
    pub(in crate::sol::api) key: UItemKey,
}
impl<'a> FwEffectMut<'a> {
    pub(in crate::sol::api) fn new(sol: &'a mut SolarSystem, key: UItemKey) -> Self {
        Self { sol, key }
    }
    pub fn get_fit(&self) -> Fit<'_> {
        get_fit(self.sol, self.key)
    }
    pub fn get_fit_mut(&mut self) -> FitMut<'_> {
        let fit_key = get_u_fw_effect(self.sol, self.key).get_fit_key();
        FitMut::new(self.sol, fit_key)
    }
    pub fn get_state(&self) -> bool {
        get_state(self.sol, self.key)
    }
}
impl<'a> ItemSealed for FwEffectMut<'a> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_key(&self) -> UItemKey {
        self.key
    }
}
impl<'a> ItemMutSealed for FwEffectMut<'a> {
    fn get_sol_mut(&mut self) -> &mut SolarSystem {
        self.sol
    }
}
impl<'a> ItemCommon for FwEffectMut<'a> {}
impl<'a> ItemMutCommon for FwEffectMut<'a> {}

fn get_fit(sol: &SolarSystem, fw_effect_key: UItemKey) -> Fit<'_> {
    let fit_key = get_u_fw_effect(sol, fw_effect_key).get_fit_key();
    Fit::new(sol, fit_key)
}
fn get_state(sol: &SolarSystem, fw_effect_key: UItemKey) -> bool {
    get_u_fw_effect(sol, fw_effect_key).get_fw_effect_state()
}
fn get_u_fw_effect(sol: &SolarSystem, fw_effect_key: UItemKey) -> &UFwEffect {
    sol.u_data.items.get(fw_effect_key).get_fw_effect().unwrap()
}
