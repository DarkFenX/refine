use std::hash::Hash;

use bitflags::bitflags;

use super::{savepoint::Savepoint, step::UndoStep};
use crate::{
    DpsProfile, FitId, FleetId, ItemId,
    ud::{ItemMutationData, UAttrMutationRequest, UAutocharges, UData, UEntityContainer, UPhysics},
    util::{DenseId, LibDefault, LibGetId, LibIncrement, LibNamed},
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
    mutations: Vec<ItemMutationData>,
    mutation_attr_singles: Vec<UAttrMutationRequest>,
    mutation_attr_batches: Vec<Vec<UAttrMutationRequest>>,
    // Side stacks of data which needs dropping recorded to since last commit; stacks which are not
    // marked are empty, other stacks are emptied on every commit
    touched: SideStacks,
}
impl UndoLog {
    pub(in crate::sol) fn new() -> Self {
        Self {
            recording: false,
            steps: Vec::new(),
            dps_profiles: Vec::new(),
            autocharges: Vec::new(),
            physics: Vec::new(),
            mutations: Vec::new(),
            mutation_attr_singles: Vec::new(),
            mutation_attr_batches: Vec::new(),
            touched: SideStacks::empty(),
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
        self.recording = false;
        // Stacks of plain data are emptied by setting their length
        self.dps_profiles.clear();
        self.physics.clear();
        self.mutation_attr_singles.clear();
        if u_data.fleets.has_detached()
            || u_data.fits.has_detached()
            || u_data.items.has_detached()
            || !self.touched.is_empty()
        {
            self.commit_drop(u_data);
        }
    }
    // Not inlined: with eviction and drops inlined, commit saves and restores all callee-saved
    // registers on every call
    #[inline(never)]
    fn commit_drop(&mut self, u_data: &mut UData) {
        u_data.fleets.evict_detached();
        u_data.fits.evict_detached();
        // Includes sub-items unmapped with their parents, and autocharges replaced by fighter type
        // changes
        u_data.items.evict_detached();
        if self.touched.contains(SideStacks::AUTOCHARGES) {
            self.autocharges.clear();
        }
        if self.touched.contains(SideStacks::MUTATIONS) {
            self.mutations.clear();
        }
        if self.touched.contains(SideStacks::MUTATION_ATTR_BATCHES) {
            self.mutation_attr_batches.clear();
        }
        self.touched = SideStacks::empty();
    }
    pub(crate) fn is_recording(&self) -> bool {
        self.recording
    }
    /// Removes an entity; while recording, unmaps it instead and records the step which puts it
    /// back.
    pub(crate) fn discard<T, ExtId, IntId, Err, S>(
        &mut self,
        container: &mut UEntityContainer<T, ExtId, IntId, Err>,
        int_id: IntId,
        step: S,
    ) where
        T: LibGetId<ExtId> + LibNamed,
        IntId: DenseId,
        ExtId: Copy + Eq + Hash + LibDefault + LibIncrement,
        Err: From<ExtId>,
        S: FnOnce(IntId) -> UndoStep,
    {
        match self.recording {
            true => {
                container.detach(int_id);
                self.steps.push(step(int_id));
            }
            false => {
                container.remove(int_id);
            }
        }
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
        self.touched.insert(SideStacks::AUTOCHARGES);
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
    pub(crate) fn record_mutation(&mut self, step: UndoStep, item_mutation_data: ItemMutationData) {
        self.steps.push(step);
        self.mutations.push(item_mutation_data);
        self.touched.insert(SideStacks::MUTATIONS);
    }
    pub(crate) fn pop_mutation(&mut self) -> ItemMutationData {
        self.mutations.pop().unwrap()
    }
    pub(crate) fn record_mutation_attr_single(&mut self, step: UndoStep, attr_mutation: UAttrMutationRequest) {
        self.steps.push(step);
        self.mutation_attr_singles.push(attr_mutation);
    }
    pub(crate) fn pop_mutation_attr_single(&mut self) -> UAttrMutationRequest {
        self.mutation_attr_singles.pop().unwrap()
    }
    pub(crate) fn record_mutation_attr_batch(&mut self, step: UndoStep, attr_mutations: Vec<UAttrMutationRequest>) {
        self.steps.push(step);
        self.mutation_attr_batches.push(attr_mutations);
        self.touched.insert(SideStacks::MUTATION_ATTR_BATCHES);
    }
    pub(crate) fn pop_mutation_attr_batch(&mut self) -> Vec<UAttrMutationRequest> {
        self.mutation_attr_batches.pop().unwrap()
    }
}

bitflags! {
    #[derive(Clone, Copy)]
    struct SideStacks: u8 {
        const AUTOCHARGES = 1 << 0;
        const MUTATIONS = 1 << 1;
        const MUTATION_ATTR_BATCHES = 1 << 2;
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
                || !self.mutations.is_empty()
                || !self.mutation_attr_singles.is_empty()
                || !self.mutation_attr_batches.is_empty()
            {
                return Err(Default::default());
            }
            Ok(())
        }
    }
}
