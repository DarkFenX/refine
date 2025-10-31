use crate::{
    def::AttrVal,
    misc::Spool,
    sol::api::FitMut,
    svc::vast::{StatRemoteRepItemKinds, StatTank},
};

impl<'a> FitMut<'a> {
    pub fn get_stat_remote_rps(
        &mut self,
        item_kinds: StatRemoteRepItemKinds,
        spool: Option<Spool>,
    ) -> StatTank<AttrVal> {
        self.sol
            .svc
            .get_stat_fit_remote_rps(&self.sol.u_data, self.key, item_kinds, spool)
    }
    pub fn get_stat_remote_cps(&mut self) -> AttrVal {
        self.sol.svc.get_stat_fit_remote_cps(&self.sol.u_data, self.key)
    }
}
