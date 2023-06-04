use crate::{
    ad, adg,
    defs::VERSION,
    ed,
    util::{Error, ErrorKind, IntError, IntResult, Result},
};

/// Data source.
///
/// Data source is a top-level entity which manages EVE and adapted data handlers to do necessary
/// preparations and expose adapted data to solar system and its services.
#[derive(Debug)]
pub struct Src {
    pub(crate) ahandler: Box<dyn ad::AdaptedDataHandler>,
}
impl Src {
    pub fn new(ehandler: Box<dyn ed::EveDataHandler>, mut ahandler: Box<dyn ad::AdaptedDataHandler>) -> Result<Self> {
        log::info!("initializing new source with {ehandler:?} and {ahandler:?}",);
        let eversion = get_eversion(&ehandler);
        if need_to_adapt(eversion.clone(), &mut ahandler) {
            let adata = adapt_data(&ehandler).map_err(|e| Error::new(ErrorKind::SrcADataGenFailed(e.msg)))?;
            update_adata(eversion, &mut ahandler, adata);
        }
        Ok(Self { ahandler })
    }
}

fn get_eversion(ehandler: &Box<dyn ed::EveDataHandler>) -> Option<String> {
    match ehandler.get_data_version() {
        Ok(eversion) => Some(eversion),
        Err(e) => {
            log::info!("unable to get EVE data version: {e}");
            None
        }
    }
}

fn get_efingerprint(eversion: &str) -> String {
    format!("{}_{}", eversion, VERSION)
}

fn need_to_adapt(eversion: Option<String>, ahandler: &mut Box<dyn ad::AdaptedDataHandler>) -> bool {
    // Failure to read EVE data version is not fatal,
    // we just always generate adapted data in this case
    let eversion = match eversion {
        Some(version) => version,
        None => return true,
    };
    // Failure to load cache is not fatal as well
    match ahandler.load_cache() {
        Ok(_) => (),
        Err(e) => {
            log::info!("unable to load cache: {e}");
            return true;
        }
    }
    let efp = get_efingerprint(&eversion);
    let afp_opt = ahandler.get_data_fingerprint();
    match afp_opt {
        Some(afp) => {
            if &efp != afp {
                log::info!("fingerprint mismatch: {efp} EVE data vs {afp} adapted data");
                return true;
            };
        }
        None => {
            log::info!("no adapted data fingerprint");
            return true;
        }
    }
    false
}

fn adapt_data(ehandler: &Box<dyn ed::EveDataHandler>) -> IntResult<ad::AData> {
    log::info!("generating adapted data...");
    // If we have to generate adapted data, failure to generate one is fatal
    adg::generate_adapted_data(ehandler.as_ref())
        .map_err(|e| IntError::new(format!("failed to generate adapted data: {e}")))
}

fn update_adata(eversion: Option<String>, ahandler: &mut Box<dyn ad::AdaptedDataHandler>, adata: ad::AData) {
    let eversion = eversion.unwrap_or("none".into());
    let efp = get_efingerprint(&eversion);
    ahandler.update_data(adata, efp)
}
