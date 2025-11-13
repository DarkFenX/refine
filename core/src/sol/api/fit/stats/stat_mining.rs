use crate::{
    sol::api::FitMut,
    svc::vast::{StatMiningItemKinds, StatMiningKinds},
};

impl<'a> FitMut<'a> {
    pub fn get_stat_mps(&mut self, item_kinds: StatMiningItemKinds) -> StatMiningKinds {
        self.sol.svc.get_stat_fit_mps(&self.sol.u_data, self.key, item_kinds)
    }
}
