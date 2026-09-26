use std::time::{Duration, Instant};

use crate::Refine;

impl Refine {
    #[tracing::instrument(name = "sol-cln", level = "error", skip_all)]
    pub(crate) async fn cleanup_sols(&self, sol_inact_limit: Duration) {
        tracing::debug!("starting cleanup");
        let mut id_sol_map = self.id_sol_map.write();
        let now = Instant::now();
        // Detection & removal in the same lock to avoid sols being taken to do something, and then
        // removed
        let cleaned_count = id_sol_map
            .extract_if(|_, inner_sol| match inner_sol.try_lock() {
                Ok(sol) => sol.get_last_accessed() + sol_inact_limit < now,
                // If it's locked - it means it's being worked on, we don't touch that
                Err(..) => false,
            })
            .count();
        drop(id_sol_map);
        match cleaned_count {
            0 => tracing::debug!("nothing to clean"),
            _ => tracing::info!("{cleaned_count} solar systems cleaned up"),
        }
    }
}
