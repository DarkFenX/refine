use crate::{
    sol::{UndoLog, reuse::SolReuse, rev_projs::RevProjs},
    src::Src,
    svc::Svc,
    ud::UData,
};

/// Holds all the user data, and exposes the interface to manipulate, process and fetch it.
///
/// Everything is accessed through a solar system: fits, fleets and items are reached by their IDs,
/// which returns a short-lived handle borrowing the solar system. Since handles borrow, only one
/// mutable handle can be held at a time.
pub struct SolarSystem {
    pub(crate) u_data: UData,
    pub(crate) svc: Svc,
    pub(crate) rev_projs: RevProjs,
    pub(crate) reuse: SolReuse,
    pub(crate) undo: UndoLog,
}
impl SolarSystem {
    pub fn new(src: &Src) -> Self {
        Self {
            u_data: UData::new(src.r_data.clone()),
            svc: Svc::new(),
            rev_projs: RevProjs::new(),
            reuse: SolReuse::new(),
            undo: UndoLog::new(),
        }
    }
}
impl Clone for SolarSystem {
    fn clone(&self) -> Self {
        // A clone made within a transaction would carry its log, with no transaction to close it
        assert!(
            !self.undo.is_recording(),
            "solar system cannot be cloned within a transaction"
        );
        Self {
            u_data: self.u_data.clone(),
            svc: self.svc.clone(),
            rev_projs: self.rev_projs.clone(),
            reuse: self.reuse.clone(),
            undo: self.undo.clone(),
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

    impl SolarSystem {
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
