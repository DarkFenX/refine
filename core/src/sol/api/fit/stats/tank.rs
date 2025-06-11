use crate::sol::{AttrVal, DmgKinds, api::FitMut, svc::vast::StatTank};

impl<'a> FitMut<'a> {
    pub fn get_hp(&mut self) -> Option<StatTank<AttrVal>> {
        self.get_ship_mut().and_then(|mut v| v.get_hp())
    }
    pub fn get_resists(&mut self) -> Option<StatTank<DmgKinds<AttrVal>>> {
        self.get_ship_mut().and_then(|mut v| v.get_resists())
    }
}
