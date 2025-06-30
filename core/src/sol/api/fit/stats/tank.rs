use crate::{
    def::AttrVal,
    misc::DmgKinds,
    sol::api::FitMut,
    svc::vast::{StatLayerHp, StatTank},
};

impl<'a> FitMut<'a> {
    pub fn get_stat_hp(&mut self) -> Option<StatTank<StatLayerHp>> {
        self.get_ship_mut().and_then(|mut v| v.get_stat_hp())
    }
    pub fn get_stat_resists(&mut self) -> Option<StatTank<DmgKinds<AttrVal>>> {
        self.get_ship_mut().and_then(|mut v| v.get_stat_resists())
    }
}
