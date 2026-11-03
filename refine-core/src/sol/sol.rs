use std::ops::{Deref, DerefMut};

use crate::{
    Src,
    sol::{UndoLog, reuse::SolReuse, rev_projs::RevProjs},
    svc::Svc,
    ud::UData,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
/// Solar system, owned by the user.
///
/// Derefs to [`Sol`], which exposes the interface. Only operations unavailable within a
/// transaction live here: starting one, switching source and cloning.
pub type Core = SolGen<[CoreInner; 1]>;

/// Interface to a solar system.
///
/// Everything is accessed through it: fits, fleets and items are reached by their IDs, which
/// returns a short-lived handle borrowing the solar system. Since handles borrow mutably, only one
/// handle can be held at a time. Unsized, so it can't be moved, replaced or cloned through a
/// reference, e.g. the one a transaction gives out.
pub type Sol = SolGen<[CoreInner]>;

impl Core {
    pub fn new(src: &Src) -> Self {
        Self {
            inner: [CoreInner::new(src)],
        }
    }
}
impl Clone for Core {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}
impl Deref for Core {
    type Target = Sol;

    fn deref(&self) -> &Sol {
        self
    }
}
impl DerefMut for Core {
    fn deref_mut(&mut self) -> &mut Sol {
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
// The inner state sits in a one-element array, so that the owned form coerces to an unsized view,
// and the view gets a thin reference to the inner state back by indexing
#[doc(hidden)]
pub struct SolGen<T: ?Sized> {
    inner: T,
}

impl Sol {
    pub(crate) fn inner(&self) -> &CoreInner {
        &self.inner[0]
    }
    pub(crate) fn inner_mut(&mut self) -> &mut CoreInner {
        &mut self.inner[0]
    }
}

#[doc(hidden)]
#[derive(Clone)]
pub struct CoreInner {
    pub(crate) u_data: UData,
    pub(crate) svc: Svc,
    pub(crate) rev_projs: RevProjs,
    pub(crate) reuse: SolReuse,
    pub(crate) undo: UndoLog,
}
impl CoreInner {
    fn new(src: &Src) -> Self {
        Self {
            u_data: UData::new(src.r_data.clone()),
            svc: Svc::new(),
            rev_projs: RevProjs::new(),
            reuse: SolReuse::new(),
            undo: UndoLog::new(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Debugging
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "dev")]
mod debug {
    use super::*;
    use crate::dbg::DebugResult;

    impl CoreInner {
        pub(crate) fn internal_consistency_check(&self) -> DebugResult {
            // Check solar system structure
            self.u_data.consistency_check()?;
            // Check services
            self.svc.consistency_check(&self.u_data)?;
            // Check solar system helper data containers
            self.rev_projs.consistency_check(&self.u_data)?;
            self.undo.consistency_check()?;
            Ok(())
        }
    }
}
