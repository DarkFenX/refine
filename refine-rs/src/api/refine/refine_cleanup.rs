use std::time::Duration;

use crate::Refine;

impl Refine {
    pub async fn setup_periodic_cleanup(&self, cleanup_interval: Duration, sol_inact_limit: Duration) {
        let mut timer = tokio::time::interval(cleanup_interval);
        timer.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
        loop {
            timer.tick().await;
            self.cleanup_sols(sol_inact_limit).await;
        }
    }
}
