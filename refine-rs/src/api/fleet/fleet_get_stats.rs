use crate::{
    Fleet,
    stats::{StatFleetOptions, StatFleetResult},
};

impl Fleet<'_, '_> {
    pub async fn get_stats(&mut self, stat_opts: StatFleetOptions) -> StatFleetResult {
        // Variables for move
        let fleet_id = self.id;
        self.sol
            .exec_standard_safe(move |core_sol| {
                // Holding mutex on sol - nothing can remove the core fleet without consuming the
                // high-level Fleet
                let mut core_fleet = core_sol.get_fleet_mut(&fleet_id).unwrap();
                stat_opts.execute(&mut core_fleet)
            })
            .await
    }
}
