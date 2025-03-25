use std::sync::Arc;

use crate::{ad, ed, src::SrcInitError};

use super::inner::InnerSrc;

/// Data source.
///
/// Data source is a top-level entity which manages EVE and adapted data handlers to do necessary
/// preparations and expose adapted data to solar system and its services.
#[derive(Clone)]
pub struct Src {
    inner: Arc<InnerSrc>,
}
impl Src {
    #[tracing::instrument(name = "src-new", level = "trace", skip_all)]
    pub fn new(
        e_handler: Box<dyn ed::EveDataHandler>,
        a_handler: Box<dyn ad::AdaptedDataHandler>,
    ) -> Result<Self, SrcInitError> {
        let inner_src = InnerSrc::new(e_handler, a_handler)?;
        let src = Self {
            inner: Arc::new(inner_src),
        };
        Ok(src)
    }
    pub(crate) fn get_a_item(&self, id: &ad::AItemId) -> Option<&ad::ArcItem> {
        self.inner.a_handler.get_item(id)
    }
    pub(crate) fn get_a_attr(&self, id: &ad::AAttrId) -> Option<&ad::ArcAttr> {
        self.inner.a_handler.get_attr(id)
    }
    pub(crate) fn get_a_effect(&self, id: &ad::AEffectId) -> Option<&ad::ArcEffect> {
        self.inner.a_handler.get_effect(id)
    }
    pub(crate) fn get_a_mutator(&self, id: &ad::AItemId) -> Option<&ad::ArcMuta> {
        self.inner.a_handler.get_mutator(id)
    }
    pub(crate) fn get_a_buff(&self, id: &ad::ABuffId) -> Option<&ad::ArcBuff> {
        self.inner.a_handler.get_buff(id)
    }
}
