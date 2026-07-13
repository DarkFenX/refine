use crate::{
    cmd::{RemoveFleetCmd, RemoveFleetError},
    fleet::Fleet,
};

impl Fleet<'_, '_> {
    #[tracing::instrument(name = "flt-rmv", level = "trace", skip_all)]
    pub async fn remove(self, cmd: RemoveFleetCmd) {
        match self
            .sol
            .exec_standard_safe(move |core_sol| cmd.execute(core_sol, &self.id))
            .await
        {
            Ok(_) => (),
            // Holding mutex on sol - nothing can remove the fleet before we do
            Err(RemoveFleetError::FleetGetFailed(_)) => unreachable!(),
        }
    }
}
