use std::{collections::HashMap, sync::Arc};

use tokio::{
    sync::{Mutex, RwLock},
    time,
};
use uuid::Uuid;

use crate::{
    bridge::SolarSystem,
    info::{FitInfoMode, ItemInfoMode, SolSysInfo, SolSysInfoMode},
    util::{Error, ErrorKind, Result},
};

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
    pub(crate) async fn add_sol_sys(
        &self,
        src: Arc<reefast::Src>,
        ss_mode: SolSysInfoMode,
        fit_mode: FitInfoMode,
        item_mode: ItemInfoMode,
    ) -> SolSysInfo {
        let id = get_id();
        let id_mv = id.clone();
        let (core_ss, ss_info) = tokio_rayon::spawn_fifo(move || {
            let mut core_ss = reefast::SolarSystem::new(src);
            let ss_info = SolSysInfo::mk_info(id_mv, &mut core_ss, ss_mode, fit_mode, item_mode);
            (core_ss, ss_info)
        })
        .await;
        self.id_ss_map
            .write()
            .await
            .insert(id.clone(), Arc::new(Mutex::new(SolarSystem::new(id, core_ss))));
        ss_info
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
    async fn cleanup_sol_sys(&self, lifetime: u64) {
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
