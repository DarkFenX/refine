use crate::{Fleet, FleetInfo, FleetInfoModes};

impl Fleet<'_, '_> {
    pub async fn get_info(&mut self, modes: FleetInfoModes) -> FleetInfo {
        // Variables for move
        let fleet_id = self.id;
        self.sol
            .exec_standard_safe(move |core_sol| {
                // Holding mutex on sol - nothing can remove the core fleet without consuming the
                // high-level Fleet
                let mut core_fleet = core_sol.get_fleet_mut(&fleet_id).unwrap();
                FleetInfo::from_core(&mut core_fleet, modes)
            })
            .await
    }
}
