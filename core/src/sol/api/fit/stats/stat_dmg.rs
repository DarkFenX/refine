use crate::{
    def::AttrVal,
    misc::{DmgKinds, Spool},
    sol::api::FitMut,
};

impl<'a> FitMut<'a> {
    pub fn get_stat_dps(&mut self, reload: bool, spool: Option<Spool>) -> DmgKinds<AttrVal> {
        self.sol.svc.get_stat_fit_dps(&self.sol.u_data, self.key, reload, spool)
    }
    pub fn get_stat_volley(&mut self, spool: Option<Spool>) -> DmgKinds<AttrVal> {
        self.sol.svc.get_stat_fit_volley(&self.sol.u_data, self.key, spool)
    }
}
