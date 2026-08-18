use crate::{
    SolarSystem,
    val::{SolValCmd, SolValResult},
};

impl SolarSystem<'_> {
    #[tracing::instrument(name = "sol-val", level = "trace", skip_all)]
    pub async fn validate(&mut self, val_cmd: SolValCmd) -> SolValResult {
        self.exec_standard_safe(move |core_sol| val_cmd.execute(core_sol)).await
    }
}
