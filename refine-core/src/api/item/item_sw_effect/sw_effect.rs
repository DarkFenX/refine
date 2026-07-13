use crate::{
    api::{ItemCommon, ItemMutCommon, ItemMutSealed, ItemSealed},
    sol::SolarSystem,
    ud::{UItemId, USwEffect},
};

pub struct SwEffect<'s> {
    pub(in crate::api) sol: &'s SolarSystem,
    pub(in crate::api) uid: UItemId,
}
impl<'s> SwEffect<'s> {
    pub(in crate::api) fn new(sol: &'s SolarSystem, uid: UItemId) -> Self {
        Self { sol, uid }
    }
    pub fn get_state(&self) -> bool {
        get_state(self.sol, self.uid)
    }
}
impl<'s> ItemSealed for SwEffect<'s> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_uid(&self) -> UItemId {
        self.uid
    }
}
impl<'s> ItemCommon for SwEffect<'s> {}

pub struct SwEffectMut<'s> {
    pub(in crate::api) sol: &'s mut SolarSystem,
    pub(in crate::api) uid: UItemId,
}
impl<'s> SwEffectMut<'s> {
    pub(in crate::api) fn new(sol: &'s mut SolarSystem, uid: UItemId) -> Self {
        Self { sol, uid }
    }
    pub fn get_state(&self) -> bool {
        get_state(self.sol, self.uid)
    }
}
impl<'s> ItemSealed for SwEffectMut<'s> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_uid(&self) -> UItemId {
        self.uid
    }
}
impl<'s> ItemMutSealed for SwEffectMut<'s> {
    fn get_sol_mut(&mut self) -> &mut SolarSystem {
        self.sol
    }
}
impl<'s> ItemCommon for SwEffectMut<'s> {}
impl<'s> ItemMutCommon for SwEffectMut<'s> {}

fn get_state(sol: &SolarSystem, sw_effect_uid: UItemId) -> bool {
    get_u_sw_effect(sol, sw_effect_uid).get_sw_effect_state()
}
fn get_u_sw_effect(sol: &SolarSystem, sw_effect_uid: UItemId) -> &USwEffect {
    sol.u_data.items.get(sw_effect_uid).dc_sw_effect().unwrap()
}
