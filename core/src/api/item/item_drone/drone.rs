use crate::{
    api::{Coordinates, Fit, FitMut, ItemCommon, ItemMutCommon, ItemMutSealed, ItemSealed, MinionState, Movement},
    misc::{NpcProp, StOption},
    sol::SolarSystem,
    ud::{UDrone, UItemId},
};

pub struct Drone<'a> {
    pub(in crate::api) sol: &'a SolarSystem,
    pub(in crate::api) uid: UItemId,
}
impl<'a> Drone<'a> {
    pub(in crate::api) fn new(sol: &'a SolarSystem, uid: UItemId) -> Self {
        Self { sol, uid }
    }
    pub fn get_fit(&self) -> Fit<'_> {
        get_fit(self.sol, self.uid)
    }
    pub fn get_state(&self) -> MinionState {
        get_state(self.sol, self.uid)
    }
    pub fn get_coordinates(&self) -> Coordinates {
        get_coordinates(self.sol, self.uid)
    }
    pub fn get_movement(&self) -> Movement {
        get_movement(self.sol, self.uid)
    }
    pub fn get_npc_prop(&self) -> StOption<NpcProp> {
        get_npc_prop(self.sol, self.uid)
    }
}
impl<'a> ItemSealed for Drone<'a> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_uid(&self) -> UItemId {
        self.uid
    }
}
impl<'a> ItemCommon for Drone<'a> {}

pub struct DroneMut<'a> {
    pub(in crate::api) sol: &'a mut SolarSystem,
    pub(in crate::api) uid: UItemId,
}
impl<'a> DroneMut<'a> {
    pub(in crate::api) fn new(sol: &'a mut SolarSystem, uid: UItemId) -> Self {
        Self { sol, uid }
    }
    pub fn get_fit(&self) -> Fit<'_> {
        get_fit(self.sol, self.uid)
    }
    pub fn get_fit_mut(&mut self) -> FitMut<'_> {
        let fit_uid = get_u_drone(self.sol, self.uid).get_fit_uid();
        FitMut::new(self.sol, fit_uid)
    }
    pub fn get_state(&self) -> MinionState {
        get_state(self.sol, self.uid)
    }
    pub fn get_coordinates(&self) -> Coordinates {
        get_coordinates(self.sol, self.uid)
    }
    pub fn get_movement(&self) -> Movement {
        get_movement(self.sol, self.uid)
    }
    pub fn get_npc_prop(&self) -> StOption<NpcProp> {
        get_npc_prop(self.sol, self.uid)
    }
}
impl<'a> ItemSealed for DroneMut<'a> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_uid(&self) -> UItemId {
        self.uid
    }
}
impl<'a> ItemMutSealed for DroneMut<'a> {
    fn get_sol_mut(&mut self) -> &mut SolarSystem {
        self.sol
    }
}
impl<'a> ItemCommon for DroneMut<'a> {}
impl<'a> ItemMutCommon for DroneMut<'a> {}

fn get_fit(sol: &SolarSystem, drone_uid: UItemId) -> Fit<'_> {
    let fit_uid = get_u_drone(sol, drone_uid).get_fit_uid();
    Fit::new(sol, fit_uid)
}
fn get_state(sol: &SolarSystem, drone_uid: UItemId) -> MinionState {
    get_u_drone(sol, drone_uid).get_drone_state()
}
fn get_coordinates(sol: &SolarSystem, drone_uid: UItemId) -> Coordinates {
    Coordinates::from_xyz(get_u_drone(sol, drone_uid).get_physics().coordinates)
}
fn get_movement(sol: &SolarSystem, drone_uid: UItemId) -> Movement {
    Movement::from_u_physics(get_u_drone(sol, drone_uid).get_physics())
}
fn get_npc_prop(sol: &SolarSystem, drone_uid: UItemId) -> StOption<NpcProp> {
    get_u_drone(sol, drone_uid).get_npc_prop()
}
fn get_u_drone(sol: &SolarSystem, drone_uid: UItemId) -> &UDrone {
    sol.u_data.items.get(drone_uid).dc_drone().unwrap()
}
