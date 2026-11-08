use std::time::{Duration, Instant};

use crate::Refine;

impl Refine {
    #[tracing::instrument(name = "sol-cln", level = "error", skip_all)]
    pub(crate) fn cleanup_sols(&self, sol_inact_limit: Duration) {
        tracing::debug!("starting cleanup");
        let now = Instant::now();
        let removed_sols = self
            .id_sol_map
            .remove_if(|sol| sol.is_poisoned() || sol.get_last_accessed() + sol_inact_limit < now);
        match removed_sols.len() {
            0 => tracing::debug!("nothing to clean"),
            cleaned_count => {
                // Free them up on a separate thread, not to put extra load on tokio threads or
                // rayon thread pools servicing requests
                tokio::task::spawn_blocking(move || drop(removed_sols));
                tracing::info!("{cleaned_count} solar systems cleaned up");
            }
        }
    }
}
