use crate::{cmd::RemoveFleetCmd, fleet::Fleet};

impl Fleet<'_, '_> {
    #[tracing::instrument(name = "flt-rmv", level = "trace", skip_all)]
    pub async fn remove(self, cmd: RemoveFleetCmd) {
        let fleet_id = self.id;
        self.sol
            .exec_standard_safe(move |core_sol| {
                // Holding mutex on sol - nothing can remove the fleet before we get it
                let core_fleet = core_sol.get_fleet_mut(&fleet_id).unwrap();
                cmd.execute(core_fleet)
            })
            .await
    }
}
