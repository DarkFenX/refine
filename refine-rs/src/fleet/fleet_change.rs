use crate::{
    cmd::{ChangeFleetCmd, ChangeFleetError},
    fleet::Fleet,
};

impl Fleet<'_, '_> {
    #[tracing::instrument(name = "flt-chg", level = "trace", skip_all)]
    pub async fn change(&mut self, cmd: ChangeFleetCmd) -> Result<(), ChangeFleetError> {
        let fleet_id = self.id;
        self.sol
            .exec_standard_fallible(move |core_sol| {
                // Holding mutex on sol - nothing can remove the core fleet without consuming the
                // high-level Fleet
                let mut core_fleet = core_sol.get_fleet_mut(&fleet_id).unwrap();
                cmd.execute(&mut core_fleet)
            })
            .await?;
        Ok(())
    }
}
