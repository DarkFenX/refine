use crate::{misc::Mining, sol::api::FitMut, svc::vast::StatMiningItemKinds};

impl<'a> FitMut<'a> {
    pub fn get_stat_mps_ore(&mut self, item_kinds: StatMiningItemKinds) -> Mining {
        self.sol
            .svc
            .get_stat_fit_mps_ore(&self.sol.u_data, self.key, item_kinds)
    }
    pub fn get_stat_mps_ice(&mut self, item_kinds: StatMiningItemKinds) -> Mining {
        self.sol
            .svc
            .get_stat_fit_mps_ice(&self.sol.u_data, self.key, item_kinds)
    }
    pub fn get_stat_mps_gas(&mut self, item_kinds: StatMiningItemKinds) -> Mining {
        self.sol
            .svc
            .get_stat_fit_mps_gas(&self.sol.u_data, self.key, item_kinds)
    }
}
