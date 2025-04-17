use crate::sol::{
    ItemKey, SolarSystem,
    api::{Fit, FitMut, ItemCommon, ItemMutCommon, ItemMutSealed, ItemSealed},
    uad::item::UadStance,
};

pub struct Stance<'a> {
    pub(in crate::sol) sol: &'a SolarSystem,
    pub(in crate::sol) key: ItemKey,
}
impl<'a> Stance<'a> {
    pub(in crate::sol) fn new(sol: &'a SolarSystem, key: ItemKey) -> Self {
        Self { sol, key }
    }
    pub fn get_fit(&self) -> Fit {
        get_fit(self.sol, self.key)
    }
    pub fn get_state(&self) -> bool {
        get_state(self.sol, self.key)
    }
}
impl<'a> ItemSealed for Stance<'a> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_key(&self) -> ItemKey {
        self.key
    }
}
impl<'a> ItemCommon for Stance<'a> {}

pub struct StanceMut<'a> {
    pub(in crate::sol) sol: &'a mut SolarSystem,
    pub(in crate::sol) key: ItemKey,
}
impl<'a> StanceMut<'a> {
    pub(in crate::sol) fn new(sol: &'a mut SolarSystem, key: ItemKey) -> Self {
        Self { sol, key }
    }
    pub fn get_fit(&self) -> Fit {
        get_fit(self.sol, self.key)
    }
    pub fn get_fit_mut(&mut self) -> FitMut {
        let fit_key = get_uad_stance(self.sol, self.key).get_fit_key();
        FitMut::new(self.sol, fit_key)
    }
    pub fn get_state(&self) -> bool {
        get_state(self.sol, self.key)
    }
}
impl<'a> ItemSealed for StanceMut<'a> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_key(&self) -> ItemKey {
        self.key
    }
}
impl<'a> ItemMutSealed for StanceMut<'a> {
    fn get_sol_mut(&mut self) -> &mut SolarSystem {
        self.sol
    }
}
impl<'a> ItemCommon for StanceMut<'a> {}
impl<'a> ItemMutCommon for StanceMut<'a> {}

fn get_fit(sol: &SolarSystem, item_key: ItemKey) -> Fit {
    let fit_key = get_uad_stance(sol, item_key).get_fit_key();
    Fit::new(sol, fit_key)
}
fn get_state(sol: &SolarSystem, item_key: ItemKey) -> bool {
    get_uad_stance(sol, item_key).get_stance_state()
}
fn get_uad_stance(sol: &SolarSystem, item_key: ItemKey) -> &UadStance {
    sol.uad.items.get(item_key).get_stance().unwrap()
}
