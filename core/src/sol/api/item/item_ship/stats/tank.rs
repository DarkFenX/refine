use crate::{
    def::AttrVal,
    misc::{DmgKinds, DpsProfile},
    sol::api::ShipMut,
    svc::vast::{StatLayerEhp, StatLayerHp, StatTank},
};

impl<'a> ShipMut<'a> {
    pub fn get_stat_hp(&mut self) -> Option<StatTank<StatLayerHp>> {
        self.sol.svc.get_stat_item_hp(&self.sol.uad, self.key)
    }
    pub fn get_stat_ehp(&mut self, incoming_dps: Option<&DpsProfile>) -> Option<StatTank<StatLayerEhp>> {
        self.sol.svc.get_stat_item_ehp(&self.sol.uad, self.key, incoming_dps)
    }
    pub fn get_stat_wc_ehp(&mut self) -> Option<StatTank<StatLayerEhp>> {
        self.sol.svc.get_stat_item_wc_ehp(&self.sol.uad, self.key)
    }
    pub fn get_stat_resists(&mut self) -> Option<StatTank<DmgKinds<AttrVal>>> {
        self.sol.svc.get_stat_item_resists(&self.sol.uad, self.key)
    }
}
