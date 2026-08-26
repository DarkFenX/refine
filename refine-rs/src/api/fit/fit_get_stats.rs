use crate::{
    Fit,
    stats::{FitStatsCmd, FitStatsResp},
};

impl Fit<'_, '_> {
    #[tracing::instrument(name = "fit-stt", level = "trace", skip_all)]
    pub async fn get_stats(&mut self, stats_cmd: FitStatsCmd) -> FitStatsResp {
        // Variables for move
        let fit_id = self.id;
        self.sol
            .exec_standard_infallible(move |core_sol| {
                // Holding mutex on sol - nothing can remove the core fit without consuming the
                // high-level Fit
                let mut core_fit = core_sol.get_fit_mut(&fit_id).unwrap();
                stats_cmd.execute(&mut core_fit)
            })
            .await
    }
}
