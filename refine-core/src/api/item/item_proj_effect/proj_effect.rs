use crate::{
    api::{ItemCommon, ItemMutCommon, ItemMutSealed, ItemSealed},
    sol::SolarSystem,
    ud::{UItemId, UProjEffect},
};

pub struct ProjEffect<'s> {
    pub(in crate::api) sol: &'s SolarSystem,
    pub(in crate::api) uid: UItemId,
}
impl<'s> ProjEffect<'s> {
    pub(in crate::api) fn new(sol: &'s SolarSystem, uid: UItemId) -> Self {
        Self { sol, uid }
    }
    pub fn get_state(&self) -> bool {
        get_state(self.sol, self.uid)
    }
}
impl<'s> ItemSealed for ProjEffect<'s> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_uid(&self) -> UItemId {
        self.uid
    }
}
impl<'s> ItemCommon for ProjEffect<'s> {}

pub struct ProjEffectMut<'s> {
    pub(in crate::api) sol: &'s mut SolarSystem,
    pub(in crate::api) uid: UItemId,
}
impl<'s> ProjEffectMut<'s> {
    pub(in crate::api) fn new(sol: &'s mut SolarSystem, uid: UItemId) -> Self {
        Self { sol, uid }
    }
    pub fn get_state(&self) -> bool {
        get_state(self.sol, self.uid)
    }
}
impl<'s> ItemSealed for ProjEffectMut<'s> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_uid(&self) -> UItemId {
        self.uid
    }
}
impl<'s> ItemMutSealed for ProjEffectMut<'s> {
    fn get_sol_mut(&mut self) -> &mut SolarSystem {
        self.sol
    }
}
impl<'s> ItemCommon for ProjEffectMut<'s> {}
impl<'s> ItemMutCommon for ProjEffectMut<'s> {}

fn get_state(sol: &SolarSystem, proj_effect_uid: UItemId) -> bool {
    get_u_proj_effect(sol, proj_effect_uid).get_proj_effect_state()
}
fn get_u_proj_effect(sol: &SolarSystem, proj_effect_uid: UItemId) -> &UProjEffect {
    sol.u_data.items.get(proj_effect_uid).dc_proj_effect().unwrap()
}
