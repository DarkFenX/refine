use crate::{
    def::AttrVal,
    misc::Spool,
    sol::api::FitMut,
    svc::vast::{StatOutRepItemKinds, StatTank},
};

impl<'a> FitMut<'a> {
    pub fn get_stat_outgoing_rps(
        &mut self,
        item_kinds: StatOutRepItemKinds,
        spool: Option<Spool>,
    ) -> StatTank<AttrVal> {
        self.sol
            .svc
            .get_stat_fit_outgoing_rps(&self.sol.u_data, self.key, item_kinds, spool)
    }
    pub fn get_stat_outgoing_cps(&mut self) -> AttrVal {
        self.sol.svc.get_stat_fit_outgoing_cps(&self.sol.u_data, self.key)
    }
}
