use crate::{
    sol::api::FitMut,
    svc::vast::{StatMining, StatMiningItemKinds},
};

impl<'a> FitMut<'a> {
    pub fn get_stat_mps(&mut self, item_kinds: StatMiningItemKinds, reload: bool) -> StatMining {
        self.sol
            .svc
            .get_stat_fit_mps(&self.sol.u_data, self.key, item_kinds, reload)
    }
}
