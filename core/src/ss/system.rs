use std::sync::Arc;

use crate::{
    src::{Src, SrcMgr},
    Error, ErrorKind, Result,
};

pub struct SolarSystem {
    src_mgr: Arc<SrcMgr>,
    src: Arc<Src>,
}
impl SolarSystem {
    pub fn new(src_mgr: Arc<SrcMgr>) -> Result<SolarSystem> {
        let src = src_mgr
            .get_default()
            .ok_or_else(|| Error::new(ErrorKind::SrcNotFound, "no default source assigned"))?;
        Ok(SolarSystem { src_mgr, src })
    }
    pub fn new_with_alias(src_mgr: Arc<SrcMgr>, alias: &str) -> Result<SolarSystem> {
        let src = src_mgr.get(alias).ok_or_else(|| {
            Error::new(
                ErrorKind::SrcNotFound,
                format!("source with alias \"{}\" is not found", alias),
            )
        })?;
        Ok(SolarSystem { src_mgr, src })
    }
}
