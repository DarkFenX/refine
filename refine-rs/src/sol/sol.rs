use std::sync::Arc;

use tokio::sync::{Mutex, MutexGuard, OwnedMutexGuard, TryLockError};
use tokio_rayon::AsyncThreadPool;

use crate::{refine::Refine, sol::SolarSystemId};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct SolarSystem<'r> {
    pub(crate) refine: &'r Refine,
    pub(super) id: SolarSystemId,
    inner: SolOwnedMutexGuard,
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
    pub(crate) async fn exec_std_fallible<T, E, F>(&mut self, func: F) -> Result<T, E>
    where
        T: Send + 'static,
        E: Send + 'static,
        F: Fn(&mut rc::SolarSystem) -> Result<T, E> + Send + Sync + 'static,
    {
        let mut core_sol = self.take_core().unwrap();
        let core_sol_backup = core_sol.clone();
        let sync_span = tracing::trace_span!("sync");
        match self
            .refine
            .tpool
            .standard
            .spawn_fifo_async(move || {
                let _sg = sync_span.enter();
                let ret = func(&mut core_sol)?;
                Ok((core_sol, ret))
            })
            .await
        {
            Ok((core_sol, ret)) => {
                self.put_core_back(core_sol);
                Ok(ret)
            }
            Err(error) => {
                self.put_core_back(core_sol_backup);
                Err(error)
            }
        }
    }
    pub(crate) fn get_inner(&mut self) -> &mut SolarSystemInner {
        &mut self.inner
    }
    fn take_core(&mut self) -> Option<Box<rc::SolarSystem>> {
        self.inner.take_core()
    }
    fn put_core_back(&mut self, core_sol: Box<rc::SolarSystem>) {
        self.inner.put_core_back(core_sol);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Inner guarded
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone)]
pub(crate) struct SolarSystemInnerGuarded(Arc<Mutex<SolarSystemInner>>);
impl SolarSystemInnerGuarded {
    pub(super) fn new(core_sol: rc::SolarSystem) -> Self {
        Self(Arc::new(Mutex::new(SolarSystemInner::new(core_sol))))
    }
    fn try_lock(&self) -> Result<MutexGuard<'_, SolarSystemInner>, TryLockError> {
        self.0.try_lock()
    }
    // Like regular lock, but updates timestamp on inner sol during drop
    pub(super) async fn lock_touch_owned(&self) -> SolOwnedMutexGuard {
        SolOwnedMutexGuard {
            guard: self.0.clone().lock_owned().await,
        }
    }
    pub(super) async fn into_lock_touch_owned(self) -> SolOwnedMutexGuard {
        SolOwnedMutexGuard {
            guard: self.0.lock_owned().await,
        }
    }
}

struct SolOwnedMutexGuard {
    guard: OwnedMutexGuard<SolarSystemInner>,
}
impl Drop for SolOwnedMutexGuard {
    fn drop(&mut self) {
        self.guard.touch();
    }
}
impl std::ops::Deref for SolOwnedMutexGuard {
    type Target = SolarSystemInner;
    fn deref(&self) -> &Self::Target {
        &self.guard
    }
}
impl<'m> std::ops::DerefMut for SolOwnedMutexGuard {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.guard
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Inner unguarded
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(crate) struct SolarSystemInner {
    accessed: chrono::DateTime<chrono::Utc>,
    core_sol: Option<Box<rc::SolarSystem>>,
}
impl SolarSystemInner {
    fn new(core_sol: rc::SolarSystem) -> Self {
        Self {
            accessed: chrono::Utc::now(),
            core_sol: Some(Box::new(core_sol)),
        }
    }
    fn touch(&mut self) {
        self.accessed = chrono::Utc::now();
    }
    fn take_core(&mut self) -> Option<Box<rc::SolarSystem>> {
        self.core_sol.take()
    }
    fn put_core_back(&mut self, core_sol: Box<rc::SolarSystem>) {
        self.core_sol = Some(core_sol);
    }
}
