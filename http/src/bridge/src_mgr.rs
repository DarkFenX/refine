use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use tokio::sync::RwLock;
use tracing::{event, Level};

use crate::util::{Error, ErrorKind, Result};

pub(crate) struct SrcMgr {
    cache_folder: Option<String>,
    alias_src_map: RwLock<HashMap<String, Arc<rc::Src>>>,
    default_alias: RwLock<Option<String>>,
    locked_aliases: RwLock<HashSet<String>>,
}
impl SrcMgr {
    // Crate-wide methods
    pub(crate) fn new(cache_folder: Option<String>) -> Self {
        Self {
            cache_folder,
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
            return Err(Error::new(ErrorKind::SrcAliasNotAvailable(alias)));
        }
        self.lock_alias(&alias).await;
        let alias_cloned = alias.clone();
        let cache_folder_cloned = self.cache_folder.clone();
        match tokio_rayon::spawn_fifo(move || {
            create_src(alias_cloned, data_base_url, data_version, cache_folder_cloned)
        })
        .await
        {
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
    pub(crate) async fn get(&self, alias: Option<&str>) -> Result<Arc<rc::Src>> {
        match alias {
            Some(a) => self.get_src_by_alias(a).await,
            None => self.get_default_src().await,
        }
    }
    pub(crate) async fn del(&self, alias: &str) -> Result<()> {
        event!(Level::INFO, "removing source with alias \"{}\"", alias);
        self.alias_src_map
            .write()
            .await
            .remove(alias)
            .ok_or_else(|| Error::new(ErrorKind::SrcNotFound(alias.to_string())))?;
        let default_alias = self.default_alias.read().await.clone();
        match default_alias {
            Some(a) if a == alias => *self.default_alias.write().await = None,
            _ => (),
        };
        Ok(())
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
    async fn get_src_by_alias(&self, alias: &str) -> Result<Arc<rc::Src>> {
        self.alias_src_map
            .read()
            .await
            .get(alias)
            .cloned()
            .ok_or_else(|| Error::new(ErrorKind::SrcNotFound(alias.to_string())))
    }
    async fn get_default_src(&self) -> Result<Arc<rc::Src>> {
        match self.default_alias.read().await.as_ref() {
            Some(a) => self.get_src_by_alias(a).await,
            None => Err(Error::new(ErrorKind::NoDefaultSrc)),
        }
    }
}

fn create_src(
    alias: String,
    data_base_url: String,
    data_version: String,
    cache_folder: Option<String>,
) -> Result<rc::Src> {
    let dh = Box::new(rp::PhbHttpEdh::new(data_base_url.as_str(), data_version).map_err(|e| {
        let reason = format!("{e}");
        Error::new(ErrorKind::EdhInitFailed(e.kind, reason))
    })?);
    let ch: Box<dyn rc::erh::CacheHandler> = match cache_folder {
        // Use cache handler with persistent storage if cache path is specified
        Some(cf) => Box::new(rc::erh_impls::JsonFileCHandler::new(cf.into(), alias)),
        // Use RAM-only cache handler if path is not specified
        None => Box::new(rc::erh_impls::RamOnlyCHandler::new()),
    };
    rc::Src::new(dh, ch).map_err(|e| {
        let reason = format!("{e}");
        Error::new(ErrorKind::SrcInitFailed(e.kind, reason))
    })
}
