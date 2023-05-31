use crate::{
    defs::VERSION,
    edh, erg, erh,
    util::{Error, ErrorKind, IntError, IntResult, Result},
};

/// Data source.
///
/// Data source is a top-level entity which manages data and cache handlers to process data and
/// expose it to other parts of the library.
#[derive(Debug)]
pub struct Src {
    pub(crate) cache_handler: Box<dyn erh::CacheHandler>,
}
impl Src {
    pub fn new(
        data_handler: Box<dyn edh::EveDataHandler>,
        mut cache_handler: Box<dyn erh::CacheHandler>,
    ) -> Result<Self> {
        log::info!(
            "initializing new source with {:?} and {:?}",
            data_handler,
            cache_handler
        );
        let dv = get_data_version(&data_handler);
        if need_cache_regen(dv.clone(), &mut cache_handler) {
            let ch_data = regen_cache(&data_handler).map_err(|e| Error::new(ErrorKind::SrcCacheGenFailed(e.msg)))?;
            update_cache(dv, &mut cache_handler, ch_data);
        }
        Ok(Self { cache_handler })
    }
}

fn get_data_version(data_handler: &Box<dyn edh::EveDataHandler>) -> Option<String> {
    match data_handler.get_version() {
        Ok(dv) => Some(dv),
        Err(e) => {
            log::info!("unable to get data version: {}", e);
            None
        }
    }
}

fn get_data_fingerprint(data_version: &str) -> String {
    format!("{}_{}", data_version, VERSION)
}

fn need_cache_regen(data_version: Option<String>, cache_handler: &mut Box<dyn erh::CacheHandler>) -> bool {
    // Failure to read version is not fatal, we just always generate cache in this case
    let data_version = match data_version {
        Some(dv) => dv,
        None => return true,
    };
    // Failure to load cache is not fatal as well
    match cache_handler.load_cache() {
        Ok(_) => (),
        Err(e) => {
            log::info!("unable to load cache: {}", e);
            return true;
        }
    }
    let data_fp = get_data_fingerprint(&data_version);
    let cache_fp = cache_handler.get_fingerprint();
    match cache_fp {
        Some(f) => {
            if &data_fp != f {
                log::info!("fingerprint mismatch: {} data vs {} cache", data_fp, f);
                return true;
            };
        }
        None => {
            log::info!("cache returned no data fingerprint");
            return true;
        }
    }
    false
}

fn regen_cache(data_handler: &Box<dyn edh::EveDataHandler>) -> IntResult<erh::Data> {
    log::info!("regenerating cache...");
    // If we have to regenerate cache, failure to generate one is fatal
    erg::generate_erts(data_handler.as_ref()).map_err(|e| IntError::new(format!("failed to generate cache: {}", e)))
}

fn update_cache(data_version: Option<String>, cache_handler: &mut Box<dyn erh::CacheHandler>, ch_data: erh::Data) {
    let data_version = data_version.unwrap_or("none".into());
    let data_fp = get_data_fingerprint(&data_version);
    cache_handler.update_cache(ch_data, data_fp)
}
