use crate::{Fleet, FleetInfo, FleetInfoArgs, info::FleetInfoModesInt};

impl Fleet<'_, '_> {
    pub async fn get_info(&mut self, info_args: FleetInfoArgs) -> FleetInfo {
        // Variables for move
        let fleet_id = self.id;
        self.sol
            .exec_standard_safe(move |core_sol| {
                // Holding mutex on sol - nothing can remove the core fleet without consuming the
                // high-level Fleet
                let mut core_fleet = core_sol.get_fleet_mut(&fleet_id).unwrap();
                let fleet_info_modes = FleetInfoModesInt::from_pub_mode(info_args.fleet);
                FleetInfo::from_core(&mut core_fleet, &fleet_info_modes)
            })
            .await
    }
}
