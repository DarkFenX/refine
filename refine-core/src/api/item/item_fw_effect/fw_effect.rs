use crate::{
    api::{Fit, FitMut, ItemCommon, ItemMutCommon, ItemMutSealed, ItemSealed},
    sol::SolarSystem,
    ud::{UFwEffect, UItemId},
};

pub struct FwEffect<'a> {
    pub(in crate::api) sol: &'a SolarSystem,
    pub(in crate::api) uid: UItemId,
}
impl<'a> FwEffect<'a> {
    pub(in crate::api) fn new(sol: &'a SolarSystem, uid: UItemId) -> Self {
        Self { sol, uid }
    }
    pub fn get_fit(&self) -> Fit<'_> {
        get_fit(self.sol, self.uid)
    }
    pub fn get_state(&self) -> bool {
        get_state(self.sol, self.uid)
    }
}
impl<'a> ItemSealed for FwEffect<'a> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_uid(&self) -> UItemId {
        self.uid
    }
}
impl<'a> ItemCommon for FwEffect<'a> {}

pub struct FwEffectMut<'a> {
    pub(in crate::api) sol: &'a mut SolarSystem,
    pub(in crate::api) uid: UItemId,
}
impl<'a> FwEffectMut<'a> {
    pub(in crate::api) fn new(sol: &'a mut SolarSystem, uid: UItemId) -> Self {
        Self { sol, uid }
    }
    pub fn get_fit(&self) -> Fit<'_> {
        get_fit(self.sol, self.uid)
    }
    pub fn get_fit_mut(&mut self) -> FitMut<'_> {
        let fit_uid = get_u_fw_effect(self.sol, self.uid).get_fit_uid();
        FitMut::new(self.sol, fit_uid)
    }
    pub fn get_state(&self) -> bool {
        get_state(self.sol, self.uid)
    }
}
impl<'a> ItemSealed for FwEffectMut<'a> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_uid(&self) -> UItemId {
        self.uid
    }
}
impl<'a> ItemMutSealed for FwEffectMut<'a> {
    fn get_sol_mut(&mut self) -> &mut SolarSystem {
        self.sol
    }
}
impl<'a> ItemCommon for FwEffectMut<'a> {}
impl<'a> ItemMutCommon for FwEffectMut<'a> {}

fn get_fit(sol: &SolarSystem, fw_effect_uid: UItemId) -> Fit<'_> {
    let fit_uid = get_u_fw_effect(sol, fw_effect_uid).get_fit_uid();
    Fit::new(sol, fit_uid)
}
fn get_state(sol: &SolarSystem, fw_effect_uid: UItemId) -> bool {
    get_u_fw_effect(sol, fw_effect_uid).get_fw_effect_state()
}
fn get_u_fw_effect(sol: &SolarSystem, fw_effect_uid: UItemId) -> &UFwEffect {
    sol.u_data.items.get(fw_effect_uid).dc_fw_effect().unwrap()
}
