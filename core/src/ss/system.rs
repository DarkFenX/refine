use std::sync::Arc;

use crate::{
    src::{Src, SrcMgr},
    Error, ErrorKind, Result,
};

pub struct SolarSystem {
    srcmgr: SrcMgr,
    src: Arc<Src>,
}
impl SolarSystem {
    pub fn new(srcmgr: SrcMgr) -> Result<SolarSystem> {
        let src = srcmgr
            .get_default()
            .ok_or_else(|| Error::new(ErrorKind::SrcNotFound, "default source is not found"))?;
        Ok(SolarSystem { srcmgr, src })
    }
    pub fn new_with_alias(srcmgr: SrcMgr, alias: &str) -> Result<SolarSystem> {
        let src = srcmgr.get(alias).ok_or_else(|| {
            Error::new(
                ErrorKind::SrcNotFound,
                format!("source with alias \"{}\" is not found", alias),
            )
        })?;
        Ok(SolarSystem { srcmgr, src })
    }
}
