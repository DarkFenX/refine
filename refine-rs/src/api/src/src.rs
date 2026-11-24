use crate::{Refine, SrcAlias, svc::SrcInnerGuarded};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
/// Data source.
///
/// Data source is a high-level entity which stores EVE data processed for the needs of the library.
/// You can have multiple and switch between them: for example, you can work with a solar system
/// using data from Tranquility server, then switch to source generated using data from Singularity
/// server.
///
/// Has an alias attached to it for ease of management.
pub struct Src<'refine> {
    pub(super) refine: &'refine Refine,
    pub(super) inner: SrcInnerGuarded,
}
impl<'refine> Src<'refine> {
    pub fn get_alias(&self) -> SrcAlias {
        self.inner.get_alias()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'refine> Src<'refine> {
    pub(super) fn new(refine: &'refine Refine, inner: SrcInnerGuarded) -> Self {
        Self { refine, inner }
    }
}
