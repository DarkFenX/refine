use crate::{ad, adg, def::VERSION, ed, src::SrcInitError};

pub(in crate::src) fn prepare_adapted_data(
    ed_handler: Box<dyn ed::EveDataHandler>,
    ad_cacher: Option<Box<dyn ad::AdaptedDataCacher>>,
) -> Result<ad::AData, SrcInitError> {
    match ad_cacher {
        Some(mut ad_cacher) => {
            tracing::info!("initializing new source with {ed_handler:?} and {ad_cacher:?}");
            let ed_version = get_ed_version(&ed_handler);
            match get_relevant_a_data(ed_version.clone(), &mut ad_cacher) {
                Some(a_data) => Ok(a_data),
                None => {
                    let a_data = adapt_data(&ed_handler)?;
                    // Cache is updated only if EVE data version is specified
                    if let Some(ed_version) = ed_version {
                        let current_fingerprint = get_current_fingerprint(ed_version, &ad_cacher);
                        ad_cacher.write_cache(&a_data, &current_fingerprint);
                    }
                    Ok(a_data)
                }
            }
        }
        None => {
            tracing::info!("initializing new source with {ed_handler:?} without caching");
            adapt_data(&ed_handler)
        }
    }
}

#[allow(clippy::borrowed_box)]
fn get_ed_version(ed_handler: &Box<dyn ed::EveDataHandler>) -> Option<String> {
    match ed_handler.get_data_version() {
        Ok(ed_version) => Some(ed_version),
        Err(e) => {
            tracing::info!("unable to get EVE data version: {e}");
            None
        }
    }
}

fn get_current_fingerprint(ed_version: String, ad_cacher: &Box<dyn ad::AdaptedDataCacher>) -> String {
    let adc_version = ad_cacher.get_cacher_version();
    format!("ed{ed_version}_adc{adc_version}_core{VERSION}")
}

fn get_relevant_a_data(
    ed_version: Option<String>,
    ad_cacher: &mut Box<dyn ad::AdaptedDataCacher>,
) -> Option<ad::AData> {
    // Failure to read EVE data version is not fatal, we just always generate adapted data in this
    // case
    let ed_version = match ed_version {
        Some(version) => version,
        None => return None,
    };
    let current_fingeprint = get_current_fingerprint(ed_version, ad_cacher);
    match ad_cacher.get_cache_fingerprint() {
        Some(cache_fingerprint) => {
            if current_fingeprint != cache_fingerprint {
                tracing::info!("fingerprint mismatch: {current_fingeprint} current vs {cache_fingerprint} cached");
                return None;
            };
        }
        None => {
            tracing::info!("no adapted data fingerprint");
            return None;
        }
    }
    // Failure to load cache is not fatal as well
    match ad_cacher.load_from_cache() {
        Ok(a_data) => Some(a_data),
        Err(e) => {
            tracing::info!("unable to load cache: {e}");
            None
        }
    }
}

#[allow(clippy::borrowed_box)]
fn adapt_data(ed_handler: &Box<dyn ed::EveDataHandler>) -> Result<ad::AData, SrcInitError> {
    tracing::info!("generating adapted data...");
    adg::generate_adapted_data(ed_handler.as_ref())
}
