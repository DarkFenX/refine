use crate::{Fleet, FleetInfo, info::FleetInfoCmd};

impl Fleet<'_, '_> {
    #[tracing::instrument(name = "flt-inf", level = "trace", skip_all)]
    pub async fn get_info(&mut self, info_cmd: FleetInfoCmd) -> FleetInfo {
        // Variables for move
        let fleet_id = self.id;
        self.sol
            .exec_standard_safe(move |core_sol| {
                // Holding mutex on sol - nothing can remove the core fleet without consuming the
                // high-level Fleet
                let mut core_fleet = core_sol.get_fleet_mut(&fleet_id).unwrap();
                info_cmd.execute(&mut core_fleet)
            })
            .await
    }
}
