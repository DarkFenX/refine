use crate::sol::{AttrVal, api::ShipMut, svc::vast::Vast};

impl<'a> ShipMut<'a> {
    pub fn get_speed(&mut self) -> Option<AttrVal> {
        Vast::get_item_speed(&self.sol.uad, &mut self.sol.svc.calc, self.key)
    }
    pub fn get_agility_factor(&mut self) -> Option<AttrVal> {
        Vast::get_item_agility_factor(&self.sol.uad, &mut self.sol.svc.calc, self.key)
    }
    pub fn get_align_time(&mut self) -> Option<AttrVal> {
        Vast::get_align_time(&self.sol.uad, &mut self.sol.svc.calc, self.key)
    }
}
