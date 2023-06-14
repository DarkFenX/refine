use std::collections::HashMap;

use tokio::{sync::RwLock, time};
use tracing::Instrument;
use uuid::Uuid;

use crate::{
    bridge::HGuardedSs,
    info::{HFitInfoMode, HItemInfoMode, HSsInfo, HSsInfoMode},
    util::{HError, HErrorKind, HResult},
};

pub(crate) struct HSsMgr {
    id_ss_map: RwLock<HashMap<String, HGuardedSs>>,
}
impl HSsMgr {
    pub(crate) fn new() -> Self {
        Self {
            id_ss_map: RwLock::new(HashMap::new()),
        }
    }
    // Solar system methods
    pub(crate) async fn add_ss(
        &self,
        src: rc::Src,
        ss_mode: HSsInfoMode,
        fit_mode: HFitInfoMode,
        item_mode: HItemInfoMode,
    ) -> HSsInfo {
        let id = get_id();
        let id_mv = id.clone();
        let (core_ss, ss_info) = tokio_rayon::spawn_fifo(move || {
            let mut core_ss = rc::SolarSystem::new(src);
            let ss_info = HSsInfo::mk_info(id_mv, &mut core_ss, ss_mode, fit_mode, item_mode);
            (core_ss, ss_info)
        })
        .await;
        self.id_ss_map
            .write()
            .await
            .insert(id.clone(), HGuardedSs::new(id, core_ss));
        ss_info
    }
    pub(crate) async fn get_ss(&self, id: &str) -> HResult<HGuardedSs> {
        self.id_ss_map
            .read()
            .await
            .get(id)
            .ok_or_else(|| HError::new(HErrorKind::SsNotFound(id.to_string())))
            .cloned()
    }
    pub(crate) async fn delete_ss(&self, id: &str) -> HResult<()> {
        match self.id_ss_map.write().await.remove(id) {
            Some(_) => Ok(()),
            None => Err(HError::new(HErrorKind::SsNotFound(id.to_string()))),
        }
    }
    // Cleanup methods
    async fn cleanup_ss(&self, lifetime: u64) {
        let span = tracing::trace_span!("ss-cleanup");
        async move {
            tracing::debug!("starting cleanup");
            let now = chrono::Utc::now();
            let lifetime = chrono::Duration::seconds(lifetime as i64);
            let to_clean: Vec<_> = self
                .id_ss_map
                .read()
                .await
                .iter()
                .filter(|(_, v)| match v.try_lock() {
                    Ok(ss) => *ss.last_accessed() + lifetime < now,
                    // If it's locked - it means it's being worked on, we don't touch that
                    Err(_) => false,
                })
                .map(|(k, _)| k.clone())
                .collect();
            if to_clean.is_empty() {
                tracing::debug!("nothing to remove");
                return;
            }
            self.id_ss_map.write().await.drain_filter(|k, _| to_clean.contains(k));
            tracing::debug!("{} solar systems removed", to_clean.len());
        }
        .instrument(span)
        .await
    }
    pub(crate) async fn periodic_cleanup(&self, interval: u64, lifetime: u64) {
        let mut timer = time::interval(time::Duration::from_secs(interval));
        loop {
            timer.tick().await;
            self.cleanup_ss(lifetime).await;
        }
    }
}

fn get_id() -> String {
    Uuid::new_v4().to_string()
}
