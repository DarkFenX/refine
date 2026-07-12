use std::sync::Arc;

use crate::{refine::Refine, src::SrcAlias};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct Src<'r> {
    pub(super) refine: &'r Refine,
    pub(super) inner: SrcInnerGuarded,
}
impl<'r> Src<'r> {
    pub fn get_alias(&self) -> &SrcAlias {
        &self.inner.get_alias()
    }
}
impl<'r> Src<'r> {
    pub(super) fn new(refine: &'r Refine, inner: SrcInnerGuarded) -> Self {
        Self { refine, inner }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Inner guarded
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone)]
pub(crate) struct SrcInnerGuarded(Arc<SrcInner>);
impl SrcInnerGuarded {
    pub(super) fn new(alias: SrcAlias, core_src: Arc<rc::Src>) -> Self {
        Self(Arc::new(SrcInner { alias, core_src }))
    }
    pub(super) fn get_alias(&self) -> &SrcAlias {
        &self.0.alias
    }
    pub(crate) fn get_core(&self) -> &Arc<rc::Src> {
        &self.0.core_src
    }
    pub(super) fn ptr_eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Inner unguarded
////////////////////////////////////////////////////////////////////////////////////////////////////
struct SrcInner {
    alias: SrcAlias,
    core_src: Arc<rc::Src>,
}
