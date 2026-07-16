use std::time::Duration;

use crate::Refine;

impl Refine {
    pub async fn setup_periodic_cleanup(
        &self,
        cleanup_interval: Duration,
        sol_inact_limit: Duration,
    ) -> Result<(), CleanupSetupError> {
        let sol_inact_limit = chrono::TimeDelta::from_std(sol_inact_limit)?;
        let mut timer = tokio::time::interval(cleanup_interval);
        timer.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
        loop {
            timer.tick().await;
            self.cleanup_sols(sol_inact_limit).await;
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum CleanupSetupError {
    #[error("failed to initialize sol inactivity limit: {0}")]
    SolLimitInitFailed(#[from] chrono::OutOfRangeError),
}
