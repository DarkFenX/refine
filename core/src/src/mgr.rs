use std::{collections::HashMap, rc::Rc};

use log;

use crate::{cg, ch::CacheHandler, dh::DataHandler, Error, Result, VERSION};

use super::src::Src;

/// Source manager.
///
/// Source manager is a top-level entity which handles everything related to EVE data. It allows
/// other parts of the library to conveniently switch between different data versions (for example,
/// from Tranquility data to Singularity data).
pub struct SrcMgr {
    sources: HashMap<String, Rc<Src>>,
    default: Option<Rc<Src>>,
}
impl SrcMgr {
    /// Construct new `SrcMgr`.
    pub fn new() -> SrcMgr {
        SrcMgr {
            sources: HashMap::new(),
            default: None,
        }
    }
    /// Add new data source with custom alias, and optionally make it default.
    pub fn add(
        &mut self,
        alias: &str,
        data_handler: Box<dyn DataHandler>,
        mut cache_handler: Box<dyn CacheHandler>,
        make_default: bool,
    ) -> Result<()> {
        log::info!("adding source with alias \"{}\", default={}", alias, make_default);
        log::info!("using {:?} as data handler", data_handler);
        log::info!("using {:?} as cache handler", cache_handler);
        if self.sources.contains_key(alias) {
            return Err(Error::new(format!("source with alias \"{}\" already exists", alias)));
        }
        let mut regen = false;
        // Failure to read version is not fatal, we just always generate cache in this case
        let data_version = match data_handler.get_version() {
            Ok(v) => v,
            Err(e) => {
                log::info!("unable to get data version: {}", e);
                regen = true;
                String::new()
            }
        };
        if !regen {
            // Failure to load cache is not fatal as well
            match cache_handler.load_cache() {
                Ok(_) => (),
                Err(e) => {
                    log::info!("unable to load cache: {}", e);
                    regen = true;
                }
            }
        }
        let data_fp = format!("{}_{}", data_version, VERSION);
        if !regen {
            let cache_fp = cache_handler.get_fingerprint();
            if &data_fp != cache_fp {
                log::info!("fingerprint mismatch: {} data vs {} cache", data_fp, cache_fp);
                regen = true
            };
        }
        if regen {
            log::info!("regenerating cache...");
            // If we have to regenerate cache, failure to generate one is fatal
            let ch_data = cg::generate_cache(data_handler.as_ref())
                .map_err(|e| Error::new(format!("failed to generate cache: {}", e)))?;
            cache_handler.update_cache(ch_data, data_fp);
        }
        let src = Rc::new(Src::new(alias.into(), cache_handler));
        if make_default {
            self.default = Some(src.clone());
        };
        self.sources.insert(src.alias.clone(), src);
        Ok(())
    }

    pub(crate) fn get(&self, alias: &str) -> Option<&Rc<Src>> {
        self.sources.get(alias)
    }

    pub(crate) fn get_default(&self) -> Option<&Rc<Src>> {
        self.default.as_ref()
    }

    /// Remove data source which was stored against passed alias.
    pub fn del(&mut self, alias: &str) -> Result<()> {
        log::info!("removing source with alias \"{}\"", alias);
        self.sources
            .remove(alias)
            .ok_or(Error::new(format!("no source with alias \"{}\"", alias)))?;
        match &self.default {
            Some(s) if s.alias == alias => self.default = None,
            _ => (),
        };
        Ok(())
    }
}
