use crate::sol::{
    AdjustableCount, ItemKey, SolarSystem,
    api::{Fit, FitMut, ItemCommon, ItemMutCommon, ItemMutSealed, ItemSealed},
    uad::item::{MinionState, UadFighter},
};

pub struct Fighter<'a> {
    pub(in crate::sol::api) sol: &'a SolarSystem,
    pub(in crate::sol::api) key: ItemKey,
}
impl<'a> Fighter<'a> {
    pub(in crate::sol::api) fn new(sol: &'a SolarSystem, key: ItemKey) -> Self {
        Self { sol, key }
    }
    pub fn get_fit(&self) -> Fit {
        get_fit(self.sol, self.key)
    }
    pub fn get_state(&self) -> MinionState {
        get_state(self.sol, self.key)
    }
    pub fn get_count(&self) -> Option<AdjustableCount> {
        get_count(self.sol, self.key)
    }
}
impl<'a> ItemSealed for Fighter<'a> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_key(&self) -> ItemKey {
        self.key
    }
}
impl<'a> ItemCommon for Fighter<'a> {}

pub struct FighterMut<'a> {
    pub(in crate::sol::api) sol: &'a mut SolarSystem,
    pub(in crate::sol::api) key: ItemKey,
}
impl<'a> FighterMut<'a> {
    pub(in crate::sol::api) fn new(sol: &'a mut SolarSystem, key: ItemKey) -> Self {
        Self { sol, key }
    }
    pub fn get_fit(&self) -> Fit {
        get_fit(self.sol, self.key)
    }
    pub fn get_fit_mut(&mut self) -> FitMut {
        let fit_key = get_uad_fighter(self.sol, self.key).get_fit_key();
        FitMut::new(self.sol, fit_key)
    }
    pub fn get_state(&self) -> MinionState {
        get_state(self.sol, self.key)
    }
    pub fn get_count(&self) -> Option<AdjustableCount> {
        get_count(self.sol, self.key)
    }
}
impl<'a> ItemSealed for FighterMut<'a> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_key(&self) -> ItemKey {
        self.key
    }
}
impl<'a> ItemMutSealed for FighterMut<'a> {
    fn get_sol_mut(&mut self) -> &mut SolarSystem {
        self.sol
    }
}
impl<'a> ItemCommon for FighterMut<'a> {}
impl<'a> ItemMutCommon for FighterMut<'a> {}

fn get_fit(sol: &SolarSystem, item_key: ItemKey) -> Fit {
    let fit_key = get_uad_fighter(sol, item_key).get_fit_key();
    Fit::new(sol, fit_key)
}
fn get_state(sol: &SolarSystem, item_key: ItemKey) -> MinionState {
    get_uad_fighter(sol, item_key).get_fighter_state()
}
fn get_count(sol: &SolarSystem, item_key: ItemKey) -> Option<AdjustableCount> {
    get_uad_fighter(sol, item_key).get_count()
}
fn get_uad_fighter(sol: &SolarSystem, item_key: ItemKey) -> &UadFighter {
    sol.uad.items.get(item_key).get_fighter().unwrap()
}
