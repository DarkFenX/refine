use crate::{
    Refine, SolarSystemId,
    src::SrcAlias,
    svc::{SolInnerGuarded, SolOwnedMutexGuard},
};

pub struct SolarSystem<'r> {
    pub(crate) refine: &'r Refine,
    pub(crate) inner: SolOwnedMutexGuard,
}
impl<'r> SolarSystem<'r> {
    pub fn get_id(&self) -> SolarSystemId {
        self.inner.get_id()
    }
    pub fn get_src_alias(&self) -> SrcAlias {
        self.inner.get_src_alias()
    }
}
// Private part
impl<'r> SolarSystem<'r> {
    pub(super) async fn new(refine: &'r Refine, inner: SolInnerGuarded) -> Self {
        Self {
            refine,
            inner: inner.into_lock_touch_owned().await,
        }
    }
}
