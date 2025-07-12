use crate::{def::AttrVal, sol::api::ItemMut};

impl<'a> ItemMut<'a> {
    pub fn get_stat_speed(&mut self) -> Option<AttrVal> {
        match self {
            ItemMut::Drone(drone) => drone.get_stat_speed(),
            ItemMut::Fighter(fighter) => fighter.get_stat_speed(),
            ItemMut::Ship(ship) => ship.get_stat_speed(),
            _ => None,
        }
    }
    pub fn get_stat_agility(&mut self) -> Option<AttrVal> {
        match self {
            ItemMut::Drone(drone) => drone.get_stat_agility(),
            ItemMut::Fighter(fighter) => fighter.get_stat_agility(),
            ItemMut::Ship(ship) => ship.get_stat_agility(),
            _ => None,
        }
    }
    pub fn get_stat_align_time(&mut self) -> Option<AttrVal> {
        match self {
            ItemMut::Drone(drone) => drone.get_stat_align_time(),
            ItemMut::Fighter(fighter) => fighter.get_stat_align_time(),
            ItemMut::Ship(ship) => ship.get_stat_align_time(),
            _ => None,
        }
    }
}
