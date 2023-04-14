use std::{collections::HashMap, sync::Arc};

use tokio::{
    sync::{Mutex, RwLock},
    time,
};
use uuid::Uuid;

use crate::util::{Error, ErrorKind, Result};

use super::ss::SolarSystem;

pub(crate) struct SolSysMgr {
    id_ss_map: RwLock<HashMap<String, Arc<Mutex<SolarSystem>>>>,
}
impl SolSysMgr {
    pub(crate) fn new() -> Self {
        Self {
            id_ss_map: RwLock::new(HashMap::new()),
        }
    }
    // Solar system methods
    pub(crate) async fn add_sol_sys(&self, src: Arc<reefast::Src>) -> String {
        let core_ss = tokio_rayon::spawn_fifo(move || reefast::SolarSystem::new(src)).await;
        let id = get_id();
        self.id_ss_map
            .write()
            .await
            .insert(id.clone(), Arc::new(Mutex::new(SolarSystem::new(core_ss))));
        id
    }
    pub(crate) async fn get_sol_sys(&self, id: &str) -> Result<Arc<Mutex<SolarSystem>>> {
        self.id_ss_map
            .read()
            .await
            .get(id)
            .ok_or_else(|| Error::new(ErrorKind::SolSysNotFound(id.to_string())))
            .cloned()
    }
    pub(crate) async fn delete_sol_sys(&self, id: &str) -> Result<()> {
        match self.id_ss_map.write().await.remove(id) {
            Some(_) => Ok(()),
            None => Err(Error::new(ErrorKind::SolSysNotFound(id.to_string()))),
        }
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
            .filter(|(_, v)| match v.try_lock() {
                Ok(ss) => *ss.last_accessed() + lifetime < now,
                // If it's locked - it means it's being worked on, we don't touch that
                Err(_) => false,
            })
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
