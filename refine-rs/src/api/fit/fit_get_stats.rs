use crate::{
    Fit,
    stats::{FitStatsOptions, FitStatsResult},
};

impl Fit<'_, '_> {
    pub async fn get_stats(&mut self, stat_opts: FitStatsOptions) -> FitStatsResult {
        // Variables for move
        let fit_id = self.id;
        self.sol
            .exec_standard_safe(move |core_sol| {
                // Holding mutex on sol - nothing can remove the core fit without consuming the
                // high-level Fit
                let mut core_fit = core_sol.get_fit_mut(&fit_id).unwrap();
                stat_opts.execute(&mut core_fit)
            })
            .await
    }
}
