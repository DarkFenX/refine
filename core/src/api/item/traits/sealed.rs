use crate::{
    api::{MinionState, ModuleState},
    rd::RState,
    sol::SolarSystem,
    ud::{UEffectUpdates, UItem, UItemId},
};

pub(in crate::api) trait ItemSealed: Sized {
    fn get_sol(&self) -> &SolarSystem;
    fn get_uid(&self) -> UItemId;
}

pub(in crate::api) trait ItemMutSealed: ItemSealed {
    fn get_sol_mut(&mut self) -> &mut SolarSystem;
    // If some stat requested state to be ignored, this method checks their state, and if it's not
    // high enough, saves their old state and enables/activates them
    fn preprocess_for_active_stat(
        &mut self,
        include_charges: bool,
        ignore_state: bool,
        reuse_eupdates: &mut UEffectUpdates,
    ) -> Option<SavedItemState> {
        if !ignore_state {
            return None;
        }
        let item_uid = self.get_uid();
        let item = self.get_sol().u_data.items.get(item_uid);
        if !item.is_loaded() {
            return None;
        }
        // TODO: add autocharge+parent and charge+parent
        match item {
            // For drones, change state only if they are not in engaging state
            UItem::Drone(drone) => {
                let drone_state = drone.get_drone_state();
                match drone_state {
                    MinionState::InBay | MinionState::InSpace => {
                        self.get_sol_mut()
                            .internal_set_drone_state(item_uid, MinionState::Engaging, reuse_eupdates);
                        Some(SavedItemState::Drone(drone_state))
                    }
                    MinionState::Engaging => None,
                }
            }
            // For fighters, there are two parts:
            // - fighter state is switched up to engaging if fighter is in in-bay/in-space state
            // - autocharges are enabled if:
            //   - stats request specified that item's charges should be considered
            //   - autocharge is force-disabled
            UItem::Fighter(fighter) => {
                let fighter_state = fighter.get_fighter_state();
                let mut saved_autocharge_states = Vec::new();
                if include_charges {
                    for autocharge_uid in fighter.get_autocharges().values() {
                        let autocharge = self.get_sol().u_data.items.get(autocharge_uid).dc_autocharge().unwrap();
                        if autocharge.get_force_disabled() {
                            saved_autocharge_states.push(BoolItemInfo {
                                uid: autocharge_uid,
                                state: false,
                            });
                        }
                    }
                    for autocharge_info in saved_autocharge_states.iter() {
                        self.get_sol_mut()
                            .internal_set_autocharge_state(autocharge_info.uid, true, reuse_eupdates);
                    }
                }
                let saved_fighter_state = match fighter_state {
                    MinionState::InBay | MinionState::InSpace => {
                        self.get_sol_mut()
                            .internal_set_fighter_state(item_uid, MinionState::Engaging, reuse_eupdates);
                        Some(fighter_state)
                    }
                    MinionState::Engaging => None,
                };
                match saved_fighter_state.is_some() || !saved_autocharge_states.is_empty() {
                    true => Some(SavedItemState::Fighter(saved_fighter_state, saved_autocharge_states)),
                    false => None,
                }
            }
            // For modules, there are two parts:
            // - module state is switched up to active state if:
            //   - module is in online or lower state
            //   - module supports active or higher state
            // - charge is enabled if:
            //   - stats request specified that item's charges should be considered
            //   - module has charge, which is force-disabled
            UItem::Module(module) => {
                let charge_uid = module.get_charge_uid();
                let module_state = module.get_module_state();
                let saved_module_state = match module_state {
                    ModuleState::Disabled | ModuleState::Offline | ModuleState::Online => {
                        match module.get_max_state().unwrap() {
                            RState::Ghost | RState::Disabled | RState::Offline | RState::Online => None,
                            RState::Active | RState::Overload => {
                                self.get_sol_mut().internal_set_module_state(
                                    item_uid,
                                    ModuleState::Active,
                                    reuse_eupdates,
                                );
                                Some(module_state)
                            }
                        }
                    }
                    ModuleState::Active | ModuleState::Overload => None,
                };
                let saved_charge_state = match (charge_uid, include_charges) {
                    (Some(charge_uid), true) => {
                        let charge = self.get_sol().u_data.items.get(charge_uid).dc_charge().unwrap();
                        let charge_state = !charge.get_force_disabled();
                        match charge_state {
                            true => None,
                            false => {
                                self.get_sol_mut()
                                    .internal_set_charge_state(item_uid, true, reuse_eupdates);
                                Some(BoolItemInfo {
                                    uid: charge_uid,
                                    state: charge_state,
                                })
                            }
                        }
                    }
                    _ => None,
                };
                match saved_module_state.is_some() || saved_charge_state.is_some() {
                    true => Some(SavedItemState::Module(saved_module_state, saved_charge_state)),
                    false => None,
                }
            }
            _ => None,
        }
    }
    fn postprocess_for_active_stat(
        &mut self,
        saved_state: Option<SavedItemState>,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let saved_state = match saved_state {
            Some(saved_state) => saved_state,
            None => return,
        };
        let item_uid = self.get_uid();
        match saved_state {
            SavedItemState::Drone(drone_state) => {
                self.get_sol_mut()
                    .internal_set_drone_state(item_uid, drone_state, reuse_eupdates);
            }
            SavedItemState::Fighter(fighter_state, autocharge_infos) => {
                let sol = self.get_sol_mut();
                if let Some(fighter_state) = fighter_state {
                    sol.internal_set_fighter_state(item_uid, fighter_state, reuse_eupdates);
                }
                for autocharge_info in autocharge_infos.into_iter() {
                    sol.internal_set_autocharge_state(autocharge_info.uid, autocharge_info.state, reuse_eupdates);
                }
            }
            SavedItemState::Module(module_state, charge_info) => {
                let sol = self.get_sol_mut();
                if let Some(module_state) = module_state {
                    sol.internal_set_module_state(item_uid, module_state, reuse_eupdates);
                }
                if let Some(charge_info) = charge_info {
                    sol.internal_set_charge_state(charge_info.uid, charge_info.state, reuse_eupdates);
                }
            }
        }
    }
}

pub(in crate::api) enum SavedItemState {
    Drone(MinionState),
    Fighter(Option<MinionState>, Vec<BoolItemInfo>),
    Module(Option<ModuleState>, Option<BoolItemInfo>),
}

pub(in crate::api) struct BoolItemInfo {
    uid: UItemId,
    state: bool,
}
