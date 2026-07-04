use super::{
    error::SrcInitError,
    info::{SrcInfo, SrcOrigin, SrcOriginCached, SrcOriginGenFpMismatch, SrcOriginGenReason},
};
use crate::{
    ad::{AData, ADataGenerator, AFingerprint, AdaptedDataCacher},
    ed::EveDataHandler,
};

pub(in crate::src) fn prepare_adapted_data(
    ed_handler: &dyn EveDataHandler,
    ad_cacher: Option<&mut Box<dyn AdaptedDataCacher>>,
) -> Result<(AData, SrcInfo), SrcInitError> {
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
    // Cannot load cached data - generate adapted data and cache it
    let a_data = match ad_cacher.load_from_cache() {
        Ok(a_data) => a_data,
        Err(error) => {
            return generate_and_cache(
                ed_handler,
                SrcOrigin::Generated(SrcOriginGenReason::CacheLoadFailed(error.to_string())),
                ad_cacher,
                current_fingerprint,
            );
        }
    };
    let info = SrcInfo {
        origin: SrcOrigin::Cached(SrcOriginCached {
            fingerprint: cached_fingerprint.into_string(),
        }),
        warnings: false,
    };
    Ok((a_data, info))
}

fn generate(ed_handler: &dyn EveDataHandler, origin: SrcOrigin) -> Result<(AData, SrcInfo), SrcInitError> {
    let a_data = ADataGenerator::new().generate(ed_handler)?;
    let info = SrcInfo {
        origin,
        warnings: false,
    };
    Ok((a_data, info))
}

fn generate_and_cache(
    ed_handler: &dyn EveDataHandler,
    origin: SrcOrigin,
    ad_cacher: &mut dyn AdaptedDataCacher,
    fingerprint: AFingerprint,
) -> Result<(AData, SrcInfo), SrcInitError> {
    let a_data = ADataGenerator::new().generate(ed_handler)?;
    let info = SrcInfo {
        origin,
        warnings: false,
    };
    ad_cacher.write_cache(&a_data, fingerprint);
    Ok((a_data, info))
}
