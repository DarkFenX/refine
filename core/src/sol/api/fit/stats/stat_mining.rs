use crate::{misc::MiningKinds, sol::api::FitMut, svc::vast::StatMiningItemKinds};

impl<'a> FitMut<'a> {
    pub fn get_stat_mps(&mut self, item_kinds: StatMiningItemKinds) -> MiningKinds {
        self.sol.svc.get_stat_fit_mps(&self.sol.u_data, self.key, item_kinds)
    }
}
