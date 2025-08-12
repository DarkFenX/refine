use crate::{
    misc::Spool,
    sol::api::FleetMut,
    svc::vast::{StatDmg, StatDmgItemKinds},
};

impl<'a> FleetMut<'a> {
    pub fn get_stat_dps(&mut self, item_kinds: StatDmgItemKinds, reload: bool, spool: Option<Spool>) -> StatDmg {
        let u_fleet = self.sol.u_data.fleets.get(self.key);
        self.sol
            .svc
            .get_stat_fits_dps(&self.sol.u_data, u_fleet.iter_fits(), item_kinds, reload, spool)
    }
    pub fn get_stat_volley(&mut self, item_kinds: StatDmgItemKinds, spool: Option<Spool>) -> StatDmg {
        let u_fleet = self.sol.u_data.fleets.get(self.key);
        self.sol
            .svc
            .get_stat_fits_volley(&self.sol.u_data, u_fleet.iter_fits(), item_kinds, spool)
    }
}
