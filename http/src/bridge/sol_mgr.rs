use std::collections::HashMap;

use tokio::{sync::RwLock, time};
use uuid::Uuid;

use crate::{
    bridge::{HBrError, HGuardedSol},
    cmd::HAddSolCmd,
    info::{HFitInfoMode, HFleetInfoMode, HItemInfoMode, HSolInfo, HSolInfoMode},
};

pub(crate) struct HSolMgr {
    id_sol_map: RwLock<HashMap<String, HGuardedSol>>,
}
impl HSolMgr {
    pub(crate) fn new() -> Self {
        Self {
            id_sol_map: RwLock::new(HashMap::new()),
        }
    }
    // Solar system methods
    #[tracing::instrument(name = "solmgr-add", level = "trace", skip_all)]
    pub(crate) async fn add_sol(
        &self,
        command: HAddSolCmd,
        src: rc::Src,
        sol_mode: HSolInfoMode,
        fleet_mode: HFleetInfoMode,
        fit_mode: HFitInfoMode,
        item_mode: HItemInfoMode,
    ) -> Result<HSolInfo, HBrError> {
        let id = get_id();
        let id_mv = id.clone();
        let sync_span = tracing::trace_span!("sync");
        match tokio_rayon::spawn_fifo(move || {
            let _sg = sync_span.enter();
            let mut core_sol = command.execute(src).map_err(HBrError::from)?;
            let sol_info = HSolInfo::mk_info(id_mv, &mut core_sol, sol_mode, fleet_mode, fit_mode, item_mode);
            Ok((core_sol, sol_info))
        })
        .await
        {
            Ok((core_sol, sol_info)) => {
                self.id_sol_map
                    .write()
                    .await
                    .insert(id.clone(), HGuardedSol::new(id, core_sol));
                Ok(sol_info)
            }
            Err(br_err) => Err(br_err),
        }
    }
    pub(crate) async fn get_sol(&self, id: &str) -> Result<HGuardedSol, HBrError> {
        self.id_sol_map
            .read()
            .await
            .get(id)
            .ok_or_else(|| HBrError::SolNotFound(id.to_string()))
            .cloned()
    }
    #[tracing::instrument(name = "solmgr-del", level = "trace", skip_all)]
    pub(crate) async fn delete_sol(&self, id: &str) -> Result<(), HBrError> {
        match self.id_sol_map.write().await.remove(id) {
            Some(_) => Ok(()),
            None => Err(HBrError::SolNotFound(id.to_string())),
        }
    }
    // Cleanup methods
    #[tracing::instrument(name = "solmgr-cleanup", level = "trace", skip_all)]
    async fn cleanup_sols(&self, lifetime: u64) {
        tracing::debug!("starting cleanup");
        let now = chrono::Utc::now();
        let lifetime = match chrono::TimeDelta::try_seconds(lifetime as i64) {
            Some(lifetime) => lifetime,
            None => {
                tracing::warn!("unable to initialize timedelta with {lifetime}, cleanup failed");
                return;
            }
        };
        let to_clean: Vec<_> = self
            .id_sol_map
            .read()
            .await
            .iter()
            .filter(|(_, v)| match v.try_lock() {
                Ok(sol) => *sol.last_accessed() + lifetime < now,
                // If it's locked - it means it's being worked on, we don't touch that
                Err(_) => false,
            })
            .map(|(k, _)| k.clone())
            .collect();
        if to_clean.is_empty() {
            tracing::debug!("nothing to remove");
            return;
        }
        self.id_sol_map.write().await.retain(|k, _| !to_clean.contains(k));
        tracing::info!("{} solar systems removed", to_clean.len());
    }
    pub(crate) async fn periodic_cleanup(&self, interval: u64, lifetime: u64) {
        let mut timer = time::interval(time::Duration::from_secs(interval));
        timer.set_missed_tick_behavior(time::MissedTickBehavior::Skip);
        loop {
            timer.tick().await;
            self.cleanup_sols(lifetime).await;
        }
    }
}

fn get_id() -> String {
    Uuid::new_v4().to_string()
}
