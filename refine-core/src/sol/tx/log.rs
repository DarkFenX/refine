use super::{savepoint::Savepoint, step::UndoStep};
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
    mutation_attr: Vec<UAttrMutationRequest>,
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
            mutation_attr: Vec::new(),
            mutation_attrs: Vec::new(),
        }
    }
    /// Starts recording, returning the point changes can be rolled back to.
    pub(crate) fn savepoint(
        &mut self,
        fleet_id_counter: FleetId,
        fit_id_counter: FitId,
        item_id_counter: ItemId,
    ) -> Savepoint {
        let savepoint = Savepoint {
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
    pub(crate) fn close(&mut self, savepoint: Savepoint) {
        self.recording = savepoint.was_recording;
    }
    /// Keeps all the recorded changes: stops recording, clears the log, and evicts entities which
    /// were unmapped while recording.
    pub(crate) fn commit(&mut self, u_data: &mut UData) {
        self.steps.clear();
        u_data.fleets.evict_detached();
        u_data.fits.evict_detached();
        // Includes children unmapped with their parents, and autocharges replaced by fighter type
        // changes
        u_data.items.evict_detached();
        self.dps_profiles.clear();
        self.autocharges.clear();
        self.physics.clear();
        self.mutation_attr.clear();
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
    pub(crate) fn pop(&mut self, savepoint: &Savepoint) -> Option<UndoStep> {
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
    pub(crate) fn record_mutation_attr(&mut self, step: UndoStep, attr_mutation: UAttrMutationRequest) {
        self.steps.push(step);
        self.mutation_attr.push(attr_mutation);
    }
    pub(crate) fn pop_mutation_attr(&mut self) -> UAttrMutationRequest {
        self.mutation_attr.pop().unwrap()
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
                || !self.mutation_attr.is_empty()
                || !self.mutation_attrs.is_empty()
            {
                return Err(Default::default());
            }
            Ok(())
        }
    }
}
