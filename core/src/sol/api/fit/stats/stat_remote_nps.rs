use crate::{def::AttrVal, sol::api::FitMut, svc::vast::StatRemoteNpsItemKinds};

impl<'a> FitMut<'a> {
    pub fn get_stat_remote_nps(&mut self, item_kinds: StatRemoteNpsItemKinds) -> AttrVal {
        self.sol
            .svc
            .get_stat_fit_remote_nps(&self.sol.u_data, self.key, item_kinds)
    }
}
