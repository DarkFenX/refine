use crate::{
    def::AttrVal,
    misc::Spool,
    sol::api::FleetMut,
    svc::vast::{StatOutRepItemKinds, StatTank},
};

impl<'a> FleetMut<'a> {
    pub fn get_stat_outgoing_rps(
        &mut self,
        item_kinds: StatOutRepItemKinds,
        spool: Option<Spool>,
    ) -> StatTank<AttrVal> {
        let u_fleet = self.sol.u_data.fleets.get(self.key);
        self.sol
            .svc
            .get_stat_fits_outgoing_rps(&self.sol.u_data, u_fleet.iter_fits(), item_kinds, spool)
    }
    pub fn get_stat_outgoing_cps(&mut self) -> AttrVal {
        let u_fleet = self.sol.u_data.fleets.get(self.key);
        self.sol
            .svc
            .get_stat_fits_outgoing_cps(&self.sol.u_data, u_fleet.iter_fits())
    }
}
