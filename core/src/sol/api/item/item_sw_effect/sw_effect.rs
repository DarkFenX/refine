use crate::{
    sol::{
        SolarSystem,
        api::{ItemCommon, ItemMutCommon, ItemMutSealed, ItemSealed},
    },
    uad::{UadItemKey, UadSwEffect},
};

pub struct SwEffect<'a> {
    pub(in crate::sol::api) sol: &'a SolarSystem,
    pub(in crate::sol::api) key: UadItemKey,
}
impl<'a> SwEffect<'a> {
    pub(in crate::sol::api) fn new(sol: &'a SolarSystem, key: UadItemKey) -> Self {
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
    fn get_key(&self) -> UadItemKey {
        self.key
    }
}
impl<'a> ItemCommon for SwEffect<'a> {}

pub struct SwEffectMut<'a> {
    pub(in crate::sol::api) sol: &'a mut SolarSystem,
    pub(in crate::sol::api) key: UadItemKey,
}
impl<'a> SwEffectMut<'a> {
    pub(in crate::sol::api) fn new(sol: &'a mut SolarSystem, key: UadItemKey) -> Self {
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
    fn get_key(&self) -> UadItemKey {
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

fn get_state(sol: &SolarSystem, item_key: UadItemKey) -> bool {
    get_uad_sw_effect(sol, item_key).get_sw_effect_state()
}
fn get_uad_sw_effect(sol: &SolarSystem, item_key: UadItemKey) -> &UadSwEffect {
    sol.uad.items.get(item_key).get_sw_effect().unwrap()
}
