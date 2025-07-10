use crate::{def::AttrVal, misc::SpoolOptions, sol::api::FitMut};

impl<'a> FitMut<'a> {
    pub fn get_stat_rr_shield(&mut self) -> AttrVal {
        self.sol.svc.get_stat_fit_orr_shield(&self.sol.uad, self.key)
    }
    pub fn get_stat_rr_armor(&mut self, spool: Option<SpoolOptions>) -> AttrVal {
        self.sol.svc.get_stat_fit_orr_armor(&self.sol.uad, self.key, spool)
    }
    pub fn get_stat_rr_hull(&mut self) -> AttrVal {
        self.sol.svc.get_stat_fit_orr_hull(&self.sol.uad, self.key)
    }
    pub fn get_stat_rr_capacitor(&mut self) -> AttrVal {
        self.sol.svc.get_stat_fit_orr_cap(&self.sol.uad, self.key)
    }
}
