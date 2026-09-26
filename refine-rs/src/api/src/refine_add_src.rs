use std::sync::Arc;

use crate::{
    Refine,
    src::{Src, SrcAlias},
    svc::SrcInnerGuarded,
};

impl Refine {
    /// Add a data source, using passed EVE data handler and optional adapted data cacher.
    #[tracing::instrument(name = "src-add", level = "error", skip_all)]
    pub async fn add_src(
        &self,
        alias: SrcAlias,
        make_default: bool,
        ed_handler: rc::ed::EveDataHandler,
        ad_cacher: Option<rc::ad::AdaptedDataCacher>,
    ) -> Result<Src<'_>, SrcAddError> {
        tracing::debug!("creating source with alias \"{alias}\", default={make_default}");
        // Source creation time is the time request was received
        let time_created = time::UtcDateTime::now();
        let Some(reservation) = self.src_alias_locks.reserve(alias) else {
            return Err(SrcAddError::SrcAliasNotAvailable(alias));
        };
        if self.src_alias_data.read().await.map.contains_key(&alias) {
            return Err(SrcAddError::SrcAliasNotAvailable(alias));
        }
        // Create source in a heavy threadpool
        let inner_src = self
            .tpool
            .exec_heavy(move || {
                create_core_src(ed_handler, ad_cacher)
                    .map(|core_src| SrcInnerGuarded::new(alias, time_created, Arc::new(core_src)))
            })
            .await?;
        // Write results, then release the alias
        let mut alias_data = self.src_alias_data.write().await;
        alias_data.map.insert(alias, inner_src.clone());
        if make_default {
            alias_data.default = Some(inner_src.clone());
        }
        drop(alias_data);
        drop(reservation);
        Ok(Src::new(self, inner_src))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SrcAddError {
    #[error("alias \"{0}\" already exists")]
    SrcAliasNotAvailable(SrcAlias),
    #[error("source initialization failed")]
    SrcInit(#[from] rc::src::err::SrcInitError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Sync processing
////////////////////////////////////////////////////////////////////////////////////////////////////
fn create_core_src(
    ed_handler: rc::ed::EveDataHandler,
    ad_cacher: Option<rc::ad::AdaptedDataCacher>,
) -> Result<rc::Src, SrcAddError> {
    tracing::info!(
        "initializing new source with {:?} and {}",
        ed_handler,
        match ad_cacher.as_ref() {
            Some(ad_cacher) => format!("{:?}", ad_cacher),
            None => "no caching".to_string(),
        }
    );
    let core_src = rc::Src::new(&ed_handler, ad_cacher.as_ref())?;

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
        rc::src::SrcOrigin::Cached(..) => tracing::info!("source data was loaded from cache"),
    }
}

fn log_warnings(core_src: &rc::Src) {
    let core_info = core_src.get_info();
    // Report data fetching errors under EVE data handler span, since that's where they originate
    // from
    tracing::error_span!("edh").in_scope(|| {
        for warning in core_info.warnings.eve_data_fetch.iter() {
            tracing::warn!("{}", warning);
        }
    });
    tracing::error_span!("adg").in_scope(|| {
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
        tracing::error_span!("adc").in_scope(|| {
            tracing::warn!("{}", warning);
        });
    }
}
