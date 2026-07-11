use tokio_rayon::AsyncThreadPool;

use super::{mgr::SrcMgr, src::Src};
use crate::{
    info::{SrcInfo, SrcInfoMode},
    tpool::ThreadPool,
};

impl SrcMgr {
    #[tracing::instrument(name = "srcm-add", level = "trace", skip_all)]
    pub async fn create(
        &self,
        tpool: &ThreadPool,
        alias: String,
        data_version: String,
        data_base_url: String,
        make_default: bool,
        src_mode: SrcInfoMode,
    ) -> Result<SrcInfo, CreateSrcError> {
        tracing::debug!("creating source with alias \"{alias}\", default={make_default}");
        // Disallow creating of sources with the same name until this one is created/fails
        if !self.check_alias_availability(&alias).await {
            return Err(CreateSrcError::SrcAliasNotAvailable(alias));
        }
        self.lock_alias(&alias).await;
        // Create source and info in heavy threadpool
        let alias_cloned = alias.clone();
        let cache_folder_cloned = self.cache_folder.clone();
        let sync_span = tracing::trace_span!("sync");
        let result = tpool
            .heavy
            .spawn_fifo_async(move || {
                let _sg = sync_span.enter();
                create_core_src(alias_cloned, data_base_url, data_version, cache_folder_cloned).map(|core_src| {
                    let src_info = SrcInfo::from_core(core_src.get_info(), src_mode);
                    let src = Src::from_core(core_src);
                    (src, src_info)
                })
            })
            .await;
        // Write results and unlock alias
        let result = match result {
            Ok((src, src_info)) => {
                if make_default {
                    *self.default_alias.write().await = Some(alias.clone())
                };
                self.alias_src_map.write().await.insert(alias.clone(), src);
                Ok(src_info)
            }
            Err(e) => Err(e),
        };
        self.unlock_alias(&alias).await;
        result
    }
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
}

fn create_core_src(
    alias: String,
    data_base_url: String,
    data_version: String,
    cache_folder: Option<String>,
) -> Result<rc::Src, CreateSrcError> {
    let edh: Box<dyn rc::ed::EveDataHandler> = Box::new(
        redh::PhbHttpEdh::try_new(data_base_url.as_str(), data_version)
            .map_err(|e| CreateSrcError::EdhInitFailed(e.to_string()))?,
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
    let core_src =
        rc::Src::new(edh.as_ref(), adc.as_mut()).map_err(|e| CreateSrcError::SrcInitFailed(e.to_string()))?;
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

#[derive(thiserror::Error, Debug)]
pub enum CreateSrcError {
    #[error("unable to create source: source with alias \"{0}\" already exists")]
    SrcAliasNotAvailable(String),
    #[error("EVE data handler initialization failed: {0}")]
    EdhInitFailed(String),
    #[error("source initialization failed: {0}")]
    SrcInitFailed(String),
}
