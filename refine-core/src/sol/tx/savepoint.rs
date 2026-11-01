use crate::{FitId, FleetId, ItemId};

/// Point in undo log which changes can be rolled back to.
#[must_use]
pub(crate) struct Savepoint {
    pub(super) len: usize,
    pub(super) was_recording: bool,
    // Rollback removes everything added after the savepoint, so ID allocation can resume from here
    pub(crate) fleet_id_counter: FleetId,
    pub(crate) fit_id_counter: FitId,
    pub(crate) item_id_counter: ItemId,
}
