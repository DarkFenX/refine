use crate::{
    misc::{Coordinates, MinionState, Movement, NpcProp},
    sol::{
        SolarSystem,
        api::{Fit, FitMut, ItemCommon, ItemMutCommon, ItemMutSealed, ItemSealed},
    },
    ud::{UDrone, UItemKey},
};

pub struct Drone<'a> {
    pub(in crate::sol::api) sol: &'a SolarSystem,
    pub(in crate::sol::api) key: UItemKey,
}
impl<'a> Drone<'a> {
    pub(in crate::sol::api) fn new(sol: &'a SolarSystem, key: UItemKey) -> Self {
        Self { sol, key }
    }
    pub fn get_fit(&self) -> Fit<'_> {
        get_fit(self.sol, self.key)
    }
    pub fn get_state(&self) -> MinionState {
        get_state(self.sol, self.key)
    }
    pub fn get_coordinates(&self) -> Coordinates {
        get_coordinates(self.sol, self.key)
    }
    pub fn get_movement(&self) -> Movement {
        get_movement(self.sol, self.key)
    }
    pub fn get_prop_mode(&self) -> NpcProp {
        get_prop_mode(self.sol, self.key)
    }
}
impl<'a> ItemSealed for Drone<'a> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_key(&self) -> UItemKey {
        self.key
    }
}
impl<'a> ItemCommon for Drone<'a> {}

pub struct DroneMut<'a> {
    pub(in crate::sol::api) sol: &'a mut SolarSystem,
    pub(in crate::sol::api) key: UItemKey,
}
impl<'a> DroneMut<'a> {
    pub(in crate::sol::api) fn new(sol: &'a mut SolarSystem, key: UItemKey) -> Self {
        Self { sol, key }
    }
    pub fn get_fit(&self) -> Fit<'_> {
        get_fit(self.sol, self.key)
    }
    pub fn get_fit_mut(&mut self) -> FitMut<'_> {
        let fit_key = get_u_drone(self.sol, self.key).get_fit_key();
        FitMut::new(self.sol, fit_key)
    }
    pub fn get_state(&self) -> MinionState {
        get_state(self.sol, self.key)
    }
    pub fn get_coordinates(&self) -> Coordinates {
        get_coordinates(self.sol, self.key)
    }
    pub fn get_movement(&self) -> Movement {
        get_movement(self.sol, self.key)
    }
    pub fn get_prop_mode(&self) -> NpcProp {
        get_prop_mode(self.sol, self.key)
    }
}
impl<'a> ItemSealed for DroneMut<'a> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_key(&self) -> UItemKey {
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

fn get_fit(sol: &SolarSystem, drone_key: UItemKey) -> Fit<'_> {
    let fit_key = get_u_drone(sol, drone_key).get_fit_key();
    Fit::new(sol, fit_key)
}
fn get_state(sol: &SolarSystem, drone_key: UItemKey) -> MinionState {
    get_u_drone(sol, drone_key).get_drone_state()
}
fn get_coordinates(sol: &SolarSystem, drone_key: UItemKey) -> Coordinates {
    get_u_drone(sol, drone_key).get_physics().coordinates.into()
}
fn get_movement(sol: &SolarSystem, drone_key: UItemKey) -> Movement {
    get_u_drone(sol, drone_key).get_physics().into()
}
fn get_prop_mode(sol: &SolarSystem, drone_key: UItemKey) -> NpcProp {
    get_u_drone(sol, drone_key).get_prop_mode().into()
}
fn get_u_drone(sol: &SolarSystem, drone_key: UItemKey) -> &UDrone {
    sol.u_data.items.get(drone_key).dc_drone().unwrap()
}
