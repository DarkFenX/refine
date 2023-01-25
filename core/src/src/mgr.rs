use std::collections::HashMap;

use log;

use crate::{
    cg,
    ch::CacheHandler,
    defines::VERSION,
    dh::DataHandler,
    util::{Error, Result},
};

use super::src::Src;

pub struct SrcMgr<'d, CH>
where
    CH: CacheHandler,
{
    sources: HashMap<String, Src<CH>>,
    default: Option<&'d Src<CH>>,
}
impl<'d, CH> SrcMgr<'d, CH>
where
    CH: CacheHandler,
{
    pub fn new() -> SrcMgr<'d, CH> {
        SrcMgr::<CH> {
            sources: HashMap::new(),
            default: None,
        }
    }

    pub fn add<A: Into<String>, DH: DataHandler>(
        &mut self,
        alias: A,
        data_handler: DH,
        mut cache_handler: CH,
        make_default: bool,
    ) -> Result<()> {
        let alias = alias.into();
        log::info!("adding source with alias \"{}\"", alias);
        if self.sources.contains_key(&alias) {
            return Err(Error::new(format!("source with alias \"{}\" already exists", alias)));
        }
        let mut regen = false;
        // Failure to read version is not fatal, we just always generate cache in this case
        let data_version = match data_handler.get_version() {
            Ok(v) => v,
            Err(e) => {
                log::info!("unable to get version: {}", e);
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
                regen = true
            };
        }
        if regen {
            // If we have to regenerate cache, failure to generate one is fatal
            let ch_data = cg::generate_cache(&data_handler)
                .map_err(|e| Error::new(format!("failed to generate cache: {}", e)))?;
            cache_handler.update_cache(ch_data, data_fp);
        }
        let src = Src::new(alias, cache_handler);
        // if make_default {
        //     self.default = Some(&src)
        // };
        self.sources.insert(src.alias.clone(), src);
        Ok(())
    }

    pub fn get<A: Into<String>>(&self, alias: A) -> Option<&Src<CH>> {
        self.sources.get(alias.into().as_str())
    }
}
