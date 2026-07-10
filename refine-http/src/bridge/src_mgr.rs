use std::collections::{HashMap, HashSet};

use tokio::sync::RwLock;
use tokio_rayon::AsyncThreadPool;

use crate::{
    bridge::{HSrc, HThreadPool},
    err::HBrError,
    info::{HSrcInfo, HSrcInfoMode},
};

pub(crate) struct HSrcMgr {
    cache_folder: Option<String>,
    alias_src_map: RwLock<HashMap<String, HSrc>>,
    default_alias: RwLock<Option<String>>,
    locked_aliases: RwLock<HashSet<String>>,
}
impl HSrcMgr {
    // Crate-wide methods
    pub(crate) fn new(cache_folder: Option<String>) -> Self {
        Self {
            cache_folder,
            alias_src_map: RwLock::new(HashMap::new()),
            default_alias: RwLock::new(None),
            locked_aliases: RwLock::new(HashSet::new()),
        }
    }
    #[tracing::instrument(name = "srcmgr-add", level = "trace", skip_all)]
    pub(crate) async fn add(
        &self,
        tpool: &HThreadPool,
        alias: String,
        data_version: String,
        data_base_url: String,
        make_default: bool,
        src_mode: HSrcInfoMode,
    ) -> Result<HSrcInfo, HBrError> {
        tracing::debug!("adding source with alias \"{alias}\", default={make_default}");

        if !self.check_alias_availability(&alias).await {
            return Err(HBrError::SrcAliasNotAvailable(alias));
        }
        self.lock_alias(&alias).await;
        let alias_cloned = alias.clone();
        let cache_folder_cloned = self.cache_folder.clone();

        let sync_span = tracing::trace_span!("sync");
        match tpool
            .heavy
            .spawn_fifo_async(move || {
                let _sg = sync_span.enter();
                create_core_src(alias_cloned, data_base_url, data_version, cache_folder_cloned)
            })
            .await
        {
            Ok(core_src) => {
                if make_default {
                    *self.default_alias.write().await = Some(alias.clone())
                };
                let src_info = HSrcInfo::from_core(core_src.get_info(), src_mode);
                let h_src = HSrc::from_core(core_src);
                self.alias_src_map.write().await.insert(alias.clone(), h_src);
                self.unlock_alias(&alias).await;
                Ok(src_info)
            }
            Err(e) => {
                self.unlock_alias(&alias).await;
                Err(e)
            }
        }
    }
    pub(crate) async fn get(&self, alias: Option<&str>) -> Result<HSrc, HBrError> {
        match alias {
            Some(a) => self.get_src_by_alias(a).await,
            None => self.get_default_src().await,
        }
    }
    #[tracing::instrument(name = "srcmgr-del", level = "trace", skip_all)]
    pub(crate) async fn del(&self, alias: &str) -> Result<(), HBrError> {
        tracing::debug!("removing source with alias \"{alias}\"");
        self.alias_src_map
            .write()
            .await
            .remove(alias)
            .ok_or_else(|| HBrError::SrcNotFound(alias.to_string()))?;
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
        tracing::trace!("locking alias \"{alias}\"");
        self.locked_aliases.write().await.insert(alias.into());
    }
    async fn unlock_alias(&self, alias: &str) {
        tracing::trace!("unlocking alias \"{alias}\"");
        if !self.locked_aliases.write().await.remove(alias) {
            tracing::warn!("attempt to unlock alias which is not locked")
        }
    }
    async fn get_src_by_alias(&self, alias: &str) -> Result<HSrc, HBrError> {
        self.alias_src_map
            .read()
            .await
            .get(alias)
            .cloned()
            .ok_or_else(|| HBrError::SrcNotFound(alias.to_string()))
    }
    async fn get_default_src(&self) -> Result<HSrc, HBrError> {
        match self.default_alias.read().await.as_ref() {
            Some(a) => self.get_src_by_alias(a).await,
            None => Err(HBrError::NoDefaultSrc),
        }
    }
}

fn create_core_src(
    alias: String,
    data_base_url: String,
    data_version: String,
    cache_folder: Option<String>,
) -> Result<rc::Src, HBrError> {
    let edh: Box<dyn rc::ed::EveDataHandler> = Box::new(
        redh::PhbHttpEdh::try_new(data_base_url.as_str(), data_version)
            .map_err(|e| HBrError::EdhInitFailed(e.to_string()))?,
    );
    let mut adc: Option<Box<dyn rc::ad::AdaptedDataCacher>> = match cache_folder {
        Some(cf) => Some(Box::new(radc::JsonZfileAdc::new(cf.into(), alias))),
        None => None,
    };
    tracing::info!(
        "initializing new source with {:?} and {}",
        edh,
        match adc.as_ref() {
            Some(adc) => format!("{:?}", adc),
            None => "no caching".to_string(),
        }
    );
    let core_src = rc::Src::new(edh.as_ref(), adc.as_mut()).map_err(|e| HBrError::SrcInitFailed(e.to_string()))?;
    log_warnings(&core_src);
    Ok(core_src)
}

fn log_warnings(core_src: &rc::Src) {
    let core_info = core_src.get_info();
    // Report data fetching errors under EVE data handler span, since that's where they originate
    // from
    tracing::trace_span!("edh").in_scope(|| {
        for warning in core_info.warnings.eve_data_fetch.iter() {
            tracing::warn!("{}", warning);
        }
    });
    tracing::trace_span!("adg").in_scope(|| {
        for warning in core_info.warnings.adg_pk_duplicates.iter() {
            tracing::warn!("{}", warning);
        }
        // Cleanup is a normal process, so just record those "warnings" as infos
        match core_info.warnings.adg_cleanup.is_empty() {
            true => tracing::info!("no unused data found during cleanup"),
            false => {
                for warning in core_info.warnings.adg_cleanup.iter() {
                    tracing::info!("{}", warning);
                }
            }
        }
        for warning in core_info.warnings.adg_validation.iter() {
            tracing::warn!("{}", warning);
        }
        for warning in core_info.warnings.adg_conversion_main.iter() {
            tracing::warn!("{}", warning);
        }
        // Customization errors are fairly low-priority, log those as infos as well
        for warning in core_info.warnings.adg_customization.iter() {
            tracing::info!("{}", warning);
        }
        for warning in core_info.warnings.adg_conversion_aux.iter() {
            tracing::warn!("{}", warning);
        }
    });
    if let Some(warning) = core_info.warnings.cache_write.as_ref() {
        tracing::trace_span!("adc").in_scope(|| {
            tracing::warn!("{}", warning);
        });
    }
}
