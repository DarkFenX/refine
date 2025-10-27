use crate::{def::AttrVal, sol::api::FitMut};

impl<'a> FitMut<'a> {
    pub fn get_stat_remote_nps(&mut self) -> AttrVal {
        self.sol.svc.get_stat_fit_remote_nps(&self.sol.u_data, self.key)
    }
}
