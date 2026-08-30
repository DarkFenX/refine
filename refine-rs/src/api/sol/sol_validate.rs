use crate::{
    SolarSystem,
    val::{SolValCmd, SolValResult},
};

impl SolarSystem<'_> {
    #[tracing::instrument(name = "sol-val", level = "trace", skip_all)]
    pub async fn validate(&mut self, val_cmd: SolValCmd) -> SolValResult {
        self.exec_standard_infallible(|core_sol| val_cmd.execute_owned(core_sol))
            .await
    }
}
