use crate::{
    def::AttrVal,
    misc::Spool,
    sol::api::FleetMut,
    svc::vast::{StatRemoteRepItemKinds, StatTank},
};

impl<'a> FleetMut<'a> {
    pub fn get_stat_remote_rps(
        &mut self,
        item_kinds: StatRemoteRepItemKinds,
        spool: Option<Spool>,
    ) -> StatTank<AttrVal> {
        let u_fleet = self.sol.u_data.fleets.get(self.key);
        self.sol
            .svc
            .get_stat_fits_remote_rps(&self.sol.u_data, u_fleet.iter_fits(), item_kinds, spool)
    }
    pub fn get_stat_remote_cps(&mut self) -> AttrVal {
        let u_fleet = self.sol.u_data.fleets.get(self.key);
        self.sol
            .svc
            .get_stat_fits_remote_cps(&self.sol.u_data, u_fleet.iter_fits())
    }
}
