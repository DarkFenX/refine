use std::collections::HashMap;

use chrono::{DateTime, Duration, Utc};
use tokio::sync::RwLock;
use uuid::Uuid;

struct ManagedSolSys {
    sol_sys: reefast::SolarSystem,
    accessed: DateTime<Utc>,
}
impl ManagedSolSys {
    fn new_with_sol_sys(sol_sys: reefast::SolarSystem) -> ManagedSolSys {
        ManagedSolSys {
            sol_sys,
            accessed: Utc::now(),
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
    pub(crate) async fn cleanup_sol_sys(&self) {
        let now = Utc::now();
        let lifetime = Duration::seconds(2);
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
}

fn get_id() -> String {
    Uuid::new_v4().to_string()
}
