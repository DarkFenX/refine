use crate::{
    api::{ItemCommon, ItemMutCommon, ItemMutSealed, ItemSealed},
    sol::SolarSystem,
    ud::{UItemId, UProjEffect},
};

pub struct ProjEffect<'a> {
    pub(in crate::api) sol: &'a SolarSystem,
    pub(in crate::api) uid: UItemId,
}
impl<'a> ProjEffect<'a> {
    pub(in crate::api) fn new(sol: &'a SolarSystem, uid: UItemId) -> Self {
        Self { sol, uid }
    }
    pub fn get_state(&self) -> bool {
        get_state(self.sol, self.uid)
    }
}
impl<'a> ItemSealed for ProjEffect<'a> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_uid(&self) -> UItemId {
        self.uid
    }
}
impl<'a> ItemCommon for ProjEffect<'a> {}

pub struct ProjEffectMut<'a> {
    pub(in crate::api) sol: &'a mut SolarSystem,
    pub(in crate::api) uid: UItemId,
}
impl<'a> ProjEffectMut<'a> {
    pub(in crate::api) fn new(sol: &'a mut SolarSystem, uid: UItemId) -> Self {
        Self { sol, uid }
    }
    pub fn get_state(&self) -> bool {
        get_state(self.sol, self.uid)
    }
}
impl<'a> ItemSealed for ProjEffectMut<'a> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_uid(&self) -> UItemId {
        self.uid
    }
}
impl<'a> ItemMutSealed for ProjEffectMut<'a> {
    fn get_sol_mut(&mut self) -> &mut SolarSystem {
        self.sol
    }
}
impl<'a> ItemCommon for ProjEffectMut<'a> {}
impl<'a> ItemMutCommon for ProjEffectMut<'a> {}

fn get_state(sol: &SolarSystem, proj_effect_uid: UItemId) -> bool {
    get_u_proj_effect(sol, proj_effect_uid).get_proj_effect_state()
}
fn get_u_proj_effect(sol: &SolarSystem, proj_effect_uid: UItemId) -> &UProjEffect {
    sol.u_data.items.get(proj_effect_uid).dc_proj_effect().unwrap()
}
