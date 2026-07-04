use std::sync::Arc;

use super::{
    error::SrcInitError,
    info::{SrcInfo, SrcOrigin, SrcOriginCached, SrcOriginGenFpMismatch, SrcOriginGenReason, SrcWarnings},
};
use crate::{
    ad::{AData, ADataGenerator, AFingerprint, AdaptedDataCacher},
    ed::EveDataHandler,
    rd::RData,
};

/// Data source.
///
/// Data source is a top-level entity which manages EVE data handler and adapted data cacher to do
/// necessary preparations and expose processed data to solar system and its services.
// Under the hood it's an entity which builds runtime data container, and then provides access to
// its contents
#[derive(Clone)]
pub struct Src {
    pub(crate) r_data: Arc<RData>,
    info: SrcInfo,
}
impl Src {
    pub fn new(
        ed_handler: &dyn EveDataHandler,
        ad_cacher: Option<&mut Box<dyn AdaptedDataCacher>>,
    ) -> Result<Self, SrcInitError> {
        // No cacher - just generate adapted data (no cacher to write it)
        let ad_cacher = match ad_cacher {
            Some(ad_cacher) => ad_cacher.as_mut(),
            None => return generate(ed_handler, SrcOrigin::Generated(SrcOriginGenReason::NoCacher)),
        };
        // No EVE data version - just generate adapted data (no EVE data part of the fingerprint)
        let ed_version = match ed_handler.get_data_version() {
            Ok(ed_version) => ed_version,
            Err(error) => {
                return generate(
                    ed_handler,
                    SrcOrigin::Generated(SrcOriginGenReason::NoEveDataVersion(error.to_string())),
                );
            }
        };
        let current_fingerprint = AFingerprint::new(&ed_version, &ad_cacher.get_cacher_version());
        // No cached fingerprint - generate adapted data and cache it
        let cached_fingerprint = match ad_cacher.get_cache_fingerprint() {
            Ok(cached_fingerprint) => cached_fingerprint,
            Err(error) => {
                return generate_and_cache(
                    ed_handler,
                    SrcOrigin::Generated(SrcOriginGenReason::NoCachedFingerprint(error.to_string())),
                    ad_cacher,
                    current_fingerprint,
                );
            }
        };
        // Fingerprint mismatch - generate adapted data and cache it
        if cached_fingerprint != current_fingerprint {
            return generate_and_cache(
                ed_handler,
                SrcOrigin::Generated(SrcOriginGenReason::FingerprintMismatch(SrcOriginGenFpMismatch {
                    needed: current_fingerprint.get_str().to_string(),
                    cached: cached_fingerprint.into_string(),
                })),
                ad_cacher,
                current_fingerprint,
            );
        }
        match ad_cacher.load_from_cache() {
            Ok(a_data) => process(
                a_data,
                SrcOrigin::Cached(SrcOriginCached {
                    fingerprint: cached_fingerprint.into_string(),
                }),
            ),
            // Cannot load cached data - generate adapted data and cache it
            Err(error) => generate_and_cache(
                ed_handler,
                SrcOrigin::Generated(SrcOriginGenReason::CacheLoadFailed(error.to_string())),
                ad_cacher,
                current_fingerprint,
            ),
        }
    }
    pub fn get_info(&self) -> &SrcInfo {
        &self.info
    }
}

fn process(mut a_data: AData, origin: SrcOrigin) -> Result<Src, SrcInitError> {
    let warnings = SrcWarnings::from_adapted_warnings(&mut a_data);
    let r_data = RData::from_a_data(a_data);
    let src_info = SrcInfo { origin, warnings };
    Ok(Src {
        r_data: Arc::new(r_data),
        info: src_info,
    })
}

fn generate(ed_handler: &dyn EveDataHandler, origin: SrcOrigin) -> Result<Src, SrcInitError> {
    let a_data = ADataGenerator::new().generate(ed_handler)?;
    process(a_data, origin)
}

fn generate_and_cache(
    ed_handler: &dyn EveDataHandler,
    origin: SrcOrigin,
    ad_cacher: &mut dyn AdaptedDataCacher,
    fingerprint: AFingerprint,
) -> Result<Src, SrcInitError> {
    let mut a_data = ADataGenerator::new().generate(ed_handler)?;
    let mut warnings = SrcWarnings::from_adapted_warnings(&mut a_data);
    if let Err(error) = ad_cacher.write_cache(&a_data, fingerprint) {
        warnings.cache_write = Some(error.to_string());
    }
    let r_data = RData::from_a_data(a_data);
    let src_info = SrcInfo { origin, warnings };
    Ok(Src {
        r_data: Arc::new(r_data),
        info: src_info,
    })
}
