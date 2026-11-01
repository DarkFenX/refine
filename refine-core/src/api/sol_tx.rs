use crate::{
    sol::{SolTx, SolarSystem, UndoSavepoint, UndoStep},
    ud::UItemMutationRequest,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolarSystem {
    pub fn begin_tx(&mut self) -> Result<SolTx<'_>, TxBeginError> {
        if self.undo.is_recording() {
            return Err(TxBeginError::InTransaction);
        }
        let savepoint = self.internal_tx_savepoint();
        Ok(SolTx::new(self, savepoint))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum TxBeginError {
    #[error("transaction cannot be started within a transaction")]
    InTransaction,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolarSystem {
    pub(in crate::api) fn internal_tx_savepoint(&mut self) -> UndoSavepoint {
        self.undo.savepoint(
            self.u_data.fleets.get_id_counter(),
            self.u_data.fits.get_id_counter(),
            self.u_data.items.get_id_counter(),
        )
    }
    pub(crate) fn internal_tx_commit(&mut self) {
        self.undo.commit(&mut self.u_data);
    }
    pub(crate) fn internal_tx_rollback(&mut self, savepoint: UndoSavepoint) {
        // Steps are undone by the same methods which record them, and we do not want to record it
        self.undo.pause();
        while let Some(step) = self.undo.pop(&savepoint) {
            match step {
                // Solar system
                UndoStep::SecZone(sec_zone) => self.set_sec_zone(sec_zone),
                UndoStep::DefaultNpcProp(npc_prop) => self.set_default_npc_prop(npc_prop),
                UndoStep::DefaultOptionalReloads(optional_reloads) => {
                    self.set_default_optional_reloads(optional_reloads);
                }
                UndoStep::DefaultRearmMinions(rearm_minions) => self.set_default_rearm_minions(rearm_minions),
                UndoStep::DefaultSpool(spool) => self.set_default_spool(spool),
                UndoStep::DefaultIncomingDps => {
                    let dps_profile = self.undo.pop_dps_profile().unwrap();
                    self.set_default_incoming_dps(dps_profile);
                }
                // Fleet
                UndoStep::FleetAdded(fleet_uid) => self.internal_remove_fleet(fleet_uid),
                UndoStep::FleetRemoved(fleet_uid) => self.u_data.fleets.reattach(fleet_uid),
                // Fit
                UndoStep::FitAdded(fit_uid) => self.internal_remove_fit(fit_uid),
                UndoStep::FitRemoved(fit_uid) => self.internal_reattach_fit(fit_uid),
                UndoStep::FitFleet(fit_uid, fleet_uid) => match fleet_uid {
                    Some(fleet_uid) => self.internal_set_fit_fleet(fit_uid, fleet_uid),
                    None => self.internal_unset_fit_fleet(fit_uid).unwrap(),
                },
                UndoStep::FitRahIncomingDps(fit_uid) => match self.undo.pop_dps_profile() {
                    Some(dps_profile) => self.internal_set_fit_rah_incoming_dps(fit_uid, dps_profile),
                    None => self.internal_remove_fit_rah_incoming_dps(fit_uid).unwrap(),
                },
                UndoStep::FitSecStatus(fit_uid, sec_status) => self.internal_set_fit_sec_status(fit_uid, sec_status),
                // Item - general
                UndoStep::ItemEffectMode(item_uid, effect_aid, effect_mode) => {
                    self.internal_set_effect_id_mode(item_uid, effect_aid, effect_mode);
                }
                UndoStep::ProjAdded(projector_uid, projectee_uid) => {
                    self.internal_remove_projection(projector_uid, projectee_uid).unwrap();
                }
                UndoStep::ProjRemoved(projector_uid, projectee_uid) => {
                    self.internal_add_projection(projector_uid, projectee_uid).unwrap();
                }
                // Item - autocharge
                UndoStep::AutochargeState(autocharge_uid, state) => {
                    self.internal_set_autocharge_state(autocharge_uid, state);
                }
                // Item - booster
                UndoStep::BoosterAdded(booster_uid) => self.internal_remove_booster(booster_uid),
                UndoStep::BoosterRemoved(booster_uid) => self.internal_reattach_booster(booster_uid),
                UndoStep::BoosterState(booster_uid, state) => self.internal_set_booster_state(booster_uid, state),
                UndoStep::BoosterType(booster_uid, type_aid) => {
                    self.internal_set_booster_type_aid(booster_uid, type_aid);
                }
                // Item - character
                UndoStep::CharacterAdded(character_uid) => self.internal_remove_character(character_uid),
                UndoStep::CharacterRemoved(character_uid) => self.internal_reattach_character(character_uid),
                UndoStep::CharacterState(character_uid, state) => {
                    self.internal_set_character_state(character_uid, state);
                }
                UndoStep::CharacterType(character_uid, type_aid) => {
                    self.internal_set_character_type_aid(character_uid, type_aid);
                }
                // Item - charge
                UndoStep::ChargeAdded(charge_uid) => self.internal_remove_charge(charge_uid),
                UndoStep::ChargeRemoved(charge_uid) => self.internal_reattach_charge(charge_uid),
                UndoStep::ChargeState(charge_uid, state) => {
                    self.internal_set_charge_state(charge_uid, state);
                }
                UndoStep::ChargeType(charge_uid, type_aid) => {
                    self.internal_set_charge_type_aid(charge_uid, type_aid);
                }
                // Item - drone
                UndoStep::DroneAdded(drone_uid) => self.internal_remove_drone(drone_uid),
                UndoStep::DroneRemoved(drone_uid) => self.internal_reattach_drone(drone_uid),
                UndoStep::DroneMutationAdded(drone_uid) => {
                    self.internal_remove_drone_mutation(drone_uid).unwrap();
                }
                UndoStep::DroneMutationAttrs(drone_uid) => {
                    let attr_mutations = self.undo.pop_mutation_attrs();
                    self.internal_change_drone_mutation_attrs(drone_uid, attr_mutations)
                        .unwrap();
                }
                UndoStep::DroneMutationRemoved(drone_uid, mutator_type_aid) => {
                    let attrs = self.undo.pop_mutation_attrs();
                    let mutation = UItemMutationRequest {
                        mutator_type_aid,
                        attrs,
                    };
                    self.internal_add_drone_mutation(drone_uid, mutation).unwrap();
                }
                UndoStep::DroneMutator(drone_uid, mutator_type_aid) => {
                    self.internal_set_drone_mutator_type_aid(drone_uid, mutator_type_aid)
                        .unwrap();
                }
                UndoStep::DroneNpcPropOverride(drone_uid, npc_prop_override) => {
                    self.internal_set_drone_npc_prop_override(drone_uid, npc_prop_override);
                }
                UndoStep::DronePhysics(drone_uid) => {
                    let physics = self.undo.pop_physics();
                    self.internal_restore_drone_physics(drone_uid, physics);
                }
                UndoStep::DroneState(drone_uid, state) => {
                    self.internal_set_drone_state(drone_uid, state);
                }
                UndoStep::DroneType(drone_uid, type_aid) => {
                    self.internal_set_drone_type_aid(drone_uid, type_aid);
                }
                // Item - fighter
                UndoStep::FighterAdded(fighter_uid) => self.internal_remove_fighter(fighter_uid),
                UndoStep::FighterRemoved(fighter_uid) => self.internal_reattach_fighter(fighter_uid),
                UndoStep::FighterCountOverride(fighter_uid, count_override) => {
                    self.internal_set_fighter_count_override(fighter_uid, count_override);
                }
                UndoStep::FighterPhysics(fighter_uid) => {
                    let physics = self.undo.pop_physics();
                    self.internal_restore_fighter_physics(fighter_uid, physics);
                }
                UndoStep::FighterRearmMinionOverride(fighter_uid, rearm_minion_override) => {
                    self.internal_set_fighter_rearm_minion_override(fighter_uid, rearm_minion_override);
                }
                UndoStep::FighterState(fighter_uid, state) => {
                    self.internal_set_fighter_state(fighter_uid, state);
                }
                UndoStep::FighterType(fighter_uid, type_aid) => {
                    let autocharges = self.undo.pop_autocharges();
                    self.internal_restore_fighter_type_aid(fighter_uid, type_aid, autocharges);
                }
                // Item - fit-wide effect
                UndoStep::FwEffectAdded(fw_effect_uid) => self.internal_remove_fw_effect(fw_effect_uid),
                UndoStep::FwEffectRemoved(fw_effect_uid) => self.internal_reattach_fw_effect(fw_effect_uid),
                UndoStep::FwEffectState(fw_effect_uid, state) => {
                    self.internal_set_fw_effect_state(fw_effect_uid, state);
                }
                UndoStep::FwEffectType(fw_effect_uid, type_aid) => {
                    self.internal_set_fw_effect_type_aid(fw_effect_uid, type_aid);
                }
                // Item - implant
                UndoStep::ImplantAdded(implant_uid) => self.internal_remove_implant(implant_uid),
                UndoStep::ImplantRemoved(implant_uid) => self.internal_reattach_implant(implant_uid),
                UndoStep::ImplantState(implant_uid, state) => self.internal_set_implant_state(implant_uid, state),
                UndoStep::ImplantType(implant_uid, type_aid) => {
                    self.internal_set_implant_type_aid(implant_uid, type_aid);
                }
                // Item - module
                UndoStep::ModuleAdded(module_uid, pos_mode) => self.internal_remove_module(module_uid, pos_mode),
                UndoStep::ModuleRemoved(module_uid, pos_mode) => self.internal_reattach_module(module_uid, pos_mode),
                UndoStep::ModuleMutationAdded(module_uid) => {
                    self.internal_remove_module_mutation(module_uid).unwrap();
                }
                UndoStep::ModuleMutationAttrs(module_uid) => {
                    let attr_mutations = self.undo.pop_mutation_attrs();
                    self.internal_change_module_mutation_attrs(module_uid, attr_mutations)
                        .unwrap();
                }
                UndoStep::ModuleMutationRemoved(module_uid, mutator_type_aid) => {
                    let attrs = self.undo.pop_mutation_attrs();
                    let mutation = UItemMutationRequest {
                        mutator_type_aid,
                        attrs,
                    };
                    self.internal_add_module_mutation(module_uid, mutation).unwrap();
                }
                UndoStep::ModuleMutator(module_uid, mutator_type_aid) => {
                    self.internal_set_module_mutator_aid(module_uid, mutator_type_aid)
                        .unwrap();
                }
                UndoStep::ModuleMoved(module_uid, pos_mode) => self.internal_move_module(module_uid, pos_mode),
                UndoStep::ModuleOptionalReloadOverride(module_uid, optional_reload_override) => {
                    self.internal_set_module_optional_reload_override(module_uid, optional_reload_override);
                }
                UndoStep::ModuleSpoolOverride(module_uid, spool_override) => {
                    self.internal_set_module_spool_override(module_uid, spool_override);
                }
                UndoStep::ModuleState(module_uid, state) => {
                    self.internal_set_module_state(module_uid, state);
                }
                UndoStep::ModuleType(module_uid, type_aid) => {
                    self.internal_set_module_type_aid(module_uid, type_aid);
                }
                // Item - projected effect
                UndoStep::ProjEffectAdded(proj_effect_uid) => self.internal_remove_proj_effect(proj_effect_uid),
                UndoStep::ProjEffectRemoved(proj_effect_uid) => self.internal_reattach_proj_effect(proj_effect_uid),
                UndoStep::ProjEffectState(proj_effect_uid, state) => {
                    self.internal_set_proj_effect_state(proj_effect_uid, state);
                }
                UndoStep::ProjEffectType(proj_effect_uid, type_aid) => {
                    self.internal_set_proj_effect_type_aid(proj_effect_uid, type_aid);
                }
                // Item - rig
                UndoStep::RigAdded(rig_uid) => self.internal_remove_rig(rig_uid),
                UndoStep::RigRemoved(rig_uid) => self.internal_reattach_rig(rig_uid),
                UndoStep::RigState(rig_uid, state) => self.internal_set_rig_state(rig_uid, state),
                UndoStep::RigType(rig_uid, type_aid) => {
                    self.internal_set_rig_type_aid(rig_uid, type_aid);
                }
                // Item - service
                UndoStep::ServiceAdded(service_uid) => self.internal_remove_service(service_uid),
                UndoStep::ServiceRemoved(service_uid) => self.internal_reattach_service(service_uid),
                UndoStep::ServiceState(service_uid, state) => self.internal_set_service_state(service_uid, state),
                UndoStep::ServiceType(service_uid, type_aid) => {
                    self.internal_set_service_type_aid(service_uid, type_aid);
                }
                // Item - ship
                UndoStep::ShipAdded(ship_uid) => self.internal_remove_ship(ship_uid),
                UndoStep::ShipRemoved(ship_uid) => self.internal_reattach_ship(ship_uid),
                UndoStep::ShipPhysics(ship_uid) => {
                    let physics = self.undo.pop_physics();
                    self.internal_restore_ship_physics(ship_uid, physics);
                }
                UndoStep::ShipState(ship_uid, state) => self.internal_set_ship_state(ship_uid, state),
                UndoStep::ShipType(ship_uid, type_aid) => {
                    self.internal_set_ship_type_aid(ship_uid, type_aid);
                }
                // Item - skill
                UndoStep::SkillAdded(skill_uid) => self.internal_remove_skill(skill_uid),
                UndoStep::SkillRemoved(skill_uid) => self.internal_reattach_skill(skill_uid),
                UndoStep::SkillLevel(skill_uid, level) => self.internal_set_skill_level(skill_uid, level),
                UndoStep::SkillState(skill_uid, state) => self.internal_set_skill_state(skill_uid, state),
                UndoStep::SkillType(skill_uid, type_aid) => {
                    self.internal_set_skill_type_aid(skill_uid, type_aid).unwrap();
                }
                // Item - stance
                UndoStep::StanceAdded(stance_uid) => self.internal_remove_stance(stance_uid),
                UndoStep::StanceRemoved(stance_uid) => self.internal_reattach_stance(stance_uid),
                UndoStep::StanceState(stance_uid, state) => self.internal_set_stance_state(stance_uid, state),
                UndoStep::StanceType(stance_uid, type_aid) => {
                    self.internal_set_stance_type_aid(stance_uid, type_aid);
                }
                // Item - subsystem
                UndoStep::SubsystemAdded(subsystem_uid) => self.internal_remove_subsystem(subsystem_uid),
                UndoStep::SubsystemRemoved(subsystem_uid) => self.internal_reattach_subsystem(subsystem_uid),
                UndoStep::SubsystemState(subsystem_uid, state) => {
                    self.internal_set_subsystem_state(subsystem_uid, state);
                }
                UndoStep::SubsystemType(subsystem_uid, type_aid) => {
                    self.internal_set_subsystem_type_aid(subsystem_uid, type_aid);
                }
                // Item - system-wide effect
                UndoStep::SwEffectAdded(sw_effect_uid) => self.internal_remove_sw_effect(sw_effect_uid),
                UndoStep::SwEffectRemoved(sw_effect_uid) => self.internal_reattach_sw_effect(sw_effect_uid),
                UndoStep::SwEffectState(sw_effect_uid, state) => {
                    self.internal_set_sw_effect_state(sw_effect_uid, state);
                }
                UndoStep::SwEffectType(sw_effect_uid, type_aid) => {
                    self.internal_set_sw_effect_type_aid(sw_effect_uid, type_aid);
                }
            }
        }
        self.u_data.items.set_id_counter(savepoint.item_id_counter);
        self.u_data.fits.set_id_counter(savepoint.fit_id_counter);
        self.u_data.fleets.set_id_counter(savepoint.fleet_id_counter);
        self.undo.close(savepoint);
    }
}
