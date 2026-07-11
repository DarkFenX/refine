use std::sync::Arc;

use crate::{refine::Refine, src::SrcAlias};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct Src<'a> {
    pub(super) refine: &'a mut Refine,
    pub(super) inner: SrcInner,
}
impl<'a> Src<'a> {
    pub fn get_alias(&self) -> &SrcAlias {
        &self.inner.alias
    }
}
impl<'a> Src<'a> {
    pub(super) fn new(refine: &'a mut Refine, inner: SrcInner) -> Self {
        Self { refine, inner }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Inner
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(crate) struct SrcInner {
    pub(super) alias: SrcAlias,
    pub(super) core_src: Arc<rc::Src>,
}
impl SrcInner {
    pub(super) fn new(alias: SrcAlias, core_src: Arc<rc::Src>) -> Self {
        Self { alias, core_src }
    }
}
