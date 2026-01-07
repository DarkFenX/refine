use crate::{
    api::{
        Adjustable, Coordinates, Fit, FitMut, ItemCommon, ItemMutCommon, ItemMutSealed, ItemSealed, MinionState,
        Movement,
    },
    sol::SolarSystem,
    ud::{UFighter, UItemId},
};

pub struct Fighter<'a> {
    pub(in crate::api) sol: &'a SolarSystem,
    pub(in crate::api) key: UItemId,
}
impl<'a> Fighter<'a> {
    pub(in crate::api) fn new(sol: &'a SolarSystem, key: UItemId) -> Self {
        Self { sol, key }
    }
    pub fn get_fit(&self) -> Fit<'_> {
        get_fit(self.sol, self.key)
    }
    pub fn get_state(&self) -> MinionState {
        get_state(self.sol, self.key)
    }
    pub fn get_count(&self) -> Option<Adjustable> {
        get_count(self.sol, self.key)
    }
    pub fn get_coordinates(&self) -> Coordinates {
        get_coordinates(self.sol, self.key)
    }
    pub fn get_movement(&self) -> Movement {
        get_movement(self.sol, self.key)
    }
    pub fn get_rearm_minions(&self) -> Option<bool> {
        get_rearm_minions(self.sol, self.key)
    }
}
impl<'a> ItemSealed for Fighter<'a> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_key(&self) -> UItemId {
        self.key
    }
}
impl<'a> ItemCommon for Fighter<'a> {}

pub struct FighterMut<'a> {
    pub(in crate::api) sol: &'a mut SolarSystem,
    pub(in crate::api) key: UItemId,
}
impl<'a> FighterMut<'a> {
    pub(in crate::api) fn new(sol: &'a mut SolarSystem, key: UItemId) -> Self {
        Self { sol, key }
    }
    pub fn get_fit(&self) -> Fit<'_> {
        get_fit(self.sol, self.key)
    }
    pub fn get_fit_mut(&mut self) -> FitMut<'_> {
        let fit_key = get_u_fighter(self.sol, self.key).get_fit_uid();
        FitMut::new(self.sol, fit_key)
    }
    pub fn get_state(&self) -> MinionState {
        get_state(self.sol, self.key)
    }
    pub fn get_count(&self) -> Option<Adjustable> {
        get_count(self.sol, self.key)
    }
    pub fn get_coordinates(&self) -> Coordinates {
        get_coordinates(self.sol, self.key)
    }
    pub fn get_movement(&self) -> Movement {
        get_movement(self.sol, self.key)
    }
    pub fn get_rearm_minions(&self) -> Option<bool> {
        get_rearm_minions(self.sol, self.key)
    }
}
impl<'a> ItemSealed for FighterMut<'a> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_key(&self) -> UItemId {
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

fn get_fit(sol: &SolarSystem, fighter_key: UItemId) -> Fit<'_> {
    let fit_key = get_u_fighter(sol, fighter_key).get_fit_uid();
    Fit::new(sol, fit_key)
}
fn get_state(sol: &SolarSystem, fighter_key: UItemId) -> MinionState {
    get_u_fighter(sol, fighter_key).get_fighter_state()
}
fn get_count(sol: &SolarSystem, fighter_key: UItemId) -> Option<Adjustable> {
    get_u_fighter(sol, fighter_key).get_count()
}
fn get_coordinates(sol: &SolarSystem, fighter_key: UItemId) -> Coordinates {
    get_u_fighter(sol, fighter_key).get_physics().coordinates.into()
}
fn get_movement(sol: &SolarSystem, fighter_key: UItemId) -> Movement {
    get_u_fighter(sol, fighter_key).get_physics().into()
}
fn get_rearm_minions(sol: &SolarSystem, fighter_key: UItemId) -> Option<bool> {
    get_u_fighter(sol, fighter_key).get_rearm_minions()
}
fn get_u_fighter(sol: &SolarSystem, fighter_key: UItemId) -> &UFighter {
    sol.u_data.items.get(fighter_key).dc_fighter().unwrap()
}
