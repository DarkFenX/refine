use crate::sol::{AttrVal, DmgKinds, api::FitMut, svc::vast::StatTank};

impl<'a> FitMut<'a> {
    pub fn get_hp(&mut self) -> Option<StatTank<AttrVal>> {
        self.get_ship_mut().and_then(|mut v| v.get_hp())
    }
    pub fn get_resists(&mut self) -> Option<StatTank<DmgKinds<AttrVal>>> {
        self.get_ship_mut().and_then(|mut v| v.get_resists())
    }
    pub fn get_shield_resists(&mut self) -> Option<DmgKinds<AttrVal>> {
        self.get_ship_mut().and_then(|mut v| v.get_shield_resists())
    }
    pub fn get_armor_resists(&mut self) -> Option<DmgKinds<AttrVal>> {
        self.get_ship_mut().and_then(|mut v| v.get_armor_resists())
    }
    pub fn get_structure_resists(&mut self) -> Option<DmgKinds<AttrVal>> {
        self.get_ship_mut().and_then(|mut v| v.get_structure_resists())
    }
}
