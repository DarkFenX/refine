use std::collections::HashMap;

use tokio::{sync::RwLock, time};
use uuid::Uuid;

use super::ss::ManagedSolSys;

pub(crate) struct SolSysManager {
    id_ss_map: RwLock<HashMap<String, ManagedSolSys>>,
}
impl SolSysManager {
    pub(crate) fn new() -> Self {
        Self {
            id_ss_map: RwLock::new(HashMap::new()),
        }
    }
    // Solar system methods
    pub(crate) async fn add_sol_sys(&self, sol_sys: reefast::SolarSystem) -> String {
        let id = get_id();
        self.id_ss_map
            .write()
            .await
            .insert(id.clone(), ManagedSolSys::new(sol_sys));
        id
    }
    pub(crate) async fn delete_sol_sys(&self, id: &str) -> bool {
        self.id_ss_map.write().await.remove(id).is_some()
    }
    // Cleanup methods
    pub(crate) async fn cleanup_sol_sys(&self, lifetime: u64) {
        let now = chrono::Utc::now();
        let lifetime = chrono::Duration::seconds(lifetime as i64);
        let to_clean: Vec<_> = self
            .id_ss_map
            .read()
            .await
            .iter()
            .filter(|(_, v)| !v.is_busy() && *v.last_accessed() + lifetime < now)
            .map(|(k, _)| k.clone())
            .collect();
        if to_clean.is_empty() {
            return;
        }
        self.id_ss_map.write().await.drain_filter(|k, _| to_clean.contains(k));
    }
    pub(crate) async fn periodic_cleanup(&self, interval: u64, lifetime: u64) {
        let mut timer = time::interval(time::Duration::from_secs(interval));
        loop {
            timer.tick().await;
            self.cleanup_sol_sys(lifetime).await;
        }
    }
}

fn get_id() -> String {
    Uuid::new_v4().to_string()
}
