use super::{savepoint::UndoSavepoint, step::UndoStep};
use crate::{
    DpsProfile, FitId, FleetId, ItemId,
    ud::{UAttrMutationRequest, UAutocharges, UData, UPhysics},
};

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
