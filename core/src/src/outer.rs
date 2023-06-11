use std::sync::Arc;

use crate::{ad, defs::ReeInt, ed, util::Result};

use super::inner::InnerSrc;

/// Data source.
///
/// Data source is a top-level entity which manages EVE and adapted data handlers to do necessary
/// preparations and expose adapted data to solar system and its services.
#[derive(Clone, Debug)]
pub struct Src {
    inner: Arc<InnerSrc>,
}
impl Src {
    pub fn new(e_handler: Box<dyn ed::EveDataHandler>, a_handler: Box<dyn ad::AdaptedDataHandler>) -> Result<Self> {
        let inner_src = InnerSrc::new(e_handler, a_handler)?;
        let src = Self {
            inner: Arc::new(inner_src),
        };
        Ok(src)
    }
    pub(crate) fn get_a_item(&self, id: &ReeInt) -> Option<Arc<ad::AItem>> {
        self.inner.a_handler.get_item(id)
    }
    pub(crate) fn get_a_attr(&self, id: &ReeInt) -> Option<Arc<ad::AAttr>> {
        self.inner.a_handler.get_attr(id)
    }
    pub(crate) fn get_a_effect(&self, id: &ReeInt) -> Option<Arc<ad::AEffect>> {
        self.inner.a_handler.get_effect(id)
    }
    pub(crate) fn get_a_muta(&self, id: &ReeInt) -> Option<Arc<ad::AMuta>> {
        self.inner.a_handler.get_muta(id)
    }
    pub(crate) fn get_a_buff(&self, id: &ReeInt) -> Option<Arc<ad::ABuff>> {
        self.inner.a_handler.get_buff(id)
    }
}
