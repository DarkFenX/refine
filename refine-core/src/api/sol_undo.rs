use crate::{
    sol::{SolarSystem, UndoSavepoint, UndoStep},
    ud::UItemId,
};

impl SolarSystem {
    pub(in crate::api) fn internal_savepoint(&mut self) -> UndoSavepoint {
        self.undo.savepoint(self.u_data.items.get_id_counter())
    }
    pub(in crate::api) fn internal_rollback(&mut self, savepoint: UndoSavepoint) {
        // Steps are undone by the same methods which record them, and we do not want to record it
        self.undo.pause();
        while let Some(step) = self.undo.pop(&savepoint) {
            match step {
                // Items
                UndoStep::AutochargeState(autocharge_uid, state) => {
                    self.internal_set_autocharge_state(autocharge_uid, state);
                }
                UndoStep::BoosterAdded(booster_uid) => self.internal_remove_booster(booster_uid),
                UndoStep::BoosterRemoved(booster_uid) => self.internal_reattach_booster(booster_uid),
                UndoStep::CharacterAdded(character_uid) => self.internal_remove_character(character_uid),
                UndoStep::CharacterRemoved(character_uid) => self.internal_reattach_character(character_uid),
                UndoStep::ChargeAdded(charge_uid) => self.internal_remove_charge(charge_uid),
                UndoStep::ChargeRemoved(charge_uid) => self.internal_reattach_charge(charge_uid),
                UndoStep::ChargeState(charge_uid, state) => {
                    self.internal_set_charge_state(charge_uid, state);
                }
                UndoStep::DroneAdded(drone_uid) => self.internal_remove_drone(drone_uid),
                UndoStep::DroneRemoved(drone_uid) => self.internal_reattach_drone(drone_uid),
                UndoStep::DroneState(drone_uid, state) => {
                    self.internal_set_drone_state(drone_uid, state);
                }
                UndoStep::FighterAdded(fighter_uid) => self.internal_remove_fighter(fighter_uid),
                UndoStep::FighterRemoved(fighter_uid) => self.internal_reattach_fighter(fighter_uid),
                UndoStep::FighterState(fighter_uid, state) => {
                    self.internal_set_fighter_state(fighter_uid, state);
                }
                UndoStep::FwEffectAdded(fw_effect_uid) => self.internal_remove_fw_effect(fw_effect_uid),
                UndoStep::FwEffectRemoved(fw_effect_uid) => self.internal_reattach_fw_effect(fw_effect_uid),
                UndoStep::ImplantAdded(implant_uid) => self.internal_remove_implant(implant_uid),
                UndoStep::ImplantRemoved(implant_uid) => self.internal_reattach_implant(implant_uid),
                UndoStep::ModuleAdded(module_uid, pos_mode) => self.internal_remove_module(module_uid, pos_mode),
                UndoStep::ModuleMoved(module_uid, pos_mode) => self.internal_move_module(module_uid, pos_mode),
                UndoStep::ModuleRemoved(module_uid, pos_mode) => self.internal_reattach_module(module_uid, pos_mode),
                UndoStep::ModuleState(module_uid, state) => {
                    self.internal_set_module_state(module_uid, state);
                }
                UndoStep::ProjEffectAdded(proj_effect_uid) => self.internal_remove_proj_effect(proj_effect_uid),
                UndoStep::ProjEffectRemoved(proj_effect_uid) => self.internal_reattach_proj_effect(proj_effect_uid),
                UndoStep::RigAdded(rig_uid) => self.internal_remove_rig(rig_uid),
                UndoStep::RigRemoved(rig_uid) => self.internal_reattach_rig(rig_uid),
                UndoStep::ServiceAdded(service_uid) => self.internal_remove_service(service_uid),
                UndoStep::ServiceRemoved(service_uid) => self.internal_reattach_service(service_uid),
                UndoStep::ShipAdded(ship_uid) => self.internal_remove_ship(ship_uid),
                UndoStep::ShipRemoved(ship_uid) => self.internal_reattach_ship(ship_uid),
                UndoStep::SkillAdded(skill_uid) => self.internal_remove_skill(skill_uid),
                UndoStep::SkillRemoved(skill_uid) => self.internal_reattach_skill(skill_uid),
                UndoStep::StanceAdded(stance_uid) => self.internal_remove_stance(stance_uid),
                UndoStep::StanceRemoved(stance_uid) => self.internal_reattach_stance(stance_uid),
                UndoStep::SubsystemAdded(subsystem_uid) => self.internal_remove_subsystem(subsystem_uid),
                UndoStep::SubsystemRemoved(subsystem_uid) => self.internal_reattach_subsystem(subsystem_uid),
                UndoStep::SwEffectAdded(sw_effect_uid) => self.internal_remove_sw_effect(sw_effect_uid),
                UndoStep::SwEffectRemoved(sw_effect_uid) => self.internal_reattach_sw_effect(sw_effect_uid),
                // Item projections
                UndoStep::ProjAdded(projector_uid, projectee_uid) => {
                    self.internal_remove_projection(projector_uid, projectee_uid).unwrap();
                }
                UndoStep::ProjRemoved(projector_uid, projectee_uid) => {
                    self.internal_add_projection(projector_uid, projectee_uid).unwrap();
                }
            }
        }
        self.u_data.items.set_id_counter(savepoint.item_id_counter);
        self.undo.close(savepoint);
    }
    /// Removes item from user data:
    /// - if recording, item is just unmapped;
    /// - if not recording, it is removed completely.
    pub(in crate::api) fn internal_discard_item<S>(&mut self, item_uid: UItemId, step: S)
    where
        S: FnOnce(UItemId) -> UndoStep,
    {
        match self.undo.is_recording() {
            true => {
                self.u_data.items.detach(item_uid);
                self.undo.record(step(item_uid));
            }
            false => {
                self.u_data.items.remove(item_uid);
            }
        }
    }
    /// Child undo data is carried with parent undo data, so children are supposed to be removed
    /// without separate record, via this method.
    pub(in crate::api) fn internal_discard_child_item(&mut self, item_uid: UItemId) {
        match self.undo.is_recording() {
            true => self.u_data.items.detach(item_uid),
            false => {
                self.u_data.items.remove(item_uid);
            }
        }
    }
}
