use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use tokio::sync::RwLock;
use tracing::{event, Level};

use crate::util::{Error, ErrorKind, Result};

pub(crate) struct SrcMgr {
    alias_src_map: RwLock<HashMap<String, Arc<reefast::Src>>>,
    default_alias: RwLock<Option<String>>,
    locked_aliases: RwLock<HashSet<String>>,
}
impl SrcMgr {
    pub(crate) fn new() -> SrcMgr {
        SrcMgr {
            alias_src_map: RwLock::new(HashMap::new()),
            default_alias: RwLock::new(None),
            locked_aliases: RwLock::new(HashSet::new()),
        }
    }
    pub(crate) async fn add(
        &self,
        alias: String,
        data_version: String,
        data_base_url: String,
        make_default: bool,
    ) -> Result<()> {
        event!(
            Level::INFO,
            "adding source with alias \"{}\", default={}",
            alias,
            make_default
        );

        if !self.check_alias_availability(&alias).await {
            return Err(Error::new(
                ErrorKind::SrcAliasNotAvailable,
                format!("source alias \"{}\" is not available", alias),
            ));
        }
        self.lock_alias(&alias).await;
        let alias_cloned = alias.clone();
        match tokio_rayon::spawn_fifo(move || create_src(alias_cloned, data_base_url, data_version)).await {
            Ok(src) => {
                let src = Arc::new(src);
                if make_default {
                    *self.default_alias.write().await = Some(alias.clone())
                };
                self.alias_src_map.write().await.insert(alias.clone(), src);
                self.unlock_alias(&alias).await;
                Ok(())
            }
            Err(e) => {
                self.unlock_alias(&alias).await;
                Err(e)
            }
        }
    }
    /// Remove data source which is stored against passed alias.
    pub(crate) async fn del(&self, alias: &str) -> Result<()> {
        event!(Level::INFO, "removing source with alias \"{}\"", alias);
        self.alias_src_map
            .write()
            .await
            .remove(alias)
            .ok_or_else(|| Error::new(ErrorKind::SrcNotFound, format!("no source with alias \"{}\"", alias)))?;
        let default_alias = self.default_alias.read().await.clone();
        match default_alias {
            Some(a) if a == alias => *self.default_alias.write().await = None,
            _ => (),
        };
        Ok(())
    }
    // Crate methods
    pub(crate) async fn get(&self, alias: &str) -> Option<Arc<reefast::Src>> {
        self.alias_src_map.read().await.get(alias).cloned()
    }
    pub(crate) async fn get_default(&self) -> Option<Arc<reefast::Src>> {
        match self.default_alias.read().await.as_ref() {
            Some(a) => self.get(a).await,
            None => None,
        }
    }
    // Private methods
    async fn check_alias_availability(&self, alias: &str) -> bool {
        !self.alias_src_map.read().await.contains_key(alias) && !self.locked_aliases.read().await.contains(alias)
    }
    async fn lock_alias(&self, alias: &str) {
        event!(Level::DEBUG, "locking alias \"{}\"", alias);
        self.locked_aliases.write().await.insert(alias.into());
    }
    async fn unlock_alias(&self, alias: &str) {
        event!(Level::DEBUG, "unlocking alias \"{}\"", alias);
        if !self.locked_aliases.write().await.remove(alias) {
            event!(Level::ERROR, "attempt to unlock alias which is not locked")
        }
    }
}

fn create_src(alias: String, data_base_url: String, data_version: String) -> Result<reefast::Src> {
    let dh = Box::new(
        reefast::dh_impls::phobos::PhbHttpDHandler::new(data_base_url.as_str(), data_version)
            .map_err(|e| Error::new(ErrorKind::DhInitFailed, e.msg))?,
    );
    let ch = Box::new(reefast::ch_impls::json_file::JsonFileCHandler::new(
        "/home/dfx/Workspace/eve/reefast/cache",
        alias,
    ));
    reefast::Src::new(dh, ch).map_err(|e| Error::new(ErrorKind::SrcInitFailed, e.msg))
}
