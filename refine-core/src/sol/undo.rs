//! Rollback functionality works via transaction-alike interface.
//!
//! Internally, when transaction is initiated, it stores change log. When requested, the changes are
//! rolled back in LIFO order, to make sure solar system is roughly in the same state as it was
//! before the change. The pre-rollback state is not exact: it is guaranteed to keep solar system's
//! user settings, external IDs, and internal IDs of user entities (like fleets, fits, or items).

use crate::{MinionState, ModuleState, ud::UItemId};

/// Inverse steps of changes done to a solar system while recording.
#[derive(Clone)]
pub(crate) struct UndoLog {
    recording: bool,
    steps: Vec<UndoStep>,
}
impl UndoLog {
    pub(in crate::sol) fn new() -> Self {
        Self {
            recording: false,
            steps: Vec::new(),
        }
    }
    pub(crate) fn is_recording(&self) -> bool {
        self.recording
    }
    pub(crate) fn record(&mut self, step: UndoStep) {
        self.steps.push(step);
    }
    /// Starts recording, returning the point changes can be rolled back to.
    pub(crate) fn savepoint(&mut self) -> UndoSavepoint {
        let savepoint = UndoSavepoint {
            len: self.steps.len(),
            was_recording: self.recording,
        };
        self.recording = true;
        savepoint
    }
    /// Stops recording, so that undoing steps does not record their inverses.
    pub(crate) fn pause(&mut self) {
        self.recording = false;
    }
    /// Takes the newest step recorded after the savepoint.
    pub(crate) fn pop(&mut self, savepoint: &UndoSavepoint) -> Option<UndoStep> {
        match self.steps.len() > savepoint.len {
            true => self.steps.pop(),
            false => None,
        }
    }
    /// Restores recording state which was there before the savepoint.
    pub(crate) fn close(&mut self, savepoint: UndoSavepoint) {
        self.recording = savepoint.was_recording;
    }
}

/// Point in undo log which changes can be rolled back to.
#[must_use]
pub(crate) struct UndoSavepoint {
    len: usize,
    was_recording: bool,
}

/// Data needed to revert a single change done to a solar system.
#[derive(Clone)]
#[expect(clippy::enum_variant_names)]
pub(crate) enum UndoStep {
    AutochargeState(UItemId, bool),
    ChargeState(UItemId, bool),
    DroneState(UItemId, MinionState),
    FighterState(UItemId, MinionState),
    ModuleState(UItemId, ModuleState),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Debugging
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "dev")]
mod debug {
    use super::*;
    use crate::dbg::DebugResult;

    impl UndoLog {
        pub(in crate::sol) fn consistency_check(&self) -> DebugResult {
            // Changes are recorded only within calls which roll them back or commit them
            if self.recording || !self.steps.is_empty() {
                return Err(Default::default());
            }
            Ok(())
        }
    }
}
