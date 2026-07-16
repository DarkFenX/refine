use crate::{Refine, src::SrcAlias, svc::SrcInnerGuarded};

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
