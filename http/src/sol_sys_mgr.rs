use std::{collections::HashMap, time::Instant};

use tokio::sync::RwLock;
use uuid::Uuid;

struct ManagedSolSys {
    sol_sys: reefast::SolarSystem,
    accessed: Instant,
}
impl ManagedSolSys {
    fn new_with_sol_sys(sol_sys: reefast::SolarSystem) -> ManagedSolSys {
        ManagedSolSys {
            sol_sys,
            accessed: Instant::now(),
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
}

fn get_id() -> String {
    Uuid::new_v4().to_string()
}
