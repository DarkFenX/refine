use std::sync::Arc;

use crate::{
    Refine, Src, SrcAlias,
    src::{AdaptedDataCacher, EveDataHandler, SrcInfo, SrcInfoMode, SrcOrigin, SrcOriginGeneratedReason, SrcWarnings},
    svc::SrcInnerGuarded,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Refine {
    /// Add a data source, using passed EVE data handler and optional adapted data cacher.
    #[tracing::instrument(name = "src-add", level = "error", skip_all)]
    pub async fn add_src(
        &self,
        alias: SrcAlias,
        make_default: bool,
        ed_handler: EveDataHandler,
        ad_cacher: Option<AdaptedDataCacher>,
    ) -> Result<Src<'_>, SrcAddError> {
        let inner_src = self
            .internal_add_src(alias, make_default, ed_handler, ad_cacher)
            .await?;
        Ok(Src::new(self, inner_src))
    }
    /// Add a data source, using passed EVE data handler and optional adapted data cacher, and get
    /// its info.
    #[tracing::instrument(name = "src-add-inf", level = "error", skip_all)]
    pub async fn add_src_and_get_info(
        &self,
        alias: SrcAlias,
        make_default: bool,
        ed_handler: EveDataHandler,
        ad_cacher: Option<AdaptedDataCacher>,
        info_mode: SrcInfoMode,
    ) -> Result<(Src<'_>, SrcInfo), SrcAddError> {
        let inner_src = self
            .internal_add_src(alias, make_default, ed_handler, ad_cacher)
            .await?;
        let is_default = self.src_alias_data.read().is_default(&inner_src);
        let src_info = SrcInfo::from_inner(&inner_src, is_default, info_mode);
        Ok((Src::new(self, inner_src), src_info))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SrcAddError {
    #[error("alias \"{0}\" already exists")]
    SrcAliasNotAvailable(SrcAlias),
    #[error("source initialization failed")]
    SrcInit(#[from] crate::src::err::core::SrcInitError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private - general
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Refine {
    async fn internal_add_src(
        &self,
        alias: SrcAlias,
        make_default: bool,
        ed_handler: EveDataHandler,
        ad_cacher: Option<AdaptedDataCacher>,
    ) -> Result<SrcInnerGuarded, SrcAddError> {
        tracing::debug!("creating source with alias \"{alias}\", default={make_default}");
        // Source creation time is the time request was received
        let time_created = time::UtcDateTime::now();
        let Some(reservation) = self.src_alias_locks.reserve(alias) else {
            return Err(SrcAddError::SrcAliasNotAvailable(alias));
        };
        if self.src_alias_data.read().map.contains_key(&alias) {
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
        let mut alias_data = self.src_alias_data.write();
        alias_data.map.insert(alias, inner_src.clone());
        if make_default {
            alias_data.default = Some(inner_src.clone());
        }
        drop(alias_data);
        drop(reservation);
        Ok(inner_src)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private - sync processing
////////////////////////////////////////////////////////////////////////////////////////////////////
fn create_core_src(ed_handler: EveDataHandler, ad_cacher: Option<AdaptedDataCacher>) -> Result<rc::Src, SrcAddError> {
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
    match core_src.get_origin() {
        SrcOrigin::Generated(reason) => {
            let prefix = "source data was generated";
            match reason {
                SrcOriginGeneratedReason::NoCacher => {
                    tracing::info!("{prefix}: caching is disabled")
                }
                SrcOriginGeneratedReason::NoEveDataVersion(msg) => {
                    tracing::info!("{prefix}: failed to get EVE data version: {msg}")
                }
                SrcOriginGeneratedReason::NoCachedFingerprint(msg) => {
                    tracing::info!("{prefix}: failed to get cache fingerprint: {msg}")
                }
                SrcOriginGeneratedReason::FingerprintMismatch(msg) => {
                    tracing::info!("{prefix}: fingerprint mismatch: {msg}")
                }
                SrcOriginGeneratedReason::CacheLoadFailed(msg) => {
                    tracing::info!("{prefix}: failed to load cache: {msg}")
                }
            }
        }
        SrcOrigin::Cached(..) => tracing::info!("source data was loaded from cache"),
    }
}

fn log_warnings(core_src: &rc::Src) {
    let SrcWarnings {
        eve_data_fetch,
        adg_pk_duplicates,
        adg_cleanup,
        adg_validation,
        adg_conversion_main,
        adg_customization,
        adg_conversion_aux,
        cache_write,
    } = core_src.get_warnings();
    // Report data fetching errors under EVE data handler span, since that's where they originate
    // from
    tracing::error_span!("edh").in_scope(|| {
        for warning in eve_data_fetch.iter() {
            tracing::warn!("{}", warning);
        }
    });
    tracing::error_span!("adg").in_scope(|| {
        for warning in adg_pk_duplicates.iter() {
            tracing::warn!("{}", warning);
        }
        // Cleanup is a normal process, so just record those "warnings" as infos
        match adg_cleanup.is_empty() {
            true => tracing::info!("no unused data found during cleanup"),
            false => {
                for warning in adg_cleanup.iter() {
                    tracing::info!("{}", warning);
                }
            }
        }
        for warning in adg_validation.iter() {
            tracing::warn!("{}", warning);
        }
        for warning in adg_conversion_main.iter() {
            tracing::warn!("{}", warning);
        }
        // Customization errors are fairly low-priority, log those as infos as well
        for warning in adg_customization.iter() {
            tracing::info!("{}", warning);
        }
        for warning in adg_conversion_aux.iter() {
            tracing::warn!("{}", warning);
        }
    });
    if let Some(warning) = cache_write.as_ref() {
        tracing::error_span!("adc").in_scope(|| {
            tracing::warn!("{}", warning);
        });
    }
}
