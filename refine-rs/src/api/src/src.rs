use crate::{Refine, src::SrcAlias, svc::SrcInnerGuarded};

/// Data source.
///
/// Data source is a high-level entity which stores EVE data processed for the needs of the library.
/// You can have multiple and switch between them: for example, you can work with a solar system
/// using data from Tranquility server, then switch to source generated using data from Singularity
/// server.
///
/// Has an alias attached to it for ease of management.
pub struct Src<'r> {
    pub(super) refine: &'r Refine,
    pub(super) inner: SrcInnerGuarded,
}
impl<'r> Src<'r> {
    pub fn get_alias(&self) -> &SrcAlias {
        &self.inner.get_alias()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Non-public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'r> Src<'r> {
    pub(super) fn new(refine: &'r Refine, inner: SrcInnerGuarded) -> Self {
        Self { refine, inner }
    }
}
