use std::{collections::HashMap, sync::Arc};

use tokio::sync::{Mutex, RwLock};

use crate::sol::{SolarSystemId, SolarSystemInner};

pub(crate) struct SolMap {
    pub(super) inner: RwLock<HashMap<SolarSystemId, Arc<Mutex<SolarSystemInner>>>>,
}
impl SolMap {
    pub(crate) fn new() -> Self {
        Self {
            inner: RwLock::new(HashMap::new()),
        }
    }
}
