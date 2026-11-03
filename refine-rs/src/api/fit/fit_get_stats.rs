use crate::{
    Fit,
    stats::{FitStatsCmd, FitStatsResp},
};

impl Fit<'_, '_> {
    #[tracing::instrument(name = "fit-stt", level = "error", skip_all)]
    pub async fn get_stats(&mut self, stats_cmd: FitStatsCmd) -> FitStatsResp {
        // Variables for move
        let fit_id = self.id;
        self.sol
            .exec_standard_infallible(move |core| {
                // Holding mutex on sol - nothing can remove the core fit without consuming the
                // high-level Fit
                let mut core_fit = core.get_fit(&fit_id).unwrap();
                stats_cmd.execute(&mut core_fit)
            })
            .await
    }
}
