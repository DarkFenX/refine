use crate::{FitInfoMode, FleetInfoMode, ItemInfoMode, SolInfo, SolInfoMode, SolarSystem};

impl SolarSystem<'_> {
    pub async fn get_info(
        &mut self,
        sol_mode: SolInfoMode,
        fleet_mode: FleetInfoMode,
        fit_mode: FitInfoMode,
        item_mode: ItemInfoMode,
    ) -> SolInfo {
        let sol_id = self.id;
        self.exec_standard_safe(move |core_sol| {
            SolInfo::from_id_and_core(sol_id, core_sol, sol_mode, fleet_mode, fit_mode, item_mode)
        })
        .await
    }
}
