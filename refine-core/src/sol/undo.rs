//! Rollback functionality works via transaction-alike interface.
//!
//! Internally, when transaction is initiated, it stores change log. When requested, the changes are
//! rolled back in LIFO order, to make sure solar system is roughly in the same state as it was
//! before the change. The pre-rollback state is not exact: it is guaranteed to keep solar system's
//! user settings, external IDs, and internal IDs of user entities (like fleets, fits, or items).

use crate::{
    CountNz, EffectMode, ItemId, MinionState, ModuleState, MoveMode, NpcProp, OptionalReload, RearmMinion, RemoveMode,
    ServiceState, SkillLevel, Spool,
    ad::AEffectId,
    ud::{UItemId, UPhysics},
};

/// Inverse steps of changes done to a solar system while recording.
#[derive(Clone)]
pub(crate) struct UndoLog {
    recording: bool,
    steps: Vec<UndoStep>,
    // Data which does not fit into steps; it is recorded and taken together with its step
    physics: Vec<UPhysics>,
}
impl UndoLog {
    pub(in crate::sol) fn new() -> Self {
        Self {
            recording: false,
            steps: Vec::new(),
            physics: Vec::new(),
        }
    }
    pub(crate) fn is_recording(&self) -> bool {
        self.recording
    }
    pub(crate) fn record(&mut self, step: UndoStep) {
        self.steps.push(step);
    }
    pub(crate) fn record_physics(&mut self, step: UndoStep, physics: UPhysics) {
        self.steps.push(step);
        self.physics.push(physics);
    }
    pub(crate) fn pop_physics(&mut self) -> UPhysics {
        self.physics.pop().unwrap()
    }
    /// Starts recording, returning the point changes can be rolled back to.
    pub(crate) fn savepoint(&mut self, item_id_counter: ItemId) -> UndoSavepoint {
        let savepoint = UndoSavepoint {
            len: self.steps.len(),
            was_recording: self.recording,
            item_id_counter,
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
    // Rollback removes everything added after the savepoint, so ID allocation can resume from here
    pub(crate) item_id_counter: ItemId,
}

/// Data needed to revert a single change done to a solar system.
#[derive(Clone)]
pub(crate) enum UndoStep {
    // Item - autocharge
    AutochargeState(UItemId, bool),
    // Item - booster
    BoosterAdded(UItemId),
    BoosterRemoved(UItemId),
    BoosterState(UItemId, bool),
    // Item - character
    CharacterAdded(UItemId),
    CharacterRemoved(UItemId),
    CharacterState(UItemId, bool),
    // Item - charge
    ChargeAdded(UItemId),
    ChargeRemoved(UItemId),
    ChargeState(UItemId, bool),
    // Item - drone
    DroneAdded(UItemId),
    DroneNpcPropOverride(UItemId, Option<NpcProp>),
    DronePhysics(UItemId),
    DroneRemoved(UItemId),
    DroneState(UItemId, MinionState),
    // Item - fighter
    FighterAdded(UItemId),
    FighterCountOverride(UItemId, Option<CountNz>),
    FighterPhysics(UItemId),
    FighterRearmMinionOverride(UItemId, Option<RearmMinion>),
    FighterRemoved(UItemId),
    FighterState(UItemId, MinionState),
    // Item - fit-wide effect
    FwEffectAdded(UItemId),
    FwEffectRemoved(UItemId),
    FwEffectState(UItemId, bool),
    // Item - implant
    ImplantAdded(UItemId),
    ImplantRemoved(UItemId),
    ImplantState(UItemId, bool),
    // Item -module
    ModuleAdded(UItemId, RemoveMode),
    ModuleMoved(UItemId, MoveMode),
    ModuleOptionalReloadOverride(UItemId, Option<OptionalReload>),
    ModuleRemoved(UItemId, RemoveMode),
    ModuleSpoolOverride(UItemId, Option<Spool>),
    ModuleState(UItemId, ModuleState),
    // Item - projected effect
    ProjEffectAdded(UItemId),
    ProjEffectRemoved(UItemId),
    ProjEffectState(UItemId, bool),
    // Item - rig
    RigAdded(UItemId),
    RigRemoved(UItemId),
    RigState(UItemId, bool),
    // Item - service
    ServiceAdded(UItemId),
    ServiceRemoved(UItemId),
    ServiceState(UItemId, ServiceState),
    // Item - ship
    ShipAdded(UItemId),
    ShipPhysics(UItemId),
    ShipRemoved(UItemId),
    ShipState(UItemId, bool),
    // Item - skill
    SkillAdded(UItemId),
    SkillLevel(UItemId, SkillLevel),
    SkillRemoved(UItemId),
    SkillState(UItemId, bool),
    // Item - stance
    StanceAdded(UItemId),
    StanceRemoved(UItemId),
    StanceState(UItemId, bool),
    // Item - subsystem
    SubsystemAdded(UItemId),
    SubsystemRemoved(UItemId),
    SubsystemState(UItemId, bool),
    // Item - system-wide effect
    SwEffectAdded(UItemId),
    SwEffectRemoved(UItemId),
    SwEffectState(UItemId, bool),
    // Item - general
    ItemEffectMode(UItemId, AEffectId, EffectMode),
    ProjAdded(UItemId, UItemId),   // Projector and projectee
    ProjRemoved(UItemId, UItemId), // Projector and projectee
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
            if self.recording || !self.steps.is_empty() || !self.physics.is_empty() {
                return Err(Default::default());
            }
            Ok(())
        }
    }
}
