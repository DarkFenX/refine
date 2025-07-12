use crate::{
    def::AttrVal,
    misc::{DmgKinds, DpsProfile},
    sol::api::ItemMut,
    svc::vast::{StatLayerEhp, StatLayerHp, StatTank},
};

impl<'a> ItemMut<'a> {
    pub fn get_stat_hp(&mut self) -> Option<StatTank<StatLayerHp>> {
        match self {
            ItemMut::Drone(drone) => drone.get_stat_hp(),
            ItemMut::Fighter(fighter) => fighter.get_stat_hp(),
            ItemMut::Ship(ship) => ship.get_stat_hp(),
            _ => None,
        }
    }
    pub fn get_stat_ehp(&mut self, incoming_dps: Option<&DpsProfile>) -> Option<StatTank<StatLayerEhp>> {
        match self {
            ItemMut::Drone(drone) => drone.get_stat_ehp(incoming_dps),
            ItemMut::Fighter(fighter) => fighter.get_stat_ehp(incoming_dps),
            ItemMut::Ship(ship) => ship.get_stat_ehp(incoming_dps),
            _ => None,
        }
    }
    pub fn get_stat_wc_ehp(&mut self) -> Option<StatTank<StatLayerEhp>> {
        match self {
            ItemMut::Drone(drone) => drone.get_stat_wc_ehp(),
            ItemMut::Fighter(fighter) => fighter.get_stat_wc_ehp(),
            ItemMut::Ship(ship) => ship.get_stat_wc_ehp(),
            _ => None,
        }
    }
    pub fn get_stat_resists(&mut self) -> Option<StatTank<DmgKinds<AttrVal>>> {
        match self {
            ItemMut::Drone(drone) => drone.get_stat_resists(),
            ItemMut::Fighter(fighter) => fighter.get_stat_resists(),
            ItemMut::Ship(ship) => ship.get_stat_resists(),
            _ => None,
        }
    }
}
