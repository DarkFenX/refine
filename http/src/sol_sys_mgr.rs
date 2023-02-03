use std::collections::HashMap;

use crate::config;
use tokio::{
    sync::RwLock,
    time::{interval, Duration},
};
use uuid::Uuid;

struct ManagedSolSys {
    sol_sys: reefast::SolarSystem,
    accessed: chrono::DateTime<chrono::Utc>,
}
impl ManagedSolSys {
    fn new_with_sol_sys(sol_sys: reefast::SolarSystem) -> ManagedSolSys {
        ManagedSolSys {
            sol_sys,
            accessed: chrono::Utc::now(),
        }
    }
}

pub(crate) struct SolSysManager {
    id_sol_sys_map: RwLock<HashMap<String, ManagedSolSys>>,
}
impl SolSysManager {
    pub(crate) fn new() -> SolSysManager {
        SolSysManager {
            id_sol_sys_map: RwLock::new(HashMap::new()),
        }
    }
    // Solar system methods
    pub(crate) async fn add_sol_sys(&self, sol_sys: reefast::SolarSystem) -> String {
        let id = get_id();
        self.id_sol_sys_map
            .write()
            .await
            .insert(id.clone(), ManagedSolSys::new_with_sol_sys(sol_sys));
        id
    }
    pub(crate) async fn delete_sol_sys(&self, id: &str) -> bool {
        self.id_sol_sys_map.write().await.remove(id).is_some()
    }
    // Cleanup methods
    pub(crate) async fn cleanup_sol_sys(&self) {
        let now = chrono::Utc::now();
        let lifetime = chrono::Duration::seconds(config::SOL_SYS_LIFETIME);
        let to_clean: Vec<_> = self
            .id_sol_sys_map
            .read()
            .await
            .iter()
            .filter(|(k, v)| v.accessed + lifetime < now)
            .map(|(k, _)| k.clone())
            .collect();
        if to_clean.is_empty() {
            return;
        }
        self.id_sol_sys_map
            .write()
            .await
            .drain_filter(|k, _| to_clean.contains(k));
    }
    pub(crate) async fn periodic_cleanup(&self) {
        let mut timer = interval(Duration::from_secs(config::CLEANUP_INTERVAL));
        loop {
            timer.tick().await;
            self.cleanup_sol_sys().await;
        }
    }
}

fn get_id() -> String {
    Uuid::new_v4().to_string()
}
