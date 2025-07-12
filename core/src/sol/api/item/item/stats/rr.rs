use crate::{def::AttrVal, misc::Spool, sol::api::ItemMut};

impl<'a> ItemMut<'a> {
    pub fn get_stat_rr_shield(&mut self, spool: Option<Spool>, ignore_state: bool) -> Option<AttrVal> {
        match self {
            ItemMut::Drone(drone) => drone.get_stat_rr_shield(spool, ignore_state),
            ItemMut::Fighter(fighter) => fighter.get_stat_rr_shield(spool, ignore_state),
            ItemMut::Module(module) => module.get_stat_rr_shield(spool, ignore_state),
            _ => None,
        }
    }
}
