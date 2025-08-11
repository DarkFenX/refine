use crate::{
    misc::Spool,
    sol::api::FitMut,
    svc::vast::{StatDmg, StatDmgItemKinds},
};

impl<'a> FitMut<'a> {
    pub fn get_stat_dps(&mut self, item_kinds: StatDmgItemKinds, reload: bool, spool: Option<Spool>) -> StatDmg {
        self.sol
            .svc
            .get_stat_fit_dps(&self.sol.u_data, self.key, item_kinds, reload, spool)
    }
    pub fn get_stat_volley(&mut self, item_kinds: StatDmgItemKinds, spool: Option<Spool>) -> StatDmg {
        self.sol
            .svc
            .get_stat_fit_volley(&self.sol.u_data, self.key, item_kinds, spool)
    }
}
