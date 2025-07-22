use crate::{ad, adg, def::VERSION, ed, src::SrcInitError};

pub(in crate::src) struct InnerSrc {
    pub(in crate::src) a_handler: Box<dyn ad::AdaptedDataHandler>,
}
impl InnerSrc {
    pub(in crate::src) fn new(
        e_handler: Box<dyn ed::EveDataHandler>,
        mut a_handler: Box<dyn ad::AdaptedDataHandler>,
    ) -> Result<Self, SrcInitError> {
        tracing::info!("initializing new source with {e_handler:?} and {a_handler:?}",);
        let ed_version = get_ed_version(&e_handler);
        if need_to_adapt(ed_version.clone(), &mut a_handler) {
            let a_data = adapt_data(&e_handler)?;
            update_a_data(ed_version, &mut a_handler, a_data);
        }
        Ok(Self { a_handler })
    }
}

#[allow(clippy::borrowed_box)]
fn get_ed_version(e_handler: &Box<dyn ed::EveDataHandler>) -> Option<String> {
    match e_handler.get_data_version() {
        Ok(ed_version) => Some(ed_version),
        Err(e) => {
            tracing::info!("unable to get EVE data version: {e}");
            None
        }
    }
}

fn get_e_fingerprint(ed_version: String, adh_version: String) -> String {
    format!("ed{ed_version}_adh{adh_version}_core{VERSION}")
}

fn need_to_adapt(ed_version: Option<String>, a_handler: &mut Box<dyn ad::AdaptedDataHandler>) -> bool {
    // Failure to read EVE data version is not fatal,
    // we just always generate adapted data in this case
    let ed_version = match ed_version {
        Some(version) => version,
        None => return true,
    };
    // Failure to load cache is not fatal as well
    match a_handler.load_cache() {
        Ok(_) => (),
        Err(e) => {
            tracing::info!("unable to load cache: {e}");
            return true;
        }
    }
    let adh_version = a_handler.get_handler_version();
    let e_fingeprint = get_e_fingerprint(ed_version, adh_version);
    match a_handler.get_data_fingerprint() {
        Some(a_fingerprint) => {
            if e_fingeprint != a_fingerprint {
                tracing::info!("fingerprint mismatch: {e_fingeprint} current vs {a_fingerprint} adapted/cached");
                return true;
            };
        }
        None => {
            tracing::info!("no adapted data fingerprint");
            return true;
        }
    }
    false
}

#[allow(clippy::borrowed_box)]
fn adapt_data(e_handler: &Box<dyn ed::EveDataHandler>) -> Result<ad::AData, SrcInitError> {
    tracing::info!("generating adapted data...");
    adg::generate_adapted_data(e_handler.as_ref())
}

fn update_a_data(ed_version: Option<String>, a_handler: &mut Box<dyn ad::AdaptedDataHandler>, a_data: ad::AData) {
    let ed_version = ed_version.unwrap_or_else(|| "none".into());
    let adh_version = a_handler.get_handler_version();
    let e_fingerprint = get_e_fingerprint(ed_version, adh_version);
    a_handler.update_data(a_data, e_fingerprint)
}
