use crate::{
    SolarSystem,
    val::{SolValInfo, ValInfoModes, ValidateSolCmd},
};

impl SolarSystem<'_> {
    #[tracing::instrument(name = "sol-val", level = "trace", skip_all)]
    pub async fn validate(&mut self, cmd: ValidateSolCmd, modes: ValInfoModes) -> SolValInfo {
        self.exec_standard_safe(move |core_sol| cmd.execute(core_sol, modes))
            .await
    }
}
