use crate::{FitInfoMode, FleetInfoMode, ItemInfoMode, SolInfo, SolInfoMode, SolarSystem};

impl SolarSystem<'_> {
    pub async fn get_info(
        &mut self,
        sol_mode: SolInfoMode,
        fleet_mode: FleetInfoMode,
        fit_mode: FitInfoMode,
        item_mode: ItemInfoMode,
    ) -> SolInfo {
        // Variables for move
        let sol_id = self.get_id();
        let src_alias = self.get_src_alias();
        self.exec_standard_safe(move |core_sol| {
            SolInfo::from_ids_and_core(sol_id, src_alias, core_sol, sol_mode, fleet_mode, fit_mode, item_mode)
        })
        .await
    }
}
