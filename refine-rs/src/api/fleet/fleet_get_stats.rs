use crate::{
    Fleet,
    stats::{FleetStats, GetFleetStatsCmd},
};

impl Fleet<'_, '_> {
    pub async fn get_stats(&mut self, cmd: GetFleetStatsCmd) -> FleetStats {
        let fleet_id = self.id;
        self.sol
            .exec_standard_safe(move |core_sol| {
                // Holding mutex on sol - nothing can remove the core fleet without consuming the
                // high-level Fleet
                let mut core_fleet = core_sol.get_fleet_mut(&fleet_id).unwrap();
                cmd.execute(&mut core_fleet)
            })
            .await
    }
}
