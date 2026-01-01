use crate::{
    api::FleetMut,
    def::AttrVal,
    svc::vast::{StatOutRepItemKinds, StatTank, StatTimeOptions},
};

impl<'a> FleetMut<'a> {
    pub fn get_stat_outgoing_rps(
        &mut self,
        item_kinds: StatOutRepItemKinds,
        time_options: StatTimeOptions,
    ) -> StatTank<AttrVal> {
        let u_fleet = self.sol.u_data.fleets.get(self.key);
        self.sol
            .svc
            .get_stat_fits_outgoing_rps(&self.sol.u_data, u_fleet.iter_fits(), item_kinds, time_options)
    }
    pub fn get_stat_outgoing_cps(&mut self, time_options: StatTimeOptions) -> AttrVal {
        let u_fleet = self.sol.u_data.fleets.get(self.key);
        self.sol
            .svc
            .get_stat_fits_outgoing_cps(&self.sol.u_data, u_fleet.iter_fits(), time_options)
    }
}
