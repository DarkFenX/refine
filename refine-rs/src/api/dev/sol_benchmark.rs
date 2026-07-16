use crate::{SolarSystem, dev::DevBenchmarkCmd};

impl SolarSystem<'_> {
    #[tracing::instrument(name = "sol-bm", level = "trace", skip_all)]
    pub async fn dev_benchmark(&mut self, cmd: DevBenchmarkCmd) {
        self.exec_standard_safe(move |core_sol| cmd.execute(core_sol)).await
    }
}
