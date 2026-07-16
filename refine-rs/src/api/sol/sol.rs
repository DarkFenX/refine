use crate::{
    Refine, SolarSystemId,
    svc::{SolOwnedMutexGuard, SolarSystemInnerGuarded},
};

pub struct SolarSystem<'r> {
    pub(crate) refine: &'r Refine,
    pub(super) id: SolarSystemId,
    pub(crate) inner: SolOwnedMutexGuard,
}
impl<'r> SolarSystem<'r> {
    pub fn get_id(&self) -> SolarSystemId {
        self.id
    }
}
// Private part
impl<'r> SolarSystem<'r> {
    pub(super) async fn new(refine: &'r Refine, id: SolarSystemId, inner: SolarSystemInnerGuarded) -> Self {
        Self {
            refine,
            id,
            inner: inner.into_lock_touch_owned().await,
        }
    }
}
