use crate::{
    cmd::{CreateFleetCmd, CreateFleetError},
    fleet::Fleet,
    sol::SolarSystem,
};

impl<'r, 's> SolarSystem<'r> {
    #[tracing::instrument(name = "sol-fleet-add", level = "trace", skip_all)]
    pub async fn create_fleet(&'s mut self, cmd: CreateFleetCmd) -> Result<Fleet<'r, 's>, CreateFleetError> {
        self.exec_std_fallible(move |core_sol| cmd.execute(core_sol))
            .await
            .map(|cmd_resp| Fleet::new(self, cmd_resp.fleet_id))
    }
}
