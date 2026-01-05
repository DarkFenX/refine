use crate::{
    api::{Coordinates, Fit, FitMut, ItemCommon, ItemMutCommon, ItemMutSealed, ItemSealed, Movement},
    sol::SolarSystem,
    ud::{UItemId, UShip},
};

pub struct Ship<'a> {
    pub(in crate::api) sol: &'a SolarSystem,
    pub(in crate::api) key: UItemId,
}
impl<'a> Ship<'a> {
    pub(in crate::api) fn new(sol: &'a SolarSystem, key: UItemId) -> Self {
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
    fn get_key(&self) -> UItemId {
        self.key
    }
}
impl<'a> ItemCommon for Ship<'a> {}

pub struct ShipMut<'a> {
    pub(in crate::api) sol: &'a mut SolarSystem,
    pub(in crate::api) key: UItemId,
}
impl<'a> ShipMut<'a> {
    pub(in crate::api) fn new(sol: &'a mut SolarSystem, key: UItemId) -> Self {
        Self { sol, key }
    }
    pub fn get_fit(&self) -> Fit<'_> {
        get_fit(self.sol, self.key)
    }
    pub fn get_fit_mut(&mut self) -> FitMut<'_> {
        let fit_key = get_u_ship(self.sol, self.key).get_fit_uid();
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
    fn get_key(&self) -> UItemId {
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

fn get_fit(sol: &SolarSystem, ship_key: UItemId) -> Fit<'_> {
    let fit_key = get_u_ship(sol, ship_key).get_fit_uid();
    Fit::new(sol, fit_key)
}
fn get_state(sol: &SolarSystem, ship_key: UItemId) -> bool {
    get_u_ship(sol, ship_key).get_ship_state()
}
fn get_coordinates(sol: &SolarSystem, ship_key: UItemId) -> Coordinates {
    get_u_ship(sol, ship_key).get_physics().coordinates.into()
}
fn get_movement(sol: &SolarSystem, ship_key: UItemId) -> Movement {
    get_u_ship(sol, ship_key).get_physics().into()
}
fn get_u_ship(sol: &SolarSystem, ship_key: UItemId) -> &UShip {
    sol.u_data.items.get(ship_key).dc_ship().unwrap()
}
