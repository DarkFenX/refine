use tokio_rayon::AsyncThreadPool;

use crate::{
    cmd::{CreateFleetCmd, CreateFleetError},
    fleet::Fleet,
    sol::SolarSystem,
};

impl<'r, 's> SolarSystem<'r> {
    #[tracing::instrument(name = "sol-fleet-add", level = "trace", skip_all)]
    pub async fn create_fleet(&'s mut self, cmd: CreateFleetCmd) -> Result<Fleet<'r, 's>, CreateFleetError> {
        let mut core_sol = self.take_core().unwrap();
        let core_sol_backup = core_sol.clone();
        let sync_span = tracing::trace_span!("sync");
        match self
            .refine
            .tpool
            .standard
            .spawn_fifo_async(move || {
                let _sg = sync_span.enter();
                let resp = cmd.execute(&mut core_sol)?;
                Ok((core_sol, resp.fleet_id))
            })
            .await
        {
            Ok((core_sol, fleet_id)) => {
                self.put_core_back(core_sol);
                Ok(Fleet::new(self, fleet_id))
            }
            Err(error) => {
                self.put_core_back(core_sol_backup);
                Err(error)
            }
        }
    }
}
