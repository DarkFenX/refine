use crate::{SolarSystem, dev::DevBenchmarkCmd};

impl SolarSystem<'_> {
    #[tracing::instrument(name = "sol-bm", level = "trace", skip_all)]
    pub async fn dev_benchmark(&mut self, dev_cmd: DevBenchmarkCmd) {
        self.exec_heavy_infallible(move |core_sol| dev_cmd.execute(core_sol))
            .await
    }
}
