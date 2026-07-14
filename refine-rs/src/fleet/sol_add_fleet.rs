use crate::{
    cmd::{AddFleetCmd, AddFleetError},
    fleet::Fleet,
    sol::SolarSystem,
};

impl<'r, 's> SolarSystem<'r> {
    #[tracing::instrument(name = "flt-add", level = "trace", skip_all)]
    pub async fn add_fleet(&'s mut self, cmd: AddFleetCmd) -> Result<Fleet<'r, 's>, AddFleetError> {
        let cmd_resp = self
            .exec_standard_fallible(move |core_sol| cmd.execute(core_sol))
            .await?;
        Ok(Fleet::new(self, cmd_resp.fleet_id))
    }
}
