use std::sync::Arc;

use tokio_rayon::AsyncThreadPool;

use crate::{
    refine::Refine,
    src::{Src, SrcAlias, SrcInnerGuarded},
};

impl Refine {
    #[tracing::instrument(name = "src-add", level = "trace", skip_all)]
    pub async fn create_src(
        &self,
        alias: SrcAlias,
        ed_handler: Box<dyn rc::ed::EveDataHandler + Send>,
        make_default: bool,
    ) -> Result<Src<'_>, CreateSrcError> {
        tracing::debug!("creating source with alias \"{alias}\", default={make_default}");
        // Disallow creating of sources with the same name until this one is created/fails
        if !self.check_alias_availability(&alias).await {
            return Err(CreateSrcError::SrcAliasNotAvailable(alias));
        }
        self.lock_alias(alias.clone()).await;
        // Create source and info in heavy threadpool
        let alias_cloned = alias.clone();
        let cache_folder_cloned = self.cache_folder.clone();
        let sync_span = tracing::trace_span!("sync");
        let result = self
            .tpool
            .heavy
            .spawn_fifo_async(move || {
                let _sg = sync_span.enter();
                create_core_src(&alias_cloned, ed_handler, cache_folder_cloned)
                    .map(|core_src| SrcInnerGuarded::new(alias_cloned, Arc::new(core_src)))
            })
            .await;
        // Write results and unlock alias
        match result {
            Ok(inner_src) => {
                let mut alias_data = self.src_alias_data.write().await;
                alias_data.map.insert(alias.clone(), inner_src.clone());
                if make_default {
                    alias_data.default = Some(inner_src.clone());
                }
                drop(alias_data);
                self.unlock_alias(&alias).await;
                Ok(Src::new(self, inner_src))
            }
            Err(e) => {
                self.unlock_alias(&alias).await;
                Err(e)
            }
        }
    }
    async fn check_alias_availability(&self, alias: &SrcAlias) -> bool {
        !self.src_alias_data.read().await.map.contains_key(alias) && !self.src_alias_locks.read().await.contains(alias)
    }
    async fn lock_alias(&self, alias: SrcAlias) {
        tracing::trace!("locking alias \"{alias}\"");
        self.src_alias_locks.write().await.insert(alias);
    }
    async fn unlock_alias(&self, alias: &SrcAlias) {
        tracing::trace!("unlocking alias \"{alias}\"");
        if !self.src_alias_locks.write().await.remove(alias) {
            tracing::warn!("attempt to unlock alias which is not locked")
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum CreateSrcError {
    #[error("alias \"{0}\" already exists")]
    SrcAliasNotAvailable(SrcAlias),
    #[error("EVE data handler initialization failed: {0}")]
    EdhInitFailed(String),
    #[error("source initialization failed: {0}")]
    SrcInitFailed(String),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Sync processing
////////////////////////////////////////////////////////////////////////////////////////////////////
fn create_core_src(
    alias: &SrcAlias,
    ed_handler: Box<dyn rc::ed::EveDataHandler>,
    cache_folder: Option<String>,
) -> Result<rc::Src, CreateSrcError> {
    let mut adc: Option<Box<dyn rc::ad::AdaptedDataCacher>> = match cache_folder {
        Some(cf) => Some(Box::new(radc::JsonZfileAdc::new(cf.into(), alias.into()))),
        None => None,
    };
    tracing::info!(
        "initializing new source with {:?} and {}",
        ed_handler,
        match adc.as_ref() {
            Some(adc) => format!("{:?}", adc),
            None => "no caching".to_string(),
        }
    );
    let core_src =
        rc::Src::new(ed_handler.as_ref(), adc.as_mut()).map_err(|e| CreateSrcError::SrcInitFailed(e.to_string()))?;

    log_reason(&core_src);
    log_warnings(&core_src);
    Ok(core_src)
}

fn log_reason(core_src: &rc::Src) {
    match &core_src.get_info().origin {
        rc::src::SrcOrigin::Generated(reason) => {
            let prefix = "source data was generated";
            match reason {
                rc::src::SrcOriginGeneratedReason::NoCacher => {
                    tracing::info!("{prefix}: caching is disabled")
                }
                rc::src::SrcOriginGeneratedReason::NoEveDataVersion(msg) => {
                    tracing::info!("{prefix}: failed to get EVE data version: {msg}")
                }
                rc::src::SrcOriginGeneratedReason::NoCachedFingerprint(msg) => {
                    tracing::info!("{prefix}: failed to get cache fingerprint: {msg}")
                }
                rc::src::SrcOriginGeneratedReason::FingerprintMismatch(msg) => {
                    tracing::info!("{prefix}: fingerprint mismatch: {msg}")
                }
                rc::src::SrcOriginGeneratedReason::CacheLoadFailed(msg) => {
                    tracing::info!("{prefix}: failed to load cache: {msg}")
                }
            }
        }
        rc::src::SrcOrigin::Cached(_) => tracing::info!("source data was loaded from cache"),
    }
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
