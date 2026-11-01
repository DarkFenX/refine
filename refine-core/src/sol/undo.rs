//! Rollback functionality works via transaction-alike interface.
//!
//! Internally, when transaction is initiated, it stores change log. When requested, the changes are
//! rolled back in LIFO order, to make sure solar system is roughly in the same state as it was
//! before the change. The pre-rollback state is not exact: it is guaranteed to keep solar system's
//! user settings, external IDs, and internal IDs of user entities (like fleets, fits, or items).

use crate::{
    CountNz, DpsProfile, EffectMode, FitId, FitSecStatus, FleetId, ItemId, MinionState, ModuleState, MoveMode, NpcProp,
    OptionalReload, RearmMinion, RemoveMode, SecZone, ServiceState, SkillLevel, Spool,
    ad::{AEffectId, AItemId},
    ud::{UAttrMutationRequest, UAutocharges, UData, UFitId, UFleetId, UItemId, UPhysics},
};

/// Data needed to revert a single change done to a solar system.
#[derive(Clone)]
pub(crate) enum UndoStep {
    // Solar system
    SecZone(SecZone),
    DefaultNpcProp(NpcProp),
    DefaultOptionalReloads(OptionalReload),
    DefaultRearmMinions(RearmMinion),
    DefaultSpool(Spool),
    DefaultIncomingDps,
    // Fleet
    FleetAdded(UFleetId),
    FleetRemoved(UFleetId),
    // Fit
    FitAdded(UFitId),
    FitRemoved(UFitId),
    FitFleet(UFitId, Option<UFleetId>),
    FitRahIncomingDps(UFitId),
    FitSecStatus(UFitId, FitSecStatus),
    // Item - general
    ItemEffectMode(UItemId, AEffectId, EffectMode),
    ProjAdded(UItemId, UItemId),   // Projector and projectee
    ProjRemoved(UItemId, UItemId), // Projector and projectee
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
    DroneRemoved(UItemId),
    DroneMutationAdded(UItemId),
    DroneMutationAttrs(UItemId),
    DroneMutationRemoved(UItemId, AItemId),
    DroneMutator(UItemId, AItemId),
    DroneNpcPropOverride(UItemId, Option<NpcProp>),
    DronePhysics(UItemId),
    DroneState(UItemId, MinionState),
    DroneType(UItemId, AItemId),
    // Item - fighter
    FighterAdded(UItemId),
    FighterRemoved(UItemId),
    FighterCountOverride(UItemId, Option<CountNz>),
    FighterPhysics(UItemId),
    FighterRearmMinionOverride(UItemId, Option<RearmMinion>),
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
    ModuleRemoved(UItemId, RemoveMode),
    ModuleMoved(UItemId, MoveMode),
    ModuleMutationAdded(UItemId),
    ModuleMutationAttrs(UItemId),
    ModuleMutationRemoved(UItemId, AItemId),
    ModuleMutator(UItemId, AItemId),
    ModuleOptionalReloadOverride(UItemId, Option<OptionalReload>),
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
    ShipRemoved(UItemId),
    ShipPhysics(UItemId),
    ShipState(UItemId, bool),
    ShipType(UItemId, AItemId),
    // Item - skill
    SkillAdded(UItemId),
    SkillRemoved(UItemId),
    SkillLevel(UItemId, SkillLevel),
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
}

/// Point in undo log which changes can be rolled back to.
#[must_use]
pub(crate) struct UndoSavepoint {
    len: usize,
    was_recording: bool,
    // Rollback removes everything added after the savepoint, so ID allocation can resume from here
    pub(crate) fleet_id_counter: FleetId,
    pub(crate) fit_id_counter: FitId,
    pub(crate) item_id_counter: ItemId,
}

/// Inverse steps of changes done to a solar system while recording.
#[derive(Clone)]
pub(crate) struct UndoLog {
    recording: bool,
    steps: Vec<UndoStep>,
    // Data which does not fit into steps; it is recorded and taken together with its step
    dps_profiles: Vec<Option<DpsProfile>>,
    autocharges: Vec<UAutocharges>,
    physics: Vec<UPhysics>,
    mutation_attrs: Vec<Vec<UAttrMutationRequest>>,
}
impl UndoLog {
    pub(in crate::sol) fn new() -> Self {
        Self {
            recording: false,
            steps: Vec::new(),
            dps_profiles: Vec::new(),
            autocharges: Vec::new(),
            physics: Vec::new(),
            mutation_attrs: Vec::new(),
        }
    }
    /// Starts recording, returning the point changes can be rolled back to.
    pub(crate) fn savepoint(
        &mut self,
        fleet_id_counter: FleetId,
        fit_id_counter: FitId,
        item_id_counter: ItemId,
    ) -> UndoSavepoint {
        let savepoint = UndoSavepoint {
            len: self.steps.len(),
            was_recording: self.recording,
            fleet_id_counter,
            fit_id_counter,
            item_id_counter,
        };
        self.recording = true;
        savepoint
    }
    /// Stops recording, so that undoing steps does not record their inverses.
    pub(crate) fn pause(&mut self) {
        self.recording = false;
    }
    /// Restores recording state which was there before the savepoint.
    pub(crate) fn close(&mut self, savepoint: UndoSavepoint) {
        self.recording = savepoint.was_recording;
    }
    /// Keeps all the recorded changes: stops recording, clears the log, and evicts entities which
    /// were unmapped while recording.
    pub(crate) fn commit(&mut self, u_data: &mut UData) {
        for step in self.steps.drain(..) {
            match step {
                UndoStep::FleetRemoved(fleet_uid) => {
                    u_data.fleets.evict(fleet_uid);
                }
                UndoStep::FitRemoved(fit_uid) => {
                    u_data.fits.evict(fit_uid);
                }
                // Children are discarded along with their parent, and have no steps of their own
                UndoStep::FighterRemoved(fighter_uid) => {
                    let u_item = u_data.items.evict(fighter_uid);
                    for autocharge_uid in u_item.dc_fighter().unwrap().get_autocharges().values() {
                        u_data.items.evict(autocharge_uid);
                    }
                }
                UndoStep::ModuleRemoved(module_uid, _) => {
                    let u_item = u_data.items.evict(module_uid);
                    if let Some(charge_uid) = u_item.dc_module().unwrap().get_charge_uid() {
                        u_data.items.evict(charge_uid);
                    }
                }
                UndoStep::BoosterRemoved(item_uid)
                | UndoStep::CharacterRemoved(item_uid)
                | UndoStep::ChargeRemoved(item_uid)
                | UndoStep::DroneRemoved(item_uid)
                | UndoStep::FwEffectRemoved(item_uid)
                | UndoStep::ImplantRemoved(item_uid)
                | UndoStep::ProjEffectRemoved(item_uid)
                | UndoStep::RigRemoved(item_uid)
                | UndoStep::ServiceRemoved(item_uid)
                | UndoStep::ShipRemoved(item_uid)
                | UndoStep::SkillRemoved(item_uid)
                | UndoStep::StanceRemoved(item_uid)
                | UndoStep::SubsystemRemoved(item_uid)
                | UndoStep::SwEffectRemoved(item_uid) => {
                    u_data.items.evict(item_uid);
                }
                _ => (),
            }
        }
        // Autocharges replaced by fighter type changes
        for autocharges in self.autocharges.drain(..) {
            for autocharge_uid in autocharges.values() {
                u_data.items.evict(autocharge_uid);
            }
        }
        self.dps_profiles.clear();
        self.physics.clear();
        self.mutation_attrs.clear();
        self.recording = false;
    }
    pub(crate) fn is_recording(&self) -> bool {
        self.recording
    }
    // Record-pop pairs
    pub(crate) fn record(&mut self, step: UndoStep) {
        self.steps.push(step);
    }
    pub(crate) fn pop(&mut self, savepoint: &UndoSavepoint) -> Option<UndoStep> {
        match self.steps.len() > savepoint.len {
            true => self.steps.pop(),
            false => None,
        }
    }
    pub(crate) fn record_dps_profile(&mut self, step: UndoStep, dps_profile: Option<DpsProfile>) {
        self.steps.push(step);
        self.dps_profiles.push(dps_profile);
    }
    pub(crate) fn pop_dps_profile(&mut self) -> Option<DpsProfile> {
        self.dps_profiles.pop().unwrap()
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
            // Changes are recorded only within transactions and calls which roll them back
            if self.recording
                || !self.steps.is_empty()
                || !self.dps_profiles.is_empty()
                || !self.autocharges.is_empty()
                || !self.physics.is_empty()
                || !self.mutation_attrs.is_empty()
            {
                return Err(Default::default());
            }
            Ok(())
        }
    }
}
