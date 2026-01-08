use crate::{
    api::{Coordinates, Fit, FitMut, ItemCommon, ItemMutCommon, ItemMutSealed, ItemSealed, Movement},
    sol::SolarSystem,
    ud::{UItemId, UShip},
};

pub struct Ship<'a> {
    pub(in crate::api) sol: &'a SolarSystem,
    pub(in crate::api) uid: UItemId,
}
impl<'a> Ship<'a> {
    pub(in crate::api) fn new(sol: &'a SolarSystem, uid: UItemId) -> Self {
        Self { sol, uid }
    }
    pub fn get_fit(&self) -> Fit<'_> {
        get_fit(self.sol, self.uid)
    }
    pub fn get_state(&self) -> bool {
        get_state(self.sol, self.uid)
    }
    pub fn get_coordinates(&self) -> Coordinates {
        get_coordinates(self.sol, self.uid)
    }
    pub fn get_movement(&self) -> Movement {
        get_movement(self.sol, self.uid)
    }
}
impl<'a> ItemSealed for Ship<'a> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_uid(&self) -> UItemId {
        self.uid
    }
}
impl<'a> ItemCommon for Ship<'a> {}

pub struct ShipMut<'a> {
    pub(in crate::api) sol: &'a mut SolarSystem,
    pub(in crate::api) uid: UItemId,
}
impl<'a> ShipMut<'a> {
    pub(in crate::api) fn new(sol: &'a mut SolarSystem, uid: UItemId) -> Self {
        Self { sol, uid }
    }
    pub fn get_fit(&self) -> Fit<'_> {
        get_fit(self.sol, self.uid)
    }
    pub fn get_fit_mut(&mut self) -> FitMut<'_> {
        let fit_uid = get_u_ship(self.sol, self.uid).get_fit_uid();
        FitMut::new(self.sol, fit_uid)
    }
    pub fn get_state(&self) -> bool {
        get_state(self.sol, self.uid)
    }
    pub fn get_coordinates(&self) -> Coordinates {
        get_coordinates(self.sol, self.uid)
    }
    pub fn get_movement(&self) -> Movement {
        get_movement(self.sol, self.uid)
    }
}
impl<'a> ItemSealed for ShipMut<'a> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_uid(&self) -> UItemId {
        self.uid
    }
}
impl<'a> ItemMutSealed for ShipMut<'a> {
    fn get_sol_mut(&mut self) -> &mut SolarSystem {
        self.sol
    }
}
impl<'a> ItemCommon for ShipMut<'a> {}
impl<'a> ItemMutCommon for ShipMut<'a> {}

fn get_fit(sol: &SolarSystem, ship_uid: UItemId) -> Fit<'_> {
    let fit_uid = get_u_ship(sol, ship_uid).get_fit_uid();
    Fit::new(sol, fit_uid)
}
fn get_state(sol: &SolarSystem, ship_uid: UItemId) -> bool {
    get_u_ship(sol, ship_uid).get_ship_state()
}
fn get_coordinates(sol: &SolarSystem, ship_uid: UItemId) -> Coordinates {
    Coordinates::from_xyz(get_u_ship(sol, ship_uid).get_physics().coordinates)
}
fn get_movement(sol: &SolarSystem, ship_uid: UItemId) -> Movement {
    Movement::from_u_physics(get_u_ship(sol, ship_uid).get_physics())
}
fn get_u_ship(sol: &SolarSystem, ship_uid: UItemId) -> &UShip {
    sol.u_data.items.get(ship_uid).dc_ship().unwrap()
}
