use std::sync::Arc;

use crate::{
    Src,
    ad::{
        AData, ADataGenerator, AFingerprint, AdaptedDataCacher,
        err::{ADataGeneratorError, AdcLoadError},
    },
    ed::EveDataHandler,
    rd::RData,
    src::{SrcOrigin, SrcOriginGeneratedReason, SrcWarnings},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Src {
    pub fn new(ed_handler: &EveDataHandler, ad_cacher: Option<&AdaptedDataCacher>) -> Result<Self, SrcInitError> {
        // No cacher - just generate adapted data (no cacher to write it)
        let ad_cacher = match ad_cacher {
            Some(ad_cacher) => ad_cacher,
            None => return generate(ed_handler, SrcOrigin::Generated(SrcOriginGeneratedReason::NoCacher)),
        };
        // No EVE data version - just generate adapted data (no EVE data part of the fingerprint)
        let ed_version = match ed_handler.get_impl().get_data_version() {
            Ok(ed_version) => ed_version,
            Err(error) => {
                return generate(
                    ed_handler,
                    SrcOrigin::Generated(SrcOriginGeneratedReason::NoEveDataVersion(
                        std::error::Report::new(&error).to_string(),
                    )),
                );
            }
        };
        let current_fingerprint = AFingerprint::new(&ed_version, &ad_cacher.get_impl().get_cacher_version());
        let reason = match ad_cacher.get_impl().load_from_cache(&current_fingerprint) {
            Ok(a_data) => return process(a_data, SrcOrigin::Cached(current_fingerprint.into_string())),
            Err(AdcLoadError::FingerprintRead(error)) => {
                SrcOriginGeneratedReason::NoCachedFingerprint(std::error::Report::new(&*error).to_string())
            }
            Err(AdcLoadError::FingerprintMismatch(cached_fingerprint)) => {
                SrcOriginGeneratedReason::FingerprintMismatch(format!(
                    "needed fingerprint {current_fingerprint}, cached fingerprint {cached_fingerprint}"
                ))
            }
            Err(AdcLoadError::DataRead(error)) => {
                SrcOriginGeneratedReason::CacheLoadFailed(std::error::Report::new(&*error).to_string())
            }
        };
        // Cached data is unusable - generate adapted data and cache it
        generate_and_cache(ed_handler, SrcOrigin::Generated(reason), ad_cacher, current_fingerprint)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SrcInitError {
    #[error(transparent)]
    ADataGeneration(#[from] ADataGeneratorError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
fn process(mut a_data: AData, origin: SrcOrigin) -> Result<Src, SrcInitError> {
    let warnings = SrcWarnings::from_adapted_warnings(&mut a_data);
    let r_data = RData::from_a_data(a_data);
    Ok(Src {
        r_data: Arc::new(r_data),
        origin,
        warnings,
    })
}

fn generate(ed_handler: &EveDataHandler, origin: SrcOrigin) -> Result<Src, SrcInitError> {
    let a_data = ADataGenerator::new().generate(ed_handler)?;
    process(a_data, origin)
}

fn generate_and_cache(
    ed_handler: &EveDataHandler,
    origin: SrcOrigin,
    ad_cacher: &AdaptedDataCacher,
    fingerprint: AFingerprint,
) -> Result<Src, SrcInitError> {
    let mut a_data = ADataGenerator::new().generate(ed_handler)?;
    // Cache first, then make warnings container, since it drains all the info from adapted data
    let cache_write_warning = ad_cacher
        .get_impl()
        .write_cache(&a_data, fingerprint)
        .map_err(|error| std::error::Report::new(&error).to_string())
        .err();
    let mut warnings = SrcWarnings::from_adapted_warnings(&mut a_data);
    warnings.cache_write = cache_write_warning;
    let r_data = RData::from_a_data(a_data);
    Ok(Src {
        r_data: Arc::new(r_data),
        origin,
        warnings,
    })
}
