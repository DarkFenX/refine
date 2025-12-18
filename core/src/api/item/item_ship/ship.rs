use crate::{
    api::{Fit, FitMut, ItemCommon, ItemMutCommon, ItemMutSealed, ItemSealed},
    misc::{Coordinates, Movement},
    sol::SolarSystem,
    ud::{UItemKey, UShip},
};

pub struct Ship<'a> {
    pub(in crate::api) sol: &'a SolarSystem,
    pub(in crate::api) key: UItemKey,
}
impl<'a> Ship<'a> {
    pub(in crate::api) fn new(sol: &'a SolarSystem, key: UItemKey) -> Self {
        Self { sol, key }
    }
    pub fn get_fit(&self) -> Fit<'_> {
        get_fit(self.sol, self.key)
    }
    pub fn get_state(&self) -> bool {
        get_state(self.sol, self.key)
    }
    pub fn get_coordinates(&self) -> Coordinates {
        get_coordinates(self.sol, self.key)
    }
    pub fn get_movement(&self) -> Movement {
        get_movement(self.sol, self.key)
    }
}
impl<'a> ItemSealed for Ship<'a> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_key(&self) -> UItemKey {
        self.key
    }
}
impl<'a> ItemCommon for Ship<'a> {}

pub struct ShipMut<'a> {
    pub(in crate::api) sol: &'a mut SolarSystem,
    pub(in crate::api) key: UItemKey,
}
impl<'a> ShipMut<'a> {
    pub(in crate::api) fn new(sol: &'a mut SolarSystem, key: UItemKey) -> Self {
        Self { sol, key }
    }
    pub fn get_fit(&self) -> Fit<'_> {
        get_fit(self.sol, self.key)
    }
    pub fn get_fit_mut(&mut self) -> FitMut<'_> {
        let fit_key = get_u_ship(self.sol, self.key).get_fit_key();
        FitMut::new(self.sol, fit_key)
    }
    pub fn get_state(&self) -> bool {
        get_state(self.sol, self.key)
    }
    pub fn get_coordinates(&self) -> Coordinates {
        get_coordinates(self.sol, self.key)
    }
    pub fn get_movement(&self) -> Movement {
        get_movement(self.sol, self.key)
    }
}
impl<'a> ItemSealed for ShipMut<'a> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_key(&self) -> UItemKey {
        self.key
    }
}
impl<'a> ItemMutSealed for ShipMut<'a> {
    fn get_sol_mut(&mut self) -> &mut SolarSystem {
        self.sol
    }
}
impl<'a> ItemCommon for ShipMut<'a> {}
impl<'a> ItemMutCommon for ShipMut<'a> {}

fn get_fit(sol: &SolarSystem, ship_key: UItemKey) -> Fit<'_> {
    let fit_key = get_u_ship(sol, ship_key).get_fit_key();
    Fit::new(sol, fit_key)
}
fn get_state(sol: &SolarSystem, ship_key: UItemKey) -> bool {
    get_u_ship(sol, ship_key).get_ship_state()
}
fn get_coordinates(sol: &SolarSystem, ship_key: UItemKey) -> Coordinates {
    get_u_ship(sol, ship_key).get_physics().coordinates.into()
}
fn get_movement(sol: &SolarSystem, ship_key: UItemKey) -> Movement {
    get_u_ship(sol, ship_key).get_physics().into()
}
fn get_u_ship(sol: &SolarSystem, ship_key: UItemKey) -> &UShip {
    sol.u_data.items.get(ship_key).dc_ship().unwrap()
}
