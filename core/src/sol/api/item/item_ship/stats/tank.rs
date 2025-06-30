use crate::{
    def::AttrVal,
    misc::DmgKinds,
    sol::api::ShipMut,
    svc::vast::{StatLayerHp, StatTank},
};

impl<'a> ShipMut<'a> {
    pub fn get_stat_hp(&mut self) -> Option<StatTank<StatLayerHp>> {
        self.sol.svc.get_stat_item_hp(&self.sol.uad, self.key)
    }
    pub fn get_stat_resists(&mut self) -> Option<StatTank<DmgKinds<AttrVal>>> {
        self.sol.svc.get_stat_item_resists(&self.sol.uad, self.key)
    }
}
