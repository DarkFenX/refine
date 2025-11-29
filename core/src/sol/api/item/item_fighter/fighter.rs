use crate::{
    misc::{AdjustableCount, Coordinates, MinionState, Movement},
    sol::{
        SolarSystem,
        api::{Fit, FitMut, ItemCommon, ItemMutCommon, ItemMutSealed, ItemSealed},
    },
    ud::{UFighter, UItemKey},
};

pub struct Fighter<'a> {
    pub(in crate::sol::api) sol: &'a SolarSystem,
    pub(in crate::sol::api) key: UItemKey,
}
impl<'a> Fighter<'a> {
    pub(in crate::sol::api) fn new(sol: &'a SolarSystem, key: UItemKey) -> Self {
        Self { sol, key }
    }
    pub fn get_fit(&self) -> Fit<'_> {
        get_fit(self.sol, self.key)
    }
    pub fn get_state(&self) -> MinionState {
        get_state(self.sol, self.key)
    }
    pub fn get_count(&self) -> Option<AdjustableCount> {
        get_count(self.sol, self.key)
    }
    pub fn get_coordinates(&self) -> Coordinates {
        get_coordinates(self.sol, self.key)
    }
    pub fn get_movement(&self) -> Movement {
        get_movement(self.sol, self.key)
    }
}
impl<'a> ItemSealed for Fighter<'a> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_key(&self) -> UItemKey {
        self.key
    }
}
impl<'a> ItemCommon for Fighter<'a> {}

pub struct FighterMut<'a> {
    pub(in crate::sol::api) sol: &'a mut SolarSystem,
    pub(in crate::sol::api) key: UItemKey,
}
impl<'a> FighterMut<'a> {
    pub(in crate::sol::api) fn new(sol: &'a mut SolarSystem, key: UItemKey) -> Self {
        Self { sol, key }
    }
    pub fn get_fit(&self) -> Fit<'_> {
        get_fit(self.sol, self.key)
    }
    pub fn get_fit_mut(&mut self) -> FitMut<'_> {
        let fit_key = get_u_fighter(self.sol, self.key).get_fit_key();
        FitMut::new(self.sol, fit_key)
    }
    pub fn get_state(&self) -> MinionState {
        get_state(self.sol, self.key)
    }
    pub fn get_count(&self) -> Option<AdjustableCount> {
        get_count(self.sol, self.key)
    }
    pub fn get_coordinates(&self) -> Coordinates {
        get_coordinates(self.sol, self.key)
    }
    pub fn get_movement(&self) -> Movement {
        get_movement(self.sol, self.key)
    }
}
impl<'a> ItemSealed for FighterMut<'a> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_key(&self) -> UItemKey {
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

fn get_fit(sol: &SolarSystem, fighter_key: UItemKey) -> Fit<'_> {
    let fit_key = get_u_fighter(sol, fighter_key).get_fit_key();
    Fit::new(sol, fit_key)
}
fn get_state(sol: &SolarSystem, fighter_key: UItemKey) -> MinionState {
    get_u_fighter(sol, fighter_key).get_fighter_state()
}
fn get_count(sol: &SolarSystem, fighter_key: UItemKey) -> Option<AdjustableCount> {
    get_u_fighter(sol, fighter_key).get_count()
}
fn get_coordinates(sol: &SolarSystem, fighter_key: UItemKey) -> Coordinates {
    get_u_fighter(sol, fighter_key).get_physics().coordinates.into()
}
fn get_movement(sol: &SolarSystem, fighter_key: UItemKey) -> Movement {
    get_u_fighter(sol, fighter_key).get_physics().into()
}
fn get_u_fighter(sol: &SolarSystem, fighter_key: UItemKey) -> &UFighter {
    sol.u_data.items.get(fighter_key).dc_fighter().unwrap()
}
