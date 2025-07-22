use crate::{
    sol::{
        SolarSystem,
        api::{ItemCommon, ItemMutCommon, ItemMutSealed, ItemSealed},
    },
    uad::{UadItemKey, UadProjEffect},
};

pub struct ProjEffect<'a> {
    pub(in crate::sol::api) sol: &'a SolarSystem,
    pub(in crate::sol::api) key: UadItemKey,
}
impl<'a> ProjEffect<'a> {
    pub(in crate::sol::api) fn new(sol: &'a SolarSystem, key: UadItemKey) -> Self {
        Self { sol, key }
    }
    pub fn get_state(&self) -> bool {
        get_state(self.sol, self.key)
    }
}
impl<'a> ItemSealed for ProjEffect<'a> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_key(&self) -> UadItemKey {
        self.key
    }
}
impl<'a> ItemCommon for ProjEffect<'a> {}

pub struct ProjEffectMut<'a> {
    pub(in crate::sol::api) sol: &'a mut SolarSystem,
    pub(in crate::sol::api) key: UadItemKey,
}
impl<'a> ProjEffectMut<'a> {
    pub(in crate::sol::api) fn new(sol: &'a mut SolarSystem, key: UadItemKey) -> Self {
        Self { sol, key }
    }
    pub fn get_state(&self) -> bool {
        get_state(self.sol, self.key)
    }
}
impl<'a> ItemSealed for ProjEffectMut<'a> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_key(&self) -> UadItemKey {
        self.key
    }
}
impl<'a> ItemMutSealed for ProjEffectMut<'a> {
    fn get_sol_mut(&mut self) -> &mut SolarSystem {
        self.sol
    }
}
impl<'a> ItemCommon for ProjEffectMut<'a> {}
impl<'a> ItemMutCommon for ProjEffectMut<'a> {}

fn get_state(sol: &SolarSystem, item_key: UadItemKey) -> bool {
    get_uad_proj_effect(sol, item_key).get_proj_effect_state()
}
fn get_uad_proj_effect(sol: &SolarSystem, item_key: UadItemKey) -> &UadProjEffect {
    sol.uad.items.get(item_key).get_proj_effect().unwrap()
}
