use crate::{
    ad, adg,
    defs::VERSION,
    ed,
    util::{Error, ErrorKind, IntError, IntResult, Result},
};

pub(crate) struct InnerSrc {
    pub(crate) a_handler: Box<dyn ad::AdaptedDataHandler>,
}
impl InnerSrc {
    pub(crate) fn new(
        e_handler: Box<dyn ed::EveDataHandler>,
        mut a_handler: Box<dyn ad::AdaptedDataHandler>,
    ) -> Result<Self> {
        tracing::info!("initializing new source with {e_handler:?} and {a_handler:?}",);
        let e_version = get_e_version(&e_handler);
        if need_to_adapt(e_version.clone(), &mut a_handler) {
            let a_data = adapt_data(&e_handler).map_err(|e| Error::new(ErrorKind::SrcADataGenFailed(e.msg)))?;
            update_a_data(e_version, &mut a_handler, a_data);
        }
        Ok(Self { a_handler })
    }
}

fn get_e_version(e_handler: &Box<dyn ed::EveDataHandler>) -> Option<String> {
    match e_handler.get_data_version() {
        Ok(e_version) => Some(e_version),
        Err(e) => {
            tracing::info!("unable to get EVE data version: {e}");
            None
        }
    }
}

fn get_e_fingerprint(e_version: &str) -> String {
    format!("{e_version}_{VERSION}")
}

fn need_to_adapt(e_version: Option<String>, a_handler: &mut Box<dyn ad::AdaptedDataHandler>) -> bool {
    // Failure to read EVE data version is not fatal,
    // we just always generate adapted data in this case
    let e_version = match e_version {
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
    let e_fingeprint = get_e_fingerprint(&e_version);
    let a_fingerprint_opt = a_handler.get_data_fingerprint();
    match a_fingerprint_opt {
        Some(a_fingerprint) => {
            if &e_fingeprint != a_fingerprint {
                tracing::info!("fingerprint mismatch: {e_fingeprint} EVE data vs {a_fingerprint} adapted data");
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

fn adapt_data(e_handler: &Box<dyn ed::EveDataHandler>) -> IntResult<ad::AData> {
    tracing::info!("generating adapted data...");
    // If we have to generate adapted data, failure to generate one is fatal
    adg::generate_adapted_data(e_handler.as_ref())
        .map_err(|e| IntError::new(format!("failed to generate adapted data: {e}")))
}

fn update_a_data(e_version: Option<String>, a_handler: &mut Box<dyn ad::AdaptedDataHandler>, a_data: ad::AData) {
    let e_version = e_version.unwrap_or("none".into());
    let e_fingerprint = get_e_fingerprint(&e_version);
    a_handler.update_data(a_data, e_fingerprint)
}
