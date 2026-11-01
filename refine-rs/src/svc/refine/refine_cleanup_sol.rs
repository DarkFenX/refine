use std::time::{Duration, Instant};

use crate::Refine;

impl Refine {
    #[tracing::instrument(name = "sol-cln", level = "error", skip_all)]
    pub(crate) fn cleanup_sols(&self, sol_inact_limit: Duration) {
        tracing::debug!("starting cleanup");
        let now = Instant::now();
        let cleaned_count = self
            .id_sol_map
            .remove_if(|sol| sol.is_poisoned() || sol.get_last_accessed() + sol_inact_limit < now);
        match cleaned_count {
            0 => tracing::debug!("nothing to clean"),
            _ => tracing::info!("{cleaned_count} solar systems cleaned up"),
        }
    }
}
