use crate::{
    SolarSystem,
    val::{SolValInfo, ValInfoMode, ValidateSolCmd},
};

impl SolarSystem<'_> {
    #[tracing::instrument(name = "sol-val", level = "trace", skip_all)]
    pub async fn validate(&mut self, cmd: ValidateSolCmd, val_mode: ValInfoMode) -> SolValInfo {
        self.exec_standard_safe(move |core_sol| cmd.execute(core_sol, val_mode))
            .await
    }
}
