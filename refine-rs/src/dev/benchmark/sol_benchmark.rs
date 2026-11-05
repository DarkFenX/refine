use crate::{Sol, dev::DevBenchmarkCmd};

impl Sol<'_> {
    #[tracing::instrument(name = "sol-bm", level = "error", skip_all)]
    pub async fn dev_benchmark(&mut self, dev_cmd: DevBenchmarkCmd) {
        self.exec_heavy_infallible(move |core_sol| dev_cmd.execute(core_sol))
            .await
    }
}
