//! Rollback functionality works via transaction-alike interface.
//!
//! Internally, when transaction is initiated, it stores change log. When requested, the changes are
//! rolled back in LIFO order, to make sure solar system is roughly in the same state as it was
//! before the change. The pre-rollback state is not exact: it is guaranteed to keep solar system's
//! user settings, external IDs, and internal IDs of user entities (like fleets, fits, or items).

use crate::{
    CountNz, EffectMode, ItemId, MinionState, ModuleState, MoveMode, NpcProp, OptionalReload, RearmMinion, RemoveMode,
    ServiceState, SkillLevel, Spool,
    ad::{AEffectId, AItemId},
    ud::{UAttrMutationRequest, UAutocharges, UItemId, UPhysics},
};

/// Inverse steps of changes done to a solar system while recording.
#[derive(Clone)]
pub(crate) struct UndoLog {
    recording: bool,
    steps: Vec<UndoStep>,
    // Data which does not fit into steps; it is recorded and taken together with its step
    autocharges: Vec<UAutocharges>,
    physics: Vec<UPhysics>,
    mutation_attrs: Vec<Vec<UAttrMutationRequest>>,
}
impl UndoLog {
    pub(in crate::sol) fn new() -> Self {
        Self {
            recording: false,
            steps: Vec::new(),
            autocharges: Vec::new(),
            physics: Vec::new(),
            mutation_attrs: Vec::new(),
        }
    }
    pub(crate) fn is_recording(&self) -> bool {
        self.recording
    }
    pub(crate) fn record(&mut self, step: UndoStep) {
        self.steps.push(step);
    }
    pub(crate) fn record_autocharges(&mut self, step: UndoStep, autocharges: UAutocharges) {
        self.steps.push(step);
        self.autocharges.push(autocharges);
    }
    pub(crate) fn pop_autocharges(&mut self) -> UAutocharges {
        self.autocharges.pop().unwrap()
    }
    pub(crate) fn record_physics(&mut self, step: UndoStep, physics: UPhysics) {
        self.steps.push(step);
        self.physics.push(physics);
    }
    pub(crate) fn pop_physics(&mut self) -> UPhysics {
        self.physics.pop().unwrap()
    }
    pub(crate) fn record_mutation_attrs(&mut self, step: UndoStep, attr_mutations: Vec<UAttrMutationRequest>) {
        self.steps.push(step);
        self.mutation_attrs.push(attr_mutations);
    }
    pub(crate) fn pop_mutation_attrs(&mut self) -> Vec<UAttrMutationRequest> {
        self.mutation_attrs.pop().unwrap()
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
    BoosterType(UItemId, AItemId),
    // Item - character
    CharacterAdded(UItemId),
    CharacterRemoved(UItemId),
    CharacterState(UItemId, bool),
    CharacterType(UItemId, AItemId),
    // Item - charge
    ChargeAdded(UItemId),
    ChargeRemoved(UItemId),
    ChargeState(UItemId, bool),
    ChargeType(UItemId, AItemId),
    // Item - drone
    DroneAdded(UItemId),
    DroneMutationAdded(UItemId),
    DroneMutationAttrs(UItemId),
    DroneMutationRemoved(UItemId, AItemId),
    DroneMutator(UItemId, AItemId),
    DroneNpcPropOverride(UItemId, Option<NpcProp>),
    DronePhysics(UItemId),
    DroneRemoved(UItemId),
    DroneState(UItemId, MinionState),
    DroneType(UItemId, AItemId),
    // Item - fighter
    FighterAdded(UItemId),
    FighterCountOverride(UItemId, Option<CountNz>),
    FighterPhysics(UItemId),
    FighterRearmMinionOverride(UItemId, Option<RearmMinion>),
    FighterRemoved(UItemId),
    FighterState(UItemId, MinionState),
    FighterType(UItemId, AItemId),
    // Item - fit-wide effect
    FwEffectAdded(UItemId),
    FwEffectRemoved(UItemId),
    FwEffectState(UItemId, bool),
    FwEffectType(UItemId, AItemId),
    // Item - implant
    ImplantAdded(UItemId),
    ImplantRemoved(UItemId),
    ImplantState(UItemId, bool),
    ImplantType(UItemId, AItemId),
    // Item -module
    ModuleAdded(UItemId, RemoveMode),
    ModuleMoved(UItemId, MoveMode),
    ModuleMutationAdded(UItemId),
    ModuleMutationAttrs(UItemId),
    ModuleMutationRemoved(UItemId, AItemId),
    ModuleMutator(UItemId, AItemId),
    ModuleOptionalReloadOverride(UItemId, Option<OptionalReload>),
    ModuleRemoved(UItemId, RemoveMode),
    ModuleSpoolOverride(UItemId, Option<Spool>),
    ModuleState(UItemId, ModuleState),
    ModuleType(UItemId, AItemId),
    // Item - projected effect
    ProjEffectAdded(UItemId),
    ProjEffectRemoved(UItemId),
    ProjEffectState(UItemId, bool),
    ProjEffectType(UItemId, AItemId),
    // Item - rig
    RigAdded(UItemId),
    RigRemoved(UItemId),
    RigState(UItemId, bool),
    RigType(UItemId, AItemId),
    // Item - service
    ServiceAdded(UItemId),
    ServiceRemoved(UItemId),
    ServiceState(UItemId, ServiceState),
    ServiceType(UItemId, AItemId),
    // Item - ship
    ShipAdded(UItemId),
    ShipPhysics(UItemId),
    ShipRemoved(UItemId),
    ShipState(UItemId, bool),
    ShipType(UItemId, AItemId),
    // Item - skill
    SkillAdded(UItemId),
    SkillLevel(UItemId, SkillLevel),
    SkillRemoved(UItemId),
    SkillState(UItemId, bool),
    SkillType(UItemId, AItemId),
    // Item - stance
    StanceAdded(UItemId),
    StanceRemoved(UItemId),
    StanceState(UItemId, bool),
    StanceType(UItemId, AItemId),
    // Item - subsystem
    SubsystemAdded(UItemId),
    SubsystemRemoved(UItemId),
    SubsystemState(UItemId, bool),
    SubsystemType(UItemId, AItemId),
    // Item - system-wide effect
    SwEffectAdded(UItemId),
    SwEffectRemoved(UItemId),
    SwEffectState(UItemId, bool),
    SwEffectType(UItemId, AItemId),
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
            if self.recording
                || !self.steps.is_empty()
                || !self.physics.is_empty()
                || !self.mutation_attrs.is_empty()
                || !self.autocharges.is_empty()
            {
                return Err(Default::default());
            }
            Ok(())
        }
    }
}
