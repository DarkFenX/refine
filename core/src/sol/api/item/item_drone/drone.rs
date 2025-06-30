use crate::{
    def::ItemKey,
    misc::MinionState,
    sol::{
        SolarSystem,
        api::{Fit, FitMut, ItemCommon, ItemMutCommon, ItemMutSealed, ItemSealed},
    },
    uad::UadDrone,
};

pub struct Drone<'a> {
    pub(in crate::sol::api) sol: &'a SolarSystem,
    pub(in crate::sol::api) key: ItemKey,
}
impl<'a> Drone<'a> {
    pub(in crate::sol::api) fn new(sol: &'a SolarSystem, key: ItemKey) -> Self {
        Self { sol, key }
    }
    pub fn get_fit(&self) -> Fit<'_> {
        get_fit(self.sol, self.key)
    }
    pub fn get_state(&self) -> MinionState {
        get_state(self.sol, self.key)
    }
}
impl<'a> ItemSealed for Drone<'a> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_key(&self) -> ItemKey {
        self.key
    }
}
impl<'a> ItemCommon for Drone<'a> {}

pub struct DroneMut<'a> {
    pub(in crate::sol::api) sol: &'a mut SolarSystem,
    pub(in crate::sol::api) key: ItemKey,
}
impl<'a> DroneMut<'a> {
    pub(in crate::sol::api) fn new(sol: &'a mut SolarSystem, key: ItemKey) -> Self {
        Self { sol, key }
    }
    pub fn get_fit(&self) -> Fit<'_> {
        get_fit(self.sol, self.key)
    }
    pub fn get_fit_mut(&mut self) -> FitMut<'_> {
        let fit_key = get_uad_drone(self.sol, self.key).get_fit_key();
        FitMut::new(self.sol, fit_key)
    }
    pub fn get_state(&self) -> MinionState {
        get_state(self.sol, self.key)
    }
}
impl<'a> ItemSealed for DroneMut<'a> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_key(&self) -> ItemKey {
        self.key
    }
}
impl<'a> ItemMutSealed for DroneMut<'a> {
    fn get_sol_mut(&mut self) -> &mut SolarSystem {
        self.sol
    }
}
impl<'a> ItemCommon for DroneMut<'a> {}
impl<'a> ItemMutCommon for DroneMut<'a> {}

fn get_fit(sol: &SolarSystem, item_key: ItemKey) -> Fit<'_> {
    let fit_key = get_uad_drone(sol, item_key).get_fit_key();
    Fit::new(sol, fit_key)
}
fn get_state(sol: &SolarSystem, item_key: ItemKey) -> MinionState {
    get_uad_drone(sol, item_key).get_drone_state()
}
fn get_uad_drone(sol: &SolarSystem, item_key: ItemKey) -> &UadDrone {
    sol.uad.items.get(item_key).get_drone().unwrap()
}
