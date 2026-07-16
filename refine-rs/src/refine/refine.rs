use std::time::Duration;

use super::tpool::ThreadPool;
use crate::{
    sol::SolMapGuarded,
    src::{SrcAliasDataGuarded, SrcAliasLocksGuarded},
};

pub struct Refine {
    pub(crate) tpool: ThreadPool,
    // Source-related fields
    pub(crate) cache_folder: Option<String>,
    pub(crate) src_alias_data: SrcAliasDataGuarded,
    pub(crate) src_alias_locks: SrcAliasLocksGuarded,
    // Sol-related fields
    pub(crate) id_sol_map: SolMapGuarded,
}
impl Refine {
    pub fn new(cache_folder: Option<String>, standard_threads: usize, heavy_threads: usize) -> Self {
        Self {
            tpool: ThreadPool::new(standard_threads, heavy_threads),
            cache_folder,
            src_alias_data: SrcAliasDataGuarded::new(),
            src_alias_locks: SrcAliasLocksGuarded::new(),
            id_sol_map: SolMapGuarded::new(),
        }
    }
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
