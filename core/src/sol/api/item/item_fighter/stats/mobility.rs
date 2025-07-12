use crate::{def::AttrVal, sol::api::FighterMut};

impl<'a> FighterMut<'a> {
    pub fn get_stat_speed(&mut self) -> Option<AttrVal> {
        self.sol.svc.get_stat_item_speed(&self.sol.uad, self.key)
    }
    pub fn get_stat_agility(&mut self) -> Option<AttrVal> {
        self.sol.svc.get_stat_item_agility(&self.sol.uad, self.key)
    }
    pub fn get_stat_align_time(&mut self) -> Option<AttrVal> {
        self.sol.svc.get_stat_item_align_time(&self.sol.uad, self.key)
    }
}
