use crate::{Fleet, FleetInfo, FleetInfoMode};

impl Fleet<'_, '_> {
    pub async fn get_info(&mut self, fleet_mode: FleetInfoMode) -> FleetInfo {
        let fleet_id = self.id;
        self.sol
            .exec_standard_safe(move |core_sol| {
                // Holding mutex on sol - nothing can remove the core fleet without consuming the
                // high-level Fleet
                let mut core_fleet = core_sol.get_fleet_mut(&fleet_id).unwrap();
                FleetInfo::from_core(&mut core_fleet, fleet_mode)
            })
            .await
    }
}
