use crate::{def::AttrVal, misc::Spool, sol::api::FitMut, svc::vast::StatTank};

impl<'a> FitMut<'a> {
    pub fn get_stat_remote_rps(&mut self, spool: Option<Spool>) -> StatTank<AttrVal> {
        self.sol.svc.get_stat_fit_remote_rps(&self.sol.uad, self.key, spool)
    }
    pub fn get_stat_remote_cps(&mut self) -> AttrVal {
        self.sol.svc.get_stat_fit_remote_cps(&self.sol.uad, self.key)
    }
}
