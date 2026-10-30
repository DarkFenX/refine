use crate::{
    ItemId,
    api::{MinionState, ModuleState},
    sol::SolarSystem,
};

/// Data needed to revert a single change done to a solar system.
#[expect(clippy::enum_variant_names)]
pub(in crate::api) enum UndoStep {
    AutochargeState(ItemId, bool),
    ChargeState(ItemId, bool),
    DroneState(ItemId, MinionState),
    FighterState(ItemId, MinionState),
    ModuleState(ItemId, ModuleState),
}

impl SolarSystem {
    pub(in crate::api) fn internal_undo(&mut self, step: UndoStep) {
        match step {
            UndoStep::AutochargeState(autocharge_id, state) => {
                let autocharge_uid = self.u_data.items.int_id_by_ext_id(&autocharge_id).unwrap();
                self.internal_set_autocharge_state(autocharge_uid, state);
            }
            UndoStep::ChargeState(charge_id, state) => {
                let charge_uid = self.u_data.items.int_id_by_ext_id(&charge_id).unwrap();
                self.internal_set_charge_state(charge_uid, state);
            }
            UndoStep::DroneState(drone_id, state) => {
                let drone_uid = self.u_data.items.int_id_by_ext_id(&drone_id).unwrap();
                self.internal_set_drone_state(drone_uid, state);
            }
            UndoStep::FighterState(fighter_id, state) => {
                let fighter_uid = self.u_data.items.int_id_by_ext_id(&fighter_id).unwrap();
                self.internal_set_fighter_state(fighter_uid, state);
            }
            UndoStep::ModuleState(module_id, state) => {
                let module_uid = self.u_data.items.int_id_by_ext_id(&module_id).unwrap();
                self.internal_set_module_state(module_uid, state);
            }
        }
    }
    pub(in crate::api) fn internal_undo_steps<I>(&mut self, steps: I)
    where
        I: IntoIterator<Item = UndoStep, IntoIter: DoubleEndedIterator>,
    {
        for step in steps.into_iter().rev() {
            self.internal_undo(step);
        }
    }
}
