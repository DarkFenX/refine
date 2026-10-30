use crate::sol::{SolarSystem, UndoSavepoint, UndoStep};

impl SolarSystem {
    pub(in crate::api) fn internal_savepoint(&mut self) -> UndoSavepoint {
        self.undo.savepoint(self.u_data.items.get_id_counter())
    }
    pub(in crate::api) fn internal_rollback(&mut self, savepoint: UndoSavepoint) {
        // Steps are undone by the same methods which record them, and we do not want to record it
        self.undo.pause();
        while let Some(step) = self.undo.pop(&savepoint) {
            match step {
                UndoStep::AutochargeState(autocharge_uid, state) => {
                    self.internal_set_autocharge_state(autocharge_uid, state);
                }
                UndoStep::ChargeAdded(charge_uid) => {
                    self.internal_remove_charge(charge_uid);
                }
                UndoStep::ChargeRemoved(charge_uid) => {
                    self.internal_reattach_charge(charge_uid);
                }
                UndoStep::ChargeState(charge_uid, state) => {
                    self.internal_set_charge_state(charge_uid, state);
                }
                UndoStep::DroneState(drone_uid, state) => {
                    self.internal_set_drone_state(drone_uid, state);
                }
                UndoStep::FighterState(fighter_uid, state) => {
                    self.internal_set_fighter_state(fighter_uid, state);
                }
                UndoStep::ModuleState(module_uid, state) => {
                    self.internal_set_module_state(module_uid, state);
                }
            }
        }
        self.u_data.items.set_id_counter(savepoint.item_id_counter);
        self.undo.close(savepoint);
    }
}
