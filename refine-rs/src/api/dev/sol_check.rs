use crate::{SolarSystem, dev::DecCheckCmd};

impl SolarSystem<'_> {
    #[tracing::instrument(name = "sol-chk", level = "trace", skip_all)]
    pub async fn dev_consistency_check(&mut self, cmd: DecCheckCmd) -> bool {
        self.exec_standard_safe(move |core_sol| cmd.execute(core_sol)).await
    }
}
