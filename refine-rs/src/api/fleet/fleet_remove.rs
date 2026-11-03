use crate::{Fleet, FleetRemoveCmd};

impl Fleet<'_, '_> {
    #[tracing::instrument(name = "flt-rmv", level = "error", skip_all)]
    pub async fn remove(self, ctl_cmd: FleetRemoveCmd) {
        // Variables for move
        let fleet_id = self.id;
        self.sol
            .exec_standard_infallible(move |core| {
                // Holding mutex on sol - nothing can remove the fleet before we get it
                let core_fleet = core.get_fleet(&fleet_id).unwrap();
                ctl_cmd.execute(core_fleet)
            })
            .await
    }
}
