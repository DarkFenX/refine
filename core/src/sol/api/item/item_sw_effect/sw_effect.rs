use crate::{
    sol::{
        SolarSystem,
        api::{ItemCommon, ItemMutCommon, ItemMutSealed, ItemSealed},
    },
    ud::{UItemKey, USwEffect},
};

pub struct SwEffect<'a> {
    pub(in crate::sol::api) sol: &'a SolarSystem,
    pub(in crate::sol::api) key: UItemKey,
}
impl<'a> SwEffect<'a> {
    pub(in crate::sol::api) fn new(sol: &'a SolarSystem, key: UItemKey) -> Self {
        Self { sol, key }
    }
    pub fn get_state(&self) -> bool {
        get_state(self.sol, self.key)
    }
}
impl<'a> ItemSealed for SwEffect<'a> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_key(&self) -> UItemKey {
        self.key
    }
}
impl<'a> ItemCommon for SwEffect<'a> {}

pub struct SwEffectMut<'a> {
    pub(in crate::sol::api) sol: &'a mut SolarSystem,
    pub(in crate::sol::api) key: UItemKey,
}
impl<'a> SwEffectMut<'a> {
    pub(in crate::sol::api) fn new(sol: &'a mut SolarSystem, key: UItemKey) -> Self {
        Self { sol, key }
    }
    pub fn get_state(&self) -> bool {
        get_state(self.sol, self.key)
    }
}
impl<'a> ItemSealed for SwEffectMut<'a> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_key(&self) -> UItemKey {
        self.key
    }
}
impl<'a> ItemMutSealed for SwEffectMut<'a> {
    fn get_sol_mut(&mut self) -> &mut SolarSystem {
        self.sol
    }
}
impl<'a> ItemCommon for SwEffectMut<'a> {}
impl<'a> ItemMutCommon for SwEffectMut<'a> {}

fn get_state(sol: &SolarSystem, item_key: UItemKey) -> bool {
    get_u_sw_effect(sol, item_key).get_sw_effect_state()
}
fn get_u_sw_effect(sol: &SolarSystem, item_key: UItemKey) -> &USwEffect {
    sol.u_data.items.get(item_key).get_sw_effect().unwrap()
}
