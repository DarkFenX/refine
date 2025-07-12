use crate::{def::AttrVal, sol::api::ItemMut};

impl<'a> ItemMut<'a> {
    pub fn get_speed(&mut self) -> Option<AttrVal> {
        match self {
            ItemMut::Drone(drone) => drone.get_speed(),
            ItemMut::Fighter(fighter) => fighter.get_speed(),
            ItemMut::Ship(ship) => ship.get_speed(),
            _ => None,
        }
    }
    pub fn get_agility(&mut self) -> Option<AttrVal> {
        match self {
            ItemMut::Drone(drone) => drone.get_agility(),
            ItemMut::Fighter(fighter) => fighter.get_agility(),
            ItemMut::Ship(ship) => ship.get_agility(),
            _ => None,
        }
    }
    pub fn get_align_time(&mut self) -> Option<AttrVal> {
        match self {
            ItemMut::Drone(drone) => drone.get_align_time(),
            ItemMut::Fighter(fighter) => fighter.get_align_time(),
            ItemMut::Ship(ship) => ship.get_align_time(),
            _ => None,
        }
    }
}
