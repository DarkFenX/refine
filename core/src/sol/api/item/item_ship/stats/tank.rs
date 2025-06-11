use crate::sol::{
    AttrVal, DmgKinds,
    api::ShipMut,
    svc::vast::{StatTank, Vast},
};

impl<'a> ShipMut<'a> {
    pub fn get_hp(&mut self) -> Option<StatTank<AttrVal>> {
        Vast::get_item_hp(&self.sol.uad, &mut self.sol.svc.calc, self.key)
    }
    pub fn get_resists(&mut self) -> Option<StatTank<DmgKinds<AttrVal>>> {
        Vast::get_item_resists(&self.sol.uad, &mut self.sol.svc.calc, self.key)
    }
    pub fn get_shield_resists(&mut self) -> Option<DmgKinds<AttrVal>> {
        Vast::get_item_shield_resists(&self.sol.uad, &mut self.sol.svc.calc, self.key)
    }
    pub fn get_armor_resists(&mut self) -> Option<DmgKinds<AttrVal>> {
        Vast::get_item_armor_resists(&self.sol.uad, &mut self.sol.svc.calc, self.key)
    }
    pub fn get_structure_resists(&mut self) -> Option<DmgKinds<AttrVal>> {
        Vast::get_item_structure_resists(&self.sol.uad, &mut self.sol.svc.calc, self.key)
    }
}
